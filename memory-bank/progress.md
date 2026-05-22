# ServerHUB · Progress

## Current Focus

**v0.2 Phase 2 — Notifier Trait 重构**

- Phase 1 安全收口已完成：identifier/CSP、known_hosts、SecretStore/keychain、明文凭证迁移、Settings keychain hint
- 自动化 gate 已复验：`pnpm check:all`、`pnpm test`、`cargo clippy --all-targets -- -D warnings`、`go vet ./...`
- Tauri dev webview 已启动验证：DevTools/console 无 CSP violation
- 下一步：按 `.claude/sdd/v0_2/plan.md` 进入 Cycle 2.1 `NOTIF-001`

## Milestones

- [x] **v0.0** 初版（commit `2002c5d`）：仓库脚手架
- [x] **v0.1.0** 监控管道基础（commit `e149418`）：alerts + probes + fleet health + 三层 retention + adapter 抽象
- [x] **项目初始化**：memory-bank + SDD 工作流落地（`/init-project` 执行完成）
- [x] **v0.2 Phase 1 安全收口**：CSP / known_hosts / SecretStore + Migration / KEY-010 UI hint，阶段 gate 通过
- [ ] **一键部署完成度**：`deployer/` 模块 + `DeployModal` 走完所有错误分支（鉴权失败 / 端口占用 / systemd 不可用 / arch 不支持）
- [ ] **Dashboard UI 升级**：`rings/` + `detail/` 全套组件设计审查通过（走 `/design-review` 或 `/plan-design-review`）
- [ ] **v0.1.x 收尾**：把当前 untracked 与 modified 的改动按 SDD 微循环节奏分批落 commit
- [ ] **v0.2 规划**：需要单独开 explore（详见 Open Questions）

## Known Tech Debt

| 项 | 影响 | 优先级 |
|---|------|--------|
| Tauri 2 mock IPC `list_servers` smoke test ignored | `mock_builder()` 下报 `Plugin not found`；当前已注释并 `#[ignore]`，需后续单独诊断 | 中 |
| Phase 2+ 通道/agent/remote probe 能力未落地 | v0.2 后续阶段仍需按 SDD micro-cycle 推进 | 高 |
| `demo_seeder` 在 debug 模式自动 seed | 调试时易污染数据；需开关化 | 低 |
| `lib.rs` setup 用 `block_on` 同步初始化 | 应用启动 I/O 阻塞，多 server 时启动变慢 | 低 |

## Open Questions / 待规划

### v0.2 路线（需要单独 explore）

ha1den 明确：当前主线是「完善功能 + UI 优化」，**v0.2 具体内容待我深入分析后给意见**。备选方向（按当前代码状态 + 个人 VPS 定位推断，**未确认**）：

- **A. 功能扩展**：加 Glances 适配器 / Webhook 告警通道 / 历史趋势对比视图
- **B. UI 收口**：完成 dashboard / detail / settings 全套设计语言，桌面级动画与响应式
- **C. 部署体验**：批量部署、agent 自动升级、跨 OS（Debian/Ubuntu/Alpine/CentOS）兼容矩阵
- **D. 数据洞察**：成本视图（VPS 月费 × 资源利用率）、流量异常检测、长期趋势

未来商业方向：免费 + 付费混合，**v0.2 阶段不实现任何商业化逻辑**。

> ⚠️ 不要在没 explore 的情况下直接选方向。下次 ha1den 说「我们规划 v0.2」时走 `/init-project` 的探索流程（场景 0 explore → auto-capture → 需求对齐）。

### 其他待回答

- agent 自动升级机制是否纳入 v0.2？
- 是否需要把 `docs/openspec-ui-redesign.md` / `docs/servercat-design-analysis.md` 的设计意图正式抽出来变成 design tokens？

## 最近一次 init-project 运行信息

- 运行日期：2026-05-21
- 模板源版本：v3.1.0
- 全局底座：ECC plugin v2.0.0-rc.1 + Protocol v3
- 状态：初次初始化（项目首次接入 SDD 工作流）
