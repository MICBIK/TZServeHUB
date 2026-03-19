# ServerHUB 架构设计方案

> 版本：v1.0 | 日期：2026-03-19
> 融合 Claude 研究（技术实现）+ Codex 研究（产品架构）

## 1. 产品定位

ServerHUB 是一个基于 Tauri 2 的桌面控制平面应用，用于监控远程服务器的系统资源状态。

核心功能域（三域分离）：
- **Host Metrics**：CPU（总/每核心）、内存、磁盘 I/O、磁盘空间
- **Network Quality**：Ping 延迟、TCP 连通性、DNS 解析、丢包率
- **Traffic Analytics**：网卡流量、接口排名、流量突发检测

产品形态：桌面控制平面 + 适配器化远端采集（非本地采集器）

## 2. 技术栈

```
桌面壳：       Tauri 2 (Rust)
前端：         React 19 + TypeScript 5.7 + Vite
状态管理：     Zustand 5
UI 组件：      Ant Design 5
图表：         Recharts 2
样式：         Tailwind CSS 4
本地存储：     SQLite (sqlx + WAL mode)
远端数据源：   node_exporter (v1) → Glances (v2) → 自研 agent (v3)
探测子系统：   surge-ping (ICMP) + tokio (TCP/DNS)
安全接入：     WireGuard / Tailscale / VPN / SSH Tunnel
```

## 3. 系统架构

```
┌─────────────────────────────────────────────────────┐
│                   Remote Servers                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │
│  │ node_    │  │ Glances  │  │ Future Custom    │  │
│  │ exporter │  │ API      │  │ Agent            │  │
│  └────┬─────┘  └────┬─────┘  └────┬─────────────┘  │
│       │              │              │                │
│  ─────┴──────────────┴──────────────┴────────────── │
│       Private Network / VPN / SSH Tunnel             │
└─────────────────────┬───────────────────────────────┘
                      │ HTTPS / Authenticated
                      ▼
┌─────────────────────────────────────────────────────┐
│              ServerHUB Desktop App                    │
│                                                      │
│  ┌─────────────────────────────────────────────┐    │
│  │              Rust Core                       │    │
│  │                                              │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  │    │
│  │  │ Adapter  │  │ Probe    │  │ Derived  │  │    │
│  │  │ Layer    │  │ Worker   │  │ Metrics  │  │    │
│  │  │          │  │          │  │ Engine   │  │    │
│  │  │ •node_exp│  │ •Ping    │  │ •counter │  │    │
│  │  │ •glances │  │ •TCP     │  │  →rate   │  │    │
│  │  │ •future  │  │ •DNS     │  │ •rollup  │  │    │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  │    │
│  │       │              │              │        │    │
│  │  ┌────▼──────────────▼──────────────▼────┐  │    │
│  │  │        Normalized Metric Store         │  │    │
│  │  │   (unified schema for all sources)     │  │    │
│  │  └────┬──────────────┬───────────────┘    │  │    │
│  │       │              │                     │  │    │
│  │  ┌────▼─────┐  ┌────▼─────┐              │  │    │
│  │  │ SQLite   │  │ Alert    │              │  │    │
│  │  │ Store    │  │ Engine   │              │  │    │
│  │  │ (WAL)    │  │          │              │  │    │
│  │  └────┬─────┘  └────┬─────┘              │  │    │
│  │       │              │                     │  │    │
│  └───────┼──────────────┼─────────────────────┘ │    │
│          │              │                        │    │
│  ┌───────▼──────────────▼─────────────────────┐ │    │
│  │           Tauri Command Bridge              │ │    │
│  └───────────────────┬─────────────────────────┘ │    │
│                      │ IPC (invoke)              │    │
│  ┌───────────────────▼─────────────────────────┐ │    │
│  │              React Frontend                  │ │    │
│  │                                              │ │    │
│  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────────┐  │ │    │
│  │  │Server│ │CPU   │ │Net   │ │Disk      │  │ │    │
│  │  │List  │ │Page  │ │Page  │ │Page      │  │ │    │
│  │  └──────┘ └──────┘ └──────┘ └──────────┘  │ │    │
│  │  ┌──────┐ ┌──────┐ ┌──────┐              │ │    │
│  │  │Probe │ │Alert │ │Settings│             │ │    │
│  │  │Page  │ │Log   │ │Page   │             │ │    │
│  │  └──────┘ └──────┘ └──────┘              │ │    │
│  └──────────────────────────────────────────┘ │    │
└──────────────────────────────────────────────────┘
```

