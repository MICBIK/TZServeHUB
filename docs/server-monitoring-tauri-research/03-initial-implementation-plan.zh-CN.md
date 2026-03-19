# Tauri 服务器监控应用初步实现方案

检查日期：2026-03-18
审计日期：2026-03-19

## 产品目标

做一个本地 `Tauri` 桌面应用，用来监控一台或多台远端服务器，覆盖：

- CPU 总使用率与每核心使用率
- 网络流量
- 网络波动
- 磁盘读写速度
- 磁盘总量、已用、可用空间
- 历史视图与基础告警

从实现上，建议把需求明确拆成三层：

- `主机指标`：CPU、磁盘 I/O、文件系统空间、网卡吞吐
- `网络质量`：Ping 时延、TCP 连通、DNS 解析、丢包
- `流量分析`：接口排行、流量突增、未来的 top talkers / flow 视角

## 推荐产品形态

推荐路线：`桌面控制平面 + 适配器化远端采集`

原因：

- 桌面应用适合做展示、通知、本地缓存、离线查看和 operator 控制台。
- 桌面应用不适合直接承担所有远端主机采集，尤其是完全依赖 SSH 的时候。
- 成熟开源方案几乎都把 `采集` 和 `UI` 分开。
- 网络波动和主机资源本来就是两套不同的问题，最好在架构上先分层。

## MVP 架构

```text
Remote Server
  |- node_exporter 或 Glances API
  |- 可选 probe target
  |- 未来可选自研轻量 agent
  |- 私有网络或带鉴权网关

Local Tauri App
  |- Rust core
  |- React UI
  |- polling scheduler
  |- adapter layer
  |- probe worker
  |- derived metrics engine
  |- local SQLite
  |- alert engine
  |- tray / notifications

Flow
  1. 拉取主机指标
  2. 执行主动探测
  3. 统一归一化到内部 schema
  4. 计算速率、利用率和派生指标
  5. 写入本地历史库
  6. 渲染图表并触发告警
```

## 技术选型建议

- 桌面壳：`Tauri 2`
- 前端：`React + TypeScript`
- 轮询模型：v1 先用 interval polling，不急着做流式订阅
- 本地存储：`SQLite`，开启 `WAL`
- 历史策略：原始点 + rollup 聚合表
- 首个远端数据源：`node_exporter`
- 第二适配器：`Glances`
- 探测子系统：本地执行 `Ping / TCP / DNS`
- 后续流量扩展：预留 `ntopng` 或自研 network collector 适配位
- 安全接入：`WireGuard / Tailscale / VPN / SSH Tunnel / 带鉴权 HTTPS 网关`

## 为什么不推荐 v1 直接走 SSH 采集

- SSH 采集在不同 Linux 发行版上的兼容成本很高。
- 解析 shell 输出不稳定，维护成本远高于 exporter 指标。
- exporter 更适合做多机轮询，也更容易统一成内部 schema。
- SSH 更适合作为未来的“诊断 / 一键安装 / 排障辅助”入口，而不是第一采集通道。

## 安全接入基线

- 不要把“把 exporter 原始端口直接暴露到公网”当成默认接入方案。
- 优先走 `WireGuard`、`Tailscale`、`VPN` 或 `SSH Tunnel` 这类私有可达路径。
- 如果确实需要 HTTPS 和鉴权，建议放到带认证的网关之后，或使用 exporter 支持的 TLS/Auth 工具链。
- 每台服务器都应记录自己的信任方式：`私网覆盖`、`隧道`、或 `鉴权网关`。

## Probe 视角模型

- 由 Tauri 客户端发起的探测，测到的是 `当前桌面到目标` 的网络路径。
- 这对 operator 视角的连通性很有价值，但不能直接代表服务器自身网络健康。
- 每条探测结果都应带 `vantage_point` 字段，例如 `desktop-local` 或 `remote-agent`。
- 如果以后要看“服务器侧网络质量”，应该新增远端探测 worker，而不是误用桌面探测结果。

## v1 非目标

- 不做 Prometheus / Netdata / Checkmk 级别的完整可观测性平台
- 不做抓包或深度流量分析
- 不急着写自研远端 agent
- 不做多用户协作和复杂 RBAC

## 建议的内部模块

### 1. Server Registry

- 保存服务器信息、标签、轮询间隔、认证信息、采集源类型
- 首期支持 `node_exporter` 和 `Glances`

### 2. Metric Adapter Layer

