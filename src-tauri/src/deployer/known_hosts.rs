//! SSH known_hosts persistence + fingerprint validation.
//!
//! Stub for v0.2 SDD RED phase (F-HOSTKEY).

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownHost {
    pub host: String,
    pub port: u16,
    pub fingerprint: String,
    pub algorithm: String,
    pub first_seen: i64,
    pub last_seen: i64,
}

/// Persistence + lookup for SSH host keys.
pub struct KnownHostsStore {
    pub pool: SqlitePool,
}

impl KnownHostsStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Find a known host record by (host, port).
    pub async fn find(&self, _host: &str, _port: u16) -> AppResult<Option<KnownHost>> {
        todo!("HOST-002/003: lookup known host")
    }

    /// Insert a new known host (TOFU). Returns the freshly-inserted record.
    pub async fn insert(
        &self,
        _host: &str,
        _port: u16,
        _fingerprint: &str,
        _algorithm: &str,
    ) -> AppResult<KnownHost> {
        todo!("HOST-002: TOFU insert")
    }

    /// Touch the last_seen column when a known host reconnects.
    pub async fn update_last_seen(&self, _host: &str, _port: u16) -> AppResult<()> {
        todo!("HOST-003: update last_seen")
    }

    /// Remove a known host record.
    pub async fn delete(&self, _host: &str, _port: u16) -> AppResult<()> {
        todo!("HOST-006: delete known host")
    }

    /// Verify a presented fingerprint against a stored record.
    /// - returns `Ok(true)` on TOFU insert or matching fingerprint
    /// - returns `Err(AppError)` on mismatch
    pub async fn verify_or_insert(
        &self,
        _host: &str,
        _port: u16,
        _fingerprint: &str,
        _algorithm: &str,
    ) -> AppResult<bool> {
        todo!("HOST-004: enforce fingerprint match")
    }
}

/// Compute the SHA-256 hex fingerprint of an SSH public key.
pub fn fingerprint_public_key(_key_bytes: &[u8]) -> String {
    todo!("HOST-007: compute sha256 hex")
}

#[cfg(test)]
mod tests {
    /// HOST-002: first connect inserts a new fingerprint row (TOFU).
    #[tokio::test]
    async fn tofu_inserts_fingerprint_on_first_seen() {
        panic!("HOST-002 RED: TOFU insert not implemented");
    }

    /// HOST-003: matching fingerprint on reconnect just bumps last_seen.
    #[tokio::test]
    async fn matching_fingerprint_updates_last_seen() {
        panic!("HOST-003 RED: last_seen update not implemented");
    }

    /// HOST-004: changed fingerprint must be rejected with an error.
    #[tokio::test]
    async fn mismatched_fingerprint_returns_error() {
        panic!("HOST-004 RED: mismatch enforcement not implemented");
    }

    /// HOST-007: stored fingerprint includes the algorithm name (e.g. ssh-ed25519).
    #[tokio::test]
    async fn fingerprint_includes_algorithm_field() {
        panic!("HOST-007 RED: algorithm field not stored");
    }
}
