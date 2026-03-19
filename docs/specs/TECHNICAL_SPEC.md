# 技术规格说明

## 1. 系统架构

### 1.1 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Desktop Application                       │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              React Frontend (Vite)                    │  │
│  │  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐    │  │
│  │  │Dashboard│  │  CPU   │  │Network │  │ Alert  │    │  │
│  │  │  Page  │  │  Page  │  │  Page  │  │  Page  │    │  │
│  │  └────────┘  └────────┘  └────────┘  └────────┘    │  │
│  │                                                       │  │
│  │  ┌──────────────────────────────────────────────┐   │  │
│  │  │         Zustand State Management             │   │  │
│  │  └──────────────────────────────────────────────┘   │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕ Tauri IPC                       │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Rust Backend (Tauri 2)                   │  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐    │  │
│  │  │  Commands  │  │  Scheduler │  │   Alerts   │    │  │
│  │  └────────────┘  └────────────┘  └────────────┘    │  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐    │  │
│  │  │  Adapters  │  │   Probes   │  │  Storage   │    │  │
│  │  └────────────┘  └────────────┘  └────────────┘    │  │
│  │                                                       │  │
│  │  ┌──────────────────────────────────────────────┐   │  │
│  │  │         SQLite Database (WAL mode)           │   │  │
│  │  └──────────────────────────────────────────────┘   │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                           ↕ HTTP/JSON
┌─────────────────────────────────────────────────────────────┐
│                    Remote Servers                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ Go Agent     │  │node_exporter │  │   Glances    │     │
│  │ (Port 8080)  │  │ (Port 9100)  │  │ (Port 61208) │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### 1.2 技术栈

| 层级 | 技术 | 版本 | 用途 |
|------|------|------|------|
| 前端框架 | React | 18.3+ | UI 组件 |
| 构建工具 | Vite | 5.0+ | 开发服务器和打包 |
| 样式 | Tailwind CSS | 3.4+ | 样式系统 |
| 状态管理 | Zustand | 4.5+ | 全局状态 |
| 图表 | Recharts | 2.10+ | 数据可视化 |
| 类型系统 | TypeScript | 5.3+ | 类型安全 |
| 桌面框架 | Tauri | 2.0+ | 跨平台桌面应用 |
| 后端语言 | Rust | 1.75+ | 系统编程 |
| 异步运行时 | Tokio | 1.35+ | 异步 I/O |
| 数据库 | SQLite | 3.40+ | 本地存储 |
| ORM | SQLx | 0.7+ | 数据库访问 |
| HTTP 客户端 | reqwest | 0.11+ | HTTP 请求 |
| 远程代理 | Go | 1.21+ | 轻量级代理 |
| Web 框架 | Gin | 1.9+ | HTTP 服务器 |
| 系统信息 | gopsutil | 3.23+ | 指标采集 |

---

## 2. 核心模块设计

### 2.1 适配器层 (Adapters)

**设计模式**: 策略模式 + 适配器模式

**接口定义**
```rust
pub trait MetricAdapter: Send + Sync {
    fn name(&self) -> &str;
    async fn fetch_host_metrics(&self, server: &ServerConfig) -> AppResult<Vec<RawMetric>>;
    async fn health_check(&self, server: &ServerConfig) -> AppResult<bool>;
}
```

**实现类**
1. **NodeExporterAdapter**
   - 解析 Prometheus 文本格式
   - 支持标准 node_exporter 指标
   - 正则表达式解析：`^(\w+)(?:\{([^}]+)\})?\s+(\S+)$`

2. **GoAgentAdapter**
   - 解析 JSON 格式
   - 支持自定义 Go Agent
   - Bearer Token 认证

3. **GlancesAdapter** (未实现)
   - 解析 Glances REST API
   - 支持 Glances 3.0+ 版本

**数据流**
```
Remote Source → HTTP Response → Adapter.parse() → Vec<RawMetric> → Storage
```

