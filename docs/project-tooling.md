# ServerHUB Project Tooling

> 更新日期：2026-03-19
> 说明：以下内容按当前仓库实际依赖、脚本与验证流程整理

## 1. 开发环境

| 工具 | 当前仓库用法 |
| --- | --- |
| Node.js | 前端构建、Tauri CLI |
| pnpm | 前端依赖与脚本执行 |
| Rust / Cargo | Tauri 后端与测试 |
| Go | `agent/` 构建与测试 |

## 2. 前端依赖

来自 [package.json](/Users/baihaibin/Documents/WorkSpares/ServerHUB/package.json)：

- `react`
- `react-dom`
- `react-router-dom`
- `zustand`
- `recharts`
- `dayjs`
- `@tauri-apps/api`
- `@tauri-apps/plugin-notification`

开发依赖：

- `vite`
- `typescript`
- `eslint`
- `typescript-eslint`
- `@vitejs/plugin-react`
- `tailwindcss`
- `postcss`
- `autoprefixer`
- `@tauri-apps/cli`

未使用、已从文档剔除：

- `antd`
- `@tanstack/react-query`

## 3. Rust 依赖

来自 [Cargo.toml](/Users/baihaibin/Documents/WorkSpares/ServerHUB/src-tauri/Cargo.toml)：

- `tauri`
- `tauri-plugin-log`
- `tauri-plugin-notification`
- `tokio`
- `serde`
- `serde_json`
- `reqwest`
- `sqlx`
- `chrono`
- `async-trait`
- `thiserror`
- `uuid`
- `surge-ping`
- `log`

## 4. 常用脚本

根目录：

- `pnpm css:generate`
- `pnpm lint`
- `pnpm build`
- `pnpm check`
- `pnpm check:rust`
- `pnpm check:go`
- `pnpm check:all`
- `pnpm tauri dev`

脚本文件：

- [scripts/check-frontend.sh](/Users/baihaibin/Documents/WorkSpares/ServerHUB/scripts/check-frontend.sh)
- [scripts/check-rust.sh](/Users/baihaibin/Documents/WorkSpares/ServerHUB/scripts/check-rust.sh)
- [scripts/check-go.sh](/Users/baihaibin/Documents/WorkSpares/ServerHUB/scripts/check-go.sh)
- [scripts/check-all.sh](/Users/baihaibin/Documents/WorkSpares/ServerHUB/scripts/check-all.sh)
- [scripts/generate-tailwind.mjs](/Users/baihaibin/Documents/WorkSpares/ServerHUB/scripts/generate-tailwind.mjs)

## 5. 当前验证基线

当前仓库的可执行验证基线：

- `pnpm lint`
- `pnpm build`
- `cargo check --manifest-path src-tauri/Cargo.toml`
- `cargo test --manifest-path src-tauri/Cargo.toml`
- `./scripts/check-go.sh`

说明：

- `pnpm build` 会先执行 `pnpm css:generate`，生成 `src/generated/tailwind.css`
- `cargo test` 中已包含 adapter parser、derived metrics、Tauri shell smoke coverage
- Go module 位于 `agent/`，因此根目录直接执行 `go test ./...` 会失败；统一通过 `./scripts/check-go.sh` 进入 `agent/` 再跑验证

## 6. 测试覆盖现状

已落地：

- Rust 单元测试：`node_exporter` labels 解析
- Rust 单元测试：`go_agent` payload 反序列化
- Rust 单元测试：连续 polling 产出 rate、labels 隔离 continuity、counter reset 不输出伪 rate
- Rust smoke test：mock runtime 启动 app 并调用 `list_servers`
- 前端 lint / build：覆盖 repo-owned CSS 生成与 hydrated monitoring pages 的编译基线

仍待后续扩展：

- 真实 probe 调度集成测试
- metrics history 的端到端 UI 验证
- 告警规则管理的完整交互测试

## 7. 开发约定

- 搜索优先用 `rg`
- Rust 数据库查询默认使用运行时 `sqlx::query`，避免离线环境被 `query!` 阻断
- 变更需要同时关注 OpenSpec、实现代码、验证脚本三层一致性
