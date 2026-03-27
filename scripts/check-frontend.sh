#!/bin/bash
set -e
pnpm build
pnpm lint
echo "Frontend checks passed"
