use crate::models::server::{AccessMethod, AdapterType, AuthType, ServerConfig};
use crate::probes::{DnsProbe, PingProbe, TcpProbe};
use sqlx::{Row, SqlitePool};
use std::{
    collections::{HashMap, HashSet},
    net::{IpAddr, SocketAddr},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::time::{interval, sleep, Duration};

const PROBE_INTERVAL_SECS: u64 = 60;
const PING_COUNT: u16 = 4;
const TCP_TIMEOUT_MS: u64 = 5000;
const DNS_TIMEOUT_MS: u64 = 5000;

pub async fn start(pool: SqlitePool) {
    tokio::spawn(async move {
        let mut probe_tasks: HashMap<String, tokio::task::JoinHandle<()>> = HashMap::new();
        loop {
            match get_enabled_servers(&pool).await {
                Ok(servers) => reconcile_probe_tasks(&pool, servers, &mut probe_tasks),
                Err(e) => log::error!("Probe scheduler: failed to query servers: {e}"),
            }
            sleep(Duration::from_secs(15)).await;
        }
    });
}

fn reconcile_probe_tasks(
    pool: &SqlitePool,
    servers: Vec<ServerConfig>,
    tasks: &mut HashMap<String, tokio::task::JoinHandle<()>>,
) {
    let current_ids: HashSet<String> = servers.iter().map(|s| s.id.clone()).collect();
    tasks.retain(|server_id, handle| {
        if current_ids.contains(server_id) {
            true
        } else {
            handle.abort();
            false
        }
    });
    for server in servers {
        if !tasks.contains_key(&server.id) {
            let pool_clone = pool.clone();
            let server_id = server.id.clone();
            tasks.insert(
                server_id,
                tokio::spawn(async move {
                    probe_server_loop(pool_clone, server).await;
                }),
            );
        }
    }
}

fn is_ip_address(host: &str) -> bool {
    host.parse::<IpAddr>().is_ok()
}

async fn resolve_host(host: &str) -> Option<IpAddr> {
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Some(ip);
    }
    // Use tokio DNS resolution for hostnames
    match tokio::net::lookup_host(format!("{}:0", host)).await {
        Ok(mut addrs) => addrs.next().map(|addr| addr.ip()),
        Err(_) => None,
    }
}

async fn probe_server_loop(pool: SqlitePool, server: ServerConfig) {
    let mut ticker = interval(Duration::from_secs(PROBE_INTERVAL_SECS));
    loop {
        ticker.tick().await;
        run_probes(&pool, &server).await;
    }
}

