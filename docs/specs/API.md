# API 文档

## 概述

ServerHUB 提供两类 API：
1. **Tauri Commands API**：前端通过 `invoke()` 调用的 Rust 后端接口
2. **Go Agent HTTP API**：远程服务器上的 Go 代理提供的 HTTP 接口

---

## 1. Tauri Commands API

### 1.1 服务器管理

#### `list_servers`

列出所有已配置的服务器。

**签名**
```rust
async fn list_servers() -> Result<Vec<ServerConfig>, String>
```

**请求参数**：无

**返回值**
```typescript
ServerConfig[] // 服务器配置列表
```

**示例**
```typescript
import { invoke } from '@tauri-apps/api/core';

const servers = await invoke<ServerConfig[]>('list_servers');
```

---

#### `add_server`

添加新服务器。

**签名**
```rust
async fn add_server(name: String, host: String, port: u16) -> Result<ServerConfig, String>
```

**请求参数**
| 参数 | 类型 | 说明 |
|------|------|------|
| name | string | 服务器名称 |
| host | string | 服务器地址 |
| port | number | 端口号 |

**返回值**
```typescript
ServerConfig // 创建的服务器配置
```

**示例**
```typescript
const server = await invoke<ServerConfig>('add_server', {
  name: 'Production Server',
  host: '192.168.1.100',
  port: 9100
});
```

---

#### `remove_server`

删除服务器。

**签名**
```rust
async fn remove_server(id: String) -> Result<(), String>
```

**请求参数**
| 参数 | 类型 | 说明 |
|------|------|------|
| id | string | 服务器 ID |

**返回值**：无

**示例**
```typescript
await invoke('remove_server', { id: 'server-uuid' });
```

---

### 1.2 指标查询

#### `get_metrics`

获取服务器当前指标。

**签名**
```rust
async fn get_metrics(server_id: String) -> Result<Vec<MetricPoint>, String>
```

**请求参数**
| 参数 | 类型 | 说明 |
|------|------|------|
| server_id | string | 服务器 ID |

**返回值**
```typescript
MetricPoint[] // 指标数据点列表
```

**示例**
```typescript
const metrics = await invoke<MetricPoint[]>('get_metrics', {
  server_id: 'server-uuid'
});
```

---

#### `get_metric_history`

获取指标历史数据。

**签名**
```rust
async fn get_metric_history(
    server_id: String,
    key: String,
    from: i64,
    to: i64
) -> Result<Vec<MetricPoint>, String>
```

**请求参数**
| 参数 | 类型 | 说明 |
|------|------|------|
| server_id | string | 服务器 ID |
| key | string | 指标键名 |
| from | number | 起始时间戳（秒） |
| to | number | 结束时间戳（秒） |

**返回值**
```typescript
MetricPoint[] // 历史指标数据点
```

**示例**
```typescript
const history = await invoke<MetricPoint[]>('get_metric_history', {
  server_id: 'server-uuid',
  key: 'cpu_usage_percent',
  from: 1710000000,
  to: 1710086400
});
```

---

### 1.3 设置管理

#### `get_settings`

获取应用设置。

**签名**
```rust
async fn get_settings() -> Result<AppSettings, String>
```

**请求参数**：无

**返回值**
```typescript
AppSettings // 应用设置
```

**示例**
```typescript
const settings = await invoke<AppSettings>('get_settings');
```

---

#### `update_settings`

更新应用设置。

**签名**
```rust
async fn update_settings(settings: AppSettings) -> Result<AppSettings, String>
```

**请求参数**
| 参数 | 类型 | 说明 |
|------|------|------|
| settings | AppSettings | 新的设置对象 |

**返回值**
```typescript
AppSettings // 更新后的设置
```

**示例**
```typescript
const updated = await invoke<AppSettings>('update_settings', {
  settings: {
    default_polling_interval_sec: 15,
    data_retention_days: 30,
    theme: 'light',
    language: 'en-US'
  }
});
```

---

## 2. Go Agent HTTP API

### 基础信息

- **Base URL**: `http://{host}:{port}`
- **认证方式**: Bearer Token（通过 `Authorization` 请求头）
- **Content-Type**: `application/json`

### 2.1 健康检查

#### `GET /api/health`

检查代理健康状态。

**请求头**：无需认证

**响应**
```json
{
  "status": "ok"
}
```

**状态码**
- `200 OK`: 服务正常

---

### 2.2 获取指标

#### `GET /api/metrics`

获取服务器所有指标数据。

**请求头**
```
Authorization: Bearer <token>
```

**响应**
```json
{
  "timestamp": 1710000000,
  "cpu": {
    "usage_percent": 45.2,
    "cores": [12.3, 56.7, 34.1, 78.9]
  },
  "memory": {
    "total_bytes": 17179869184,
    "used_bytes": 8589934592,
    "available_bytes": 8589934592,
    "usage_percent": 50.0
  },
  "disk": {
    "partitions": [
      {
        "device": "/dev/sda1",
        "mountpoint": "/",
        "total_bytes": 1099511627776,
        "used_bytes": 549755813888,
        "free_bytes": 549755813888,
        "usage_percent": 50.0
      }
    ],
    "io": {
      "read_bytes_total": 1234567890,
      "write_bytes_total": 9876543210
    }
  },
  "network": {
    "interfaces": [
      {
        "name": "eth0",
        "bytes_sent_total": 1234567890,
        "bytes_recv_total": 9876543210,
        "packets_sent_total": 123456,
        "packets_recv_total": 987654
      }
    ]
  }
}
```

