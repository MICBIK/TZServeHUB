#![allow(dead_code)]

use crate::error::AppResult;
use std::net::IpAddr;
use std::time::Duration;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};

pub struct PingProbe {
    client: Client,
}

impl PingProbe {
    pub fn new() -> AppResult<Self> {
        let config = Config::default();
        let client = Client::new(&config)?;
        Ok(Self { client })
    }

    pub async fn ping(&self, addr: IpAddr, count: u16) -> AppResult<PingResult> {
        let mut pinger = self
            .client
            .pinger(addr, PingIdentifier(rand::random()))
            .await;

        let mut rtts = Vec::new();
        let mut lost = 0;

        for seq in 0..count {
            match pinger.ping(PingSequence(seq), &[]).await {
                Ok((_, duration)) => {
                    rtts.push(duration.as_millis() as f64);
                }
                Err(_) => {
                    lost += 1;
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let avg_rtt = if !rtts.is_empty() {
            rtts.iter().sum::<f64>() / rtts.len() as f64
        } else {
            0.0
        };

        let loss_rate = (lost as f64 / count as f64) * 100.0;

        Ok(PingResult {
            avg_rtt_ms: avg_rtt,
            loss_rate,
            packets_sent: count,
            packets_lost: lost,
        })
    }
}

#[derive(Debug)]
pub struct PingResult {
    pub avg_rtt_ms: f64,
    pub loss_rate: f64,
    pub packets_sent: u16,
    pub packets_lost: u16,
}
