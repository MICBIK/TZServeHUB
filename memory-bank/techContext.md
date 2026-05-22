# ServerHUB · Tech Context

> 真实版本来自 `package.json` / `src-tauri/Cargo.toml` / `agent/go.mod`。修改依赖时记得同步本文。

## 三栈技术栈

### 1) 前端（src/）

| 项 | 版本 | 备注 |
|---|------|------|
| React | 19.2.4 | 不用 Server Components / RSC，纯 SPA |
| Vite | 8.0.1 | dev/build 入口 |
| TypeScript | 5.9.3 | tsconfig.app.json + tsconfig.node.json |
| Tailwind CSS | 4.0.0 | **不走 PostCSS 默认管线**，使用 `scripts/generate-tailwind.mjs` 自定义生成到 `src/generated/tailwind.css` |
| React Router DOM | 7.0.0 | 客户端路由 |
| Zustand | 5.0.0 | 全局状态（serverStore / metricsStore / settingsStore） |
| Recharts | 2.15 | 时序图表 |
| dayjs | 1.11 | 时间格式化 |
| @tauri-apps/api | 2.0.0 | `invoke()` IPC |
| @tauri-apps/plugin-notification | 2.0.0 | 桌面通知 |

**开发端口**：4175（Vite 与 Tauri devUrl 强绑定，见 `vite.config.ts` 和 `src-tauri/tauri.conf.json`）

### 2) Rust 后端（src-tauri/）

| 项 | 版本 | 用途 |
|---|------|------|
| Rust edition | 2021 | `rust-version = "1.77.2"` |
| Tauri | 2.10.0 | desktop runtime（features: `test`） |
| tauri-plugin-log | 2 | 调试日志 |
| tauri-plugin-notification | 2 | 桌面通知 |
| tokio | 1（full） | async runtime |
| sqlx | 0.8 | sqlite + tokio-rustls 后端 |
| reqwest | 0.12 | HTTP 抓 exporter（json + rustls-tls） |
| serde / serde_json | 1.0 | 序列化 |
| thiserror | 2 | error type |
| anyhow | 1 | error context |
| tracing / tracing-subscriber | 0.1 / 0.3 | 结构化日志 |
| chrono | 0.4（serde） | 时间 |
| sysinfo | 0.33 | 本地诊断（开发时） |
| surge-ping | 0.8 | ICMP ping probe |
| async-trait | 0.1 | adapter trait async fn |
| uuid | 1（v4, serde） | 主键 |
| rand | 0.8 | 调度抖动 |
| **russh** | 0.46 | SSH client（一键 agent 部署） |
| **russh-keys** | 0.46 | SSH key 解析 |
| **russh-sftp** | 2.0 | SFTP 上传 agent 二进制 |

### 3) Go remote agent（agent/）

| 项 | 版本 | 用途 |
|---|------|------|
| Go | 1.22 | 编译目标 |
| gin | 1.10.0 | HTTP server |
| gopsutil | v4.24.12 | 主机指标采集（CPU/MEM/IO/NET） |
| yaml.v3 | 3.0.1 | config.yaml 解析 |

## 数据库

- **引擎**：SQLite WAL 模式（`src-tauri/src/storage/database.rs`）
- **迁移**：`src-tauri/migrations/`（共 7 个，sqlx `migrate!()` 启动时跑）
  - 001 init / 002 add_labels / 003 add_auth_token / 004 add_rollup_series_identity / 005 add_auth_fields / 006 add_server_status / 007 add_probe_results
- **数据目录**：默认走 Tauri `path::data_dir()`，测试用 env `SERVERHUB_DATA_DIR` 覆盖

## 关键路径与配置

| 文件 | 作用 |
|------|------|
| `src-tauri/tauri.conf.json` | 窗口 920×620、dev URL `http://127.0.0.1:4175`、identifier `com.tauri.dev`（**TODO：发布前改正式 identifier**） |
| `src-tauri/Cargo.toml` | Rust 依赖 |
| `vite.config.ts` | port 4175, host 0.0.0.0（允许 Tauri 子进程访问） |
| `scripts/generate-tailwind.mjs` | Tailwind 自定义生成 |
| `scripts/check-*.sh` | 三栈分别 / 整体校验脚本 |
| `scripts/build-agents.sh` | （新增 untracked）批量构建 agent 多平台二进制 |
| `src-tauri/resources/` | （新增 untracked）打包进 desktop 的 agent 二进制资源 |
| `agent/config.example.yaml` | Go agent 配置模板 |
| `docs/architecture.md` | 完整架构文档（18.8KB） |

## 环境变量

| 变量 | 用途 |
|------|------|
| `SERVERHUB_DATA_DIR` | 覆盖 SQLite 存储路径（测试/调试） |
| Go agent `SERVERHUB_TOKEN`（在 config.yaml 中） | Bearer 认证 |

桌面端**没有 .env**，所有配置走 Tauri settings + SQLite。

## 跨栈构建命令

```bash
# 前端
pnpm install
pnpm dev              # vite 4175
pnpm build

# Tauri 桌面应用
pnpm tauri dev        # 同时起 vite + cargo run
pnpm tauri build      # 生产打包

# Rust
cd src-tauri && cargo check / build / test

# Go agent
cd agent && go build -o serverhub-agent cmd/serverhub-agent/main.go

# 全栈校验
pnpm check        # frontend 仅 ts + lint
pnpm check:rust   # cargo check + clippy
pnpm check:go     # go vet + build
pnpm check:all    # 三栈合一
```

## Tauri IPC 风格

- Rust handler 注册在 `src-tauri/src/lib.rs` 的 `invoke_handler` 宏
- 前端通过 `src/services/tauri.ts` 的 wrapper 调用，不要在组件里直接 `invoke()`
- 命令分组：server / metrics / settings / alerts / probes / **deploy**（新增）

## 平台与开发环境

- **目标平台**：macOS / Linux / Windows（Tauri 跨平台），主要在 macOS Darwin 25.5.0 开发
- **包管理**：pnpm（前端） / cargo（Rust） / go modules（agent）
- **节点版本**：Vite 8 默认要求 Node ≥ 20
