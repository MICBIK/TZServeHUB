# 测试策略

## 1. 测试概述

### 1.1 测试目标
- 确保代码质量和功能正确性
- 防止回归问题
- 提高代码可维护性
- 保证跨平台兼容性

### 1.2 测试覆盖率目标
- 整体代码覆盖率：≥ 70%
- 关键路径覆盖率：≥ 90%
- 核心模块覆盖率：≥ 85%

---

## 2. 测试分层策略

### 2.1 单元测试 (Unit Tests)

**目标**：测试独立函数和模块的正确性

**覆盖范围**
- Rust 后端核心逻辑
  - 适配器解析逻辑 (`adapters/`)
  - 指标派生引擎 (`metrics/derived.rs`)
  - 告警规则评估 (`alerts/rules.rs`)
  - 数据聚合逻辑 (`metrics/rollup.rs`)
- TypeScript 工具函数
  - 格式化函数 (`lib/formatters.ts`)
  - 数据转换函数
- Go Agent 采集逻辑
  - 指标采集器 (`internal/collector/`)

**测试框架**
- Rust: `cargo test` + `tokio::test`
- TypeScript: Jest + React Testing Library
- Go: `go test` + `testify`

**示例：Rust 单元测试**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_rate_from_counter() {
        let mut engine = DerivedMetricsEngine::new();

        let metric1 = RawMetric {
            key: "network_tx_bytes".to_string(),
            value: 1000.0,
            metric_type: MetricType::Counter,
            timestamp: 100,
            labels: HashMap::new(),
        };

        assert_eq!(engine.derive_rate(&metric1), None); // 首次无历史

        let metric2 = RawMetric {
            key: "network_tx_bytes".to_string(),
            value: 2000.0,
            metric_type: MetricType::Counter,
            timestamp: 110,
            labels: HashMap::new(),
        };

        let rate = engine.derive_rate(&metric2).unwrap();
        assert_eq!(rate, 100.0); // (2000-1000)/(110-100) = 100 bytes/sec
    }

    #[test]
    fn test_counter_reset_detection() {
        let mut engine = DerivedMetricsEngine::new();

        let metric1 = RawMetric {
            key: "counter".to_string(),
            value: 1000.0,
            metric_type: MetricType::Counter,
            timestamp: 100,
            labels: HashMap::new(),
        };
        engine.derive_rate(&metric1);

        // 计数器重置（值变小）
        let metric2 = RawMetric {
            key: "counter".to_string(),
            value: 100.0,
            metric_type: MetricType::Counter,
            timestamp: 110,
            labels: HashMap::new(),
        };

        assert_eq!(engine.derive_rate(&metric2), None); // 重置时返回 None
    }
}
```

**示例：TypeScript 单元测试**
```typescript
import { formatBytes, formatBytesPerSec } from '@/lib/formatters';

