# 审计报告：research-3 + optimization-claude

审计时间：2026-03-19

---

## 一、tauri-monitor-research-3.md 审计

### 问题 1：Electron 项目数量与总结不符（严重）

**位置**：Summary 部分
**原文**：
```
Total projects found: **42 projects**
- **Tauri-based**: 21 projects
- **Electron-based**: 21 projects
```

**实际情况**：正文中 Electron 部分只列出了 5 个项目（编号 1-5），而非 21 个。Tauri 部分确实列出了 21 个项目（编号 1-21，已逐一核实）。

**结论**：Summary 中 "Electron-based: 21 projects" 与正文严重不符，正文只有 5 个 Electron 项目。"Total projects found: 42" 也因此有误，实际应为 26 个（21 Tauri + 5 Electron）。

### 问题 2：Tauri 项目分类基本合理，但存在轻微分类模糊

- `felps-dev/ghtray`（GitHub PR 监控）归入 "System Monitoring Applications" 而非 "Specialized Monitoring Applications"，与其功能定位不完全吻合，但不影响整体准确性。
- `kimdj2/claude-token-monitor` 和 `boparaiamrit/claude-session-monitor` 均为 Claude 相关监控，归类在不同子分类下，可考虑合并为一类。

### 问题 3：无重大遗漏

Tauri 21 个项目逐一核实，编号连续，内容完整，无明显遗漏。

---

## 二、tauri-monitor-optimization-claude.md 审计

### 问题 1：Tauri 2.0 features 写法错误（严重）

**位置**：第 21 行
**原文**：
```toml
tauri = { version = "2.0", features = ["system-tray", "notification"] }
```

**问题**：在 Tauri 2.0 中，`system-tray` 和 `notification` 已不再是 `tauri` crate 的内置 feature，而是独立插件：
- 系统托盘 → `tauri-plugin-tray`
- 通知 → `tauri-plugin-notification`

**正确写法**：
```toml
tauri = { version = "2.0" }
tauri-plugin-tray = "2.0"
tauri-plugin-notification = "2.0"
```

### 问题 2：SystemTray / SystemTrayMenu / SystemTrayMenuItem API 已废弃（严重）

**位置**：第 256-266 行
**原文**：
```rust
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem};

fn create_system_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(SystemTrayMenuItem::new("显示", "show"))
        ...
    SystemTray::new().with_menu(tray_menu)
}
```

**问题**：`tauri::SystemTray`、`tauri::SystemTrayMenu`、`tauri::SystemTrayMenuItem` 是 Tauri v1 的 API，在 Tauri 2.0 中已完全移除。Tauri 2.0 使用 `tauri-plugin-tray` 插件，API 改为：
```rust
use tauri_plugin_tray::TrayIconBuilder;
```

### 问题 3：tauri::api::notification::Notification 已废弃（严重）

**位置**：第 295-303 行
**原文**：
```rust
use tauri::api::notification::Notification;

fn send_alert(title: &str, body: &str) {
    Notification::new("com.monitor.app")
        .title(title)
        .body(body)
        .show()
        .unwrap();
}
```

**问题**：`tauri::api::notification` 模块在 Tauri 2.0 中已移除。正确做法是使用 `tauri-plugin-notification`：
```rust
use tauri_plugin_notification::NotificationExt;

app.notification()
    .builder()
    .title(title)
    .body(body)
    .show()
    .unwrap();
```

### 问题 4：sysinfo API 使用正确（无问题）

**位置**：第 198 行
**原文**：
```rust
cpu_usage: sys.global_cpu_info().cpu_usage(),
```

**核实**：`sysinfo 0.30+` 中 `global_cpu_info()` 是正确的 API（旧版 `global_processor_info()` 已在 0.26 版本废弃，0.30 中已移除）。此处写法正确。

### 问题 5：性能对比数据缺乏依据（中等）

**位置**：第 244-249 行
**原文**：
```
| 启动速度 | 2-3秒 | 5-8秒 (含网络) |
| 内存占用 | 50-80MB | 150-300MB |
| CPU 占用 | 1-3% | 3-8% |
| 实时性 | 毫秒级 | 秒级 (受网络影响) |
```

**问题**：这些数据未注明来源，属于估算值。实际性能高度依赖具体实现、硬件配置和应用复杂度。文档末尾声称"基于对 28+ 个 Rust 桌面监控项目和 21 个 Tauri 应用的深度分析"，但 research-3.md 中 Electron 项目实际只有 5 个，"21 个 Electron 应用"的对比基础不成立，性能数据可信度存疑。

### 问题 6：reqwest 版本偏旧（轻微）

**位置**：第 22 行
**原文**：
```toml
reqwest = { version = "0.11", features = ["json"] }
```

**问题**：reqwest 当前稳定版为 0.12，0.11 与 Tauri 2.0 生态（依赖 tokio 1.x）兼容，但建议升级到 0.12 以获得最新修复。属于轻微问题，不影响编译。

---

## 汇总

| 文件 | 问题数 | 严重 | 中等 | 轻微 |
|------|--------|------|------|------|
| research-3.md | 3 | 1 | 0 | 2 |
| optimization-claude.md | 6 | 3 | 1 | 2 |

**最高优先级修复项**：
1. research-3.md：修正 Electron 项目数量（5 个，非 21 个），更新 Total 为 26
2. optimization-claude.md：移除 `features = ["system-tray", "notification"]`，改用独立插件依赖
3. optimization-claude.md：替换 Tauri v1 的 SystemTray API 为 tauri-plugin-tray
4. optimization-claude.md：替换 `tauri::api::notification` 为 tauri-plugin-notification
