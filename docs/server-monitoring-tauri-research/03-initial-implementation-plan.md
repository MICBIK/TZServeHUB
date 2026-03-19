# Initial Implementation Plan for a Tauri Server Monitoring App

Checked on: 2026-03-18
Audited on: 2026-03-19

## Product Target

Build a local Tauri desktop app that monitors one or more remote servers with:

- CPU usage and per-core usage
- Network traffic
- Network fluctuation
- Disk read/write speed
- Disk total / used / free space
- Historical views and basic alerts

The app should explicitly treat these as three different domains:

- `Host metrics`: CPU, memory, disk I/O, filesystem space, interface throughput
- `Network quality`: ping latency, TCP reachability, DNS resolution, packet loss
- `Traffic analytics`: interface ranking, bursts, top interfaces, and future flow analysis

## Recommended Product Shape

Recommendation: `desktop control plane + adapter-based remote collection`.

Reason:

- A desktop app is a good display surface, local cache, notification center, and offline-first operator console.
- A desktop app is a poor universal collector for remote servers if it depends only on SSH scraping.
- The open-source projects with the best reliability all separate metric collection from the UI.
- The projects with the best network visibility also separate `host metrics` from `active probes` or `traffic inspection`.

## MVP Architecture

```text
Remote Server
  |- node_exporter or Glances API
  |- optional probe target
  |- optional future lightweight agent
  |- private network or authenticated gateway

Local Tauri App
  |- Rust core
  |- UI shell
  |- polling scheduler with jitter/backoff
  |- adapter layer
  |- probe worker
  |- derived-metric calculator
  |- local SQLite store
  |- alert engine
  |- tray / desktop notifications

Flow
  1. Pull host metrics from exporter or agent
  2. Run active probes for network quality
  3. Normalize all inputs to one internal schema
  4. Derive rates, utilization, and anomaly candidates
  5. Persist recent history locally
  6. Render dashboards and trigger alerts
```

## Recommended Technical Choices

- Desktop shell: `Tauri 2`
- UI: `React + TypeScript`
- State and polling: interval polling in v1, with per-source jitter and timeout control
- Local storage: `SQLite` with WAL mode and rollup tables
- Charting: choose a time-series friendly chart library optimized for dense data and zooming
- Remote metric source for v1: `node_exporter` first, `Glances` as optional adapter
- Probe subsystem: local `Ping`, `TCP`, and `DNS` checks inspired by Uptime Kuma
- Future traffic expansion: keep a reserved adapter slot for `ntopng` or a self-owned network collector
- Secure connectivity: `WireGuard / Tailscale / VPN / SSH tunnel / authenticated HTTPS gateway`

## Why This Path Is Better Than Direct SSH Polling

- Exporters give you stable metric formats and lower parsing cost.
- SSH collection is brittle across Linux distributions and package layouts.
- Exporters let you support many hosts without turning the desktop app into a shell-script manager.
- You can still add SSH later for setup, diagnostics, or one-click installer flows.

## Secure Connectivity Baseline

- Do not make "open raw exporter port to the public internet" the default onboarding path.
- Prefer private reachability through `WireGuard`, `Tailscale`, `VPN`, or an `SSH tunnel`.
- If you need HTTPS and auth in front of exporter endpoints, put them behind an authenticated gateway or use exporter-side TLS/auth tooling where supported.
- Store, per server, how trust is established: `private overlay`, `tunnel`, or `authenticated gateway`.

## Probe Vantage Model

- A probe launched from the Tauri app measures the path from `this desktop` to the target.
- That is useful for operator-facing reachability, but it does not prove the server's own network health.
- Label every probe result with a `vantage_point` field such as `desktop-local` or `remote-agent`.
- If server-origin network quality becomes important, add a remote probe worker later instead of overloading desktop probes with the wrong semantics.

## Non-Goals for v1

- No attempt to replace Prometheus, Netdata, or Checkmk as a full observability backend
- No packet capture or deep flow inspection in v1
- No custom remote agent before the exporter-based path is proven
- No cross-team RBAC or multi-user collaboration model in v1

## Suggested Internal Modules

### 1. Server Registry

- Stores server identity, tags, polling interval, credentials, and metric source type
- Supports `node_exporter`, `Glances`, and reserved slot for a future self-owned agent

### 2. Metric Adapter Layer

