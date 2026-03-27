# Metrics Retention Policy

ServerHUB implements a three-tier retention strategy to balance query performance, storage efficiency, and historical visibility.

## Retention Tiers

### 1. Raw Metrics (Full Resolution)
- **Retention Period**: 7 days
- **Resolution**: 5-10 second intervals (native polling frequency)
- **Storage Table**: `raw_metrics`
- **Use Case**: Real-time monitoring, detailed troubleshooting, short-term analysis

### 2. 1-Minute Rollup
- **Retention Period**: 30 days
- **Resolution**: 1-minute aggregated buckets
- **Storage Table**: `metrics_1m`
- **Aggregations**: min, max, avg per metric per bucket
- **Use Case**: Recent trend analysis, hourly/daily comparisons

### 3. 15-Minute Rollup
- **Retention Period**: 90 days
- **Resolution**: 15-minute aggregated buckets
- **Storage Table**: `metrics_15m`
- **Aggregations**: min, max, avg per metric per bucket
- **Use Case**: Long-term capacity planning, monthly reports, historical baselines

## Storage Estimates

Assuming:
- 10 servers monitored
- 50 metrics per server (CPU, memory, disk, network)
- 8 bytes per metric value + 100 bytes overhead per row

| Tier | Rows/Day | Retention | Total Rows | Estimated Size |
|------|----------|-----------|------------|----------------|
| Raw (10s) | 4,320,000 | 7 days | ~30M | ~3.2 GB |
| 1-min rollup | 720,000 | 30 days | ~21M | ~2.3 GB |
| 15-min rollup | 48,000 | 90 days | ~4.3M | ~465 MB |

**Total estimated storage**: ~6 GB for 10 servers over 90 days.

Storage scales linearly with server count and metric cardinality.

## Cleanup Schedule

The retention manager runs automatically on an **hourly schedule** (every 3600 seconds).

### Cleanup Process
1. Calculate cutoff timestamps for each tier based on current time
2. Delete expired rows from each table:
   - `raw_metrics WHERE timestamp < (now - 7 days)`
   - `metrics_1m WHERE bucket < (now - 30 days)`
   - `metrics_15m WHERE bucket < (now - 90 days)`
3. Log deletion counts for observability

### Manual Vacuum
After large deletions, run `VACUUM` to reclaim disk space:
```sql
VACUUM;
```

This is not automated by default to avoid blocking queries during peak hours.

## Implementation Reference

The retention logic is implemented in `src-tauri/src/storage/retention.rs`:

- **Constants**: `RAW_RETENTION_DAYS = 7`, `ROLLUP_1M_RETENTION_DAYS = 30`, `ROLLUP_15M_RETENTION_DAYS = 90`
- **Scheduler**: Spawned as a background tokio task with 1-hour interval
- **Cleanup Method**: `RetentionManager::cleanup_old_data()` executes DELETE queries per tier
- **Logging**: Logs row counts deleted per tier on each run

To adjust retention periods, modify the constants in `retention.rs` and rebuild the Tauri app.

## Query Optimization Tips

- **Recent data (< 7 days)**: Query `raw_metrics` for full resolution
- **Medium-term (7-30 days)**: Query `metrics_1m` for faster aggregation
- **Long-term (30-90 days)**: Query `metrics_15m` for historical trends
- Always add `WHERE timestamp >= ?` or `WHERE bucket >= ?` to leverage indexes
- Use `LIMIT` clauses to prevent large result sets in the frontend

## Future Considerations

- **Compression**: Enable SQLite page compression for older rollup tables
- **Archival**: Export metrics_15m data older than 90 days to cold storage (S3, Parquet)
- **Adaptive Retention**: Allow per-server retention overrides for critical hosts
- **Vacuum Automation**: Schedule weekly VACUUM during maintenance windows
