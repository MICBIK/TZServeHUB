# ServerHUB — 项目配置

> 全局底座（身份/沧元图/核心约束/技术偏好/ECC skills/agents/commands）已由 `~/.Codex/AGENTS.md` Protocol v3 + ECC plugin 提供，本文件只承载项目专属内容与 SDD 工作流增量。

## 核心上下文（每次会话优先读取）

@memory-bank/projectBrief.md
@memory-bank/techContext.md
@memory-bank/systemPatterns.md

## 当前状态

@memory-bank/progress.md

## 项目专属

### Project Overview

ServerHUB is a **Tauri 2 desktop application** serving as a control plane for monitoring **remote VPS clusters owned by a single user**（个人多 VPS 监控面板）. NOT a local system monitor — it's a centralized desktop dashboard.

**Core architecture**: Desktop control plane (Tauri / Rust + React 19) + Remote Go agents + Adapter layer (node_exporter / Glances / custom Go agent).

**Three-domain separation**:
- **Host Metrics**: CPU (total/per-core), memory, disk I/O, disk space
- **Network Quality**: ping latency, TCP connectivity, DNS resolution, packet loss
- **Traffic Analytics**: NIC traffic, interface ranking, burst detection

### Development Commands

#### Frontend (React 19 + Vite 8 + Tailwind 4)

```bash
pnpm install              # install dependencies
pnpm dev                  # vite dev server :4175 (runs css:generate first)
pnpm build                # production build
pnpm lint                 # ESLint
```

#### Tauri Desktop App

```bash
pnpm tauri dev            # vite + cargo run
pnpm tauri build          # production desktop bundle
```

#### Rust Backend (src-tauri/)

```bash
cd src-tauri
cargo check               # compile check
cargo build               # debug binary
cargo build --release     # release binary
cargo test                # tests
```

#### Go Remote Agent (agent/)

```bash
cd agent
go mod tidy
go build -o serverhub-agent cmd/serverhub-agent/main.go
./serverhub-agent -config config.yaml
```

#### Cross-Stack Checks

```bash
pnpm check                # frontend (ts + lint)
pnpm check:rust           # cargo check + clippy
pnpm check:go             # go vet + build
pnpm check:all            # all three stacks
```

### Browsing & QA Tools

- **Always use `/browse` from gstack** for web browsing. Never use `mcp__claude-in-chrome__*` tools.
- `/qa` and `/qa-only` from gstack for automated testing flows.

### Architecture Deep Dive

#### Adapter Pattern

`MetricAdapter` trait at `src-tauri/src/adapters/traits.rs`:

```rust
#[async_trait]
pub trait MetricAdapter: Send + Sync {
    fn name(&self) -> &str;
    async fn fetch_host_metrics(&self, server: &ServerConfig) -> AppResult<Vec<RawMetric>>;
    async fn health_check(&self, server: &ServerConfig) -> AppResult<bool>;
}
```

Existing implementations: `NodeExporterAdapter` (Prometheus text), `GoAgentAdapter` (JSON). Future: `GlancesAdapter`.

#### Data Flow

```
Remote VPS (Go agent)
  → HTTP/JSON
  → Adapter (Rust)
  → Normalized RawMetric
  → DerivedMetricsEngine (counter → rate)
  → SQLite (WAL)
  → Frontend (Zustand + Recharts)
```

#### Storage: Tiered Retention

(`src-tauri/src/storage/retention.rs`)

1. **raw_metrics**: 7 days, full resolution (5–10s intervals)
2. **metrics_1m**: 30 days, min/max/avg
3. **metrics_15m**: 90 days, min/max/avg

7 migrations in `src-tauri/migrations/` (001–007). Tables: `servers`, `raw_metrics`, `metrics_1m`, `metrics_15m`, `alert_rules`, `alert_events`, plus `server_status`/`probe_results` (006/007).

#### Counter → Rate Derivation

`src-tauri/src/metrics/derived.rs`. Tracks previous counter value + timestamp, handles counter resets (value decrease → drop sample), cleans stale state.

#### Probe System

Three probe types under `src-tauri/src/probes/`:
- **ping**: ICMP via `surge-ping`
- **tcp**: connection latency via `tokio::net::TcpStream`
- **dns**: resolution timing via UDP

