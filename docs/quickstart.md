# 快速开始

本指南将帮助你快速搭建 ServerHUB 开发环境并运行应用。

## 前置要求

- Node.js 18+
- pnpm 8+
- Rust 1.70+ (Tauri 依赖)
- 操作系统：macOS / Linux / Windows

## 安装步骤

### 1. 克隆仓库

```bash
git clone <repository-url>
cd ServerHUB
```

### 2. 安装依赖

```bash
pnpm install
```

这将安装所有前端和 Tauri 所需的依赖包。

### 3. 启动开发服务器

```bash
pnpm tauri dev
```

该命令会：
- 启动 Vite 开发服务器（端口 4175）
- 编译 Rust 后端
- 启动 Tauri 桌面应用窗口

首次运行可能需要几分钟来编译 Rust 依赖。

## 添加第一个服务器

1. 应用启动后，点击界面上的 "添加服务器" 按钮
2. 填写服务器信息：
   - 名称：为服务器设置一个易识别的名称
   - 主机地址：服务器 IP 或域名
   - 端口：SSH 端口（默认 22）
   - 认证方式：选择密码或密钥认证
3. 点击 "保存" 完成添加
4. 服务器将开始监控，实时显示 CPU、内存、磁盘等指标

## 常见问题

### Rust 编译失败

**问题**：首次运行 `pnpm tauri dev` 时 Rust 编译报错

**解决方案**：
- 确认已安装 Rust：`rustc --version`
- 如未安装，访问 https://rustup.rs/ 安装 Rust
- macOS 用户可能需要安装 Xcode Command Line Tools：
  ```bash
  xcode-select --install
  ```

### 端口被占用

**问题**：Vite 开发服务器启动失败，提示端口 4175 被占用

**解决方案**：
- 查找占用端口的进程：
  ```bash
  lsof -i :4175
  ```
- 终止该进程或修改 `src-tauri/tauri.conf.json` 中的 `devUrl` 端口

### pnpm 命令未找到

**问题**：执行 `pnpm install` 时提示命令不存在

**解决方案**：
- 安装 pnpm：
  ```bash
  npm install -g pnpm
  ```
- 或使用 Homebrew（macOS）：
  ```bash
  brew install pnpm
  ```

### 应用窗口无法打开

**问题**：编译成功但桌面应用窗口未显示

**解决方案**：
- 检查终端是否有错误日志
- 确认防火墙未阻止应用
- 尝试清理构建缓存：
  ```bash
  rm -rf src-tauri/target
  pnpm tauri dev
  ```

### 热重载不生效

**问题**：修改代码后应用未自动刷新

**解决方案**：
- 前端代码修改应自动触发 Vite HMR
- Rust 代码修改需要重启 `pnpm tauri dev`
- 检查文件监听是否正常工作（某些文件系统可能有限制）

## 下一步

- 查看 [架构文档](./architecture.md) 了解项目结构
- 查看 [项目工具](./project-tooling.md) 了解开发工具链
- 开始开发你的第一个功能

## 获取帮助

如遇到其他问题，请：
- 查看项目 Issues
- 查阅 [Tauri 官方文档](https://tauri.app/)
- 查阅 [Vite 官方文档](https://vitejs.dev/)
