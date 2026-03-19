# 审计报告：tauri-monitor-analysis-claude.md

审计日期：2026-03-19
对比来源：tauri-monitor-research-1.md ~ research-5.md

---

## 问题 1：技术栈分布统计数据缺乏依据

**位置**：第1节"框架对比"表格

**问题**：
- React "8+" 项目数量无法从5份研究报告中验证。research-1 列出的 Tauri 项目共8个，其中使用 React 的仅有 win-system-monitor、DPanel（research-3）、resource-monitor（research-5）等少数几个，远未达到"8+"
- Vue 3 "6+" 同样无法验证。research-1~5 中明确使用 Vue 3 的 Tauri 项目只有 PulseCoreLite、system-monitor、StarDash，约3个
- Svelte "4+" 无法验证。研究报告中明确使用 Svelte 的 Tauri 项目只有 neohtop 和 glance，共2个
- Next.js "2+" 无法验证。research-5 中 resource-monitor 使用 Next.js，仅1个明确案例
- 结论：框架数量统计均为夸大，缺乏研究报告支撑

---

## 问题 2：Tauri + React 被称为"最受欢迎的组合"缺乏依据

**位置**：第1节"主流组合"第一条

**问题**：
- research-1 的 Tauri 项目中，React 使用案例（win-system-monitor）仅1个
- research-3 中 React 项目有 DPanel、r-shell、PulsePrint-Desktop 等，但 Svelte/Vue 同样有代表项目
- 研究报告未给出"最受欢迎"的排名依据，分析报告自行断言 React 最受欢迎属于无依据推断

---

## 问题 3：代码示例使用已废弃 API

**位置**：第5节"性能优化难点"代码示例

```rust
use sysinfo::{System, SystemExt, ProcessorExt};
// ...
sys.global_processor_info().cpu_usage()
```

**问题**：
- `SystemExt` 和 `ProcessorExt` trait 在 sysinfo 0.30+ 已被移除，该版本改为直接在 `System` 上调用方法
- `global_processor_info()` 方法已废弃，新 API 为 `System::global_cpu_usage()`
- 正确写法应为：
```rust
use sysinfo::System;
let mut sys = System::new_all();
sys.refresh_all();
let cpu_usage = sys.global_cpu_usage();
```
- 此代码示例会导致编译失败，误导开发者

---

## 问题 4：系统托盘插件名称错误

**位置**：第5节"系统集成难点 - 系统托盘兼容性"

**问题**：
- 报告写 `tauri-plugin-system-tray`
- Tauri 2.0 中正确的插件名为 `tauri-plugin-tray`（已从 v1 的内置功能拆分为独立插件）
- 使用错误插件名会导致开发者找不到对应依赖

---

## 问题 5：最佳实践推荐逻辑矛盾

**位置**：第4节"按功能完整度排名"

**问题**：
- PulseCoreLite（8 stars）功能完整度评为 ⭐⭐⭐⭐⭐，高于 neohtop（8,940 stars）的 ⭐⭐⭐⭐⭐（同分）
- 但 pachtop（176 stars）在"按 Star 数排名"中排第2，功能完整度却未出现在该榜单
- omnimon（2 stars）和 ActioWatch（2 stars）功能完整度均为 ⭐⭐⭐⭐，但两者 star 数极低，社区验证不足，推荐为"最佳实践"依据不充分
- 建议：低 star 项目应标注"参考价值有限，未经社区广泛验证"

---

## 问题 6：glance 技术栈描述不完整

**位置**：第3节"仪表盘模式 - 代表项目"

**问题**：
- 报告将 glance 列为仪表盘模式代表项目，但 research-5 明确标注其技术栈为 **Tauri 2.0 + Svelte 5**
- 分析报告在技术栈表格中将 glance 归入 Svelte 类，但在架构模式章节未说明其具体技术栈，信息不一致

---

## 问题 7：research-4 内容未被利用

**位置**：整体报告

**问题**：
- research-4.md 是通用系统监控工具综述（htop、btop++、Glances、NetData 等），与 Tauri 生态无直接关联
- 分析报告未说明 research-4 的定位和与 Tauri 生态的关系对比
- 若报告目的是指导 Tauri 应用开发，research-4 的竞品对比价值（Electron vs Tauri 性能优势）应被显式引用，而非忽略

---

## 问题 8：开发建议缺少安全相关内容

**位置**：第6节"开发建议"

**问题**：
- research-3 中有 omnimon（安全监控）、netguard-desktop（安全监控）等安全类项目
- 分析报告的开发建议完全未提及 Tauri 应用的安全配置（如 CSP 设置、allowlist/capability 配置、IPC 安全）
- Tauri 2.0 引入了全新的 capability-based 权限系统，这是开发建议中的重要缺失项

---

## 汇总

| # | 问题类型 | 严重程度 |
|---|---------|---------|
| 1 | 技术栈数量统计夸大，无研究报告支撑 | 高 |
| 2 | "React 最受欢迎"断言无依据 | 中 |
| 3 | 代码示例使用已废弃 sysinfo API，编译失败 | 高 |
| 4 | 系统托盘插件名称错误（Tauri 2.0）| 高 |
| 5 | 低 star 项目推荐为最佳实践，依据不足 | 中 |
| 6 | glance 技术栈描述不完整/不一致 | 低 |
| 7 | research-4 竞品对比价值未被引用 | 低 |
| 8 | 开发建议缺少 Tauri 2.0 安全配置内容 | 中 |