## 4. 后端模块设计

### 目录结构

```
src-tauri/
├── Cargo.toml
├── tauri.conf.json
├── capabilities/
│   └── default.json          # Tauri 2 权限配置
├── src/
│   ├── main.rs               # 应用入口
│   ├── lib.rs                # 模块导出
│   ├── error.rs              # 统一错误类型
│   │
│   ├── commands/             # Tauri IPC 命令
│   │   ├── mod.rs
│   │   ├── server.rs         # 服务器 CRUD
│   │   ├── metrics.rs        # 指标查询
│   │   ├── probes.rs         # 探测管理
│   │   └── settings.rs       # 配置管理
│   │
│   ├── adapters/             # 数据源适配器（三域分离核心）
│   │   ├── mod.rs
│   │   ├── traits.rs         # MetricAdapter trait 定义
│   │   ├── node_exporter.rs  # node_exporter 适配器
│   │   └── glances.rs        # Glances 适配器（v2）
│   │
│   ├── probes/               # 网络探测子系统
│   │   ├── mod.rs
│   │   ├── ping.rs           # ICMP Ping
│   │   ├── tcp.rs            # TCP 连通性
│   │   └── dns.rs            # DNS 解析
│   │
│   ├── metrics/              # 指标处理引擎
│   │   ├── mod.rs
│   │   ├── schema.rs         # 标准化 metric schema
│   │   ├── derived.rs        # counter → rate 派生计算
│   │   └── rollup.rs         # 数据聚合/降采样
│   │
│   ├── storage/              # 本地存储
│   │   ├── mod.rs
│   │   ├── database.rs       # SQLite 连接管理
│   │   ├── migrations.rs     # Schema 迁移
│   │   └── retention.rs      # 数据保留策略
│   │
│   ├── scheduler/            # 轮询调度器
│   │   ├── mod.rs
│   │   └── poller.rs         # 带 jitter/backoff 的轮询
│   │
│   ├── alerts/               # 告警引擎
│   │   ├── mod.rs
│   │   ├── rules.rs          # 告警规则
│   │   └── notifier.rs       # 通知发送
│   │
│   └── models/               # 数据模型
│       ├── mod.rs
│       ├── server.rs         # 服务器配置模型
│       ├── metric.rs         # 指标数据模型
│       └── alert.rs          # 告警模型
```

### 核心 Trait 设计

```rust
/// 数据源适配器 trait
#[async_trait]
pub trait MetricAdapter: Send + Sync {
    /// 适配器名称
    fn name(&self) -> &str;

    /// 拉取主机指标
    async fn fetch_host_metrics(&self, server: &ServerConfig) -> Result<Vec<RawMetric>>;

    /// 健康检查
    async fn health_check(&self, server: &ServerConfig) -> Result<bool>;
}

/// 标准化指标
pub struct RawMetric {
    pub key: String,           // e.g. "cpu.core.0"
    pub value: f64,
    pub metric_type: MetricType, // Counter | Gauge | State
    pub timestamp: i64,
    pub labels: HashMap<String, String>,
}

pub enum MetricType {
    Counter,   // 单调递增，需要 rate 计算
    Gauge,     // 瞬时值
    State,     // 状态枚举
}
```

## 5. 前端模块设计

### 目录结构