---

### 2.2 指标引擎 (Metrics Engine)

#### 2.2.1 派生指标引擎 (DerivedMetricsEngine)

**功能**: Counter → Rate 转换

**算法**
```rust
rate = (current_value - previous_value) / (current_time - previous_time)
```

**状态管理**
```rust
struct CounterState {
    last_value: f64,
    last_timestamp: i64,
}

HashMap<String, CounterState> // key = server_id:metric_key
```

**重置检测**
- 当 `current_value < previous_value` 时判定为计数器重置
- 重置时返回 `None`，等待下一个数据点

**清理策略**
- 每 1000 次调用清理一次过期状态
- 过期时间：5 分钟无更新

#### 2.2.2 数据聚合引擎 (RollupEngine)

**聚合策略**
```rust
struct AggregatedMetric {
    min_val: f64,
    max_val: f64,
    avg_val: f64,
    bucket: i64,  // 时间桶（分钟/15分钟）
}
```

**聚合 SQL**
```sql
INSERT INTO metrics_1m (server_id, key, min_val, max_val, avg_val, bucket)
SELECT
    server_id,
    key,
    MIN(value) as min_val,
    MAX(value) as max_val,
    AVG(value) as avg_val,
    (timestamp / 60) * 60 as bucket
FROM raw_metrics
WHERE timestamp >= ? AND timestamp < ?
GROUP BY server_id, key, bucket
ON CONFLICT (server_id, key, bucket) DO UPDATE SET
    min_val = MIN(excluded.min_val, metrics_1m.min_val),
    max_val = MAX(excluded.max_val, metrics_1m.max_val),
    avg_val = (excluded.avg_val + metrics_1m.avg_val) / 2;
```

---

### 2.3 探测系统 (Probes)

#### 2.3.1 ICMP Ping

**实现**: `surge-ping` crate

**权限要求**: macOS/Linux 需要 root 或 CAP_NET_RAW

**算法**
```rust
pub async fn ping(&self, addr: IpAddr, count: u16) -> AppResult<PingResult> {
    let mut pinger = self.client.pinger(addr, PingIdentifier(rand::random())).await;
    let mut rtts = Vec::new();
    let mut lost = 0;

    for seq in 0..count {
        match pinger.ping(PingSequence(seq), &[]).await {
            Ok((_, duration)) => rtts.push(duration.as_millis() as f64),
            Err(_) => lost += 1,
        }
    }

    let avg_rtt = rtts.iter().sum::<f64>() / rtts.len() as f64;
    let loss_rate = (lost as f64 / count as f64) * 100.0;

    Ok(PingResult { avg_rtt_ms: avg_rtt, loss_rate })
}
```

#### 2.3.2 TCP 探测

**实现**: `tokio::net::TcpStream`

**超时**: 5 秒

```rust
pub async fn probe(&self, addr: SocketAddr) -> AppResult<TcpProbeResult> {
    let start = Instant::now();

    match timeout(Duration::from_secs(5), TcpStream::connect(addr)).await {
        Ok(Ok(_)) => Ok(TcpProbeResult {
            success: true,
            latency_ms: start.elapsed().as_millis() as f64,
        }),
        _ => Ok(TcpProbeResult {
            success: false,
            latency_ms: 0.0,
        }),
    }
}
```

#### 2.3.3 DNS 探测

**实现**: UDP Socket + 手动 DNS 查询

**查询类型**: A 记录

```rust
pub async fn resolve(&self, domain: &str) -> AppResult<DnsProbeResult> {
    let start = Instant::now();
    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    // 构造 DNS 查询包
    let query = build_dns_query(domain);
    socket.send_to(&query, "8.8.8.8:53").await?;

    let mut buf = [0u8; 512];
    let (len, _) = socket.recv_from(&mut buf).await?;

    let latency = start.elapsed().as_millis() as f64;
    let ips = parse_dns_response(&buf[..len])?;

    Ok(DnsProbeResult { latency_ms: latency, resolved_ips: ips })
}
```

