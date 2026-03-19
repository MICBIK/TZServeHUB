# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ServerHUB is a **Tauri 2 desktop application** that serves as a control plane for monitoring **remote server clusters**. This is NOT a local system monitor—it's a centralized dashboard for monitoring multiple remote servers.

**Core Architecture**: Desktop control plane (Tauri/Rust + React) + Remote agents (Go) + Adapter layer (supports node_exporter, Glances, custom Go agent)

**Three-Domain Separation**:
- **Host Metrics**: CPU (total/per-core), memory, disk I/O, disk space
- **Network Quality**: Ping latency, TCP connectivity, DNS resolution, packet loss
- **Traffic Analytics**: Network interface traffic, interface ranking, burst detection

## Development Commands

### Frontend (React + Vite)
```bash
pnpm install              # Install dependencies
pnpm dev                  # Start Vite dev server (port 1420)
pnpm build                # Build frontend for production
pnpm lint                 # Run ESLint
```

### Tauri Desktop App
```bash
pnpm tauri dev            # Start Tauri in development mode (runs pnpm dev + cargo run)
pnpm tauri build          # Build production desktop app
```

### Rust Backend (src-tauri/)
```bash
cd src-tauri
cargo check               # Check compilation without building
cargo build               # Build debug binary
cargo build --release     # Build release binary
cargo test                # Run tests
```

### Go Remote Agent (agent/)
```bash
cd agent
go mod tidy               # Update dependencies
go build -o serverhub-agent cmd/serverhub-agent/main.go
./serverhub-agent -config config.yaml
```

## gstack Skills

