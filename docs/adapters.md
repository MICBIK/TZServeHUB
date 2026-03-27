# Adapter Implementation Status

This document describes the current state of metric collection adapters in ServerHUB.

## Implemented Adapters

### NodeExporter

**Status**: Implemented
**Location**: `src-tauri/src/adapters/node_exporter.rs`

Collects metrics from Prometheus Node Exporter instances.

**Connection Requirements**:
- Protocol: HTTP
- Endpoint: `http://{host}:{port}/metrics`
- Authentication: None (Node Exporter typically runs without auth)

**Data Format**: Prometheus text format

**Parsing Strategy**:
- Parses Prometheus exposition format line-by-line
- Extracts metric names, labels (key-value pairs in `{}`), and values
- Automatically infers metric type based on naming conventions:
  - Counter: metrics ending with `_total`, `_bytes_total`, or `_count`
  - Gauge: all other metrics
- Supports label parsing with quoted string values and escape sequences

**Health Check**: GET `/metrics` endpoint, expects HTTP 2xx status

---

### GoAgent

**Status**: Implemented
**Location**: `src-tauri/src/adapters/go_agent.rs`

Collects metrics from the custom Go monitoring agent (`gstack/agent`).

**Connection Requirements**:
- Protocol: HTTP
- Metrics Endpoint: `http://{host}:{port}/api/metrics`
- Health Endpoint: `http://{host}:{port}/api/health`
- Authentication: Bearer token (optional)
  - Header: `Authorization: Bearer {token}`
  - Token configured via `ServerConfig.auth_token`

**Data Format**: JSON API response

**Response Schema**:
```json
{
  "timestamp": 1742371200,
  "cpu": {
    "total_percent": 25.0,
    "per_core": [20.0, 30.0]
  },
  "memory": {
    "total": 1024,
    "used": 512,
    "available": 512,
    "used_percent": 50.0
  },
  "disks": [
    {
      "mount": "/",
      "device": "/dev/vda1",
      "total": 1000,
      "used": 400,
      "free": 600,
      "used_percent": 40.0
    }
  ],
  "disk_io": [
    {
      "device": "vda",
      "read_bytes": 10,
      "write_bytes": 20
    }
  ],
  "network": [
    {
      "interface": "eth0",
      "rx_bytes": 30,
      "tx_bytes": 40
    }
  ]
}
```

**Metric Mapping**:
- CPU: `cpu_usage_percent` (total), `cpu_core_usage_percent` (per-core with `core` label)
- Memory: `memory_total_bytes`, `memory_used_bytes`, `memory_available_bytes`, `memory_used_percent`
- Disk: `disk_total_bytes`, `disk_used_bytes`, `disk_free_bytes` (with `device` and `mount` labels)
- Disk I/O: `disk_read_bytes_total`, `disk_write_bytes_total` (counters with `device` label)
- Network: `network_transmit_bytes_total`, `network_receive_bytes_total` (counters with `interface` label)

**Health Check**: GET `/api/health` endpoint, expects HTTP 2xx status

---

## Deferred Adapters

### Glances

**Status**: Deferred to future release
**Reason**: Prioritizing Node Exporter and custom Go agent for initial release

Glances is a cross-platform monitoring tool with a REST API. Integration planned for a future version to support environments already running Glances.

---

## Authentication Patterns

| Adapter | Auth Method | Configuration |
|---------|-------------|---------------|
| NodeExporter | None | No auth required |
| GoAgent | Bearer Token | `ServerConfig.auth_token` (optional) |
| Glances (future) | Basic Auth / API Key | TBD |

---

## Adding New Adapters

To implement a new adapter:

1. Create a new module in `src-tauri/src/adapters/`
2. Implement the `MetricAdapter` trait:
   - `fn name(&self) -> &str` - unique adapter identifier
   - `async fn fetch_host_metrics(&self, server: &ServerConfig) -> AppResult<Vec<RawMetric>>`
   - `async fn health_check(&self, server: &ServerConfig) -> AppResult<bool>`
3. Register the adapter in `src-tauri/src/adapters/mod.rs`
4. Add integration tests in the adapter module
5. Update this document with connection requirements and data format details
