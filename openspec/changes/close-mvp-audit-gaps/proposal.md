## Why

The previous MVP foundation change made the repository buildable, but the audit found three verified gaps that still break the shipped experience: the desktop shell ships without compiled utility CSS, the frontend does not hydrate persisted control-plane state into usable workflows, and derived counter rates lose continuity across polling cycles. These issues need to be corrected now so the documented MVP slice matches the actual desktop behavior.

## What Changes

- Restore a working desktop styling pipeline so production and preview builds render the intended shell instead of unstyled HTML.
- Refresh the desktop shell with a ServerCat-inspired monitoring aesthetic, including dark layered surfaces, glassmorphism treatments, richer metric presentation, animated SVG accents, and an application entry animation.
- Hydrate server registry and settings data into the frontend on startup, expose minimal add/remove/select/update control-plane workflows, and make dashboard/detail pages react to no-server, no-selection, loading, error, and data-ready states.
- Preserve derived-metric counter state across polling cycles so disk and network rate metrics are emitted from real consecutive samples instead of resetting every poll.
- Tighten regression coverage and documentation around these repaired paths so future MVP work does not reintroduce the same drift.

## Capabilities

### New Capabilities
- `desktop-style-pipeline`: Guarantees that the shipped desktop client includes compiled utility CSS and renders a visibly styled shell in build and preview outputs.
- `desktop-visual-refresh`: Covers the ServerCat-inspired desktop presentation, glass surfaces, animated SVG accents, and entry motion for the MVP shell.
- `control-plane-ui-hydration`: Covers frontend hydration of persisted servers/settings plus the minimal UI workflows needed to manage targets and select an active server for monitoring.
- `metrics-rate-continuity`: Covers safe derived-rate generation across polling cycles for counter-based metric series.

### Modified Capabilities

## Impact

- Affected frontend code: `src/index.css`, `src/main.tsx`, `src/App.tsx`, `src/pages/*`, `src/stores/*`, `src/services/tauri.ts`, `src/components/*`
- Affected tooling/build: `package.json`, `vite.config.ts`, `scripts/*`
- Affected Tauri code: `src-tauri/src/scheduler/poller.rs`, `src-tauri/src/metrics/derived.rs`, `src-tauri/src/commands/metrics.rs`
- Affected validation/docs: `tests/*`, `docs/architecture.md`, `docs/project-tooling.md`
