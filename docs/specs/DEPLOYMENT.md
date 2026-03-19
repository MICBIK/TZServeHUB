# 部署指南

## 概述

ServerHUB 包含两个主要组件：
1. **桌面应用**：Tauri 2 桌面客户端（控制平面）
2. **Go Agent**：远程服务器上的轻量级代理

本指南涵盖两者的部署方法。

---

## 1. 桌面应用部署

### 1.1 系统要求

| 平台 | 最低版本 | 推荐配置 |
|------|---------|---------|
| macOS | 11.0+ (Big Sur) | macOS 13+ (Ventura) |
| Windows | 10 (1809+) | Windows 11 |
| Linux | Ubuntu 20.04+ | Ubuntu 22.04+ |
| 内存 | 2GB | 4GB+ |
| 磁盘 | 500MB | 1GB+ |

### 1.2 从源码构建

#### 前置依赖

**macOS**
```bash
# 安装 Xcode Command Line Tools
xcode-select --install

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js 和 pnpm
brew install node
npm install -g pnpm
```

**Windows**
```powershell
# 安装 Rust (下载 rustup-init.exe)
# https://www.rust-lang.org/tools/install

# 安装 Node.js (下载安装包)
# https://nodejs.org/

# 安装 pnpm
npm install -g pnpm

# 安装 WebView2 (Windows 10 需要)
# https://developer.microsoft.com/microsoft-edge/webview2/
```

**Linux (Ubuntu/Debian)**
```bash
# 安装系统依赖
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js 和 pnpm
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs
npm install -g pnpm
```

#### 构建步骤

```bash
# 克隆仓库
git clone https://github.com/ha1den/serverhub.git
cd serverhub

# 安装前端依赖
pnpm install

# 开发模式运行
pnpm tauri dev

# 生产构建
pnpm tauri build
```

#### 构建产物位置

| 平台 | 路径 | 格式 |
|------|------|------|
| macOS | `src-tauri/target/release/bundle/dmg/` | `.dmg` |
| macOS | `src-tauri/target/release/bundle/macos/` | `.app` |
| Windows | `src-tauri/target/release/bundle/msi/` | `.msi` |
| Windows | `src-tauri/target/release/bundle/nsis/` | `.exe` |
| Linux | `src-tauri/target/release/bundle/deb/` | `.deb` |
| Linux | `src-tauri/target/release/bundle/appimage/` | `.AppImage` |

### 1.3 安装预构建包

#### macOS

```bash
# 下载 DMG 文件
curl -LO https://github.com/ha1den/serverhub/releases/download/v0.1.0/ServerHUB_0.1.0_x64.dmg

# 挂载并安装
open ServerHUB_0.1.0_x64.dmg
# 拖拽 ServerHUB.app 到 Applications 文件夹

# 首次运行需要允许（系统偏好设置 → 安全性与隐私）
```

#### Windows

```powershell
# 下载 MSI 安装包
Invoke-WebRequest -Uri "https://github.com/ha1den/serverhub/releases/download/v0.1.0/ServerHUB_0.1.0_x64.msi" -OutFile "ServerHUB_0.1.0_x64.msi"

# 运行安装程序
Start-Process msiexec.exe -ArgumentList "/i ServerHUB_0.1.0_x64.msi" -Wait
```

#### Linux (Ubuntu/Debian)

```bash
# 下载 DEB 包
wget https://github.com/ha1den/serverhub/releases/download/v0.1.0/serverhub_0.1.0_amd64.deb

# 安装
sudo dpkg -i serverhub_0.1.0_amd64.deb

# 修复依赖（如果需要）
sudo apt-get install -f

# 运行
serverhub
```

#### Linux (AppImage)

```bash
# 下载 AppImage
wget https://github.com/ha1den/serverhub/releases/download/v0.1.0/ServerHUB_0.1.0_amd64.AppImage

# 添加执行权限
chmod +x ServerHUB_0.1.0_amd64.AppImage

# 运行
./ServerHUB_0.1.0_amd64.AppImage
```

### 1.4 数据目录

桌面应用的数据存储位置：

| 平台 | 路径 |
|------|------|
| macOS | `~/Library/Application Support/com.serverhub.app/` |
| Windows | `%APPDATA%\com.serverhub.app\` |
| Linux | `~/.local/share/com.serverhub.app/` |

**数据库文件**: `data.db`

**备份数据**
```bash
# macOS/Linux
cp ~/Library/Application\ Support/com.serverhub.app/data.db ~/serverhub-backup.db

