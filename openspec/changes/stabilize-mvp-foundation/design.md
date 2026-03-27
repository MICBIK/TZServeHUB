## Context

This change is driven by a verified repository-wide audit on 2026-03-19. The project already contains a React/Tauri desktop shell, a Rust backend, a Go agent, and architecture documents, but the implementation does not yet support a reliable MVP flow.

Verified audit findings:

- Frontend validation is failing:
  - `pnpm lint` fails on unused values in `src/pages/DashboardPage.tsx`, `src/services/tauri.ts`, and `src/stores/serverStore.ts`
  - `pnpm build` fails because `src/pages/DashboardPage.tsx` uses invalid relative imports and `src/components/server/ServerCard.tsx` violates `verbatimModuleSyntax`
- The frontend is mostly placeholder UI:
  - `src/pages/CpuPage.tsx`
  - `src/pages/NetworkPage.tsx`
  - `src/pages/DiskPage.tsx`
  - `src/pages/ProbePage.tsx`
  - `src/pages/AlertPage.tsx`
  - `src/pages/SettingsPage.tsx`
- Core Tauri commands are stubbed:
  - `src-tauri/src/commands/server.rs`
  - `src-tauri/src/commands/metrics.rs`
  - `src-tauri/src/commands/settings.rs`
- Storage and model contracts drift:
  - `src-tauri/migrations/001_init.sql` and `src-tauri/src/models/alert.rs` describe incompatible alert schemas
  - `src-tauri/src/storage/migrations.rs` duplicates migration intent but is not implemented
- The Go agent and Rust adapter do not share the same payload contract:
  - `agent/internal/collector/collector.go` emits fields such as `mount`, `used_percent`, `interface`, `rx_bytes`, `tx_bytes`
  - `src-tauri/src/adapters/go_agent.rs` expects `mountpoint`, `fstype`, `percent`, `name`, `bytes_sent`, `bytes_recv`
- Documentation drift exists:
  - `docs/architecture.md` references files and modules that do not exist, such as `commands/probes.rs` and `adapters/glances.rs`
  - `docs/project-tooling.md` references dependencies not present in `package.json`, such as `antd` and `@tanstack/react-query`

Constraints:

- The project should remain Linux-first for remote monitoring in the MVP.
- Unfinished feature domains are acceptable, but they must be explicitly deferred and must not break build, startup, or the primary data path.
- The change should stabilize one working vertical slice rather than trying to finish every planned feature at once.

## Goals / Non-Goals

**Goals:**

- Make the repository pass the baseline validation commands that exist today.
- Deliver one working monitoring slice: server registry + adapter collection + SQLite persistence + dashboard/history retrieval.
- Define one canonical contract for metrics payloads, metric series identity, and persisted schema.
- Ensure unfinished feature areas fail safely through explicit empty states or deferred command surfaces rather than silent placeholders.
- Re-align architecture/tooling docs to the code that actually exists.

**Non-Goals:**

- Completing the full alerts, probes, Glances, or traffic analytics product scope in this change.
- Introducing a new remote agent protocol beyond stabilizing the current Go agent contract.
- Building multi-user sync, cloud relay, or shared history across desktops.
- Replacing every placeholder with a final polished design system.

## Decisions

### 1. Stabilize one vertical slice before expanding capability breadth

Decision:
- The change will prioritize a working path for `server config -> adapter fetch -> normalized metrics -> SQLite -> dashboard/history query`.

Why:
- Most current failures come from missing glue rather than missing isolated modules.
- A stable vertical slice gives the project a trustworthy baseline for later alerts, probes, and Glances work.

Alternatives considered:
- Finish all placeholder pages first.
  - Rejected because the UI would still sit on top of stubbed commands and broken data contracts.
- Focus only on backend data flow.
  - Rejected because the frontend currently does not build, so MVP viability would remain unproven.

### 2. Treat build/lint pass as a hard foundation requirement

Decision:
- `pnpm lint`, `pnpm build`, `cargo check`, and `go test ./...` are part of the foundation exit criteria for this change.

Why:
- The frontend already fails before runtime. Without green baseline validation, later change reviews will not distinguish regressions from known breakage.

Alternatives considered:
- Allow frontend build failures while backend work continues.
  - Rejected because the desktop app is the primary product surface.

### 3. Use SQLite-backed commands as the source of truth for server registry and settings

Decision:
- Server registry and settings will be stored and queried through SQLite-backed Tauri commands rather than in-memory defaults.
- Per-server connection metadata will include any adapter-specific auth configuration required to communicate with that target.

Why:
- Current command handlers return defaults or empty arrays, which makes the app appear functional while persisting nothing.
- The migration file already establishes the intended persistence direction.
- The Go agent integration cannot be made reliable unless the desktop can persist the token needed for authenticated metric fetches.