Each probe carries `vantage_point` tag (`desktop-local` vs `remote-agent`) to distinguish where the measurement originates.

#### One-Click Deployment (Current Focus)

`src-tauri/src/deployer/` + `src/components/deploy/DeployModal.tsx`. Uses `russh` / `russh-keys` / `russh-sftp` to SSH into the target VPS, SFTP upload the bundled agent binary from `src-tauri/resources/`, render a systemd unit (via `templates.rs`), start it, and health-check `/api/health`. Progress emits structured events to the frontend modal.

### Module Organization

#### Rust Backend (`src-tauri/src/`)

```
commands/       # Tauri IPC handlers (registered in lib.rs)
  ├── server.rs      # list/add/remove servers + health summary
  ├── metrics.rs     # get_metrics, get_metric_history
  ├── settings.rs    # get/update settings
  ├── alerts.rs      # alert rules + events
  ├── probes.rs      # run_ping/tcp/dns + history
  └── deploy.rs      # deploy_agent (one-click SSH deployment)

adapters/       # MetricAdapter implementations
  ├── traits.rs
  ├── node_exporter.rs
  └── go_agent.rs

probes/         # Network quality probes (ping/tcp/dns)
metrics/        # derived.rs (rate), rollup.rs (1m/15m)
storage/        # database.rs (pool + WAL), retention.rs
scheduler/      # poller.rs + probe_scheduler.rs
alerts/         # rules.rs (eval), notifier.rs (desktop notif)
deployer/       # ssh_client.rs + deploy_steps.rs + templates.rs
models/         # server.rs, metric.rs, alert.rs
demo_seeder.rs  # debug-only seed for empty DB (#[cfg(debug_assertions)])
```

#### React Frontend (`src/`)

```
pages/          # Dashboard, Settings (and per-domain pages)
components/
  ├── layout/        # AppLayout, Sidebar
  ├── charts/        # LineChart (Recharts wrapper)
  ├── server/        # ServerListItem (replaces older ServerCard)
  ├── detail/        # ServerDetail, DiskBar, MemoryBar, MetricGrid
  ├── rings/         # ActivityRings
  ├── deploy/        # DeployModal
  └── common/        # MetricCard

stores/         # Zustand (serverStore, metricsStore, settingsStore)
services/       # tauri.ts (typed invoke wrappers)
types/          # server, metric, alert, deploy
hooks/          # usePolling, useAllServersMetrics, useMonitoringView
lib/            # constants, formatters, browserDemo, metricsTransform, uiCopy
```

#### Go Remote Agent (`agent/`)

```
cmd/serverhub-agent/main.go    # entry point
internal/
  ├── config/config.go         # yaml.v3 loader
  ├── server/server.go         # gin + Bearer auth middleware
  └── collector/collector.go   # gopsutil metric collection
```

**Endpoints**: `GET /api/health`（无认证）, `GET /api/metrics`（需 Bearer token）.

### Key Concepts

#### ServerConfig

- `adapter_type`: `NodeExporter | Glances | GoAgent`
- `access_method`: `Private | Tunnel | Gateway`（安全上下文标签）
- `polling_interval_sec`: per-server polling frequency
- `enabled`: pause monitoring without removing

#### MetricType

- `Counter`: monotonically increasing（如 bytes transmitted）
- `Gauge`: point-in-time（如 CPU %）
- `State`: discrete（如 service up/down）

#### Tauri IPC Pattern

```typescript
// services/tauri.ts wrapper
import { invoke } from '@tauri-apps/api/core';
export const listServers = () => invoke<ServerConfig[]>('list_servers');
```

Rust handler **must** be registered in `src-tauri/src/lib.rs` inside `invoke_handler!`. Forget to register = frontend gets "command not found".

#### Alert Rule Evaluation

- **Condition**: `Gt | Lt | Eq`
- **Duration**: must hold for N seconds before firing（去抖）
- **Cooldown**: minimum interval between repeat fires（防 flapping）

### Adding New Stuff

#### New Tauri Command

1. Define handler in `src-tauri/src/commands/<group>.rs`
2. Register in `src-tauri/src/lib.rs` `invoke_handler!`
3. Add typed wrapper in `src/services/tauri.ts`
4. Call from React via the service, **never** direct `invoke()` in components