# Windows
copy %APPDATA%\com.serverhub.app\data.db %USERPROFILE%\serverhub-backup.db
```

---

## 2. Go Agent 部署

### 2.1 系统要求

- 操作系统：Linux (推荐 Ubuntu 20.04+) / macOS / Windows
- 内存：≥ 128MB
- 磁盘：≥ 50MB
- Go 版本：1.21+ (仅构建时需要)

### 2.2 从源码构建

```bash
# 克隆仓库
git clone https://github.com/ha1den/serverhub.git
cd serverhub/agent

# 安装依赖
go mod tidy

# 构建二进制
go build -o serverhub-agent cmd/serverhub-agent/main.go

# 交叉编译（可选）
# Linux AMD64
GOOS=linux GOARCH=amd64 go build -o serverhub-agent-linux-amd64 cmd/serverhub-agent/main.go

# Linux ARM64
GOOS=linux GOARCH=arm64 go build -o serverhub-agent-linux-arm64 cmd/serverhub-agent/main.go

# macOS
GOOS=darwin GOARCH=amd64 go build -o serverhub-agent-darwin-amd64 cmd/serverhub-agent/main.go

# Windows
GOOS=windows GOARCH=amd64 go build -o serverhub-agent-windows-amd64.exe cmd/serverhub-agent/main.go
```

### 2.3 配置文件

创建 `config.yaml`：

```yaml
server:
  # 监听端口
  port: 8080

  # 认证令牌（必须配置）
  auth_token: "your-secret-token-here"

  # 主机名（可选，默认自动检测）
  hostname: ""

collector:
  # 采集间隔（秒���
  interval_sec: 5

logging:
  # 日志级别：debug, info, warn, error
  level: info

  # 日志文件路径（可选，默认 stdout）
  file: /var/log/serverhub-agent.log
```

**生成安全令牌**
```bash
# 使用 openssl 生成随机令牌
openssl rand -base64 32
```

### 2.4 部署方式

#### 方式 1：Systemd 服务（推荐）

**创建服务文件** `/etc/systemd/system/serverhub-agent.service`：

```ini
[Unit]
Description=ServerHUB Agent
Documentation=https://github.com/ha1den/serverhub
After=network.target

[Service]
Type=simple
User=serverhub
Group=serverhub
WorkingDirectory=/opt/serverhub
ExecStart=/opt/serverhub/serverhub-agent -config /etc/serverhub/config.yaml
Restart=on-failure
RestartSec=5s

# 安全加固
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/serverhub

# 资源限制
LimitNOFILE=65536
LimitNPROC=512

[Install]
WantedBy=multi-user.target
```

**部署步骤**：

```bash
# 创建用户
sudo useradd -r -s /bin/false serverhub

# 创建目录
sudo mkdir -p /opt/serverhub
sudo mkdir -p /etc/serverhub
sudo mkdir -p /var/log/serverhub

# 复制文件
sudo cp serverhub-agent /opt/serverhub/
sudo cp config.yaml /etc/serverhub/

# 设置权限
sudo chown -R serverhub:serverhub /opt/serverhub
sudo chown -R serverhub:serverhub /var/log/serverhub
sudo chmod 600 /etc/serverhub/config.yaml
sudo chmod 755 /opt/serverhub/serverhub-agent

# 启动服务
sudo systemctl daemon-reload
sudo systemctl enable serverhub-agent
sudo systemctl start serverhub-agent

# 查看状态
sudo systemctl status serverhub-agent

# 查看日志
sudo journalctl -u serverhub-agent -f
```

#### 方式 2：Docker 容器

**Dockerfile**：

```dockerfile
FROM golang:1.21-alpine AS builder

WORKDIR /build
COPY . .
RUN go mod download
RUN CGO_ENABLED=0 go build -o serverhub-agent cmd/serverhub-agent/main.go

FROM alpine:latest

RUN apk --no-cache add ca-certificates
WORKDIR /app

COPY --from=builder /build/serverhub-agent .
COPY config.yaml .

EXPOSE 8080

USER nobody
ENTRYPOINT ["./serverhub-agent", "-config", "config.yaml"]
```

**构建和运行**：

```bash
# 构建镜像
docker build -t serverhub-agent:0.1.0 .