- Converts different upstream formats into one internal schema
- Example normalized groups:
  - `cpu.total`
  - `cpu.core.{index}`
  - `network.if.{name}.rx_bytes`
  - `network.if.{name}.tx_bytes`
  - `disk.dev.{name}.read_bytes_per_sec`
  - `disk.dev.{name}.write_bytes_per_sec`
  - `disk.fs.{mount}.total_bytes`
  - `disk.fs.{mount}.used_bytes`
  - `disk.fs.{mount}.free_bytes`
  - `probe.ping.latency_ms`
  - `probe.tcp.connect_ms`
  - `probe.dns.lookup_ms`

### 3. Metric Semantics Guardrails

- Track whether each metric is a `counter`, `gauge`, `state`, or `derived rate`.
- Handle counter resets caused by host reboot, exporter restart, interface reset, or device rename.
- Persist source timestamps and staleness markers so charts can distinguish `zero` from `missing`.
- Keep network interfaces, disks, and filesystems separately keyed because their identities drift differently across reboots.

### 4. Local Time-Series Store

- Keep recent high-frequency samples locally with tiered retention
- Suggested first target:
  - 7 days of raw points at 5 to 10 second resolution
  - 30 days of 1-minute rollups
  - 90 days of 15-minute rollups

### 5. Derived Metrics Engine

- Compute rates from counters, utilization percentages, moving averages, and simple burst detection
- Keep this logic local so different adapters can feed the same charts

### 6. Alert Engine

- Threshold alerts for CPU, disk saturation, disk free space, packet loss, and host unreachable
- Cooldown and de-duplication are required in v1

### 7. Probe Worker

- Active checks for `Ping`, `TCP`, and `DNS`
- This is the cleanest way to define "network fluctuation" as a product feature

### 8. Security and Onboarding

- Store local secrets in OS-backed secure storage when available
- Prefer token or basic-auth adapters over shell credentials
- Keep SSH only for optional diagnostics or future installer helpers

### 9. Operational Safeguards

- Use bounded concurrency for scrapes so one slow host does not stall the whole polling loop.
- Batch writes to SQLite and run retention/compaction jobs in the background.
- Keep retry logic conservative; failing fast with staleness marking is often better than piling up retries.
- Separate raw samples from rollups so retention and alert evaluation stay predictable.

## Recommended MVP Screens

- Server list with health summary
- Server overview dashboard
- CPU page with total and per-core charts
- Network page with interface traffic and latency history
- Disk page with device I/O and filesystem usage
- Probe page with ping, TCP, and DNS health history
- Alerts and event log
- Settings page for server onboarding and polling intervals

## Delivery Phases

### Phase 1: Metrics MVP

- Support one Linux server via `node_exporter`
- Store and display CPU, per-core CPU, network traffic, disk I/O, and disk space
- Implement normalized schema, local SQLite history, and rate calculations
- Require private reachability or tunnel-based access for the exporter endpoint
- Label probes as `desktop-local` from day one
- No complex auth model yet; focus on data path correctness

### Phase 2: Usability MVP

- Add multiple server support
- Add alerts, tray notifications, and better history views
- Add simple probe checks for ping, TCP, and DNS

### Phase 3: Adapter Expansion

- Add `Glances` adapter
- Add onboarding helpers and connection diagnostics
- Add per-interface and per-disk filtering
- Add traffic-oriented summaries inspired by `ntopng`

### Phase 4: Self-Owned Agent Decision

- Only after v1 usage is clear
- Build your own agent if exporter compatibility blocks product goals

## Key Risks

- Defining "network fluctuation" too vaguely will blur host metrics and active probing into one feature
- Polling too frequently without downsampling will bloat local storage
- Cross-platform host collection is expensive if you write your own agent too early
- Dense time-series charts will feel slow if the data model is not normalized from the start
- Mixing raw counters and derived rates in one table will make charting and alert logic harder than necessary
- Treating desktop-side probes as server-side truth will produce misleading network conclusions
- Exposing exporter ports publicly for convenience creates an avoidable security weakness

## Recommended Next Build Step

- Start with one reference slice: `Tauri desktop app + node_exporter-backed Linux host + local SQLite history + ping probe`.
- Lock the normalized metric schema before adding more adapters.
- Keep the first deployment on a private overlay network or tunnel, not a raw public exporter port.
- Treat `Glances`, `ntopng`, and a future self-owned agent as later adapters, not day-one dependencies.