```
src/
├── main.tsx                  # 应用入口
├── App.tsx                   # 路由配置
├── vite-env.d.ts
│
├── components/               # UI 组件
│   ├── layout/
│   │   ├── AppLayout.tsx     # 主布局（侧边栏+内容区）
│   │   └── Sidebar.tsx       # 侧边导航
│   ├── charts/
│   │   ├── TimeSeriesChart.tsx  # 通用时序图表
│   │   ├── GaugeChart.tsx       # 仪表盘图表
│   │   └── SparkLine.tsx        # 迷你折线图
│   ├── server/
│   │   ├── ServerList.tsx       # 服务器列表
│   │   ├── ServerCard.tsx       # 服务器卡片
│   │   └── ServerForm.tsx       # 添加/编辑服务器
│   └── common/
│       ├── StatusBadge.tsx      # 状态标记
│       └── MetricValue.tsx      # 指标数值展示
│
├── pages/                    # 页面
│   ├── DashboardPage.tsx     # 总览仪表盘
│   ├── CpuPage.tsx           # CPU 详情（总+每核心）
│   ├── NetworkPage.tsx       # 网络流量+延迟
│   ├── DiskPage.tsx          # 磁盘 I/O + 空间
│   ├── ProbePage.tsx         # 探测历史
│   ├── AlertPage.tsx         # 告警日志
│   └── SettingsPage.tsx      # 设置
│
├── hooks/                    # 自定义 Hooks
│   ├── useMetrics.ts         # 指标数据获取
│   ├── usePolling.ts         # 轮询控制
│   ├── useAlerts.ts          # 告警订阅
│   └── useServerList.ts     # 服务器列表管理
│
├── stores/                   # Zustand 状态
│   ├── serverStore.ts        # 服务器状态
│   ├── metricsStore.ts       # 指标缓存
│   └── settingsStore.ts      # 用户设置
│
├── services/                 # Tauri IPC 封装
│   └── tauri.ts              # invoke 封装
│
├── types/                    # TypeScript 类型
│   ├── server.ts
│   ├── metric.ts
│   └── alert.ts
│
└── lib/                      # 工具函数
    ├── formatters.ts         # 数值格式化（bytes, percentage）
    └── constants.ts          # 常量定义
```

## 6. 数据模型

### Metric Schema（标准化指标命名）

```
# Host Metrics
cpu.total                          # CPU 总使用率 (gauge, %)
cpu.core.{index}                   # 每核心使用率 (gauge, %)
memory.used_bytes                  # 已用内存 (gauge)
memory.total_bytes                 # 总内存 (gauge)
disk.dev.{name}.read_bytes         # 磁盘读取 (counter)
disk.dev.{name}.write_bytes        # 磁盘写入 (counter)
disk.fs.{mount}.total_bytes        # 文件系统总空间 (gauge)
disk.fs.{mount}.used_bytes         # 文件系统已用 (gauge)
disk.fs.{mount}.free_bytes         # 文件系统可用 (gauge)

# Network Traffic
network.if.{name}.rx_bytes         # 接收字节 (counter)
network.if.{name}.tx_bytes         # 发送字节 (counter)

# Network Quality (Probes)
probe.ping.{target}.latency_ms     # Ping 延迟 (gauge)
probe.ping.{target}.packet_loss    # 丢包率 (gauge, %)
probe.tcp.{target}.connect_ms      # TCP 连接时间 (gauge)
probe.dns.{target}.lookup_ms       # DNS 解析时间 (gauge)
```

每条指标附带：
- `metric_type`: Counter | Gauge | State
- `vantage_point`: desktop-local | remote-agent
- `source_timestamp`: 数据源时间戳
- `staleness`: 是否过期

### SQLite Schema

```sql
-- 服务器注册表
CREATE TABLE servers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    host TEXT NOT NULL,
    port INTEGER NOT NULL DEFAULT 9100,
    adapter_type TEXT NOT NULL DEFAULT 'node_exporter',
    access_method TEXT NOT NULL DEFAULT 'private',  -- private|tunnel|gateway
    polling_interval_sec INTEGER NOT NULL DEFAULT 10,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 原始指标（7天保留）
CREATE TABLE metrics_raw (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    server_id TEXT NOT NULL REFERENCES servers(id),
    key TEXT NOT NULL,
    value REAL NOT NULL,
    metric_type TEXT NOT NULL,  -- counter|gauge|state
    vantage_point TEXT NOT NULL DEFAULT 'desktop-local',
    timestamp INTEGER NOT NULL,
    UNIQUE(server_id, key, timestamp)
);

-- 1分钟聚合（30天保留）
CREATE TABLE metrics_1m (
    server_id TEXT NOT NULL REFERENCES servers(id),
    key TEXT NOT NULL,
    min_val REAL NOT NULL,
    max_val REAL NOT NULL,
    avg_val REAL NOT NULL,
    bucket INTEGER NOT NULL,  -- 分钟级时间戳
    PRIMARY KEY (server_id, key, bucket)
);

-- 15分钟聚合（90天保留）
CREATE TABLE metrics_15m (
    server_id TEXT NOT NULL REFERENCES servers(id),
    key TEXT NOT NULL,
    min_val REAL NOT NULL,
    max_val REAL NOT NULL,
    avg_val REAL NOT NULL,
    bucket INTEGER NOT NULL,
    PRIMARY KEY (server_id, key, bucket)
);

-- 告警规则
CREATE TABLE alert_rules (
    id TEXT PRIMARY KEY,
    server_id TEXT REFERENCES servers(id),  -- NULL = 全局规则
    metric_key TEXT NOT NULL,
    condition TEXT NOT NULL,  -- gt|lt|eq
    threshold REAL NOT NULL,
    duration_sec INTEGER NOT NULL DEFAULT 60,
    cooldown_sec INTEGER NOT NULL DEFAULT 300,
    enabled INTEGER NOT NULL DEFAULT 1
);

-- 告警事件
CREATE TABLE alert_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rule_id TEXT NOT NULL REFERENCES alert_rules(id),
    server_id TEXT NOT NULL REFERENCES servers(id),
    metric_key TEXT NOT NULL,
    value REAL NOT NULL,
    status TEXT NOT NULL,  -- firing|resolved
    fired_at INTEGER NOT NULL,
    resolved_at INTEGER
);

-- 索引
CREATE INDEX idx_metrics_raw_lookup ON metrics_raw(server_id, key, timestamp);
CREATE INDEX idx_metrics_1m_lookup ON metrics_1m(server_id, key, bucket);
CREATE INDEX idx_metrics_15m_lookup ON metrics_15m(server_id, key, bucket);
CREATE INDEX idx_alert_events_server ON alert_events(server_id, fired_at);
```

