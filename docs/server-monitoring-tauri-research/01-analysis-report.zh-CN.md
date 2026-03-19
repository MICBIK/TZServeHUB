# Server Monitoring Tauri 应用调研报告

检查日期：2026-03-18
审计日期：2026-03-19

## 目标

要做一个本地 `Tauri` 桌面应用，用来监控远端服务器，重点覆盖：

- CPU 使用率
- 每核心使用率
- 网络波动与流量统计
- 磁盘读写速度
- 磁盘总量、已用、可用空间

## 核心判断

- 开源市场里成熟方案仍以 `Web / self-hosted` 为主，这不是偶然。远端采集、历史存储、告警、多机管理，本来就更适合 `hub + agent` 或 `exporter + dashboard`。
- 成熟产品几乎都把 `采集` 和 `展示` 分开。UI 只是指标的消费者，不是指标的来源。
- 成型的 `Tauri` 监控项目确实存在，但大多偏“本机系统监控”，不是“远端服务器监控”。
- 你的问题最好拆成三层：`主机资源监控`、`网络质量探测`、`网络流量分析`。市面上的开源项目通常很少能三层都做得同样强。
- 最稳的产品路径不是“桌面端直接 SSH 采集一切”，而是 `Tauri 本地客户端 + 远端 exporter/agent + 本地历史缓存`。

## 项目分析

### 1. Beszel

