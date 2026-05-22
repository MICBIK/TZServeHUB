# ServerHUB · Project Brief

## 一句话定位

> 个人多 VPS 监控面板 —— 一台桌面应用集中看自己手里所有远端服务器的健康状况。

## 项目身份

- **名称**：ServerHUB
- **类型**：Tauri 2 桌面应用（控制平面），配套 Go remote agent
- **当前版本**：v0.1.0
- **目标用户**：自己管理 3-20 台 VPS 的开发者 / 自托管爱好者 / 小团队 ops
- **作者**：ha1den
- **License**：MIT
- **代码仓库根目录**：`/Users/baihaibin/Documents/WorkSpares/ServerHUB`

## 核心目标

1. **集中监控**：在一个桌面窗口里看到所有 VPS 的实时主机/网络/流量信号
2. **零摩擦上手**：用 SSH（密钥或账密）一键把 Go agent 部署到目标机器，不需要手动 scp + systemd 配置
3. **本地优先 / 隐私优先**：所有数据本地 SQLite，不上云、不要登录
4. **优雅可读**：桌面级视觉（ActivityRings / MetricGrid / ServerDetail），而不是网页版 Grafana 的复刻

## 三大监控领域（产品分层）

| 领域 | 范围 | 数据源 |
|------|------|--------|
| Host Metrics | CPU 总/分核、内存、磁盘 I/O、磁盘空间 | Go agent / node_exporter |
| Network Quality | Ping 延迟、TCP 连通、DNS 解析、丢包 | 本地 probe + agent probe（vantage point 标签区分） |
| Traffic Analytics | 网卡流量、接口排名、突发检测 | Counter→rate 派生 |

## 关键约束

- **不是公网服务**：agent 假设运行在私网或 VPN/SSH 隧道后
- **桌面端独占**：单用户、单实例、不做多人协作
- **凭证本地保管**：SSH key / agent token 存 SQLite，加密强度待加强（见 knownIssues）
- **桌面包尺寸**：Tauri，初始窗口 920×620，不大不小

## 商业方向（远期）

后续考虑「部分功能免费 + 付费高级功能」的混合模式。**当前阶段不实现任何商业化逻辑**，主线是把 v0.2 的功能与 UI 做扎实。

## 成功指标（v0.2 之前可衡量）

- 一键部署 agent：从 SSH 连接到 agent 返回 `/api/health` ≤ 30 秒
- 单机 dashboard 渲染：≥ 60fps 流畅，无明显卡顿
- 支持同时监控 ≥ 10 台 VPS 不丢点
- UI 完成度：dashboard / detail / settings 全部走完设计审查
- 桌面安装包 ≤ 30MB（Tauri 自身约束）

## 非目标（明确不做）

- ❌ 替代 Prometheus + Grafana（不做长期归档、不做大规模 fleet）
- ❌ 业务级 APM / trace / log 聚合
- ❌ Kubernetes / Docker 容器编排监控
- ❌ 移动端 / Web 端（专注桌面）
