# ServerHUB · System Patterns

> 已落地的架构决策与跨模块约定。改架构时同步更新本文（`/project:finish-feature` 会提醒）。

## 顶层架构

```
┌──────────────────────────────────────────────────────────────┐
│ Desktop App (Tauri)                                          │
│ ┌────────────────────┐    invoke()    ┌────────────────────┐ │
│ │ React 19 + Zustand │ ◀────────────▶ │ Rust core (tokio)  │ │
│ │ pages/stores/hooks │                │ adapters/probes/   │ │
│ │                    │                │ scheduler/storage  │ │
│ └────────────────────┘                └──────────┬─────────┘ │
└──────────────────────────────────────────────────┼───────────┘
                                                   │ HTTP/JSON
                                                   ▼
                                      ┌────────────────────────┐
                                      │ Go remote agent (gin)  │
                                      │ /api/health /api/metrics│
                                      └────────────────────────┘
                                                   ▲
                                                   │ SSH/SFTP (russh)
                                      [Desktop ──── 一键部署 ─────▶ VPS]
```

## 1. Adapter 模式（核心抽象）

**位置**：`src-tauri/src/adapters/traits.rs`

```rust
#[async_trait]
pub trait MetricAdapter: Send + Sync {
    fn name(&self) -> &str;
    async fn fetch_host_metrics(&self, server: &ServerConfig) -> AppResult<Vec<RawMetric>>;
    async fn health_check(&self, server: &ServerConfig) -> AppResult<bool>;
}
```

- **目的**：统一不同数据源（node_exporter / Go agent / Glances 等）的拉取方式
- **现有实现**：`NodeExporterAdapter`（Prometheus 文本）+ `GoAgentAdapter`（JSON）
- **新增 adapter 流程**：
  1. 在 `src-tauri/src/adapters/` 新建 `<name>.rs` 实现 trait
  2. `models/server.rs` 的 `AdapterType` 加 variant
  3. `scheduler/poller.rs` 的 adapter 实例化分支补一条

## 2. 三层 retention（数据留存）

**位置**：`src-tauri/src/storage/retention.rs` + `migrations/`

| 层级 | 表 | 保留 | 来源 |
|------|------|------|------|
| 原始 | `raw_metrics` | 7 天 | adapter 直接落库 |
| 1m rollup | `metrics_1m` | 30 天 | `metrics/rollup.rs` 聚合（min/max/avg） |
| 15m rollup | `metrics_15m` | 90 天 | 同上 |

**rollup 规则**：raw → 1m 触发于落库后异步，1m → 15m 同步。退化为 SQL 聚合 + 唯一索引去重。

## 3. Counter → Rate 派生

**位置**：`src-tauri/src/metrics/derived.rs`

- 累计计数器（如 `network_transmit_bytes_total`）→ 派生速率（bytes/sec）
- 状态：内存 HashMap 存上次值 + 上次时间戳
- 计数器复位（值变小）→ 丢弃这次差分
- 长期不活跃 key → 清理

**约定**：所有 counter 类型 metric 必须经过 `DerivedMetricsEngine`，不允许前端直接用累计值画图。

## 4. Vantage Point 标签

每条 `raw_metric` 带 `vantage_point` 字段：

| 值 | 含义 |
|------|------|
| `desktop-local` | 探测从桌面侧发起（衡量「我家网络 → 这台 VPS」） |
| `remote-agent` | 探测从 agent 所在 VPS 发起（衡量「VPS → 互联网/其他 VPS」） |

用途：区分用户主观感受（家用网络）与 VPS 真实出口质量。

## 5. Alert 评估

**位置**：`src-tauri/src/alerts/rules.rs`

- **条件**：`Gt` / `Lt` / `Eq`
- **持续**：阈值需持续 N 秒才触发（去抖）
- **冷却**：两次告警最小间隔，防 flapping
- **通知**：tauri-plugin-notification（桌面原生通知中心）

## 6. Tauri IPC 模式（前后端通信）

