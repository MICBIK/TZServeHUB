# Integration Tests

## Test Strategy

### Test Pyramid

```
        /\
       /  \  E2E Tests (Tauri App Smoke Tests)
      /----\
     /      \ Integration Tests (API + Storage)
    /--------\
   /          \ Unit Tests (Business Logic)
  /____________\
```

### 1. Unit Tests
**Location**: `src-tauri/src/*/tests.rs` (inline modules)

**Scope**:
- Adapter parsing logic (node_exporter, Go agent JSON)
- Derived metrics engine (counter→rate conversion)
- Alert rule evaluation
- Metric rollup calculations

**Tools**: `cargo test`

### 2. Integration Tests
**Location**: `tests/integration/`

**Scope**:
- Database operations (SQLite CRUD, migrations)
- Adapter HTTP client behavior (mock servers)
- Scheduler polling logic with real timers
- Probe system (ping/TCP/DNS with localhost)

**Tools**: `cargo test --test integration_*`

### 3. End-to-End Tests
**Location**: `tests/e2e/`

**Scope**:
- Full Tauri app lifecycle (window creation, IPC)
- Frontend→Backend command flow
- Real adapter integration with test servers
- Desktop notification delivery

**Tools**: Tauri's WebDriver integration (TBD)

---

## Current Test Coverage

### Implemented
- [x] Unit: Adapter parsers (`node_exporter`, `go_agent`)
- [x] Unit: `DerivedMetricsEngine` counter reset handling
- [x] Smoke: Tauri shell boot + `list_servers` IPC on mock runtime

### Planned
- [ ] Integration: Database retention policy
- [ ] Integration: Poller with backoff/jitter
- [ ] E2E: Real desktop runtime smoke test (non-mock)

---

## Running Tests

### Unit Tests (Rust)
```bash
cd src-tauri
cargo test
```

### Integration Tests (Rust)
```bash
cd src-tauri
cargo test --test integration_database
cargo test --test integration_adapters
```

### E2E Tests (Tauri App)
```bash
# TBD: Requires WebDriver setup
pnpm tauri test
```

### Frontend Tests (React)
```bash
# TBD: Requires Vitest/Jest setup
pnpm test
```

---

## Test Data

### Mock Servers
- `tests/fixtures/node_exporter_response.txt`: Sample Prometheus metrics
- `tests/fixtures/go_agent_response.json`: Sample Go agent JSON

### Test Databases
- Integration tests use in-memory SQLite (`:memory:`)
- E2E tests use temporary file databases (cleaned up after run)

---

## Current Smoke Test

已实现位置：`src-tauri/src/lib.rs`

覆盖内容：

1. 使用 Tauri mock runtime 构建应用
2. 执行 `setup`，初始化数据库与后台服务
3. 创建主 webview
4. 通过 IPC 调用 `list_servers`
5. 校验返回可反序列化且默认为空数组

## Placeholder: Real Desktop Runtime Smoke Test

**File**: `tests/e2e/tauri_smoke_test.rs` (not yet implemented)

**Test Cases**:
1. Launch app and verify window opens
2. Call `list_servers` command and verify empty array
3. Call `add_server` with test config
4. Call `list_servers` again and verify server exists
5. Call `remove_server` and verify deletion
6. Close app gracefully

**Dependencies**:
- `tauri-driver` (WebDriver for Tauri)
- `tokio` for async test runtime
- `serde_json` for IPC payload validation

**Example skeleton**:
```rust
#[tokio::test]
async fn test_app_launch_and_basic_commands() {
    // 1. Start Tauri app with test config
    // 2. Wait for window ready
    // 3. Invoke list_servers via IPC
    // 4. Assert response is empty array
    // 5. Cleanup
}
```

---

## CI Integration

### GitHub Actions Workflow (TBD)
```yaml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
      - name: Run unit tests
        run: cd src-tauri && cargo test
      - name: Run integration tests
        run: cd src-tauri && cargo test --test integration_*
```

---

## Notes

- **WAL Mode**: Integration tests should verify SQLite WAL mode is enabled
- **Concurrency**: Test parallel metric writes to ensure no race conditions
- **Error Paths**: Test adapter failures, network timeouts, invalid configs
- **Cleanup**: All tests must clean up resources (temp files, mock servers)
