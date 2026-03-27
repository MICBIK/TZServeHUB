# ServerHUB Roadmap

> 版本：v1.0 | 日期：2026-03-19

## MVP (已完成)

ServerHUB v1.0 MVP 已完成核心监控能力，包括：

### 数据采集层
- **NodeExporter Adapter**: 支持 Prometheus node_exporter 作为数据源
- **GoAgent Adapter**: 自研 Go agent，提供 JSON API 接口
- **Adapter 抽象层**: `MetricAdapter` trait，支持多数据源统一接入

### 核心功能
- **Host Metrics**: CPU（总/每核心）、内存、磁盘 I/O、磁盘空间监控
- **基础 Probe**: Ping (ICMP)、TCP 连通性、DNS 解析探测
- **数据管道**: Counter→Rate 转换、三级存储（raw/1m/15m rollup）
- **本地存储**: SQLite (WAL mode) + 自动清理策略
- **桌面 UI**: React + Zustand + Recharts，支持多服务器切换

### 技术基础设施
- Tauri 2 桌面壳 + Rust 后端
- 适配器化架构，易于扩展新数据源
- 轮询调度器（支持 jitter/backoff）
- 基础告警引擎（阈值 + 持续时间）

---

## Future v2: 增强监控能力

### Glances Adapter
**目标**: 支持 Glances API 作为第三方数据源

**技术要点**:
- 实现 `GlancesAdapter` (`src-tauri/src/adapters/glances.rs`)
- 解析 Glances JSON API (`/api/3/all`)
- 映射 Glances 指标到 `RawMetric` 统一格式
- 支持 Glances 插件扩展（Docker、GPU、Sensors）

**优先级**: Medium
**预估工作量**: 3-5 天

### Alert Notifications
**目标**: 告警触发后发送通知到外部渠道

**功能范围**:
- 桌面通知（已有基础，需增强）
- Webhook 通知（Slack、Discord、企业微信）
- Email 通知（SMTP）
- 通知模板系统（支持自定义消息格式）
- 通知去重和静默期（避免告警风暴）

**技术要点**:
- 扩展 `AlertNotifier` (`src-tauri/src/alerts/notifier.rs`)
- 新增 `NotificationChannel` trait
- 配置管理（通知渠道凭证存储）

**优先级**: High
**预估工作量**: 5-7 天

---

## Future v3: 高级监控特性

### Probe Vantage Points
**目标**: 支持从多个观测点探测网络质量

**功能范围**:
- **Desktop-local probe**: 从桌面应用本地发起探测（已有基础）
- **Remote-agent probe**: 从远端 agent 发起探测（服务器间互探）
- **Third-party probe**: 集成第三方探测服务（如 Pingdom、UptimeRobot）
- Vantage point 标签系统（区分探测来源）
- 多点探测结果聚合和对比视图

**技术要点**:
- 扩展 Go agent 支持主动探测能力
- 新增 `ProbeVantagePoint` 枚举和配置
- 前端新增多点探测对比图表

**优先级**: Medium
**预估工作量**: 7-10 天

### Traffic Analytics
**目标**: 深度分析网络流量特征

**功能范围**:
- **网卡流量排名**: 按流量大小排序网卡接口
- **流量突发检测**: 识别异常流量峰值（基于统计阈值）
- **流量趋势分析**: 日/周/月流量对比
- **Top Talkers**: 识别流量最大的进程/连接（需 agent 支持）
- **流量预测**: 基于历史数据预测未来流量（可选 ML 模型）

**技术要点**:
- 扩展 `DerivedMetricsEngine` 支持流量统计
- 新增 `TrafficAnalyzer` 模块
- 前端新增流量分析专用页面
- 可选：集成 eBPF 采集（需 Linux agent 支持）

**优先级**: Low
**预估工作量**: 10-14 天

---

## 边界说明

### MVP 与 Future 的分界线

**MVP 聚焦**:
- 核心监控能力（Host Metrics + 基础 Probe）
- 单一数据源（NodeExporter + GoAgent）
- 本地存储和基础告警
- 桌面端单点观测

**Future 扩展**:
- 多数据源适配（Glances、第三方 API）
- 告警通知外发（Webhook、Email）
- 多点探测和流量深度分析
- 高级分析能力（预测、异常检测）

### 不在 Roadmap 内的功能

以下功能明确不在当前规划范围：
- **集群管理**: 不支持服务器分组、批量操作
- **配置管理**: 不支持远程配置下发、Ansible 集成
- **日志采集**: 不支持日志聚合和分析
- **APM 能力**: 不支持应用性能监控（trace、span）
- **多租户**: 不支持多用户、权限管理
- **云原生集成**: 不支持 Kubernetes、Prometheus Operator

ServerHUB 定位为**轻量级桌面监控工具**，专注于远程服务器的系统资源监控，不演化为重型监控平台。

---

## 版本规划

| 版本 | 目标 | 预计时间 |
|------|------|----------|
| v1.0 (MVP) | 核心监控能力 | 已完成 |
| v2.0 | Glances + Alert Notifications | Q2 2026 |
| v3.0 | Probe Vantage Points + Traffic Analytics | Q3 2026 |

---

## 贡献指南

如需提议新功能或修改 Roadmap，请：
1. 在 GitHub Issues 中创建 Feature Request
2. 说明功能场景和技术可行性
3. 评估与现有架构的兼容性

Roadmap 优先级会根据社区反馈动态调整。
