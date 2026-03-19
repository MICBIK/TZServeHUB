# Server Monitoring 开源项目对比表

检查日期：2026-03-18
审计日期：2026-03-19

说明：

- `Yes`：明确支持或官方直接暴露
- `Partial`：能支持，但不是核心强项，或需要额外配置
- `No`：不是这个项目的主要能力

| 项目 | 产品形态 | CPU | 每核心 CPU | 主动探测 | 流量统计 | 磁盘 I/O 速率 | 磁盘空间 | 历史 | 告警 | 远端模式 | 对桌面/Tauri参考价值 | 结论 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Beszel | Web Hub + Agent | Yes | Partial | No | Yes | Yes | Yes | Yes | Yes | Agent | Low | 最接近你的产品形态 |
| Netdata | Web Dashboard + Agent | Yes | Yes | No | Yes | Yes | Yes | Yes | Yes | Agent / Collector | Low | 最强的图表与指标覆盖参考 |
| Glances | TUI + Web UI + API | Yes | Yes | No | Yes | Partial | Yes | Partial | Partial | Client/Server + API | Low | 最适合做轻量远端适配参考 |
| node_exporter | Exporter | Yes | Yes | No | Partial | Partial | Yes | No | No | Exporter | None | 最适合做指标模型基准 |
| Uptime Kuma | Web 探测面板 | No | No | Yes | No | No | No | Yes | Yes | 主动探测 | None | 最适合补网络波动能力 |
| Nezha | Web 运维台 + Agent | Yes | Partial | Yes | Partial | Partial | Yes | Yes | Yes | Agent | Low | 轻量监控 + 运维组合参考 |
| Cockpit | Web 管理面板 | Partial | Partial | No | Partial | Partial | Partial | Partial | No | 服务端 Web | Low | 轻运维工作流参考 |
| Checkmk Raw | 基础设施监控平台 | Yes | Partial | Yes | Partial | Partial | Yes | Yes | Yes | Agent + Agentless | Low | 规模化与插件化参考 |
| ntopng | 网络流量分析平台 | No | No | No | Yes | No | No | Yes | Partial | 流量采集器 | None | 流量与网络视角专项参考 |
| Nagstamon | 桌面监控客户端 | No | No | No | No | No | No | No | Yes | 聚合外部后端 | Medium | 桌面控制台形态参考 |
| NeoHtop | Tauri 桌面应用 | Yes | Partial | No | No | No | No | No | No | 仅本机 | High | Tauri 监控 UX 参考 |
| Pachtop | Tauri 桌面应用 | Yes | Partial | No | Partial | Partial | Yes | No | No | 仅本机 | High | Tauri 技术实现参考 |
| Mission Center | 桌面系统监控器 | Yes | Partial | No | Partial | Partial | Yes | No | No | 仅本机 | Medium | 高密度桌面图表参考 |

## 各项目最值得借的点

| 项目 | 能借什么 | 不解决什么 |
| --- | --- | --- |
| Beszel | 多机监控、历史、告警、主机指标产品 framing | Tauri 桌面结构 |
| Netdata | 秒级图表、指标广度、排障视图 | 轻量产品边界 |
| Glances | 远端 API、轻部署、快速适配 | 长期产品结构 |
| node_exporter | 指标命名、主机指标 schema | UI、历史、告警 |
| Uptime Kuma | 主动探测、时延曲线、状态模型 | CPU / 磁盘等主机资源 |
| Nezha | 小规模节点场景下的实用产品边界 | 桌面端本地工作流 |
| Cockpit | 监控与运维相邻的操作体验 | 深度可观测性 |
| Checkmk Raw | 发现机制、插件架构、规则/告警体系 | 轻量桌面产品 |
| ntopng | 网络接口、主机、吞吐、时延视角 | 通用服务器资源监控 |
| Nagstamon | 托盘优先、告警聚合、桌面控制台思路 | 原始指标采集 |
| NeoHtop | Tauri UI 密度、Rust Bridge、自动刷新 | 远端机群 |
| Pachtop | React + Tauri 的落地方式 | 多机与历史存储 |
| Mission Center | 图表密度、桌面端性能与资源页布局 | 远端采集与机群管理 |

## 你的产品短名单

| 角色 | 推荐项目 |
| --- | --- |
| 产品形态基准 | Beszel |
| 指标命名与主机 schema 基准 | node_exporter |
| 轻量远端适配基准 | Glances |
| 网络波动基准 | Uptime Kuma |
| 网络流量分析基准 | ntopng |
| 桌面控制台基准 | Nagstamon |
| 运维工作流基准 | Cockpit |
| Tauri UI 基准 | NeoHtop |
| Tauri 技术实现基准 | Pachtop |

## 结论

- 想最快做出 MVP：`Tauri UI + node_exporter` 是最稳的一条线。
- 想做完整一点的产品 benchmark：先看 `Beszel`，再把 `Netdata` 和 `Checkmk` 当上限参考。
- “网络波动”不能只靠主机流量图解决，最好同时引入 `Uptime Kuma` 式探测能力。
- 桌面端体验不要只看一个项目，建议把 `Nagstamon + NeoHtop + Pachtop + Mission Center` 一起当 UI 参考组。

## 审计备注

- `流量统计` 和 `磁盘 I/O 速率` 对 exporter 类项目来说，很多时候需要本地根据 counter 做 rate 计算，并不是现成的速度字段。
- `对桌面/Tauri参考价值` 只表示它对桌面 UI、运行时结构、操作流的借鉴价值，不代表它适合作为后端 benchmark。
- 这轮审计把原先混在一起的 `历史/告警` 拆开了，因为两者放在一列会掩盖 `Cockpit`、`node_exporter`、`ntopng` 这类项目的真实差异。