**状态码**
- `200 OK`: 成功
- `401 Unauthorized`: 认证失败
- `500 Internal Server Error`: 服务器错误

---

## 3. 数据结构定义

### ServerConfig

服务器配置对象。

```typescript
interface ServerConfig {
  id: string;                    // UUID
  name: string;                  // 服务器名称
  host: string;                  // 主机地址
  port: number;                  // 端口号
  adapter_type: AdapterType;     // 适配器类型
  access_method: AccessMethod;   // 访问方式
  polling_interval_sec: number;  // 轮询间隔（秒）
  enabled: boolean;              // 是否启用
  created_at: number;            // 创建时间戳
  updated_at: number;            // 更新时间戳
}
```

### AdapterType

适配器类型枚举。

```typescript
enum AdapterType {
  NodeExporter = 'node_exporter',
  Glances = 'glances',
  GoAgent = 'go_agent'
}
```

### AccessMethod

访问方式枚举。

```typescript
enum AccessMethod {
  Private = 'private',    // 私有网络
  Tunnel = 'tunnel',      // SSH 隧道
  Gateway = 'gateway'     // 网关代理
}
```

### RawMetric

原始指标数据。

```typescript
interface RawMetric {
  key: string;                        // 指标键名
  value: number;                      // 指标值
  metric_type: MetricType;            // 指标类型
  timestamp: number;                  // 时间戳（秒）
  labels: Record<string, string>;     // 标签
}
```

### MetricType

指标类型枚举。

```typescript
enum MetricType {
  Counter = 'counter',    // 累计计数器
  Gauge = 'gauge',        // 瞬时值
  State = 'state'         // 状态值
}
```

### MetricPoint

指标数据点。

```typescript
interface MetricPoint {
  server_id: string;           // 服务器 ID
  key: string;                 // 指标键名
  value: number;               // 指标值
  metric_type: MetricType;     // 指标类型
  vantage_point: string;       // 观测点（desktop-local / remote-agent）
  timestamp: number;           // 时间戳（秒）
}
```

### AlertRule

告警规则。

```typescript
interface AlertRule {
  id: string;                  // 规则 ID
  name: string;                // 规则名称
  server_id?: string;          // 服务器 ID（可选，全局规则为空）
  metric_key: string;          // 监控指标键名
  condition: AlertCondition;   // 告警条件
  threshold: number;           // 阈值
  duration_sec: number;        // 持续时间（秒）
  cooldown_sec: number;        // 冷却时间（秒）
  enabled: boolean;            // 是否启用
  created_at: number;          // 创建时间戳
}
```

### AlertCondition

告警条件枚举。

```typescript
enum AlertCondition {
  Gt = 'gt',    // 大于
  Lt = 'lt',    // 小于
  Eq = 'eq'     // 等于
}
```

### AlertEvent

告警事件。

```typescript
interface AlertEvent {
  id: number;                  // 事件 ID
  rule_id: string;             // 规则 ID
  server_id: string;           // 服务器 ID
  metric_key: string;          // 指标键名
  value: number;               // 触发时的指标值
  status: AlertStatus;         // 告警状态
  fired_at: number;            // 触发时间戳
  resolved_at?: number;        // 恢复时间戳（可选）
}
```

### AlertStatus

告警状态枚举。

```typescript
enum AlertStatus {
  Firing = 'firing',       // 告警中
  Resolved = 'resolved'    // 已恢复
}
```

### AppSettings

应用设置。

```typescript
interface AppSettings {
  default_polling_interval_sec: number;  // 默认轮询间隔（秒）
  data_retention_days: number;           // 数据保留天数
  theme: string;                         // 主题（dark / light）
  language: string;                      // 语言（zh-CN / en-US）
}
```

---

## 4. 错误码说明

### AppError

统一错误类型。

```rust
pub enum AppError {
    Database(sqlx::Error),           // 数据库错误
    Migration(MigrateError),         // 迁移错误
    Http(reqwest::Error),            // HTTP 请求错误
    Serde(serde_json::Error),        // 序列化错误
    Io(std::io::Error),              // IO 错误
    Ping(surge_ping::SurgeError),    // Ping 错误
    Notification(String),            // 通知错误
    Custom(String),                  // 自定义错误
}
```

所有 Tauri Commands 返回的错误均为 `String` 类型，包含错误描述信息。

**错误响应示例**
```typescript
try {
  await invoke('add_server', { name: '', host: '', port: 0 });
} catch (error) {
  console.error(error); // "Database error: ..."
}
```

---

## 5. 认证机制

### Go Agent Bearer Token

Go Agent 使用 Bearer Token 认证。

**配置示例**（`agent/config.yaml`）
```yaml
server:
  port: 8080
  auth_token: "your-secret-token-here"
```

**请求示例**
```bash
curl -H "Authorization: Bearer your-secret-token-here" \
     http://192.168.1.100:8080/api/metrics
```

**认证失败响应**
```json
{
  "error": "unauthorized"
}
```

---

## 6. 速率限制

当前版本无速率限制。生产环境建议在 Go Agent 前配置反向代理（如 Nginx）实现速率限制。

---

## 7. 版本兼容性

- **Tauri Commands API**: v0.1.0+
- **Go Agent HTTP API**: v0.1.0+

API 遵循语义化版本规范。破坏性变更将在主版本号升级时发生。
