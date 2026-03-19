# ServerHUB 项目工具清单

## 开发环境

| 工具 | 版本 | 用途 |
|------|------|------|
| Rust | 1.92.0 | 后端核心 |
| Cargo | 1.92.0 | Rust 包管理 |
| Tauri CLI | 2.10.0 | Tauri 构建工具 |
| Node.js | 25.8.0 | 前端运行时 |
| pnpm | (已安装) | 前端包管理（推荐） |
| bun | (已安装) | 备选包管理/运行时 |

## Claude Code Skills（项目相关）

| Skill | 用途 | 触发场景 |
|-------|------|----------|
| `/rust-dev` | Rust 开发规范、Cargo 管理、异步编程 | 编写 Rust 后端代码 |
| `/js-ts-dev` | TypeScript/React 开发、Node 生态 | 编写前端代码 |
| `/database` | SQLite schema 设计、查询优化 | 本地存储模块 |
| `/api-design` | Tauri command 接口设计 | 前后端通信接口 |
| `/testing` | Rust #[test] + Vitest 前端测试 | 单元/集成测试 |
| `/context7` | 查询 Tauri/React/sysinfo 最新文档 | API 验证 |
| `/code-simplifier` | 代码简化与重构 | 代码优化阶段 |
| `/commit` | Git 提交规范 | 版本管理 |
| `/git-workflow` | 分支管理、PR 工作流 | 协作开发 |
| `/deep-thinking` | 复杂架构决策推理 | 架构设计 |
| `/security-architecture` | 安全架构设计 | 安全接入模块 |
| `/network-protocol` | 网络协议、DNS/TLS | 探测子系统 |
| `/docker-k8s` | 容器化部署 | 未来部署阶段 |
| `/devsecops` | CI/CD 安全 | 构建流水线 |

## MCP 工具

| MCP 工具 | 用途 | 使用场景 |
|----------|------|----------|
| `mcp__context7__resolve-library-id` | 查找库的 Context7 ID | 查询 tauri/react/sysinfo 文档前 |
| `mcp__context7__query-docs` | 查询库的最新文档和示例 | 验证 API 签名、查找用法 |

## Rust Crate 依赖规划

### 核心依赖
| Crate | 版本 | 用途 |
|-------|------|------|
| tauri | 2.x | 桌面应用框架 |
| tauri-plugin-shell | 2.x | 系统命令执行 |
| tauri-plugin-notification | 2.x | 桌面通知 |
| sysinfo | 0.33+ | 本地系统信息采集 |
| tokio | 1.x (full) | 异步运行时 |
| serde / serde_json | 1.x | 序列化 |
| sqlx | 0.8+ (sqlite, runtime-tokio) | SQLite 异步操作 |
| reqwest | 0.12+ (json, rustls-tls) | HTTP 客户端（拉取 exporter） |
| thiserror | 2.x | 错误类型定义 |
| tracing / tracing-subscriber | 0.1 / 0.3 | 结构化日志 |
| chrono | 0.4 | 时间处理 |
| surge-ping | 0.8+ | ICMP Ping 探测 |

### 前端依赖
| 包 | 版本 | 用途 |
|----|------|------|
| @tauri-apps/api | ^2.0 | Tauri 前端 API |
| @tauri-apps/plugin-notification | ^2.0 | 通知插件前端绑定 |
| react | ^19.0 | UI 框架 |
| react-dom | ^19.0 | DOM 渲染 |
| typescript | ^5.7 | 类型系统 |
| recharts | ^2.15 | 图表库 |
| antd | ^5.24 | UI 组件库 |
| tailwindcss | ^4.0 | 样式工具 |
| zustand | ^5.0 | 状态管理 |
| @tanstack/react-query | ^5.0 | 数据获取/缓存 |
| dayjs | ^1.11 | 日期处理 |

## 验证链

开发过程中 API 验证优先级：
1. `Grep` 项目内已有用法
2. `mcp__context7__query-docs` 查最新文档
3. `WebSearch` 搜索官方文档
4. 标注 `// TODO: verify` 并告知
