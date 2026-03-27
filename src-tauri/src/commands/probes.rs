use crate::error::{AppError, AppResult};
use crate::probes::{DnsProbe, PingProbe, TcpProbe};
use serde::Serialize;
use sqlx::{Row, SqlitePool};
use std::net::{IpAddr, SocketAddr};
use tauri::State;

#[derive(Debug, Serialize)]
pub struct PingProbeResult {
    pub avg_rtt_ms: f64,
    pub loss_rate: f64,
    pub packets_sent: u16,
    pub packets_lost: u16,
}

#[derive(Debug, Serialize)]
pub struct TcpProbeResult {
    pub reachable: bool,
    pub latency_ms: f64,
}

#[derive(Debug, Serialize)]
pub struct DnsProbeResult {
    pub resolved: bool,
    pub latency_ms: f64,
}

#[tauri::command]
pub async fn run_ping_probe(
    host: String,
    count: Option<u32>,
) -> Result<PingProbeResult, String> {
    let result: AppResult<PingProbeResult> = async {
        if host.trim().is_empty() {
            return Err(AppError::Custom("Host cannot be empty".to_string()));
        }

        let addr: IpAddr = host
            .trim()
            .parse()
            .map_err(|_| AppError::Custom(format!("Invalid IP address: {}", host)))?;

        let count = count.unwrap_or(4).min(20) as u16;
        let probe = PingProbe::new()?;
        let result = probe.ping(addr, count).await?;

        Ok(PingProbeResult {
            avg_rtt_ms: result.avg_rtt_ms,
            loss_rate: result.loss_rate,
            packets_sent: result.packets_sent,
            packets_lost: result.packets_lost,
        })
    }
    .await;

    result.map_err(|e| e.to_user_message())
}

#[tauri::command]
pub async fn run_tcp_probe(
    host: String,
    port: u16,
    timeout_ms: Option<u64>,
) -> Result<TcpProbeResult, String> {
    let result: AppResult<TcpProbeResult> = async {
        if host.trim().is_empty() {
            return Err(AppError::Custom("Host cannot be empty".to_string()));
        }
        if port == 0 {
            return Err(AppError::Custom("Port must be greater than 0".to_string()));
        }

        let addr: IpAddr = host
            .trim()
            .parse()
            .map_err(|_| AppError::Custom(format!("Invalid IP address: {}", host)))?;

        let socket_addr = SocketAddr::new(addr, port);
        let timeout = timeout_ms.unwrap_or(5000);
        let probe = TcpProbe::new();
        let result = probe.check(socket_addr, timeout).await?;

        Ok(TcpProbeResult {
            reachable: result.reachable,
            latency_ms: result.latency_ms,
        })
    }
    .await;

    result.map_err(|e| e.to_user_message())
}

#[tauri::command]
pub async fn run_dns_probe(
    domain: String,
    dns_server: Option<String>,
    timeout_ms: Option<u64>,
) -> Result<DnsProbeResult, String> {
    let result: AppResult<DnsProbeResult> = async {
        if domain.trim().is_empty() {
            return Err(AppError::Custom("Domain cannot be empty".to_string()));
        }

        let server_addr: IpAddr = dns_server
            .as_deref()
            .unwrap_or("8.8.8.8")
            .parse()
            .map_err(|_| AppError::Custom("Invalid DNS server address".to_string()))?;

        let timeout = timeout_ms.unwrap_or(5000);
        let probe = DnsProbe::new();
        let result = probe.resolve(domain.trim(), server_addr, timeout).await?;

        Ok(DnsProbeResult {
            resolved: result.resolved,
            latency_ms: result.latency_ms,
        })
    }
    .await;

    result.map_err(|e| e.to_user_message())
}

// Probe history commands (from probe_scheduler persistence)

#[derive(Debug, Clone, Serialize)]
pub struct ProbeResultRow {
    pub id: i64,
    pub server_id: String,
    pub probe_type: String,
    pub target: String,
    pub success: bool,
    pub latency_ms: Option<f64>,
    pub loss_rate: Option<f64>,
    pub error_message: Option<String>,
    pub timestamp: i64,
}

#[tauri::command]
pub async fn get_probe_history(
    pool: State<'_, SqlitePool>,
    server_id: String,
    probe_type: String,
    from: i64,
    to: i64,
) -> Result<Vec<ProbeResultRow>, String> {
    let rows = sqlx::query(
        "SELECT id, server_id, probe_type, target, success, latency_ms, loss_rate, error_message, timestamp
         FROM probe_results
         WHERE server_id = ? AND probe_type = ? AND timestamp >= ? AND timestamp <= ?
         ORDER BY timestamp ASC",
    )
    .bind(&server_id)
    .bind(&probe_type)
    .bind(from)
    .bind(to)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| format!("Failed to query probe history: {e}"))?;

    Ok(rows
        .iter()
        .map(|row| ProbeResultRow {
            id: row.get("id"),
            server_id: row.get("server_id"),
            probe_type: row.get("probe_type"),
            target: row.get("target"),
            success: row.get::<i32, _>("success") != 0,
            latency_ms: row.get("latency_ms"),
            loss_rate: row.get("loss_rate"),
            error_message: row.get("error_message"),
            timestamp: row.get("timestamp"),
        })
        .collect())
}

#[tauri::command]
pub async fn get_latest_probe_results(
    pool: State<'_, SqlitePool>,
    server_id: String,
) -> Result<Vec<ProbeResultRow>, String> {
    let rows = sqlx::query(
        "SELECT p.id, p.server_id, p.probe_type, p.target, p.success, p.latency_ms, p.loss_rate, p.error_message, p.timestamp
         FROM probe_results p
         INNER JOIN (
             SELECT server_id, probe_type, MAX(timestamp) as max_ts
             FROM probe_results
             WHERE server_id = ?
             GROUP BY server_id, probe_type
         ) latest ON p.server_id = latest.server_id
             AND p.probe_type = latest.probe_type
             AND p.timestamp = latest.max_ts
         ORDER BY p.probe_type",
    )
    .bind(&server_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| format!("Failed to query latest probe results: {e}"))?;

    Ok(rows
        .iter()
        .map(|row| ProbeResultRow {
            id: row.get("id"),
            server_id: row.get("server_id"),
            probe_type: row.get("probe_type"),
            target: row.get("target"),
            success: row.get::<i32, _>("success") != 0,
            latency_ms: row.get("latency_ms"),
            loss_rate: row.get("loss_rate"),
            error_message: row.get("error_message"),
            timestamp: row.get("timestamp"),
        })
        .collect())
}