---

### 2.4 告警引擎 (Alert Engine)

#### 2.4.1 规则评估

**状态机**
```
┌─────────┐  breach  ┌──────────┐  duration  ┌────────┐
│  Idle   │─────────→│ Breached │───────────→│ Firing │
└─────────┘          └──────────┘            └────────┘
     ↑                                             │
     └─────────────────────────────────────────────┘
                    recover
```

**评估逻辑**
```rust
pub fn evaluate(&mut self, metric_key: &str, value: f64) -> Vec<AlertEvent> {
    for rule in self.rules.values() {
        let breached = match rule.condition {
            AlertCondition::Gt => value > rule.threshold,
            AlertCondition::Lt => value < rule.threshold,
            AlertCondition::Eq => (value - rule.threshold).abs() < 0.001,
        };

        if breached {
            let state = self.firing_state.entry(rule.id.clone())
                .or_insert(FiringState { first_breach_at: now, last_check_at: now });

            if now - state.first_breach_at >= rule.duration_sec {
                // 触发告警
                events.push(AlertEvent::firing(...));
            }
        } else {
            if self.firing_state.remove(&rule.id).is_some() {
                // 恢复告警
                events.push(AlertEvent::resolved(...));
            }
        }
    }
}
```

#### 2.4.2 通知系统

**实现**: `tauri-plugin-notification`

**平台支持**
- macOS: NSUserNotification
- Windows: Windows Toast
- Linux: libnotify

```rust
pub fn send_alert(&self, event: &AlertEvent) -> AppResult<()> {
    self.app
        .notification()
        .builder()
        .title("ServerHUB Alert")
        .body(&format!("Alert on {}: {} = {}",
            event.server_id, event.metric_key, event.value))
        .show()?;
    Ok(())
}
```

---

### 2.5 存储层 (Storage)

#### 2.5.1 数据库配置

**SQLite 优化**
```rust
PRAGMA journal_mode = WAL;        // Write-Ahead Logging
PRAGMA synchronous = NORMAL;      // 平衡性能和安全
PRAGMA cache_size = -64000;       // 64MB 缓存
PRAGMA temp_store = MEMORY;       // 临时表存内存
```

**连接池**
```rust
SqlitePoolOptions::new()
    .max_connections(5)
    .connect(&format!("sqlite:{}?mode=rwc", db_path))
    .await?
```

#### 2.5.2 索引策略

**查询优化**
```sql
-- 按服务器和时间查询（最常用）
CREATE INDEX idx_raw_metrics_server_time
    ON raw_metrics(server_id, timestamp DESC);

-- 按指标键和时间查询
CREATE INDEX idx_raw_metrics_key_time
    ON raw_metrics(key, timestamp DESC);

-- 聚合表唯一约束（防止重复聚合）
CREATE UNIQUE INDEX idx_metrics_1m_unique
    ON metrics_1m(server_id, key, bucket);
```

#### 2.5.3 数据保留策略

| 表 | 保留时间 | 清理频率 | 预估大小 |
|---|---------|---------|---------|
| raw_metrics | 7 天 | 每小时 | ~500MB (50 服务器) |
| metrics_1m | 30 天 | 每天 | ~200MB |
| metrics_15m | 90 天 | 每周 | ~150MB |

**清理 SQL**
```sql
DELETE FROM raw_metrics WHERE timestamp < ?;
DELETE FROM metrics_1m WHERE bucket < ?;
DELETE FROM metrics_15m WHERE bucket < ?;
```

---

## 3. 数据模型

### 3.1 指标数据模型

**RawMetric**
```rust
pub struct RawMetric {
    pub key: String,              // 指标键名（如 cpu_usage_percent）
    pub value: f64,               // 指标值
    pub metric_type: MetricType,  // Counter | Gauge | State
    pub timestamp: i64,           // Unix 时间戳（秒）
    pub labels: HashMap<String, String>,  // 标签（如 cpu=0）
}
```

