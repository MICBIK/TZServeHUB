#!/bin/bash
# Cross-compile Go agent for Linux targets.
# Run from repo root: bash scripts/build-agents.sh
set -euo pipefail

AGENT_DIR="agent"
OUT_DIR="src-tauri/resources/agents"

echo "Building serverhub-agent for linux/amd64..."
GOOS=linux GOARCH=amd64 CGO_ENABLED=0 \
  go build -C "$AGENT_DIR" -ldflags="-s -w" \
  -o "../$OUT_DIR/serverhub-agent-linux-amd64" \
  cmd/serverhub-agent/main.go

echo "Building serverhub-agent for linux/arm64..."
GOOS=linux GOARCH=arm64 CGO_ENABLED=0 \
  go build -C "$AGENT_DIR" -ldflags="-s -w" \
  -o "../$OUT_DIR/serverhub-agent-linux-arm64" \
  cmd/serverhub-agent/main.go

ls -lh "$OUT_DIR"/serverhub-agent-*
echo "Done."
