# ServerHUB Architecture

> 更新日期：2026-03-19
> 范围：`close-mvp-audit-gaps` 修复后仓库的实际 MVP 边界

## 1. 当前产品边界

ServerHUB 是一个本地 Tauri 桌面控制平面，用来接入远端监控目标并展示主机指标。

当前 MVP 主线覆盖：

- 服务器注册、本地持久化、活动目标选择
- 应用启动时 hydrate 服务器清单与应用设置
- `node_exporter` / `go_agent` 两类适配器拉取
- 原始指标落 SQLite
- counter → rate 派生，且连续性在单 server poll loop 内保留
- `1m` / `15m` rollup
- Dashboard / CPU / Network / Disk / Settings / Alerts / Probes 的真实状态页
- ServerCat-inspired 暗色监控壳、玻璃面板、SVG 环境动效、启动入场动画

明确延期：

- `glances` 适配器
- 告警规则管理 UI
- Probe 历史与真实探测调度
- 更完整的图表与细粒度交互

## 2. 技术栈

- 桌面壳：Tauri 2 + Rust
- 前端：React 19 + TypeScript + Vite 8 + React Router 7
- 状态管理：Zustand
- 图表：Recharts
- 数据库：SQLite + sqlx
- HTTP 客户端：reqwest
- 远端 agent：Go + Gin + gopsutil

当前仓库没有引入 `antd`、`@tanstack/react-query` 之类额外 UI / data fetching 框架。

## 3. 运行时数据流

```text
Desktop UI
  -> app bootstrap (fetch servers + settings)
  -> active target stored in zustand
  -> AppLayout-level polling hook
  -> invoke(Tauri metrics commands)
  -> SQLite-backed command layer
  -> scheduler poller
  -> adapter fetch (node_exporter | go_agent)
  -> normalized raw_metrics
  -> per-server derived-rate engine
  -> metrics_1m / metrics_15m rollups
  -> query back to UI
```

关键点：

- 所有指标先归一化后入库
- series identity 由 `server_id + key + labels + vantage_point` 决定
- `DerivedMetricsEngine` 由 `scheduler/poller.rs` 的单 server 轮询任务持有，避免每轮 polling 丢失 counter baseline
- `go_agent` 的 `/api/metrics` 支持 Bearer token
- `/api/health` 约定为免鉴权 liveness 检查

## 4. Rust 后端结构

```text
src-tauri/src/
├── adapters/
│   ├── go_agent.rs
│   ├── node_exporter.rs
│   └── traits.rs
├── alerts/
│   ├── notifier.rs
│   └── rules.rs
├── commands/
│   ├── metrics.rs
│   ├── server.rs
│   └── settings.rs
├── metrics/
│   ├── derived.rs
│   ├── rollup.rs
│   └── schema.rs
├── models/
│   ├── alert.rs
│   ├── metric.rs
│   └── server.rs
├── probes/
│   ├── dns.rs
│   ├── ping.rs
│   └── tcp.rs
├── scheduler/poller.rs
├── storage/
│   ├── database.rs
│   ├── migrations.rs
│   └── retention.rs
├── error.rs
└── lib.rs
```

说明：

- `commands/server.rs`：服务器 CRUD
- `commands/settings.rs`：JSON 文件方式持久化应用设置
- `commands/metrics.rs`：当前值与历史查询
- `scheduler/poller.rs`：启停采集任务、入库、派生率、触发 rollup
- `storage/retention.rs`：按保留周期清理历史数据

## 5. SQLite Schema

主表：

- `servers`
- `raw_metrics`
- `metrics_1m`
- `metrics_15m`
- `alert_rules`
- `alert_events`

已补充迁移：

- `002_add_labels.sql`：为 `raw_metrics` 增加 `labels`
- `003_add_auth_token.sql`：为 `servers` 增加 `auth_token`
- `004_add_rollup_series_identity.sql`：为 rollup 表增加 `labels + vantage_point`

## 6. 前端结构

```text
src/
├── components/layout/
├── components/server/
├── hooks/usePolling.ts
├── pages/
│   ├── DashboardPage.tsx
│   ├── CpuPage.tsx
│   ├── NetworkPage.tsx
│   ├── DiskPage.tsx
│   ├── AlertPage.tsx
│   ├── ProbePage.tsx
│   └── SettingsPage.tsx
├── services/tauri.ts
├── stores/
│   ├── metricsStore.ts
│   ├── serverStore.ts
│   └── settingsStore.ts
└── types/
```

当前前端状态：

- `scripts/generate-tailwind.mjs` 使用现有 `tailwindcss` 包生成 `src/generated/tailwind.css`
- `src/main.tsx` 在入口处先加载生成的 utility CSS，再加载自定义壳样式
- `App.tsx` 启动时 hydrate servers/settings，并驱动一次性启动动画
- `AppLayout.tsx` 挂载全局 polling hook，保证 Dashboard 之外的 detail pages 也有实时数据
- `useMonitoringView` 将 `hydrating / no-server / no-selection / loading / error / ready` 统一暴露给监控页
- Settings 页面已接入 add / remove / select server 与 save settings 工作流
- Alerts / Probes 页面属于显式 deferred UI，不再伪造未实现能力

## 7. 已知限制

- 当前 detail pages 以密集状态卡与列表为主，还不是完整图表分析工作台
- `metrics/schema.rs` 仍保留后续 key 规范化入口
- 告警与探测模块已收口为真实 deferred surface，但还不是完整产品功能