### 数据保留策略

| 层级 | 粒度 | 保留时间 | 表 |
|------|------|----------|-----|
| 原始 | 5-10s | 7 天 | metrics_raw |
| 1分钟聚合 | 1min | 30 天 | metrics_1m |
| 15分钟聚合 | 15min | 90 天 | metrics_15m |

后台定时任务：
- 每小时：raw → 1m 聚合
- 每天：1m → 15m 聚合
- 每天：清理过期数据

## 7. 轮询与调度

```
远端轮询频率（默认）：
- Host metrics: 10s（可配置 5-60s）
- Network probes: 30s（可配置 10-300s）

本地处理：
- Counter → Rate 计算: 每次拉取后立即计算
- SQLite 写入: 批量写入（每 5s flush 一次）
- 聚合任务: 后台 cron（每小时/每天）
- 告警评估: 每次新数据到达时

调度保护：
- 并发上限: 同时最多 10 个 scrape 任务
- 超时: 单次 scrape 5s 超时
- Jitter: ±20% 随机偏移，避免同时请求
- Backoff: 连续失败 → 指数退避（最大 5min）
- Staleness: 超过 3 个周期无数据 → 标记 stale
```

## 8. 安全设计

### 连接安全
- 默认要求私网或隧道接入，不支持裸公网 exporter
- 每台服务器记录 `access_method`: private | tunnel | gateway
- 支持 Basic Auth / Bearer Token 认证
- 未来支持 mTLS

### 本地安全
- Tauri 2 capability 最小权限配置
- 凭据存储使用系统 Keychain（macOS）/ Secret Service（Linux）/ Credential Manager（Windows）
- CSP 严格配置，禁止外部脚本加载

### 探测语义
- 所有探测结果标记 `vantage_point: desktop-local`
- UI 明确提示：探测结果反映桌面到目标的路径质量

## 9. MVP 交付计划

### Phase 1: Metrics MVP（2周）
- [ ] Tauri 2 项目脚手架
- [ ] node_exporter 适配器
- [ ] 标准化 metric schema + counter→rate
- [ ] SQLite 存储 + WAL
- [ ] CPU 页面（总+每核心图表）
- [ ] 磁盘页面（I/O 速度 + 空间）
- [ ] 网络页面（接口流量）
- [ ] 单服务器支持

### Phase 2: Usability MVP（2周）
- [ ] 多服务器支持 + 服务器列表
- [ ] 总览仪表盘
- [ ] 告警引擎 + 桌面通知
- [ ] Ping/TCP/DNS 探测
- [ ] 探测历史页面
- [ ] 数据聚合 + 保留策略

### Phase 3: Adapter Expansion（2周）
- [ ] Glances 适配器
- [ ] 服务器添加向导 + 连接诊断
- [ ] 按接口/磁盘筛选
- [ ] 流量排名视图
- [ ] 设置页面完善

### Phase 4: Polish（2周）
- [ ] 系统托盘集成
- [ ] 暗色/亮色主题
- [ ] 国际化（中/英）
- [ ] 自动更新
- [ ] 跨平台测试 + 打包

---

*本方案融合 Claude 研究（技术选型+代码模式）和 Codex 研究（产品定位+架构决策），经三轮审计验证。*