#### New Adapter

1. Implement `MetricAdapter` in `src-tauri/src/adapters/<name>.rs`
2. Add variant to `AdapterType` enum in `models/server.rs`
3. Update adapter instantiation in `scheduler/poller.rs`

#### New Migration

- Add file in `src-tauri/migrations/` with next sequence number, no skipping
- `sqlx::migrate!()` runs on startup automatically
- WAL mode 在 `storage/database.rs` 启用

### Browser Demo Mode

`src/lib/browserDemo.ts` 为「无 Tauri runtime」场景（直接 `pnpm dev` 在浏览器开）提供 mock 数据，方便纯 UI 调试。Tauri 环境下不走这条路径。

### Important Files

- `src-tauri/tauri.conf.json` — 窗口 920×620, dev URL 4175, identifier `com.tauri.dev`（**TODO**: 发布前换正式 identifier）
- `src-tauri/migrations/001_init.sql` — 初始 schema
- `vite.config.ts` — port 4175, host 0.0.0.0
- `scripts/generate-tailwind.mjs` — Tailwind 4 自定义生成入口
- `docs/architecture.md` — 完整架构文档（18.8KB，深度阅读）
- `agent/config.example.yaml` — agent 配置模板

### Security Red Lines

- **不是公网服务**：agent 永远在私网 / VPN / SSH 隧道后
- **Bearer token**：Go agent `/api/metrics` 必须带 token
- **不打日志的敏感字段**：SSH key、agent token、password
- **CSP**：`tauri.conf.json` 目前 `csp: null`，**生产打包前必须补 nonce-based CSP**
- **凭证存储**：SQLite 明文存 SSH key / token，发布前需评估系统级 keychain 集成
- **demo_seeder**：`#[cfg(debug_assertions)]` 隔离，**release 构建禁启用**

### 反模式（不要这样做）

- ❌ 在前端组件里直接 `invoke()`，绕过 `services/tauri.ts`
- ❌ Counter 类 metric 不经 `DerivedMetricsEngine` 直接画图
- ❌ 新 Tauri command 忘记在 `lib.rs` `invoke_handler!` 注册
- ❌ 新 sqlx migration 跳号、放错目录
- ❌ 把 SSH key / token / password 写日志或写错误返回串
- ❌ Release build 启用 demo_seeder

<!-- WORKFLOW-START: scenario-0-explore v3.1.0 -->
## 场景 0：模糊度优先判断（任何输入先经过这里）

> 防止"模糊需求硬塞分析流程，AI 编造结论"。检测到模糊信号 → 主动提议进 explore，不直接进场景 1/2/3。

### 触发词

| 类别 | 关键词 |
|------|-------|
| 想法初期 | 想做、考虑、我在想、有点想、可能、也许 |
| 没思路 | 怎么搞、从哪开始、还没想好、大概、思路 |
| 新模块/方向 | 新项目、新工具、从零、未来怎么 |
| 选项对比 | X 还是 Y、哪个好、对比一下、选哪个 |
| 模糊 Bug | 奇怪、莫名、找不到规律、偶尔、有时候（仅在场景 2 两轮假设不收敛后升级） |

### 主动提议话术（固定模板）

```
这是个开放问题，建议先用 explore 模式发散，理由：<一句话>。

进入 explore 后我会：
- 读相关代码 / memory-bank / docs
- 画 ASCII 图、列方向、做 tradeoff 对比
- 不写应用代码，只创建/修改 SDD artifacts

继续吗？
- [是] 进入 explore
- [否] 强行进场景 1（我会尽力分析但可能不够准）
```

### explore 过程铁律

1. 每轮回应开头打 `[EXPLORE]` 徽章
2. **守门**：禁应用代码、禁安装依赖、禁改源码
3. **允许**：读任何文件、创建/修改 SDD artifacts、画 ASCII 图
4. 过程开放跟着用户兴趣走，不强求收敛

### 收敛产出（强制三件套）

用户出现"那我们就这么干"/"我决定 X"/"好，开始吧"/"OK 就这样"或连续 2 轮无新信息时，**必须**输出：

