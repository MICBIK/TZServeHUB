#![allow(dead_code)]

use crate::error::AppResult;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub struct TcpProbe;

impl TcpProbe {
    pub fn new() -> Self {
        Self
    }

    pub async fn check(&self, addr: SocketAddr, timeout_ms: u64) -> AppResult<TcpResult> {
        let start = std::time::Instant::now();

        let result = timeout(Duration::from_millis(timeout_ms), TcpStream::connect(addr)).await;

        let elapsed = start.elapsed().as_millis() as f64;

        match result {
            Ok(Ok(_)) => Ok(TcpResult {
                reachable: true,
                latency_ms: elapsed,
            }),
            _ => Ok(TcpResult {
                reachable: false,
                latency_ms: 0.0,
            }),
        }
    }
}

#[derive(Debug)]
pub struct TcpResult {
    pub reachable: bool,
    pub latency_ms: f64,
}