**MetricType 语义**
- **Counter**: 单调递增，需要计算速率（如网络字节数）
- **Gauge**: 瞬时值，直接使用（如 CPU 使用率）
- **State**: 离散状态（如服务状态 0/1）

### 3.2 时间序列存储

**时间桶计算**
```rust
// 1 分钟桶
let bucket_1m = (timestamp / 60) * 60;

// 15 分钟桶
let bucket_15m = (timestamp / 900) * 900;
```

**查询示例**
```sql
-- 查询最近 1 小时的 CPU 使用率
SELECT timestamp, value
FROM raw_metrics
WHERE server_id = ?
  AND key = 'cpu_usage_percent'
  AND timestamp >= ?
ORDER BY timestamp ASC;

-- 查询最近 7 天的聚合数据（1 分钟粒度）
SELECT bucket, avg_val, min_val, max_val
FROM metrics_1m
WHERE server_id = ?
  AND key = 'cpu_usage_percent'
  AND bucket >= ?
ORDER BY bucket ASC;
```

---

## 4. 并发模型

### 4.1 Rust 异步架构

**Tokio Runtime**
```rust
#[tokio::main]
async fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
}
```

**任务调度**
```rust
// 每个服务器独立轮询任务
for server in servers {
    tokio::spawn(async move {
        loop {
            let metrics = adapter.fetch_host_metrics(&server).await;
            storage.insert_metrics(metrics).await;
            tokio::time::sleep(Duration::from_secs(server.polling_interval_sec)).await;
        }
    });
}
```

### 4.2 前端状态管理

**Zustand Store**
```typescript
interface ServerStore {
  servers: ServerConfig[];
  activeServerId: string | null;
  addServer: (server: ServerConfig) => void;
  removeServer: (id: string) => void;
  setActiveServer: (id: string) => void;
}

const useServerStore = create<ServerStore>((set) => ({
  servers: [],
  activeServerId: null,
  addServer: (server) => set((state) => ({
    servers: [...state.servers, server]
  })),
  removeServer: (id) => set((state) => ({
    servers: state.servers.filter(s => s.id !== id)
  })),
  setActiveServer: (id) => set({ activeServerId: id }),
}));
```

---

## 5. 安全设计

### 5.1 认证机制

**Go Agent Bearer Token**
```go
func authMiddleware(token string) gin.HandlerFunc {
    return func(c *gin.Context) {
        auth := c.GetHeader("Authorization")
        if auth != "Bearer "+token {
            c.AbortWithStatusJSON(401, gin.H{"error": "unauthorized"})
            return
        }
        c.Next()
    }
}
```

### 5.2 数据安全

**本地存储**
- SQLite 数据库存储在用户目录：`~/.serverhub/data.db`
- 可选加密：使用 SQLCipher 加密数据库（未实现）

**网络传输**
- 支持 HTTPS（配置 TLS 证书）
- 支持 SSH 隧道（通过 `access_method: Tunnel`）

### 5.3 输入验证

**服务器配置验证**
```rust
fn validate_server_config(config: &ServerConfig) -> AppResult<()> {
    if config.name.is_empty() {
        return Err(AppError::Custom("Server name cannot be empty".into()));
    }
    if config.port == 0 || config.port > 65535 {
        return Err(AppError::Custom("Invalid port number".into()));
    }
    Ok(())
}
```

---

## 6. 性能优化

### 6.1 数据库优化

**批量插入**
```rust
pub async fn insert_metrics_batch(&self, metrics: Vec<RawMetric>) -> AppResult<()> {
    let mut tx = self.pool.begin().await?;

    for metric in metrics {
        sqlx::query("INSERT INTO raw_metrics (...) VALUES (?, ?, ?, ?)")
            .bind(&metric.server_id)
            .bind(&metric.key)
            .bind(metric.value)
            .bind(metric.timestamp)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(())
}
```

