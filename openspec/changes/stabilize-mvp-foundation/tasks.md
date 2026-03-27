## 1. Frontend Foundation

- [x] 1.1 Fix frontend lint/build blockers, including invalid import paths, unused values, and type-only imports.
- [x] 1.2 Replace user-facing TODO placeholders with explicit loading, empty, error, or data states on dashboard and detail pages.
- [x] 1.3 Align frontend stack usage and docs so declared UI/data libraries match the code that actually ships.

## 2. Persistence and Command Surface

- [x] 2.1 Initialize SQLite and run migrations during Tauri startup.
- [x] 2.2 Implement persistent `list_servers`, `add_server`, and `remove_server` commands backed by the `servers` table, including adapter-specific connection/auth fields required for collection.
- [x] 2.3 Implement persistent `get_settings` and `update_settings` commands backed by application storage.
- [x] 2.4 Return structured command errors instead of silent empty defaults for implemented flows.

## 3. Metrics Pipeline

- [x] 3.1 Define the canonical internal metric contract, including labels, metric type, timestamp, server ID, vantage point semantics, and the canonical series identity used by storage/query paths.
- [x] 3.2 Update the node_exporter adapter so labeled samples are parsed into normalized keys and labels.
- [x] 3.3 Wire polling, raw metric persistence, derived rate calculation, rollup generation, and raw-vs-rollup history query routing through the SQLite-backed pipeline.
- [x] 3.4 Persist and query the label set needed to address one metric series without merging distinct cores, disks, or interfaces.
- [x] 3.5 Return typed history responses that distinguish raw points from aggregated rollup buckets.
- [x] 3.6 Honor per-server polling settings and explicit stale/error handling in the scheduler path.

## 4. Agent and Schema Alignment

- [x] 4.1 Reconcile the Go agent response schema, desktop server auth fields, and Rust adapter deserialization model.
- [x] 4.2 Reconcile alert and metrics-related Rust models with the SQL migration schema, or explicitly reduce the persisted scope to the MVP slice.
- [x] 4.3 Align agent auth and health-check semantics with the documented desktop integration behavior, including an unauthenticated `/api/health` path and Bearer-authenticated `/api/metrics` requests.

## 5. Verification and Documentation

- [x] 5.1 Add or update regression checks covering frontend build/lint, desktop shell startup smoke coverage, Rust compile checks, and Go agent contract expectations.
- [x] 5.2 Update `docs/architecture.md` and `docs/project-tooling.md` so they match the implemented modules, dependencies, and deferred scope.
- [x] 5.3 Re-run the baseline verification commands and record the post-change status before archive.

## Verification Status

- 2026-03-19: `pnpm lint` ✅
- 2026-03-19: `pnpm build` ✅
- 2026-03-19: `cargo check` ✅
- 2026-03-19: `cargo test` ✅
- 2026-03-19: `go test ./...` ✅
- 2026-03-19: `pnpm check:all` ✅
