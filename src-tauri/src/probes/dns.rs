use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};
use std::net::{IpAddr, SocketAddr};
use crate::error::AppResult;

pub struct DnsProbe;

impl DnsProbe {
    pub fn new() -> Self {
        Self
    }

    pub async fn resolve(&self, hostname: &str, dns_server: IpAddr, timeout_ms: u64) -> AppResult<DnsResult> {
        let start = std::time::Instant::now();

        let query = Self::build_dns_query(hostname);

        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        let dns_addr = SocketAddr::new(dns_server, 53);

        socket.send_to(&query, dns_addr).await?;

        let mut buf = [0u8; 512];
        let result = timeout(
            Duration::from_millis(timeout_ms),
            socket.recv_from(&mut buf)
        ).await;

        let elapsed = start.elapsed().as_millis() as f64;

        match result {
            Ok(Ok((len, _))) => {
                let resolved = len > 12;
                Ok(DnsResult {
                    resolved,
                    latency_ms: elapsed,
                })
            }
            _ => Ok(DnsResult {
                resolved: false,
                latency_ms: 0.0,
            }),
        }
    }

    fn build_dns_query(hostname: &str) -> Vec<u8> {
        let mut query = vec![
            0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        for part in hostname.split('.') {
            query.push(part.len() as u8);
            query.extend_from_slice(part.as_bytes());
        }
        query.push(0x00);
        query.extend_from_slice(&[0x00, 0x01, 0x00, 0x01]);

        query
    }
}

#[derive(Debug)]
pub struct DnsResult {
    pub resolved: bool,
    pub latency_ms: f64,
}