This project includes [gstack](https://github.com/garrytan/gstack) - a virtual engineering team system for Claude Code.

**Available Skills**:
- `/office-hours` - Product strategy consultation, reframes problems before coding
- `/plan-ceo-review` - CEO-level plan review, challenges scope and priorities
- `/plan-eng-review` - Engineering plan review with architecture diagrams
- `/plan-design-review` - Design-focused plan review
- `/design-consultation` - Design feedback and UI/UX guidance
- `/review` - Code review with auto-fixes and issue detection
- `/ship` - Release preparation and PR creation
- `/browse` - Web browsing (use this instead of mcp__claude-in-chrome__* tools)
- `/qa` - QA testing with real browser automation
- `/qa-only` - QA-only mode without fixes
- `/design-review` - Design review for UI/UX changes
- `/setup-browser-cookies` - Configure browser cookies for authenticated testing
- `/retro` - Developer retrospective and statistics
- `/investigate` - Debug and investigate issues
- `/document-release` - Generate release documentation
- `/codex` - Code analysis and documentation
- `/careful` - Enable careful mode for sensitive operations
- `/freeze` - Freeze current state for rollback
- `/guard` - Enable guard mode for extra safety
- `/unfreeze` - Unfreeze frozen state
- `/gstack-upgrade` - Upgrade gstack to latest version

**Important**: Always use `/browse` from gstack for web browsing. Never use `mcp__claude-in-chrome__*` tools.

**Troubleshooting**: If gstack skills aren't working, run `cd .claude/skills/gstack && ./setup` to rebuild the binary and register skills.

## Architecture Deep Dive

### Adapter Pattern (Core Abstraction)

The `MetricAdapter` trait (`src-tauri/src/adapters/traits.rs`) is the foundation for supporting multiple data sources:

```rust
pub trait MetricAdapter: Send + Sync {
    fn name(&self) -> &str;
    async fn fetch_host_metrics(&self, server: &ServerConfig) -> AppResult<Vec<RawMetric>>;
    async fn health_check(&self, server: &ServerConfig) -> AppResult<bool>;
}
```

**Implementations**:
- `NodeExporterAdapter`: Parses Prometheus text format from node_exporter
- `GoAgentAdapter`: Consumes JSON from the custom Go agent
- Future: `GlancesAdapter` for Glances API

### Data Flow

```
Remote Server (Go Agent)
    → HTTP/JSON
    → Adapter Layer (Rust)
    → Normalized RawMetric
    → DerivedMetricsEngine (counter→rate)
    → SQLite (WAL mode)
    → Frontend (React/Zustand)
```

### Storage Strategy (Tiered Retention)

**Three-tier retention policy** (`src-tauri/src/storage/retention.rs`):
1. **Raw metrics**: 7 days, full resolution (5-10s intervals)
2. **1-minute rollup**: 30 days, aggregated (min/max/avg)
3. **15-minute rollup**: 90 days, aggregated (min/max/avg)

**Database**: SQLite with WAL mode for better concurrency
- Schema: `src-tauri/migrations/001_init.sql`
- Tables: `servers`, `raw_metrics`, `metrics_1m`, `metrics_15m`, `alert_rules`, `alert_events`

### Counter → Rate Derivation

The `DerivedMetricsEngine` (`src-tauri/src/metrics/derived.rs`) converts cumulative counters (e.g., `network_transmit_bytes_total`) into rates (bytes/sec):
- Tracks previous counter values and timestamps
- Handles counter resets (when value decreases)
- Cleans up stale state for inactive metrics

### Probe System (Network Quality)

Three probe types (`src-tauri/src/probes/`):
- **Ping**: ICMP using `surge-ping` crate
- **TCP**: Connection latency using `tokio::net::TcpStream`
- **DNS**: Resolution timing using UDP sockets

Each probe supports **vantage point tagging** (desktop-local vs remote-agent) to distinguish where measurements originate.

## Module Organization

### Rust Backend (src-tauri/src/)

```
commands/       # Tauri IPC handlers (exposed to frontend via invoke)
  ├── server.rs      # list_servers, add_server, remove_server
  ├── metrics.rs     # get_metrics, get_metric_history
  └── settings.rs    # get_settings, update_settings

adapters/       # Data source adapters (implements MetricAdapter trait)
  ├── traits.rs      # MetricAdapter trait definition
  ├── node_exporter.rs
  └── go_agent.rs

probes/         # Network quality probes
  ├── ping.rs        # ICMP ping
  ├── tcp.rs         # TCP connectivity
  └── dns.rs         # DNS resolution

metrics/        # Metrics processing
  ├── derived.rs     # Counter→rate conversion
  └── rollup.rs      # Data aggregation (1m, 15m)

storage/        # SQLite management
  ├── database.rs    # Connection pool, WAL mode
  └── retention.rs   # Cleanup old data

scheduler/      # Polling scheduler
  └── poller.rs      # Per-server polling with jitter/backoff

alerts/         # Alert engine
  ├── rules.rs       # Rule evaluation (threshold + duration)
  └── notifier.rs    # Desktop notifications

models/         # Data models
  ├── server.rs      # ServerConfig, AdapterType, AccessMethod
  ├── metric.rs      # RawMetric, MetricType, AggregatedMetric
  └── alert.rs       # AlertRule, AlertEvent, AlertStatus
```

### React Frontend (src/)

```
pages/          # Route components (Dashboard, CPU, Network, Disk, Probe, Alert, Settings)
components/     # Reusable UI components
  ├── layout/        # AppLayout, Sidebar
  ├── charts/        # LineChart (Recharts wrapper)
  ├── server/        # ServerCard
  └── common/        # MetricCard

stores/         # Zustand state management
  ├── serverStore.ts    # Server CRUD + activeServerId
  ├── metricsStore.ts   # Metrics data keyed by server_id
  └── settingsStore.ts  # App settings

services/       # Tauri API wrappers
  └── tauri.ts       # invoke() wrappers for all commands

types/          # TypeScript definitions
  ├── server.ts      # ServerConfig, ServerFormData
  ├── metric.ts      # MetricPoint, AggregatedMetric, HostMetrics
  └── alert.ts       # AlertRule, AlertEvent

hooks/          # Custom React hooks
  └── usePolling.ts  # Polling hook with interval and cleanup

lib/            # Utilities
  ├── constants.ts   # METRIC_COLORS, POLLING_INTERVALS
  └── formatters.ts  # formatBytes, formatBytesPerSec, formatPercent
```

### Go Remote Agent (agent/)

```
cmd/serverhub-agent/main.go    # Entry point
internal/
  ├── config/config.go         # YAML config loader
  ├── server/server.go         # Gin HTTP server + auth middleware
  └── collector/collector.go   # gopsutil metrics collection
```

**API Endpoints**:
- `GET /api/health` - Health check
- `GET /api/metrics` - Get all metrics (requires Bearer token)

## Key Concepts

### ServerConfig Fields
- `adapter_type`: `NodeExporter` | `Glances` | `GoAgent`
- `access_method`: `Private` | `Tunnel` | `Gateway` (for security context)
- `polling_interval_sec`: Per-server polling frequency
- `enabled`: Boolean flag to pause monitoring

### MetricType Enum
- `Counter`: Monotonically increasing (e.g., bytes transmitted)
- `Gauge`: Point-in-time value (e.g., CPU usage %)
- `State`: Discrete state (e.g., service up/down)

### Tauri IPC Pattern
Frontend calls Rust via `invoke()`:
```typescript
import { invoke } from '@tauri-apps/api/core';
const servers = await invoke<ServerConfig[]>('list_servers');
```

Rust handlers are registered in `src-tauri/src/lib.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    commands::server::list_servers,
    // ...
])
```

### Alert Rule Evaluation
- **Condition**: `Gt` (greater than) | `Lt` (less than) | `Eq` (equal)
- **Duration**: Alert fires only after threshold breached for N seconds
- **Cooldown**: Minimum time between repeated alerts (prevents flapping)

## Important Files

- `src-tauri/tauri.conf.json`: Tauri config (window size 1400x900, dev port 1420)
- `src-tauri/migrations/001_init.sql`: Database schema
- `vite.config.ts`: Vite config (port 1420, host 0.0.0.0 for mobile)
- `docs/architecture.md`: Comprehensive architecture documentation (18.8KB)
- `agent/config.example.yaml`: Go agent configuration template

## Development Notes

### Adding a New Tauri Command
1. Define handler in `src-tauri/src/commands/*.rs`
2. Register in `src-tauri/src/lib.rs` invoke_handler
3. Add TypeScript wrapper in `src/services/tauri.ts`
4. Call from React components via the service wrapper

### Adding a New Adapter
1. Implement `MetricAdapter` trait in `src-tauri/src/adapters/`
2. Add variant to `AdapterType` enum in `models/server.rs`
3. Update adapter instantiation logic in scheduler/poller

### Database Migrations
- Migrations live in `src-tauri/migrations/`
- Use `sqlx::migrate!()` macro to run migrations on startup
- WAL mode is enabled automatically in `storage/database.rs`

### Frontend State Management
- Use Zustand stores for global state (servers, metrics, settings)
- Use React Query / `usePolling` hook for periodic data fetching
- Tauri commands are async—always handle loading/error states

## Security Context

This is a **monitoring control plane**, not a public-facing service:
- Remote agents should run on **private networks** or behind **VPN/SSH tunnels**
- Go agent uses **Bearer token authentication** (configured in `config.yaml`)
- No raw public exporter ports—always use secure access methods
- Desktop app stores server credentials locally in SQLite (consider encryption for production)