```markdown
## What We Figured Out

**The problem**: <一句话>

**The approach**: <方向 + 关键决策>

**Open questions**: <未解决项 / 需调研项>

**Risks**: <已识别的风险 + 缓解>

**Capture plan**: <AI 即将主动写哪些 artifacts>

**Next**:
- [A] AI 主动 capture 后 → 进入需求对齐流程
- [C] AI 主动 capture 到 SDD 目录但不立 change（留作 reference）
- [D] 不 capture，到此为止
```

### Auto-capture 行为（用户选 A/C 后 AI 自动执行）

| 选择 | 动作 |
|------|------|
| [A] | 1) 创建 `.Codex/sdd/<feature>/` 目录<br>2) "What We Figured Out" → proposal.md 的 Intent/Scope/Approach<br>3) Open questions → design-notes.md 的"待解决问题"<br>4) Risks → design-notes.md 的"风险表"<br>5) **不写 specs**（留给需求对齐阶段补 GIVEN/WHEN/THEN）<br>6) 输出"已 capture，是否进入需求对齐流程？" |
| [C] | 1) 在 `.Codex/sdd/` 追加 explore notes<br>2) 或在 `memory-bank/progress.md` 的 Current Focus 段加"探索结论"<br>3) 输出"已记录到 X，结束 explore" |
| [D] | 仅输出"explore 结束，未 capture，可随时回来继续" |

**草稿水印**（强制）：每个 AI auto-captured 文件末尾加：
```html
<!-- 此文件由 explore 自动生成草稿，请审阅。生成时间：<ISO-时间戳> -->
```
<!-- WORKFLOW-END: scenario-0-explore -->

<!-- WORKFLOW-START: analysis-handoff v3.1.0 -->
## 分析→执行 衔接规则（任何分析/排查任务先走这里）

> 防止"给完结论就停"或"分析完直接乱改"。任何分析类任务结束都必须主动衔接到下一步，不允许中断。

### 场景 1：需求分析任务

**触发词**："分析一下"、"看看这个功能怎么做"、"评估"、"调研"、"帮我想想"

**执行步骤**：
1. 完成分析，输出固定四段：
   - 技术方案（含选型理由）
   - 影响范围（受影响文件/模块/接口）
   - 风险点（已知风险 + 未知/需验证）
   - 工作量预估（轻量/深度 + 大致时长）
2. 输出末尾**必须**附上衔接询问（一字不差）：
   ```
   ─────────────────────────────────────
   分析完成。如何继续？
   - [A] 进入需求对齐 + TDD 流程（推荐）
   - [B] 跳过对齐，直接进入调研 + TDD
   - [C] 仅分析，到此为止
   ─────────────────────────────────────
   ```
3. 收到选择后：
   - 选 A：创建 `.Codex/sdd/<feature>/` 目录，按完整流程走
   - 选 B：直接进入调研 → TDD RED
   - 选 C：更新 `memory-bank/progress.md` 记录本次分析结论，结束

### 场景 2：Bug 排查任务

**触发词**："这里有 bug"、"功能不对"、"报错"、"修一下"、"为什么不行"

**执行步骤**：
1. 定位根因（假设表 → 复现路径 → 二分到最小失败点）
2. 输出根因报告：根因描述（精确到文件:行号）+ 复现路径 + 影响面评估（涉及文件数、是否涉及接口/状态/权限/DB/核心链路）
3. 按影响面**强制分流**：

   **轻量 Bug**（单文件 / 纯样式 / 明确小修 / 不涉及接口与持久化）：
   ```
   根因已定位：<文件:行号> <一句话描述>
   修复方案：<diff 描述>
   这是轻量修复，是否直接执行？
   - 直接修复 → 改完跑测试验证 → 完成
   - 我想先看看 → 给出完整 diff 但不修
   ```

   **深度 Bug**（跨文件 / 接口/状态/权限/DB / 核心链路 / 影响 >3 文件）：
   ```
   根因已定位：<描述>
   影响面：<N> 个文件，涉及 <模块列表>
   建议走完整修复流程：
     1) 写回归测试覆盖此场景（RED）
     2) 最小修复使测试通过（GREEN）
     3) 跑同模块全部测试 + 验证
   是否开始？
   ```