# 运行容器
docker run -d \
  --name serverhub-agent \
  --restart unless-stopped \
  -p 8080:8080 \
  -v /path/to/config.yaml:/app/config.yaml:ro \
  serverhub-agent:0.1.0

# 查看日志
docker logs -f serverhub-agent
```

**Docker Compose**：

```yaml
version: '3.8'

services:
  serverhub-agent:
    image: serverhub-agent:0.1.0
    container_name: serverhub-agent
    restart: unless-stopped
    ports:
      - "8080:8080"
    volumes:
      - ./config.yaml:/app/config.yaml:ro
    environment:
      - TZ=Asia/Shanghai
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

```bash
# 启动
docker-compose up -d

# 停止
docker-compose down
```

#### 方式 3：直接运行（开发/测试）

```bash
# 前台运行
./serverhub-agent -config config.yaml

# 后台运行（nohup）
nohup ./serverhub-agent -config config.yaml > serverhub-agent.log 2>&1 &

# 后台运行（screen）
screen -S serverhub-agent
./serverhub-agent -config config.yaml
# Ctrl+A, D 分离会话

# 后台运行（tmux）
tmux new -s serverhub-agent
./serverhub-agent -config config.yaml
# Ctrl+B, D 分离会话
```

### 2.5 验证部署

```bash
# 健康检查
curl http://localhost:8080/api/health

# 预期响应
# {"status":"ok","hostname":"your-hostname"}

# 获取指标（需要认证）
curl -H "Authorization: Bearer your-secret-token-here" \
     http://localhost:8080/api/metrics

# 预期响应（JSON 格式的指标数据）
```

---

## 3. 网络配置

### 3.1 防火墙规则

**Linux (iptables)**
```bash
# 允许 Go Agent 端口
sudo iptables -A INPUT -p tcp --dport 8080 -j ACCEPT
sudo iptables-save > /etc/iptables/rules.v4
```

**Linux (firewalld)**
```bash
sudo firewall-cmd --permanent --add-port=8080/tcp
sudo firewall-cmd --reload
```

**Linux (ufw)**
```bash
sudo ufw allow 8080/tcp
sudo ufw reload
```

### 3.2 反向代理（可选）

#### Nginx

```nginx
server {
    listen 443 ssl http2;
    server_name monitor.example.com;

    ssl_certificate /etc/ssl/certs/monitor.crt;
    ssl_certificate_key /etc/ssl/private/monitor.key;

    location /api/ {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # 超时设置
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }
}
```

#### Caddy

```caddyfile
monitor.example.com {
    reverse_proxy /api/* localhost:8080
}
```

### 3.3 SSH 隧道（私有网络访问）

**场景**：桌面应用无法直接访问远程服务器（防火墙/NAT）

**建立隧道**：
```bash
# 在桌面端执行
ssh -L 8080:localhost:8080 user@remote-server

# 后台运行
ssh -fNL 8080:localhost:8080 user@remote-server

# 配置桌面应用
# Host: localhost
# Port: 8080
```

**自动重连**（使用 autossh）：
```bash
# 安装 autossh
sudo apt install autossh  # Ubuntu/Debian
brew install autossh      # macOS

# 运行
autossh -M 0 -fNL 8080:localhost:8080 user@remote-server
```

---

## 4. 安全加固

### 4.1 Go Agent 安全

**最小权限运行**
```bash
# 创建专用用户（无登录权限）
sudo useradd -r -s /bin/false serverhub

# 限制文件权限
sudo chown root:serverhub /opt/serverhub/serverhub-agent
sudo chmod 750 /opt/serverhub/serverhub-agent
sudo chmod 600 /etc/serverhub/config.yaml
```

**TLS 加密（可选）**

修改 Go Agent 代码支持 HTTPS：
```go
// internal/server/server.go
func Run(cfg *config.Config) error {
    r := gin.Default()
    // ... 路由配置

    // HTTPS
    return r.RunTLS(
        fmt.Sprintf(":%d", cfg.Port),
        "/etc/serverhub/server.crt",
        "/etc/serverhub/server.key",
    )
}
```

生成自签名证书：
```bash
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -nodes
```

**速率限制**

使用 Nginx 限制请求频率：
```nginx
limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s;

location /api/ {
    limit_req zone=api_limit burst=20 nodelay;
    proxy_pass http://127.0.0.1:8080;
}
```

### 4.2 网络隔离

