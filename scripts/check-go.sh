#!/bin/bash
set -e
cd agent
go test ./...
go build -o serverhub-agent cmd/serverhub-agent/main.go
echo "Go agent checks passed"
