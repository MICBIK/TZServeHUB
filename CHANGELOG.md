# 变更日志

本文档记录 ServerHUB 项目的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

---

## [Unreleased]

### 计划中
- node_exporter 适配器完整实现
- 网络质量探测（Ping/TCP/DNS）
- 流量分析和排行
- 自定义仪表盘

---

## [0.1.0] - 2026-03-19

### Added

#### 桌面应用框架
- 基于 Tauri 2 的跨平台桌面应用
- React 18 + TypeScript + Vite 前端框架
- Tailwind CSS 样式系统
- Zustand 全局状态管理
- Recharts 图表库集成

#### 服务器管理
- 添加/删除/编辑服务器配置
- 服务器列表展示和筛选
- 服务器启用/禁用开关
- 支持三种适配器类型：
  - NodeExporter（Prometheus 文本格式）
  - Glances（REST API）
  - GoAgent（自定义 JSON API）
- 支持三种访问方式：
  - Private（私有网络）
  - Tunnel（SSH 隧道）
  - Gateway（网关代理）

#### Go Agent 远程代理
- 基于 Go + Gin 的轻量级 HTTP 服务器
- gopsutil 系统指标采集
- Bearer Token 认证机制
- JSON API 响应格式
- 支持指标：
  - CPU 使用率（总体 + 单核）
  - 内存使用情况（总量/已用/可用）
  - 磁盘分区信息和 I/O 统计
  - 网络接口流量统计

#### 适配器层
- `MetricAdapter` trait 定义统一接口
- `GoAgentAdapter` 完整实现
- `NodeExporterAdapter` 骨架实现
- 指标标准化为 `RawMetric` 格式
- 支持三种指标类型：
  - Counter（累计计数器）
  - Gauge（瞬时值）
  - State（状态值）

#### 数据存储
- SQLite 数据库（WAL 模式）
- 数据库迁移系统（sqlx::migrate）
- 三层数据保留策略：
  - 原始数据：7 天，全分辨率
  - 1 分钟聚合：30 天
  - 15 分钟聚合：90 天
- 数据表：
  - `servers`：服务器配置
  - `raw_metrics`：原始指标数据
  - `metrics_1m`：1 分钟聚合数据
  - `metrics_15m`：15 分钟聚合数据
  - `alert_rules`：告警规则
  - `alert_events`：告警事件

#### 指标处理引擎
- `DerivedMetricsEngine`：Counter → Rate 转换
- 计数器重置检测和处理
- 过期状态自动清理
- `RollupEngine`：数据聚合（min/max/avg）
- `RetentionManager`：数据保留策略执行

#### 告警系统
- `AlertEngine`：规则评估引擎
- 支持三种条件：Gt（大于）/ Lt（小于）/ Eq（等于）
- 持续时间判定（避免瞬时抖动）
- 冷却时间机制（防止告警风暴）
- 桌面通知推送（tauri-plugin-notification）
- 告警状态：Firing（告警中）/ Resolved（已恢复）

#### 探测系统（骨架）
- `PingProbe`：ICMP Ping 探测（surge-ping）
- `TcpProbe`：TCP 端口探测（tokio::net::TcpStream）
- `DnsProbe`：DNS 解析探测（UDP Socket）
- 观测点标记（desktop-local / remote-agent）

#### Tauri Commands API
- `list_servers`：列出所有服务器
- `add_server`：添加新服务器
- `remove_server`：删除服务器
- `get_metrics`：获取当前指标
- `get_metric_history`：获取历史指标
- `get_settings`：获取应用设置
- `update_settings`：更新应用设置

#### 前端页面
- Dashboard：服务器概览
- CPU 监控页面：CPU 使用率时序图
- 内存监控页面：内存使用情况
- 磁盘监控页面：磁盘空间和 I/O
- 网络监控页面：网络流量统计
- 探测页面：网络质量探测（骨架）
- 告警页面：告警规则和事件
- 设置页面：应用配置

#### 文档
- `README.md`：项目介绍和快速开始
- `CLAUDE.md`：开发指南（给 Claude Code 使用）
- `docs/architecture.md`：架构文档
- `docs/specs/API.md`：API 文档
- `docs/specs/PRD.md`：产品需求文档
- `docs/specs/TESTING.md`：测试策略
- `docs/specs/TECHNICAL_SPEC.md`：技术规格说明
- `docs/specs/ROADMAP.md`：开发路线图
- `docs/specs/DEPLOYMENT.md`：部署指南
- `docs/specs/CHANGELOG.md`：变更日志（本文件）

#### 开发工具
- ESLint + TypeScript 代码检查
- Rust Clippy 代码检查
- SQLx 编译时 SQL 验证
- Git 版本控制

### Changed
- 无（首次发布）

### Deprecated
- 无

### Removed
- 无

### Fixed
- 无

### Security
- Go Agent Bearer Token 认证
- SQLite WAL 模式防止数据损坏
- 输入验证（服务器配置）

---

## 版本说明

### 版本号规则
- **主版本号（Major）**：不兼容的 API 变更
- **次版本号（Minor）**：向下兼容的功能新增
- **修订号（Patch）**：向下兼容的问题修复

### 变更类型
- **Added**：新增功能
- **Changed**：现有功能的变更
- **Deprecated**：即将移除的功能
- **Removed**：已移除的功能
- **Fixed**：问题修复
- **Security**：安全相关的修复

---

## 未来版本预告

### v0.2.0（计划 2026-Q2）
- node_exporter 适配器完整实现
- 网络质量探测功能
- 流量分析和排行
- 告警历史记录

### v0.3.0（计划 2026-Q3）
- 自定义仪表盘
- 数据导出（CSV/JSON）
- 趋势分析和预测
- Glances 适配器

### v0.4.0（计划 2026-Q4）
- 多用户支持
- 通知渠道集成（Slack/钉钉/邮件）
- 事件时间线
- 协作功能

### v1.0.0（目标 2027-Q1）
- 高可用部署
- Prometheus 集成
- 企业级功能
- 生产级稳定性

---

## 贡献指南

如果您想为 ServerHUB 贡献代码或报告问题，请参考：
- [贡献指南](../CONTRIBUTING.md)（待创建）
- [GitHub Issues](https://github.com/ha1den/serverhub/issues)（待创建）

---

## 许可证

ServerHUB 采用 MIT 许可证。详见 [LICENSE](../LICENSE) 文件。

---

## 致谢

感谢所有为 ServerHUB 做出贡献的开发者和用户！

特别感谢以下开源项目：
- [Tauri](https://tauri.app/)
- [React](https://react.dev/)
- [Rust](https://www.rust-lang.org/)
- [Go](https://go.dev/)
- [SQLite](https://www.sqlite.org/)
- [Prometheus](https://prometheus.io/)

---

[Unreleased]: https://github.com/ha1den/serverhub/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ha1den/serverhub/releases/tag/v0.1.0
