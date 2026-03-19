# Server Monitoring Tauri App Research Report

Checked on: 2026-03-18
Audited on: 2026-03-19

## Goal

Build a local Tauri desktop application for server status monitoring, with focus on:

- CPU usage
- Per-core usage
- Network fluctuation and traffic stats
- Disk read/write speed
- Disk total / used / free space

## Executive Summary

- The open-source market is still dominated by web or self-hosted dashboards. That is not accidental: remote collection, historical storage, alerting, and multi-machine management are much easier to solve with a hub or web control plane.
- Mature projects separate "data collection" from "data display". The collector is usually an agent or exporter; the UI is only one consumer of the metrics.
- Tauri references do exist, but the strong ones are mostly local system monitors, not remote server monitors. They are useful for desktop UX and Rust bridge design, not for the full server-monitoring product model.
- The fastest path for your product is: `Tauri desktop app + remote exporter/agent + local time-series cache`. The slow path is trying to make the desktop app itself become the collector for every server.
- The problem should be split into three modules inside one app: `host resource monitoring`, `network quality monitoring`, and `traffic analytics`. Open-source projects rarely do all three equally well.

## Project-Level Analysis

### 1. Beszel

- Positioning: lightweight server monitoring platform with historical data, alerts, Docker stats, API access, and a clear `hub + agent` architecture.
- Why it matters: it is the closest open-source reference to the product shape you want, except its primary UI is web.
- What to borrow: host metrics model, agent deployment pattern, historical data handling, alert thresholds, multi-server dashboard layout.
- What not to copy directly: PocketBase-centric web hub assumptions.
- Sources: [GitHub](https://github.com/henrygd/beszel), [Docs](https://www.beszel.dev/guide/what-is-beszel)

### 2. Netdata

- Positioning: real-time infrastructure monitoring with per-second collection, strong dashboards, alerts, long-term retention, and broad metric coverage.
- Why it matters: strongest benchmark for metric breadth, chart density, and troubleshooting workflow.
- What to borrow: metric granularity, chart interactions, host-to-service drill-down, anomaly and alert concepts.
- Limitation for your use case: product complexity is much higher than a focused desktop app, and licensing is GPL-3.0 on the open-source agent repo.
- Sources: [GitHub](https://github.com/netdata/netdata)

### 3. Glances

- Positioning: cross-platform system monitor with TUI, web UI, REST API, client/server mode, export pipelines, and quick remote usage.
- Why it matters: it proves a lightweight remote-monitoring path without a heavy full-stack platform.
- What to borrow: API-first exposure, low-friction deployment, fetch mode, browser discovery, and exporter adapters.
- Limitation for your use case: UI polish and long-term product structure are weaker than Beszel or Netdata.
- Sources: [GitHub](https://github.com/nicolargo/glances)

### 4. Prometheus node_exporter

- Positioning: machine metrics exporter for Unix-like hosts, focused on exposing host metrics over HTTP in Prometheus format.
- Why it matters: it is the cleanest reference for metric naming and host-level coverage such as CPU, filesystem, network, and diskstats.
- What to borrow: collector model, metric normalization, per-core CPU mapping, and filesystem / network / disk data schema.
- Limitation for your use case: it has no end-user product UX by itself; it is a data source, not an app. Also, meaningful "traffic speed" and "disk I/O speed" must be derived from counters over time.
- Sources: [GitHub](https://github.com/prometheus/node_exporter), [Prometheus Guide](https://prometheus.io/docs/guides/node-exporter/)

### 5. Uptime Kuma

- Positioning: self-hosted uptime and network probe tool with HTTP, TCP, WebSocket, Ping, DNS, push monitors, 20-second intervals, and ping charts.
- Why it matters: it covers the "network fluctuation" problem from the active probing side, which host resource collectors usually do not solve well on their own.
- What to borrow: probe definitions, response-time history, outage notifications, and status abstraction.
- Limitation for your use case: it is not a full host resource monitor.
- Sources: [GitHub](https://github.com/louislam/uptime-kuma)

### 6. Nezha

- Positioning: lightweight self-hosted server and website monitoring plus O&M tooling, with system status, HTTP/TCP/Ping checks, alerts, scheduled tasks, and web terminal.
- Why it matters: very close to the practical needs of VPS and homelab users, especially in Chinese-speaking communities.
- What to borrow: combined "resource monitoring + network checks + operations entry" product framing.
- Limitation for your use case: still fundamentally a web-admin product, not a local desktop product.
- Sources: [GitHub](https://github.com/nezhahq/nezha)

### 7. NeoHtop

- Positioning: modern desktop system monitor built with `SvelteKit + Rust + Tauri`.
- Why it matters: one of the clearest open-source references for a polished Tauri monitoring desktop app.
- What to borrow: Tauri app structure, Rust bridge patterns, process table UX, auto-refresh behavior, and compact high-density monitoring layout.
- Limitation for your use case: it focuses on local machine monitoring, not remote server fleet monitoring.
- Sources: [GitHub](https://github.com/Abdenasser/neohtop)

### 8. Pachtop

- Positioning: lightweight Tauri system monitor built with `Vite + Rust + React + TypeScript + Tauri`, using `sysinfo` for CPU, memory, disks, network, and processes.
- Why it matters: very relevant as a technical prototype for local desktop metrics panels and disk/network pages.
- What to borrow: frontend stack composition, dashboard-to-detail navigation, disk analysis page ideas, and use of Rust-side system metric crates.
- Limitation for your use case: it is still local-first, feature breadth is narrower, and several roadmap items are still experimental.
- Sources: [GitHub](https://github.com/pacholoamit/pachtop)

### 9. Cockpit

- Positioning: web-based graphical interface for Linux servers, covering storage, networking, logs, containers, and machine switching.
- Why it matters: strong reference for "light admin + glanceable monitoring" without forcing a heavy observability stack.
- What to borrow: server overview layout, operations entry points, simple multi-host switching, and admin-task adjacency to monitoring.
- Limitation for your use case: it is still server-hosted web software, and metrics depth is lighter than Netdata or Checkmk.
- Sources: [GitHub](https://github.com/cockpit-project/cockpit), [Project Site](https://cockpit-project.org/)

### 10. Checkmk Raw

- Positioning: integrated infrastructure monitoring platform with agent-based and agentless monitoring, auto-discovery, many plugins, dashboards, alerts, and automation features.
- Why it matters: strong benchmark for scale, auto-discovery, plugin ecosystem, and operational monitoring workflows.
- What to borrow: plugin architecture, service discovery, rule-based checks, and alert lifecycle patterns.
- Limitation for your use case: product scope is much wider than your v1 target, and it pulls you toward an enterprise monitoring platform.
- Sources: [GitHub](https://github.com/Checkmk/checkmk), [Product Page](https://checkmk.com/product/raw-edition)

### 11. ntopng

- Positioning: web-based traffic and security network monitoring focused on throughput, flows, hosts, protocols, latency, and traffic behavior.
- Why it matters: best specialized reference for the "network fluctuation" and "traffic analysis" part of your target.
- What to borrow: network-centric charts, per-interface throughput, top talkers, latency signals, and flow-driven drill-down.
- Limitation for your use case: it is not a full server resource monitoring product; it is a network analytics product. In addition, some advanced traffic-analysis capabilities are edition-dependent in the ntopng product line.
- Sources: [GitHub](https://github.com/ntop/ntopng), [Wiki](https://github.com/ntop/ntopng/wiki)

### 12. Nagstamon

- Positioning: desktop status monitor that connects to multiple monitoring backends such as Nagios, Icinga, Checkmk, Prometheus, and Zabbix.
- Why it matters: one of the clearest proofs that a desktop monitoring control plane is a real product shape, even when monitoring data comes from remote systems.
- What to borrow: tray-first workflow, condensed status surfaces, backend aggregation, and quick actions from alerts to SSH/RDP/VNC.
- Limitation for your use case: it consumes existing monitoring systems rather than collecting raw host metrics itself.
- Sources: [GitHub](https://github.com/HenriWahl/Nagstamon), [Project Site](https://nagstamon.de/)

### 13. Mission Center

- Positioning: Rust-based desktop system monitor focused on CPU, memory, disk, network, GPU, and process usage with efficient graph rendering.
- Why it matters: strong inspiration for dense local monitoring UI, resource-efficient graphs, and per-resource detail pages.
- What to borrow: graph density, process/resource grouping, and efficient desktop rendering ideas.
- Limitation for your use case: it is for local system monitoring rather than remote server fleets.
- Sources: [Project Site](https://missioncenter.io/), [Source Code](https://gitlab.com/mission-center-devs/mission-center)

## Recommended Direction

- Product benchmark: `Beszel`
- Metric model benchmark: `node_exporter`
- Lightweight remote adapter benchmark: `Glances`
- Network probe benchmark: `Uptime Kuma`
- Network traffic analytics benchmark: `ntopng`
- Desktop control-plane benchmark: `Nagstamon`
- Operations workflow benchmark: `Cockpit`
- Tauri desktop UX benchmark: `NeoHtop` and `Pachtop`
- Dense desktop graph benchmark: `Mission Center`

## Audit Corrections

- Do not expose raw `node_exporter` endpoints directly to the public internet as the default integration pattern. Use VPN, SSH tunnel, reverse proxy with TLS/auth, or a hub/agent design.
- Treat desktop-side probes carefully: a ping from the Tauri app measures `desktop-to-target` path quality, not necessarily `server-side network health`.
- Distinguish raw counters from derived rates in both product wording and implementation. Several candidate projects expose counters that still require local rate calculation for charts.

## Decision

- Recommended product route: `Tauri desktop client as the control plane`, while keeping remote collection in `agent/exporter` form.
- Recommended MVP route: support existing exporters first, then decide whether to build a self-owned agent.
- Recommended product boundary: do not try to replace all of Netdata, Checkmk, or Prometheus in v1; focus on host metrics, probe history, traffic overview, alerts, and a desktop-native workflow.
- Recommended product split: treat `host metrics`, `network probes`, and `traffic analysis` as separate modules from day one, even if they appear in one UI.
- Recommended connectivity baseline: prefer `VPN / WireGuard / Tailscale / SSH tunnel / authenticated gateway` over direct public scraping of exporter ports.