Alternatives considered:
- Keep settings in memory and add persistence later.
  - Rejected because server onboarding and history queries depend on durable IDs and stable configuration.

### 4. Define a canonical internal metric contract independent of adapter-specific payload shape

Decision:
- The metrics pipeline will normalize external payloads into one internal representation with explicit metric type, labels, timestamp, server ID, vantage point, and a canonical series identity.
- Raw and rolled-up storage must preserve the label set needed to query one series unambiguously.
- History queries must target a single series via `server_id + key + labels + vantage_point`, or an equivalent canonical selector with reversible label semantics.
- Rolled-up history responses must expose explicit aggregate buckets rather than pretending rollups are raw points.

Why:
- Exporter-style sources expose counters and labels differently.
- The current code mixes raw counters, derived rates, and adapter-specific keys in a way that will not scale to multiple adapters.
- History queries for per-core, per-disk, and per-interface metrics become ambiguous if the series label set is not preserved across storage and retrieval.

Alternatives considered:
- Store adapter-native keys verbatim and normalize only in the UI.
  - Rejected because rate derivation, rollups, alerting, and history queries all need stable semantics below the UI layer.

### 5. Make the Go agent contract explicit and testable

Decision:
- The Go agent API contract will be treated as a formal integration boundary, including field names, desktop auth configuration, and endpoint auth expectations.
- If a Go agent token is configured, the desktop MUST send it as a Bearer token on `/api/metrics`.
- `/api/health` remains unauthenticated so the desktop can distinguish liveness failures from credential failures.

Why:
- The current Rust adapter cannot safely deserialize the current Go agent payload shape.
- The README says `/api/metrics` requires Bearer auth, but the current Gin middleware protects all endpoints, including health.
- The current desktop server model does not yet describe how authenticated agent requests obtain their token.

Alternatives considered:
- Require auth on both `/api/health` and `/api/metrics`.
  - Rejected because basic reachability checks would become indistinguishable from credential failures during onboarding and recovery.
- Leave the Rust adapter permissive and patch deserialization ad hoc.
  - Rejected because silent drift will recur without one declared contract.

### 6. Explicitly defer unfinished domains instead of shipping hidden TODO behavior

Decision:
- Alerts and probes may remain incomplete in this change, but the UI and command surface must present that state as explicit read-only/deferred pages rather than relying on placeholder TODO text or missing modules.
- `glances` is not part of the MVP adapter baseline for this change.

Why:
- The user indicated unfinished work is acceptable, but hidden incompleteness is different from explicit deferral.

Alternatives considered:
- Keep existing TODO placeholders.
  - Rejected because they leak internal implementation status into user-facing surfaces and fail the “working MVP baseline” goal.
- Temporarily hide unfinished routes.
  - Rejected because explicit deferred states are easier to test and keep the remaining scope visible.

## Risks / Trade-offs

- [Cross-cutting scope] This touches frontend, Rust, Go, SQL, and docs at once.
  - Mitigation: constrain the change to one vertical slice and document deferred areas explicitly.
- [Schema churn] Aligning models and migrations may require choosing between current SQL shape and current Rust structs.
  - Mitigation: pick one canonical schema, update both sides, and avoid dual representations.
- [Contract breakage] Changing agent fields or auth behavior can break existing local agent usage.
  - Mitigation: document the contract clearly and keep the MVP contract minimal.
- [UI scope creep] Replacing placeholder pages can turn into a design exercise.
  - Mitigation: require correct state handling first; treat visual polish as secondary.

## Migration Plan

1. Fix frontend import/type issues and remove lint/build blockers.
2. Establish database initialization and migrate server/settings commands to persistent storage, including adapter auth configuration.
3. Align SQL schema, Rust models, and command return shapes, including series labels/selectors and rollup DTOs.
4. Reconcile Go agent payload, token handling, and health endpoint semantics with the Rust adapter.
5. Wire the minimum polling -> normalized raw storage -> derived rate/rollup -> history query routing path.
6. Replace placeholder pages with explicit empty/loading/error/data states backed by the working commands.
7. Update `docs/architecture.md` and `docs/project-tooling.md` to match shipped reality and deferred scope.

Rollback strategy:

- If a schema or contract decision proves too disruptive, keep the old code paths disabled and ship the narrower node_exporter-backed slice first.
- Do not partially roll out mixed agent/schema contracts without corresponding adapter changes.

## Resolved Scope Notes

- Supported MVP adapters for this change are `node_exporter` and `go_agent`; `glances` remains deferred.
- `/api/health` remains unauthenticated for liveness checks, while `/api/metrics` requires a Bearer token when the Go agent is configured with one.
- Alerts and probes stay routable in the desktop shell as explicit deferred/read-only pages rather than being hidden.