**查询缓存**
- 使用 Rust `Arc<RwLock<HashMap>>` 缓存最近查询结果
- TTL: 10 秒

### 6.2 前端优化

**虚拟滚动**
- 使用 `react-window` 渲染大量服务器列表

**图表优化**
- 数据点降采样（超过 1000 点时）
- 使用 `useMemo` 缓存图表数据

**懒加载**
```typescript
const CPUPage = lazy(() => import('./pages/CPUPage'));
const NetworkPage = lazy(() => import('./pages/NetworkPage'));
```

---

## 7. 错误处理

### 7.1 错误类型

```rust
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Ping error: {0}")]
    Ping(#[from] surge_ping::SurgeError),

    #[error("{0}")]
    Custom(String),
}
```

### 7.2 重试策略

**指数退避**
```rust
let mut retry_count = 0;
let max_retries = 3;

loop {
    match adapter.fetch_host_metrics(&server).await {
        Ok(metrics) => break,
        Err(e) if retry_count < max_retries => {
            retry_count += 1;
            let delay = Duration::from_secs(2u64.pow(retry_count));
            tokio::time::sleep(delay).await;
        }
        Err(e) => return Err(e),
    }
}
```

---

## 8. 测试策略

### 8.1 单元测试覆盖率

| 模块 | 覆盖率目标 |
|------|-----------|
| Adapters | 90% |
| Metrics Engine | 90% |
| Alert Engine | 85% |
| Storage | 80% |
| Probes | 75% |

### 8.2 集成测试

**Mock HTTP Server**
```rust
#[tokio::test]
async fn test_adapter_integration() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("GET", "/api/metrics")
        .with_status(200)
        .with_body(r#"{"cpu": {"usage_percent": 45.2}}"#)
        .create_async()
        .await;

    // 测试逻辑
}
```

---

## 9. 部署架构

### 9.1 桌面应用

**打包格式**
- macOS: `.dmg` + `.app`
- Windows: `.msi` + `.exe`
- Linux: `.deb` + `.AppImage`

**安装路径**
- macOS: `/Applications/ServerHUB.app`
- Windows: `C:\Program Files\ServerHUB`
- Linux: `/opt/serverhub`

### 9.2 Go Agent 部署

**Systemd Service**
```ini
[Unit]
Description=ServerHUB Agent
After=network.target

[Service]
Type=simple
User=serverhub
ExecStart=/usr/local/bin/serverhub-agent -config /etc/serverhub/config.yaml
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

---

## 10. 监控与日志

### 10.1 日志级别

```rust
use tracing::{info, warn, error, debug};

info!("Server {} polling started", server.name);
warn!("Failed to fetch metrics from {}: {}", server.host, err);
error!("Database connection lost: {}", err);
debug!("Parsed {} metrics from response", metrics.len());
```

### 10.2 性能指标

**内部指标**
- 轮询延迟（P50/P95/P99）
- 数据库查询时间
- 内存使用量
- 告警触发次数

---

## 附录

### A. 依赖版本锁定

**Rust (Cargo.toml)**
```toml
[dependencies]
tauri = "2.0"
tokio = { version = "1.35", features = ["full"] }
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
chrono = "0.4"
surge-ping = "0.8"
```

**TypeScript (package.json)**
```json
{
  "dependencies": {
    "react": "^18.3.0",
    "zustand": "^4.5.0",
    "recharts": "^2.10.0",
    "@tauri-apps/api": "^2.0.0"
  }
}
```

### B. 配置文件格式

**Go Agent (config.yaml)**
```yaml
server:
  port: 8080
  auth_token: "your-secret-token"

collector:
  interval_sec: 5

logging:
  level: info
  file: /var/log/serverhub-agent.log
```