async fn run_probes(pool: &SqlitePool, server: &ServerConfig) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let host = &server.host;
    let is_ip = is_ip_address(host);

    // Run ping probe
    if let Some(ip) = resolve_host(host).await {
        match PingProbe::new() {
            Ok(ping) => match ping.ping(ip, PING_COUNT).await {
                Ok(result) => {
                    let success = result.loss_rate < 100.0;
                    if let Err(e) = store_probe_result(
                        pool,
                        &server.id,
                        "ping",
                        host,
                        success,
                        Some(result.avg_rtt_ms),
                        Some(result.loss_rate),
                        None,
                        now,
                    )
                    .await
                    {
                        log::error!("Failed to store ping result for {}: {e}", server.name);
                    }
                }
                Err(e) => {
                    if let Err(store_err) = store_probe_result(
                        pool,
                        &server.id,
                        "ping",
                        host,
                        false,
                        None,
                        None,
                        Some(&e.to_string()),
                        now,
                    )
                    .await
                    {
                        log::error!("Failed to store ping error for {}: {store_err}", server.name);
                    }
                }
            },
            Err(e) => {
                log::error!("Failed to create PingProbe for {}: {e}", server.name);
            }
        }
    } else {
        if let Err(e) = store_probe_result(
            pool,
            &server.id,
            "ping",
            host,
            false,
            None,
            None,
            Some("Failed to resolve host"),
            now,
        )
        .await
        {
            log::error!("Failed to store ping resolve error for {}: {e}", server.name);
        }
    }

    // Run TCP probe
    let tcp_target = format!("{}:{}", host, server.port);
    if let Some(ip) = resolve_host(host).await {
        let addr = SocketAddr::new(ip, server.port);
        let tcp = TcpProbe::new();
        match tcp.check(addr, TCP_TIMEOUT_MS).await {
            Ok(result) => {
                if let Err(e) = store_probe_result(
                    pool,
                    &server.id,
                    "tcp",
                    &tcp_target,
                    result.reachable,
                    Some(result.latency_ms),
                    None,
                    if result.reachable {
                        None
                    } else {
                        Some("Connection failed or timed out")
                    },
                    now,
                )
                .await
                {
                    log::error!("Failed to store TCP result for {}: {e}", server.name);
                }
            }
            Err(e) => {
                if let Err(store_err) = store_probe_result(
                    pool,
                    &server.id,
                    "tcp",
                    &tcp_target,
                    false,
                    None,
                    None,
                    Some(&e.to_string()),
                    now,
                )
                .await
                {
                    log::error!("Failed to store TCP error for {}: {store_err}", server.name);
                }
            }
        }
    } else {
        if let Err(e) = store_probe_result(
            pool,
            &server.id,
            "tcp",
            &tcp_target,
            false,
            None,
            None,
            Some("Failed to resolve host"),
            now,
        )
        .await
        {
            log::error!(
                "Failed to store TCP resolve error for {}: {e}",
                server.name
            );
        }
    }

    // Run DNS probe only for hostnames (not IPs)
    if !is_ip {
        let dns = DnsProbe::new();
        // Use Google's public DNS as the resolver
        let dns_server: IpAddr = "8.8.8.8".parse().unwrap();
        match dns.resolve(host, dns_server, DNS_TIMEOUT_MS).await {
            Ok(result) => {
                if let Err(e) = store_probe_result(
                    pool,
                    &server.id,
                    "dns",
                    host,
                    result.resolved,
                    Some(result.latency_ms),
                    None,
                    if result.resolved {
                        None
                    } else {
                        Some("DNS resolution failed")
                    },
                    now,
                )
                .await
                {
                    log::error!("Failed to store DNS result for {}: {e}", server.name);
                }
            }
            Err(e) => {
                if let Err(store_err) = store_probe_result(
                    pool,
                    &server.id,
                    "dns",
                    host,
                    false,
                    None,
                    None,
                    Some(&e.to_string()),
                    now,
                )
                .await
                {
                    log::error!("Failed to store DNS error for {}: {store_err}", server.name);
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn store_probe_result(
    pool: &SqlitePool,
    server_id: &str,
    probe_type: &str,
    target: &str,
    success: bool,
    latency_ms: Option<f64>,
    loss_rate: Option<f64>,
    error_message: Option<&str>,
    timestamp: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO probe_results (server_id, probe_type, target, success, latency_ms, loss_rate, error_message, timestamp)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(server_id)
    .bind(probe_type)
    .bind(target)
    .bind(success as i32)
    .bind(latency_ms)
    .bind(loss_rate)
    .bind(error_message)
    .bind(timestamp)
    .execute(pool)
    .await?;
    Ok(())
}

fn row_to_server(row: &sqlx::sqlite::SqliteRow) -> ServerConfig {
    ServerConfig {
        id: row.get("id"),
        name: row.get("name"),
        host: row.get("host"),
        port: row.get::<i64, _>("port") as u16,
        adapter_type: AdapterType::from_db(&row.get::<String, _>("adapter_type")),
        access_method: AccessMethod::from_db(&row.get::<String, _>("access_method")),
        polling_interval_sec: row.get::<i64, _>("polling_interval_sec") as u32,
        enabled: row.get::<i64, _>("enabled") != 0,
        auth_token: row.get("auth_token"),
        auth_type: AuthType::from_db(&row.get::<String, _>("auth_type")),
        ssh_key_path: row.get("ssh_key_path"),
        ssh_passphrase: row.get("ssh_passphrase"),
        password: row.get("password"),
        status: row.get("status"),
        last_seen_at: row.get("last_seen_at"),
        last_error: row.get("last_error"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

async fn get_enabled_servers(pool: &SqlitePool) -> Result<Vec<ServerConfig>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, name, host, port, adapter_type, access_method, polling_interval_sec, enabled, auth_token, auth_type, ssh_key_path, ssh_passphrase, password, status, last_seen_at, last_error, created_at, updated_at FROM servers WHERE enabled = 1",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.iter().map(row_to_server).collect())
}