describe('formatters', () => {
  test('formatBytes should format bytes correctly', () => {
    expect(formatBytes(0)).toBe('0 B');
    expect(formatBytes(1024)).toBe('1.00 KB');
    expect(formatBytes(1048576)).toBe('1.00 MB');
    expect(formatBytes(1073741824)).toBe('1.00 GB');
  });

  test('formatBytesPerSec should format rate correctly', () => {
    expect(formatBytesPerSec(1024)).toBe('1.00 KB/s');
    expect(formatBytesPerSec(1048576)).toBe('1.00 MB/s');
  });
});
```

---

### 2.2 集成测试 (Integration Tests)

**目标**：测试模块间交互和数据流

**覆盖范围**
- Adapter → Storage 数据流
- Metrics Engine → Database 写入
- Alert Engine → Notifier 触发
- Tauri Commands → Storage 查询

**测试策略**
- 使用内存 SQLite (`:memory:`) 进行数据库测试
- Mock HTTP 客户端测试 Adapter
- 使用测试 AppHandle 测试 Tauri Commands

**示例：Rust 集成测试**
```rust
#[tokio::test]
async fn test_go_agent_adapter_integration() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("GET", "/api/metrics")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "timestamp": 1710000000,
            "cpu": {"usage_percent": 45.2},
            "memory": {"total_bytes": 17179869184, "used_bytes": 8589934592}
        }"#)
        .create_async()
        .await;

    let adapter = GoAgentAdapter::new();
    let server_config = ServerConfig {
        host: server.host_with_port(),
        port: server.port(),
        ..Default::default()
    };

    let metrics = adapter.fetch_host_metrics(&server_config).await.unwrap();

    assert!(metrics.iter().any(|m| m.key == "cpu_usage_percent"));
    assert!(metrics.iter().any(|m| m.key == "memory_used_bytes"));

    mock.assert_async().await;
}
```

**示例：数据库集成测试**
```rust
#[tokio::test]
async fn test_storage_retention_cleanup() {
    let db = Database::new(":memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&db.pool).await.unwrap();

    // 插入过期数据
    let old_timestamp = Utc::now().timestamp() - (8 * 86400); // 8 天前
    sqlx::query("INSERT INTO raw_metrics (server_id, key, value, timestamp) VALUES (?, ?, ?, ?)")
        .bind("test-server")
        .bind("cpu_usage")
        .bind(50.0)
        .bind(old_timestamp)
        .execute(&db.pool)
        .await
        .unwrap();

    // 执行清理
    let retention = RetentionManager::new(db.pool.clone());
    retention.cleanup_old_data().await.unwrap();

    // 验证数据已删除
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM raw_metrics")
        .fetch_one(&db.pool)
        .await
        .unwrap();

    assert_eq!(count, 0);
}
```

---

### 2.3 端到端测试 (E2E Tests)

**目标**：测试完整用户流程

**覆盖范围**
- 添加服务器 → 查看指标 → 创建告警 → 接收通知
- 多服务器对比视图
- 历史数据查询和图表渲染
- 设置修改和持久化

**测试框架**
- Playwright (跨浏览器测试)
- Tauri WebDriver (桌面应用测试)

**测试场景**
```typescript
// tests/e2e/server-management.spec.ts
import { test, expect } from '@playwright/test';

test('should add and remove server', async ({ page }) => {
  await page.goto('http://localhost:1420');

  // 添加服务器
  await page.click('[data-testid="add-server-button"]');
  await page.fill('[data-testid="server-name"]', 'Test Server');
  await page.fill('[data-testid="server-host"]', '192.168.1.100');
  await page.fill('[data-testid="server-port"]', '9100');
  await page.click('[data-testid="submit-button"]');

  // 验证服务器出现在列表中
  await expect(page.locator('text=Test Server')).toBeVisible();

  // 删除服务器
  await page.click('[data-testid="server-menu"]');
  await page.click('[data-testid="delete-server"]');
  await page.click('[data-testid="confirm-delete"]');

  // 验证服务器已删除
  await expect(page.locator('text=Test Server')).not.toBeVisible();
});

test('should display metrics chart', async ({ page }) => {
  await page.goto('http://localhost:1420/cpu');

  // 等待图表加载
  await page.waitForSelector('[data-testid="cpu-chart"]');

  // 验证图表存在
  const chart = page.locator('[data-testid="cpu-chart"]');
  await expect(chart).toBeVisible();

  // 验证时间范围选择器
  await page.click('[data-testid="time-range-selector"]');
  await page.click('text=24h');

  // 验证图表更新
  await page.waitForTimeout(1000);
  await expect(chart).toBeVisible();
});
```

---

### 2.4 性能测试 (Performance Tests)

**目标**：验证系统性能指标

**测试场景**
- 大量服务器并发轮询（50+ 服务器）
- 数据库查询性能（历史数据查询 < 500ms）
- UI 渲染性能（图表渲染 < 100ms）
- 内存占用（长时间运行不泄漏）

**工具**
- Rust: `criterion` (基准测试)
- TypeScript: Lighthouse (性能审计)
- 数据库: `EXPLAIN QUERY PLAN` (查询优化)

**示例：Rust 性能测试**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_metric_parsing(c: &mut Criterion) {
    let adapter = NodeExporterAdapter::new();
    let sample_data = include_str!("../fixtures/node_exporter_sample.txt");

    c.bench_function("parse node_exporter metrics", |b| {
        b.iter(|| {
            adapter.parse_prometheus_text(black_box(sample_data))
        });
    });
}

criterion_group!(benches, benchmark_metric_parsing);
criterion_main!(benches);
```

---

## 3. 测试数据管理

### 3.1 测试夹具 (Fixtures)

**位置**
- Rust: `src-tauri/tests/fixtures/`
- TypeScript: `src/__tests__/fixtures/`
- Go: `agent/testdata/`

**内容**
- 示例 Prometheus 指标数据
- 示例 Go Agent JSON 响应
- 示例数据库状态

**示例**
```
tests/fixtures/
├── node_exporter_sample.txt      # Prometheus 文本格式
├── go_agent_response.json        # Go Agent JSON 响应
├── alert_rules.json               # 告警规则示例
└── metric_history.json            # 历史数据示例
```

### 3.2 Mock 数据

**策略**
- HTTP 请求：使用 `mockito` (Rust) / `msw` (TypeScript)
- 数据库：使用内存 SQLite
- 时间：使用固定时间戳避免时区问题

---

## 4. 持续集成 (CI)

### 4.1 GitHub Actions 工作流

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test-rust:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Run tests
        run: cd src-tauri && cargo test --all-features
      - name: Run clippy
        run: cd src-tauri && cargo clippy -- -D warnings

  test-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: pnpm/action-setup@v2
      - uses: actions/setup-node@v3
        with:
          node-version: 18
          cache: 'pnpm'
      - run: pnpm install
      - run: pnpm test
      - run: pnpm lint

  test-go-agent:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-go@v4
        with:
          go-version: '1.21'
      - name: Run tests
        run: cd agent && go test -v -race -coverprofile=coverage.out ./...
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./agent/coverage.out
```

### 4.2 测试报告

- 代码覆盖率报告：Codecov
- 测试结果可视化：GitHub Actions Summary
- 性能回归检测：Criterion 基准对比

---

## 5. 测试最佳实践

### 5.1 编写可测试代码

**依赖注入**
```rust
// ❌ 不好：硬编码依赖
pub struct MetricCollector {
    client: reqwest::Client,
}

// ✅ 好：依赖注入
pub struct MetricCollector<C: HttpClient> {
    client: C,
}
```

**纯函数优先**
```rust
// ✅ 纯函数，易于测试
pub fn calculate_rate(current: f64, previous: f64, duration: i64) -> f64 {
    (current - previous) / duration as f64
}
```

### 5.2 测试命名规范

```rust
#[test]
fn test_<功能>_<场景>_<预期结果>() {
    // 示例
    // test_derive_rate_with_counter_reset_returns_none
    // test_alert_rule_with_threshold_exceeded_fires_alert
}
```

### 5.3 AAA 模式

```rust
#[test]
fn test_example() {
    // Arrange（准备）
    let engine = DerivedMetricsEngine::new();
    let metric = create_test_metric();

    // Act（执行）
    let result = engine.derive_rate(&metric);

    // Assert（断言）
    assert_eq!(result, Some(100.0));
}
```

### 5.4 避免测试脆弱性

- 不依赖外部服务（使用 Mock）
- 不依赖文件系统（使用内存数据库）
- 不依赖时间（使用固定时间戳）
- 不依赖执行顺序（测试独立）

---

## 6. 测试检查清单

### 6.1 代码提交前

- [ ] 所有单元测试通过
- [ ] 新增代码有对应测试
- [ ] 代码覆盖率未下降
- [ ] Clippy/ESLint 无警告

### 6.2 发布前

- [ ] 所有集成测试通过
- [ ] E2E 测试通过
- [ ] 性能测试无回归
- [ ] 跨平台测试通过（macOS/Windows/Linux）
- [ ] 手动冒烟测试通过

---

## 7. 测试工具链

### 7.1 Rust
- `cargo test`: 单元测试和集成测试
- `cargo-tarpaulin`: 代码覆盖率
- `criterion`: 性能基准测试
- `mockito`: HTTP Mock
- `tokio::test`: 异步测试

### 7.2 TypeScript
- Jest: 单元测试框架
- React Testing Library: 组件测试
- Playwright: E2E 测试
- MSW: API Mock
- Vitest: 快速单元测试（可选）

### 7.3 Go
- `go test`: 标准测试工具
- `testify`: 断言库
- `go-mock`: Mock 生成
- `go test -race`: 竞态检测
- `go test -bench`: 性能测试

---

## 8. 测试覆盖率目标

| 模块 | 目标覆盖率 | 优先级 |
|------|-----------|--------|
| Adapters | 90% | 高 |
| Metrics Engine | 90% | 高 |
| Alert Engine | 85% | 高 |
| Storage | 80% | 中 |
| Probes | 75% | 中 |
| Commands | 70% | 中 |
| UI Components | 60% | 低 |

---

## 9. 已知测试限制

### 9.1 ICMP Ping 测试
- 需要 root 权限，CI 环境可能无法执行
- 解决方案：使用 Mock 或跳过集成测试

### 9.2 桌面通知测试
- 无头环境无法测试通知显示
- 解决方案：测试通知 API 调用，不测试 UI 显示

### 9.3 跨平台测试
- Windows 路径分隔符差异
- 解决方案：使用 `std::path::Path` 处理路径

---

## 10. 测试维护

### 10.1 定期审查
- 每月审查测试覆盖率
- 删除过时的测试
- 更新测试数据

### 10.2 测试债务管理
- 标记 `#[ignore]` 的测试需要修复
- 跟踪 TODO 注释
- 优先修复失败的测试

---

## 附录：测试命令速查

```bash
# Rust 测试
cd src-tauri
cargo test                          # 运行所有测试
cargo test --test integration       # 运行集成测试
cargo test -- --nocapture           # 显示 println! 输出
cargo test --release                # Release 模式测试

# TypeScript 测试
pnpm test                           # 运行所有测试
pnpm test --watch                   # 监听模式
pnpm test --coverage                # 生成覆盖率报告
pnpm test:e2e                       # 运行 E2E 测试

# Go 测试
cd agent
go test ./...                       # 运行所有测试
go test -v ./...                    # 详细输出
go test -race ./...                 # 竞态检测
go test -cover ./...                # 覆盖率
```