- 定位：轻量级服务器监控平台，典型 `hub + agent` 架构。
- 价值：最接近你要做的产品形态，带历史数据、告警、容器统计、多机视图。
- 借鉴点：主机指标模型、Agent 部署方式、阈值告警、总览页布局。
- 不适合直接照搬：它的中心思路仍然是 Web Hub。
- 来源：[GitHub](https://github.com/henrygd/beszel) | [官方文档](https://www.beszel.dev/guide/what-is-beszel)

### 2. Netdata

- 定位：高实时性、广覆盖的基础设施监控平台。
- 价值：图表密度、秒级数据、异常提示、排障链路都很成熟。
- 借鉴点：图表交互、秒级粒度、按主机到服务的钻取路径。
- 风险：产品复杂度明显高于你的 v1 目标。
- 来源：[GitHub](https://github.com/netdata/netdata)

### 3. Glances

- 定位：跨平台系统监控工具，支持 TUI、Web UI、REST API、Client/Server 模式。
- 价值：证明“轻量远端监控”可以不依赖重型平台。
- 借鉴点：API 化暴露、低门槛接入、轻量部署方式。
- 风险：产品结构和 UI 完整度不如 Beszel / Netdata。
- 来源：[GitHub](https://github.com/nicolargo/glances)

### 4. Prometheus node_exporter

- 定位：主机指标 exporter，专门暴露 CPU、filesystem、network、diskstats 等指标。
- 价值：最适合作为你内部指标命名和采集模型的参考基准。
- 借鉴点：指标命名、每核心 CPU、文件系统空间、网卡流量、磁盘计数器模型。
- 风险：它不是产品，只是数据源，而且“流量速度”“磁盘读写速度”本质上都需要基于计数器做时间差计算。
- 来源：[GitHub](https://github.com/prometheus/node_exporter) | [官方指南](https://prometheus.io/docs/guides/node-exporter/)

### 5. Uptime Kuma

- 定位：可用性与网络探测工具，擅长 HTTP、TCP、Ping、DNS 等主动检查。
- 价值：非常适合补足你说的“网络波动”这一块。
- 借鉴点：探测任务定义、时延曲线、状态抽象、通知机制。
- 风险：它不是完整的主机资源监控产品。
- 来源：[GitHub](https://github.com/louislam/uptime-kuma)

### 6. Nezha

- 定位：轻量服务器与站点监控平台，结合了监控和运维入口。
- 价值：很贴近 VPS / 小规模节点场景下的真实用户需求。
- 借鉴点：资源监控和运维操作入口的组合方式。
- 风险：本质仍然是 Web 运维台，不是本地桌面产品。
- 来源：[GitHub](https://github.com/nezhahq/nezha)

### 7. Cockpit

- 定位：Linux 服务器的 Web 管理界面，覆盖存储、网络、日志、容器等。
- 价值：适合参考“轻运维 + 轻监控”的产品 framing。
- 借鉴点：主机切换、概览页布局、监控与运维相邻的工作流。
- 风险：指标深度不如 Netdata / Checkmk。
- 来源：[GitHub](https://github.com/cockpit-project/cockpit) | [官网](https://cockpit-project.org/)

### 8. Checkmk Raw

- 定位：完整的基础设施监控平台，支持 agent、agentless、发现、规则、告警。
- 价值：适合研究大规模监控产品的插件化与自动发现机制。
- 借鉴点：插件架构、服务发现、规则引擎、告警生命周期。
- 风险：范围太大，容易把 v1 做成“企业监控平台”。
- 来源：[GitHub](https://github.com/Checkmk/checkmk) | [产品页](https://checkmk.com/product/raw-edition)

### 9. ntopng

- 定位：网络流量与安全分析平台，擅长接口、主机、协议、吞吐、时延等网络视角。
- 价值：是“网络波动 / 流量分析”这条线最值得看的专项参考。
- 借鉴点：按接口与主机看流量、突发流量、网络钻取路径。
- 风险：不适合作为主机资源监控基座，而且 ntopng 产品线里部分高级能力存在 edition 差异。
- 来源：[GitHub](https://github.com/ntop/ntopng) | [Wiki](https://github.com/ntop/ntopng/wiki)

### 10. Nagstamon

- 定位：桌面状态监控客户端，可聚合 Nagios、Icinga、Checkmk、Prometheus、Zabbix 等后端。
- 价值：证明“桌面端作为监控控制台”是成立的产品形态。
- 借鉴点：托盘优先、告警聚合、状态压缩展示、从告警跳转到操作。
- 风险：它消费的是已有监控系统，不是原始主机采集器。
- 来源：[GitHub](https://github.com/HenriWahl/Nagstamon) | [官网](https://nagstamon.de/)

### 11. NeoHtop

- 定位：`SvelteKit + Rust + Tauri` 的桌面系统监控器。
- 价值：很适合借鉴 Tauri 应用结构、Rust Bridge、桌面监控 UI。
- 借鉴点：高密度视图、自动刷新、进程列表、桌面交互。
- 风险：偏本机，不解决远端采集。
- 来源：[GitHub](https://github.com/Abdenasser/neohtop)

### 12. Pachtop

- 定位：`React + TypeScript + Rust + Tauri` 的本机系统监控器。
- 价值：更接近你可能采用的前端栈。
- 借鉴点：Dashboard 结构、磁盘页、网络页、Rust 指标采集接法。
- 风险：仍是 local-first，缺少多机与历史能力。
- 来源：[GitHub](https://github.com/pacholoamit/pachtop)

### 13. Mission Center

- 定位：Rust 桌面系统监控器，擅长高效图表和资源页渲染。
- 价值：适合参考高密度图表、资源页布局和渲染效率。
- 借鉴点：图表密度、资源聚合、细分详情页。
- 风险：本机监控，不解决远端机群问题。
- 来源：[官网](https://missioncenter.io/) | [源码](https://gitlab.com/mission-center-devs/mission-center)

## 推荐参考组合

- 产品形态基准：`Beszel`
- 指标模型基准：`node_exporter`
- 轻量远端适配基准：`Glances`
- 网络探测基准：`Uptime Kuma`
- 网络流量分析基准：`ntopng`
- 桌面控制台基准：`Nagstamon`
- 运维工作流基准：`Cockpit`
- Tauri 桌面 UI 基准：`NeoHtop + Pachtop`
- 高密度桌面图表基准：`Mission Center`

## 审计修正

- 不建议把原始 `node_exporter` 端口直接暴露到公网，默认应走 `VPN / SSH Tunnel / 反向代理 + TLS + 鉴权 / hub-agent` 之一。
- 要明确区分“桌面侧探测”和“服务器侧网络健康”。Tauri 客户端发出的 Ping 测到的是 `当前桌面到目标` 的路径质量。
- 文档里提到的“流量速度”和“磁盘 I/O 速度”在很多 exporter 里不是现成字段，而是需要本地做 rate 计算的派生指标。

## 结论

- 推荐产品路线：`Tauri 本地客户端作为控制平面`，远端数据采集继续使用 `exporter/agent`。
- 推荐 v1 边界：只做 `主机指标 + 探测历史 + 流量概览 + 告警 + 桌面工作流`。
- 推荐架构约束：从第一天就把 `主机指标`、`网络探测`、`流量分析` 拆成独立模块，即使它们最后展示在同一个 UI 里。
- 推荐接入基线：优先使用 `VPN / WireGuard / Tailscale / SSH Tunnel / 带鉴权网关`，不要把 exporter 裸露到公网当默认方案。
