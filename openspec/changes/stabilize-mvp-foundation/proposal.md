## Why

The repository has crossed the point where architectural intent and executable reality are no longer aligned. The desktop shell, Tauri command layer, SQLite schema, and Go agent contract all contain verified gaps that make the current MVP foundation unreliable to build on.

This change is needed now because new feature work would compound drift instead of shipping a usable monitoring baseline. The project needs one stable vertical slice before expanding alerts, probes, Glances, or deeper analytics.

## What Changes

- Stabilize the desktop application so the frontend lint/build pipeline passes and the app shell renders typed monitoring states instead of placeholder pages and broken imports.
- Implement a real persistence path for server registry and app settings using SQLite-backed Tauri commands.
- Implement the minimum metrics ingestion path for enabled servers: adapter fetch, normalization, derived rate calculation, storage, and history query.
- Reconcile the wire contract between the Go agent and the Rust adapter so health checks, auth behavior, and metrics payload fields are compatible.
- Align the Rust models, SQL migrations, and project documentation with the actual MVP scope. Remove or explicitly defer drifted assumptions instead of leaving them implicit.

## Capabilities

### New Capabilities
- `desktop-monitoring-ui`: A buildable desktop shell with working dashboard/detail page states for loading, empty, error, and data-ready flows.
- `control-plane-persistence`: Persistent server registry and app settings exposed through Tauri commands and backed by SQLite.
- `metrics-pipeline`: Collection, normalization, storage, derived-rate handling, and history query support for the MVP monitoring data path.
- `agent-contract`: A stable and testable contract between the Go agent API and the Rust adapter/auth expectations.

### Modified Capabilities
- None.

## Impact

- Affected frontend code: `src/App.tsx`, `src/pages/*`, `src/stores/*`, `src/services/tauri.ts`, `src/components/*`
- Affected Tauri code: `src-tauri/src/commands/*`, `src-tauri/src/storage/*`, `src-tauri/src/metrics/*`, `src-tauri/src/scheduler/*`, `src-tauri/src/adapters/*`, `src-tauri/src/models/*`
- Affected agent code: `agent/internal/collector/*`, `agent/internal/server/*`, `agent/README.md`
- Affected schema/docs: `src-tauri/migrations/001_init.sql`, `docs/architecture.md`, `docs/project-tooling.md`
- Affected validation commands: `pnpm lint`, `pnpm build`, `cargo check`, `go test ./...`
