//! IPC commands for known SSH hosts.
//!
//! Stub for v0.2 SDD RED phase (HOST-005, HOST-006).

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::deployer::known_hosts::KnownHost;
use crate::error::AppResult;

/// List all known SSH hosts sorted by last_seen DESC.
pub async fn list_known_hosts(_pool: &sqlx::SqlitePool) -> AppResult<Vec<KnownHost>> {
    todo!("HOST-005: list known hosts")
}

/// Remove one known host entry.
pub async fn remove_known_host(
    _pool: &sqlx::SqlitePool,
    _host: &str,
    _port: u16,
) -> AppResult<()> {
    todo!("HOST-006: remove known host")
}

#[cfg(test)]
mod tests {
    /// HOST-005: list_known_hosts sorts records by last_seen DESC.
    #[tokio::test]
    async fn list_known_hosts_returns_records_sorted_by_last_seen() {
        panic!("HOST-005 RED: list_known_hosts not implemented");
    }

    /// HOST-006: remove_known_host deletes the row.
    #[tokio::test]
    async fn remove_known_host_deletes_record() {
        panic!("HOST-006 RED: remove_known_host not implemented");
    }
}
