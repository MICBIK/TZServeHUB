#!/bin/bash
set -e

./scripts/check-frontend.sh
./scripts/check-rust.sh
./scripts/check-go.sh

echo "All baseline checks passed"