4. 修复完成后**必须**：
   - 原路径红绿复验
   - 更新 `memory-bank/knownIssues.md` 移除该项
   - 更新 `progress.md`
   - **评估是否需更新 `systemPatterns.md`**（把修复教训沉淀为规范，防同类 bug 复发）
5. **两轮假设不收敛**（验证两个根因假设都失败）→ 主动建议升级到场景 0 explore：「两轮假设都不收敛，建议升级 explore 建假设地图后再回来。继续吗？[是]进 explore /[否]我再激进搜证一轮」

### 场景 3：用户意图不明

**触发词**："帮我看看"、"这块代码有问题吗"、"怎么样"、"看一下"

**处理方式**：先分类再行动，禁猜测式直接动手。
```
为了准确响应，先确认意图：
- [仅分析] 我只想看意见，你别动
- [分析后修复] 给意见，如果合理可以直接动手
- [直接修复] 我已经知道问题，请你动手
```

### 强制约束（违反即视为未完成）

| 规则 | 内容 |
|------|------|
| 双产出 | 分析结论 + 衔接询问，二者缺一即未完成 |
| 不跨越 | 没有用户明确选择前，禁止从分析直接跳到代码修改 |
| 例外 | 用户原话含明确执行意图（"修复 X"、"实现 Y"、"重构 Z"）按对应场景的执行步骤直接进行，不再问 |
| 兜底 | 任何衔接询问后超过一轮用户没明确回答 → 默认走 [C] 仅分析停止 |
<!-- WORKFLOW-END: analysis-handoff -->

<!-- WORKFLOW-START: tdd-ironrules v3.1.0 -->
## SDD 增量层（叠加在 ECC PRP + /tdd 之上）

> 基础 TDD（RED/GREEN/REFACTOR + coverage + checkpoint commits）由 ECC 的 `/tdd` 命令和 `tdd-guide` agent 处理。
> 需求文档由 ECC `/prp-prd` 处理，实施计划由 ECC `/prp-plan` 处理。
> 本段定义 Spec-Driven Development 增量：test-map 前置、tasks 微循环结构、commit 节奏、NO-TDD 规则。

### 1. SDD artifact 目录

`.Codex/sdd/<feature>/` 存放 SDD 特有的 artifacts：

```
.Codex/sdd/<feature>/
├── specs/                ← GIVEN/WHEN/THEN 用例（每个 capability 一个子目录）
├── test-map.md           ← spec-id → 测试函数映射表
└── design-notes.md       ← explore 阶段产出的设计笔记（可选）
```

需求文档（PRD）和实施计划（plan）由 ECC PRP 管理，存放在 `.Codex/PRPs/` 下。

### 2. test-map 强制前置（无 test-map 禁生成 tasks）

`.Codex/sdd/<feature>/test-map.md` 必须已存在：

```markdown
| Spec-ID | Test Layer | Test File | Test Function | Notes |
|---------|-----------|-----------|---------------|-------|
| auth-session-001 | unit | <path>/auth.test.ts | refreshTokenSuccess | <runtime> |
```

每条 spec → 测试函数名 + 文件路径 + 层级（unit/integration/e2e）。

### 3. PRP plan 的 Tasks 段必须使用微循环结构（1 spec = 1 微循环）

ECC `/prp-plan` 产出的 plan 文件中，Tasks 段必须使用 SDD 微循环格式：

```
1. <module-name>
  1.1.a [TEST-RED]  写 auth-session-001 的失败测试
  1.1.b [IMPL-GREEN] 实现 refresh token 逻辑
  1.2.a [TEST-RED]  写 auth-ui-001 测试
  1.2.b [IMPL-GREEN] 实现 Login 表单
```

- 禁止纯实现任务（没有对应 [TEST-RED] 的 [IMPL-GREEN]）
- 禁止把所有测试集中放最后一组
- 单微循环 ≤ 30 分钟工作量

### 4. 微循环执行证据（与 ECC /tdd 协同）

- RED：必须是**真实终端输出**含 `FAIL`/`FAILED`，声明式 RED 视为违规
- RED 环境不可用：补 `[RED-补证]` 任务挂起当前微循环，禁先实现回头补
- GREEN：必须粘贴真实 PASS 输出

### 5. TDD Commit 节奏

每微循环两个 commit：
- `test(<scope>): <spec-id>` ← RED
- `feat(<scope>): <spec-id>` ← GREEN

