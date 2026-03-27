## 1. Desktop Style Pipeline

- [x] 1.1 Add a repository-owned CSS generation step that compiles the shipped utility classes into build output without introducing new external dependencies.
- [x] 1.2 Update the frontend CSS entry path so preview and production builds load the generated stylesheet and render a visibly styled shell.
- [x] 1.3 Redesign the desktop shell surfaces and metric presentation with the approved ServerCat-inspired visual language, including dark layered panels and glass effects.
- [x] 1.4 Add the SVG ambient accents and bounded application startup animation required by the visual refresh.

## 2. Control-Plane UI Hydration

- [x] 2.1 Hydrate persisted servers and settings during frontend startup, keep active server selection in the store, and expose loading/error state in the client stores.
- [x] 2.2 Implement the minimal settings-page workflows to add, remove, and select servers plus update persisted application settings.
- [x] 2.3 Update the dashboard and detail pages to render explicit no-server, no-selection, loading, error, and data-ready states from hydrated store data.

## 3. Metrics Rate Continuity

- [x] 3.1 Move derived-rate engine lifetime to the per-server polling loop so counter continuity survives across polling cycles.
- [x] 3.2 Extend regression coverage for consecutive counter polls, reset handling, and series identity continuity.

## 4. Verification and Docs

- [x] 4.1 Update affected documentation to match the repaired styling and frontend hydration behavior.
- [x] 4.2 Re-run the baseline validation commands and record the post-fix status in the change tasks.

## Validation Notes

- 2026-03-19: `pnpm lint` ✅
- 2026-03-19: `pnpm build` ✅
- 2026-03-19: `cargo check --manifest-path src-tauri/Cargo.toml` ✅ (warnings only)
- 2026-03-19: `cargo test --manifest-path src-tauri/Cargo.toml` ✅
- 2026-03-19: `./scripts/check-go.sh` ✅