**仅允许特定 IP 访问**

iptables：
```bash
# 仅允许桌面端 IP
sudo iptables -A INPUT -p tcp --dport 8080 -s 192.168.1.100 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 8080 -j DROP
```

Nginx：
```nginx
location /api/ {
    allow 192.168.1.100;
    deny all;
    proxy_pass http://127.0.0.1:8080;
}
```

### 4.3 令牌管理

**定期轮换令牌**
```bash
# 生成新令牌
NEW_TOKEN=$(openssl rand -base64 32)

# 更新配置
sudo sed -i "s/auth_token: .*/auth_token: \"$NEW_TOKEN\"/" /etc/serverhub/config.yaml

# 重启服务
sudo systemctl restart serverhub-agent

# 更新桌面应用配置
```

---

## 5. 监控与维护

### 5.1 日志管理

**日志轮转** `/etc/logrotate.d/serverhub-agent`：

```
/var/log/serverhub/*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    create 0640 serverhub serverhub
    postrotate
        systemctl reload serverhub-agent > /dev/null 2>&1 || true
    endscript
}
```

**查看日志**：
```bash
# Systemd 日志
sudo journalctl -u serverhub-agent -f

# 文件日志
sudo tail -f /var/log/serverhub/serverhub-agent.log

# 查看最近 100 行
sudo journalctl -u serverhub-agent -n 100
```

### 5.2 健康检查

**监控脚本** `check_agent.sh`：

```bash
#!/bin/bash

ENDPOINT="http://localhost:8080/api/health"
TIMEOUT=5

if curl -sf --max-time $TIMEOUT "$ENDPOINT" > /dev/null; then
    echo "OK: Agent is healthy"
    exit 0
else
    echo "CRITICAL: Agent is down"
    # 尝试重启
    sudo systemctl restart serverhub-agent
    exit 2
fi
```

**Cron 定时检查**：
```bash
# 每 5 分钟检查一次
*/5 * * * * /opt/serverhub/check_agent.sh >> /var/log/serverhub/health-check.log 2>&1
```

### 5.3 性能监控

**资源使用情况**：
```bash
# CPU 和内存
ps aux | grep serverhub-agent

# 详细统计
top -p $(pgrep serverhub-agent)

# Systemd 资源统计
systemctl status serverhub-agent
```

**网络连接**：
```bash
# 查看监听端口
sudo netstat -tlnp | grep serverhub-agent

# 查看活动连接
sudo ss -tnp | grep serverhub-agent
```

---

## 6. 升级与回滚

### 6.1 桌面应用升级

**macOS/Windows**：
- 下载新版本安装包
- 运行安装程序（自动覆盖旧版本）
- 数据库自动迁移

**Linux**：
```bash
# 下载新版本
wget https://github.com/ha1den/serverhub/releases/download/v0.2.0/serverhub_0.2.0_amd64.deb

# 升级
sudo dpkg -i serverhub_0.2.0_amd64.deb
```

### 6.2 Go Agent 升级

```bash
# 备份当前版本
sudo cp /opt/serverhub/serverhub-agent /opt/serverhub/serverhub-agent.bak

# 停止服务
sudo systemctl stop serverhub-agent

# 替换二进制
sudo cp serverhub-agent-new /opt/serverhub/serverhub-agent
sudo chmod 755 /opt/serverhub/serverhub-agent

# 启动服务
sudo systemctl start serverhub-agent

# 验证
curl http://localhost:8080/api/health
```

### 6.3 回滚

**Go Agent 回滚**：
```bash
# 停止服务
sudo systemctl stop serverhub-agent

# 恢复旧版本
sudo cp /opt/serverhub/serverhub-agent.bak /opt/serverhub/serverhub-agent

# 启动服务
sudo systemctl start serverhub-agent
```

**数据库回滚**：
```bash
# 恢复备份
cp ~/serverhub-backup.db ~/Library/Application\ Support/com.serverhub.app/data.db
```

---

## 7. 故障排查

### 7.1 常见问题

#### 桌面应用无法启动

**macOS**：
```bash
# 检查 Gatekeeper
xattr -d com.apple.quarantine /Applications/ServerHUB.app

# 查看崩溃日志
cat ~/Library/Logs/DiagnosticReports/ServerHUB*.crash
```

**Linux**：
```bash
# 检查依赖
ldd /usr/bin/serverhub

# 查看日志
journalctl -xe | grep serverhub
```

