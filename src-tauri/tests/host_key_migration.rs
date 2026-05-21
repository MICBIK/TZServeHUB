//! HOST-001 — migration 008 must create the `known_hosts` table.
//!
//! Uses `SERVERHUB_DATA_DIR` to redirect SQLite to a tempdir, runs
//! database::init() (which applies migrations), then queries the new table.

use sqlx::Row;
use std::{env, fs};

/// HOST-001: known_hosts table created with the right shape.
#[tokio::test]
async fn migration_008_creates_known_hosts_table() {
    let dir = env::temp_dir().join(format!("serverhub-known-hosts-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&dir).expect("test data dir");
    env::set_var("SERVERHUB_DATA_DIR", &dir);

    let pool = serverhub_lib::storage::database::init()
        .await
        .expect("database init should succeed");

    // Table exists (count(*) does not error and returns 0 rows initially).
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM known_hosts")
        .fetch_one(&pool)
        .await
        .expect("known_hosts table must exist (migration 008)");
    assert_eq!(count, 0, "fresh DB should have zero known_hosts rows");

    // Schema sanity — required columns present.
    let row = sqlx::query("PRAGMA table_info(known_hosts)")
        .fetch_all(&pool)
        .await
        .expect("PRAGMA table_info should succeed");
    let cols: Vec<String> = row.iter().map(|r| r.get::<String, _>("name")).collect();
    for required in [
        "host",
        "port",
        "fingerprint",
        "algorithm",
        "first_seen",
        "last_seen",
    ] {
        assert!(
            cols.iter().any(|c| c == required),
            "known_hosts must have column {required} (got {cols:?})"
        );
    }

    fs::remove_dir_all(dir).ok();
}