- **Rust 侧**：所有 handler 必须在 `src-tauri/src/lib.rs` 的 `invoke_handler!` 宏里显式注册，否则前端调不到
- **TypeScript 侧**：
  - 不在组件里直接 `invoke()`
  - 所有 IPC 走 `src/services/tauri.ts` 的 typed wrapper
  - wrapper 函数命名与 Rust handler 一致（snake_case → camelCase 自动）

## 7. 状态管理（前端）

| 全局状态 | Store | 范围 |
|---------|------|------|
| 服务器列表 + 当前激活 | `serverStore` | 持久化（Tauri 侧 SQLite） |
| 各服务器 metrics 实时数据 | `metricsStore` | 仅内存，按 `server_id` 索引 |
| 用户设置 | `settingsStore` | 持久化 |

**约定**：
- Server CRUD：永远通过 Tauri 命令，不在前端直接改 store
- Metrics 实时更新：`usePolling` hook 触发 `get_metrics` invoke → 写入 metricsStore

## 8. 一键部署（新增，Current Focus）

**位置**：`src-tauri/src/deployer/` + `src/components/deploy/DeployModal.tsx`

```
DeployModal (UI 表单)
    ↓ invoke('deploy_agent', form)
deploy::deploy_agent (Rust handler)
    ↓
deployer/ssh_client (russh 连接)
    ↓
deployer/deploy_steps (步骤编排)
  1. SSH 连接（key 或 password）
  2. 检测 OS / arch
  3. SFTP 上传 agent 二进制（来自 src-tauri/resources/）
  4. 渲染 systemd unit（templates/）
  5. 启动 + healthcheck
    ↓
events emitted → DeployModal 显示进度
```

**进度协议**（`src/types/deploy.ts`）：`{ step, total, label, status: 'running'|'done'|'error', detail? }`

## 9. Demo Seeder（仅 debug 构建）

**位置**：`src-tauri/src/demo_seeder.rs`，`#[cfg(debug_assertions)]` 隔离

启动时如果没有任何 server，写入若干 mock server + mock metrics，方便不连真实 VPS 也能跑 UI。**生产构建不会编译这段**。

前端对应的 `src/lib/browserDemo.ts` 在「纯浏览器（无 Tauri runtime）」时提供 mock，便于 `pnpm dev` 单跑 UI 调试。

## 10. 错误处理约定

- Rust 侧：`AppError` 统一错误类型（`src-tauri/src/error.rs`），`AppResult<T>` 别名
- IPC 边界：错误转 String 返回前端（Tauri 自动）
- 前端：`services/tauri.ts` 包装层捕获并 throw，由调用方 try/catch
- **不允许**：silent fail / `.unwrap()` 在 IPC 命令路径上 / `console.log` 替代错误处理

## 11. API 响应形状（Tauri 命令）

Tauri 命令返回值就是 `Result<T, String>`，前端拿到 `T` 或 throw。**不需要 `{ success, data, error }` envelope**（那是 HTTP API 模式，桌面 IPC 不适用）。

Go agent 的 HTTP 接口可以用 envelope，但目前简单返回 JSON。

## 12. 三大领域分离（产品层）

代码组织上：
- Host：`adapters/` + `metrics/`
- Network Quality：`probes/` + `scheduler/probe_scheduler.rs`
- Traffic：复用 host adapter，counter→rate 路径

UI 上：每个领域有独立页面（Dashboard 汇总 / CPU / Network / Disk / Probe / Alert / Settings）。

## 反模式（明确禁止）

- ❌ 在前端组件里直接 `invoke()`，绕过 `services/tauri.ts`
- ❌ Counter 类 metric 不经过 derived engine 直接画图
- ❌ 新加 Tauri command 忘记在 `lib.rs` 的 `invoke_handler!` 注册
- ❌ 新加 sqlx migration 跳号、不写在 `migrations/` 目录
- ❌ 把凭证（SSH key / agent token）以明文写日志
- ❌ 在 `release` 模式启用 demo_seeder