#### Go Agent 无法连接

```bash
# 检查服务状态
sudo systemctl status serverhub-agent

# 检查端口监听
sudo netstat -tlnp | grep 8080

# 检查防火墙
sudo iptables -L -n | grep 8080

# 测试连接
curl -v http://localhost:8080/api/health
```

#### 认证失败

```bash
# 检查令牌配置
sudo cat /etc/serverhub/config.yaml | grep auth_token

# 测试认证
curl -H "Authorization: Bearer your-token" http://localhost:8080/api/metrics
```

### 7.2 调试模式

**Go Agent 调试**：
```yaml
# config.yaml
logging:
  level: debug
```

```bash
# 重启服务
sudo systemctl restart serverhub-agent

# 查看详细日志
sudo journalctl -u serverhub-agent -f
```

**桌面应用调试**：
```bash
# 开发模式运行
pnpm tauri dev

# 查看 Rust 日志
RUST_LOG=debug pnpm tauri dev

# 查看前端控制台（开发者工具）
```

---

## 8. 生产环境检查清单

### 部署前

- [ ] 已生成强随机令牌（≥32 字符）
- [ ] 已配置防火墙规则
- [ ] 已创建专用用户（非 root）
- [ ] 已设置文件权限（600 for config, 755 for binary）
- [ ] 已配置日志轮转
- [ ] 已设置健康检查
- [ ] 已备份配置文件

### 部署后

- [ ] 服务正常启动（systemctl status）
- [ ] 健康检查通过（/api/health）
- [ ] 认证正常工作（/api/metrics）
- [ ] 日志正常输出
- [ ] 资源使用正常（CPU < 5%, 内存 < 100MB）
- [ ] 桌面应用可连接
- [ ] 指标数据正常采集

### 安全检查

- [ ] 令牌未使用默认值
- [ ] 配置文件权限正确（600）
- [ ] 服务以非特权用户运行
- [ ] 防火墙规则已配置
- [ ] 仅必要端口开放
- [ ] 考虑使用 TLS/SSH 隧道
- [ ] 日志不包含敏感信息

---

## 9. 参考资源

### 官方文档
- [Tauri 部署指南](https://tauri.app/v1/guides/building/)
- [Go 交叉编译](https://go.dev/doc/install/source#environment)
- [Systemd 服务管理](https://www.freedesktop.org/software/systemd/man/systemd.service.html)

### 社区资源
- GitHub Issues: https://github.com/ha1den/serverhub/issues
- 讨论区: https://github.com/ha1den/serverhub/discussions

---

## 附录

### A. 配置文件完整示例

**config.yaml**（Go Agent）：
```yaml
server:
  port: 8080
  auth_token: "replace-with-secure-random-token"
  hostname: "prod-server-01"

collector:
  interval_sec: 5

logging:
  level: info
  file: /var/log/serverhub/agent.log
```

### B. Systemd 服务完整示例

```ini
[Unit]
Description=ServerHUB Agent - Remote Server Monitoring Agent
Documentation=https://github.com/ha1den/serverhub
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=serverhub
Group=serverhub
WorkingDirectory=/opt/serverhub
ExecStart=/opt/serverhub/serverhub-agent -config /etc/serverhub/config.yaml
ExecReload=/bin/kill -HUP $MAINPID
Restart=on-failure
RestartSec=5s
TimeoutStopSec=30s

# 安全加固
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/serverhub
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictRealtime=true
RestrictNamespaces=true

# 资源限制
LimitNOFILE=65536
LimitNPROC=512
MemoryLimit=256M
CPUQuota=50%

# 日志
StandardOutput=journal
StandardError=journal
SyslogIdentifier=serverhub-agent

[Install]
WantedBy=multi-user.target
```

### C. Docker Compose 完整示例

```yaml
version: '3.8'

services:
  serverhub-agent:
    image: serverhub-agent:0.1.0
    container_name: serverhub-agent
    restart: unless-stopped
    ports:
      - "8080:8080"
    volumes:
      - ./config.yaml:/app/config.yaml:ro
      - agent-logs:/var/log/serverhub
    environment:
      - TZ=Asia/Shanghai
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/health"]
      interval: 30s
      timeout: 5s
      retries: 3
      start_period: 10s
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 256M
        reservations:
          cpus: '0.1'
          memory: 64M

volumes:
  agent-logs:
```