commit-msg hook 强制：未带 `[no-tdd]` 的 `feat:` 提交，前 5 commit 必须有同 scope 的 `test:`。

### 6. NO-TDD 例外（仅限）

- 纯样式（CSS/Tailwind class、视觉调整不改交互）
- 文档与 SDD 元文件（*.md/README/docs）
- commit message 必须显式加 `[no-tdd]` 标签
- hook 做白名单文件级二次验证：staged 只能是 *.css/*.scss/*.sass/*.less/*.md/*.mdx/*.txt/*.rst
- **重构 / 配置变更 / 依赖增减不在 NO-TDD 范围**

### 7. TDD 阶段徽章

```
🏹 柳七月·开弓态 [TDD: RED 写测试中]
🏹 柳七月·开弓态 [TDD: RED 已 FAIL ✓]
⚔️ 孟川·出刀态 [TDD: GREEN 写实现中]
⚔️ 孟川·出刀态 [TDD: GREEN 已 PASS ✓]
⚔️ 孟川·出刀态 [TDD: REFACTOR]
🛡️ 秦五·镇山态 [TDD: NO-TDD 已加 [no-tdd]]
```

### 8. SDD-PRP 路由规则

当 `.Codex/sdd/<feature>/` 存在时，该 feature 的任务以 SDD test-map + micro-cycle 为 SSOT，禁止 ECC `/plan`、`/prp-plan` 对同一 feature 生成独立任务清单绕过 SDD 结构。ECC `/tdd` 和 `verification-loop` 正常使用。
<!-- WORKFLOW-END: tdd-ironrules -->

<!-- WORKFLOW-START: flow-diagram v3.1.0 -->
## 完整开发流程（唯一权威定义）

> ECC 提供基础能力（PRP pipeline、/tdd、search-first、verification-loop、code-reviewer）。
> 本流程定义 SDD 增量：场景路由 → 需求对齐 → 微循环 → 质量门。
> 具体步骤见 `.Codex/commands/new-feature.md`。

```
用户输入
  │
  ├─ 模糊 → 场景 0 explore → auto-capture → 需求对齐
  ├─ 清晰需求 → 场景 1 衔接 → 用户选 [A] → 需求对齐
  ├─ Bug → 场景 2 排查 → 分流（轻量直修 / 深度走 ECC /tdd）
  └─ 意图不明 → 场景 3 反问 → 分类后进对应场景
                               │
                               ↓
                    需求对齐（ECC /prp-prd → specs → test-map）
                               │
                               ↓
                    实施计划（ECC /prp-plan + SDD micro-cycle 增强）
                               │
                               ↓
       ┌────────── 每微循环（ECC /tdd + SDD 增量）──────────┐
       │  🏹 [TEST-RED] 写测试 + 粘 FAIL                     │
       │  git: test(<scope>): <spec-id>                       │
       │  ⚔️ [IMPL-GREEN] 实现 + 粘 PASS                      │
       │  git: feat(<scope>): <spec-id>   ← commit-msg hook   │
       │  （可选）REFACTOR                                     │
       └──────────────────────────────────────────────────────┘
                               │
                               ↓
       质量门（ECC verification-loop）→ 归档 → 更新 memory-bank
```

## 任务完成后的必做事项

1. 更新 `memory-bank/progress.md`（勾选完成项 + 更新 Current Focus）
2. Bug 修复后：更新 `knownIssues.md` + 评估是否需更新 `systemPatterns.md`
3. 若完成 Feature：运行 `/project:finish-feature` 收尾
4. 长 session 收尾：运行 ECC `/save-session` 保存会话状态

## Memory Bank + ECC 持久化边界

| 信息类型 | 写到哪里 | 谁负责 |
|---------|---------|--------|
| 项目是什么 / 技术栈 / 架构决策 | `memory-bank/` | `/project:update-context` |
| Feature 完成/里程碑 | `memory-bank/progress.md` | `/project:finish-feature` |
| 本次 session 尝试/失败/下一步 | `~/.Codex/session-data/` | ECC `/save-session` |
| 编码行为习惯 | ECC instincts | ECC `continuous-learning-v2` |
<!-- WORKFLOW-END: flow-diagram -->
