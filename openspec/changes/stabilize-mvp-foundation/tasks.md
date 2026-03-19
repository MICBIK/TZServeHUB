## 1. Frontend Foundation

- [ ] 1.1 Fix frontend lint/build blockers, including invalid import paths, unused values, and type-only imports.
- [ ] 1.2 Replace user-facing TODO placeholders with explicit loading, empty, error, or data states on dashboard and detail pages.
- [ ] 1.3 Align frontend stack usage and docs so declared UI/data libraries match the code that actually ships.

## 2. Persistence and Command Surface

- [ ] 2.1 Initialize SQLite and run migrations during Tauri startup.
- [ ] 2.2 Implement persistent `list_servers`, `add_server`, and `remove_server` commands backed by the `servers` table.
- [ ] 2.3 Implement persistent `get_settings` and `update_settings` commands backed by application storage.
- [ ] 2.4 Return structured command errors instead of silent empty defaults for implemented flows.

## 3. Metrics Pipeline

- [ ] 3.1 Define the canonical internal metric contract, including labels, metric type, timestamp, server ID, and vantage point semantics.
- [ ] 3.2 Update the node_exporter adapter so labeled samples are parsed into normalized keys and labels.
- [ ] 3.3 Wire polling, raw metric persistence, derived rate calculation, and history queries through the SQLite-backed pipeline.
- [ ] 3.4 Honor per-server polling settings and explicit stale/error handling in the scheduler path.

## 4. Agent and Schema Alignment

- [ ] 4.1 Reconcile the Go agent response schema with the Rust adapter deserialization model.
- [ ] 4.2 Reconcile alert and metrics-related Rust models with the SQL migration schema, or explicitly reduce the persisted scope to the MVP slice.
- [ ] 4.3 Align agent auth and health-check semantics with the documented desktop integration behavior.

## 5. Verification and Documentation

- [ ] 5.1 Add or update regression checks covering frontend build/lint, Rust compile checks, and Go agent contract expectations.
- [ ] 5.2 Update `docs/architecture.md` and `docs/project-tooling.md` so they match the implemented modules, dependencies, and deferred scope.
- [ ] 5.3 Re-run the baseline verification commands and record the post-change status before archive.
