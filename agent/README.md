# ServerHUB Agent

Go-based metrics collection agent for ServerHUB monitoring system.

## Features

- CPU metrics (total + per-core usage)
- Memory metrics (total, used, available)
- Disk metrics (partitions, usage, I/O counters)
- Network metrics (interface traffic counters)
- HTTP API with token authentication
- Lightweight and cross-platform

## Build

```bash
go build -o serverhub-agent cmd/serverhub-agent/main.go
```

## Configuration

Create `config.yaml`:

```yaml
port: 9100
token: "your-secret-token-here"
hostname: ""
interval: 5
```

## Run

```bash
./serverhub-agent -config config.yaml
```

## API Endpoints

- `GET /api/health` - Health check
- `GET /api/metrics` - Get all metrics (requires Bearer token)