- 把不同来源的指标统一成一套内部 schema
- 建议先统一为：
  - `cpu.total`
  - `cpu.core.{index}`
  - `network.if.{name}.rx_bytes`
  - `network.if.{name}.tx_bytes`
  - `disk.dev.{name}.read_bytes_per_sec`
  - `disk.dev.{name}.write_bytes_per_sec`
  - `disk.fs.{mount}.total_bytes`
  - `disk.fs.{mount}.used_bytes`
  - `disk.fs.{mount}.free_bytes`
  - `probe.ping.latency_ms`
  - `probe.tcp.connect_ms`
  - `probe.dns.lookup_ms`

### 3. 指标语义守卫

- 明确标记每个指标是 `counter`、`gauge`、`state` 还是 `derived rate`。
- 要处理主机重启、exporter 重启、网卡重置、设备重命名导致的 counter reset。
- 保留源时间戳和 staleness 标记，让图表能区分 `值为 0` 和 `数据缺失`。
- 网卡、磁盘设备、文件系统要分开建模，因为它们跨重启后的身份漂移方式不同。

### 4. 本地时序存储

- 本地保留近期高频数据，并做分层聚合
- 建议保留：
  - 7 天原始点，5 到 10 秒粒度
  - 30 天 1 分钟聚合
  - 90 天 15 分钟聚合

### 5. 派生指标引擎

- 负责把 counter 算成速率，把原始值算成利用率和移动平均
- 这层要独立，否则后面接 `Glances / ntopng / 自研 agent` 会很乱

### 6. 告警引擎

- 先做阈值告警：CPU 过高、磁盘空间不足、磁盘 I/O 饱和、主机不可达、丢包升高
- 需要冷却时间和去重

### 7. Probe Worker

- 主动执行 `Ping / TCP / DNS`
- 这层就是“网络波动”能力的核心，不要混进主机指标适配层里

### 8. 安全与接入

- 本地凭据优先使用系统安全存储
- 尽量使用 token / basic-auth，而不是 shell 凭据
- SSH 先留作可选诊断入口

### 9. 运行期保护

- 抓取要做并发上限，不能让单台慢机器拖住整个轮询周期。
- SQLite 写入要批量化，保留清理和压缩任务放后台执行。
- 重试要克制，很多场景下快速失败并标记 stale 比堆积重试更可靠。
- 原始点和聚合点要物理分离，避免保留策略和告警计算互相污染。

## 推荐的 MVP 页面

- 服务器列表页
- 服务器总览页
- CPU 页面
- 网络页面
- 磁盘页面
- Probe 页面
- 告警与事件页
- 设置与接入页

## 分阶段落地

### Phase 1：指标 MVP

- 只支持 1 台 Linux + `node_exporter`
- 打通 CPU、每核心、网卡流量、磁盘 I/O、磁盘空间
- 建立统一 schema、本地历史库、速率计算
- 要求 exporter 通过私网或隧道接入
- 从第一天开始把 probe 标记成 `desktop-local`
- 暂不引入复杂鉴权模型，优先保证数据链路正确

### Phase 2：可用性 MVP

- 支持多服务器
- 增加桌面通知、告警和更好的历史视图
- 增加 `Ping / TCP / DNS` 探测

### Phase 3：适配器扩展

- 增加 `Glances` 适配器
- 增加接入诊断和配置助手
- 增加接口级和磁盘级筛选
- 加入受 `ntopng` 启发的流量概览

### Phase 4：是否自研 Agent

- 等 v1 使用反馈明确以后再决定
- 只有 exporter 方案卡住产品目标时，再考虑自研 agent

## 主要风险

- “网络波动”定义不清，会把主机指标和主动探测混成一团
- 轮询过密又不做聚合，会把本地数据库快速撑大
- 太早自研 agent，会把项目复杂度拉到不必要的高度
- 原始 counter 和派生速率如果混存，后面的图表和告警会变复杂
- 如果把桌面侧探测误当成服务器侧真相，会得出错误的网络结论
- 为了方便而把 exporter 裸露到公网，会引入完全可以避免的安全弱点

## 推荐的下一步实现切片

- 先锁定一个参考切片：`Tauri 桌面端 + node_exporter + 本地 SQLite + Ping probe`
- 先把内部指标 schema 定稳，再接第二个适配器
- 第一套部署必须放在私网覆盖网络或隧道里，不要直接抓公网 exporter 端口
- `Glances`、`ntopng`、自研 agent 都放到后续阶段，不作为首版前置依赖
