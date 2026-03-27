## Context

The audit after `stabilize-mvp-foundation` verified that the repository builds and the backend command surface is mostly in place, but the shipped desktop experience still breaks in three places:

- Build output ships raw Tailwind directives instead of compiled CSS, so preview renders unstyled HTML.
- Frontend stores expose persistence and settings workflows, but the routed shell does not hydrate or use them at runtime.
- The scheduler recreates the derived-rate engine for every storage batch, so counter continuity is lost between polls.

This repair change is intentionally narrower than the previous MVP foundation work. It should close the audited gaps without widening scope into full alerts, probe history, or polished analytics UX.

## Goals / Non-Goals

**Goals:**
- Restore a working CSS generation path using dependencies already present in the repository.
- Reframe the desktop shell with a stronger monitoring-specific visual language based on verified ServerCat references while keeping the current routed structure.
- Make the desktop shell hydrate servers/settings, support minimal server CRUD and selection, and expose real empty/loading/error/data page states.
- Preserve derived counter state across polling cycles so rate metrics are materially useful.
- Add regression checks that prove the repaired paths stay working.

**Non-Goals:**
- Introducing a new design system or major UI redesign.
- Completing alerts, probes, or the deferred `glances` adapter.
- Reworking the broader metrics schema beyond the audited rate continuity gap.

## Decisions

### 1. Repair the styling pipeline with a repository-owned CSS generation step

Decision:
- Add a local Tailwind CSS generation step driven by the already-installed `tailwindcss` package instead of introducing a new external plugin dependency.
- Build output will consume a generated stylesheet rather than shipping raw `@tailwind` directives.

Why:
- The current dependency set already contains the Tailwind compiler API.
- Network-restricted or offline environments should still be able to build the desktop shell.
- This is the smallest change that restores visible styling without rewriting every utility class by hand.

Alternatives considered:
- Add `@tailwindcss/vite` or `@tailwindcss/postcss`.
  - Rejected because that requires a dependency change the current workspace cannot guarantee immediately.
- Replace utility classes with handwritten CSS.
  - Rejected because the shell already relies on utility classes across many files and a rewrite would create unnecessary scope.

### 2. Hydrate control-plane state once at app startup and keep active selection in the frontend store

Decision:
- App startup will fetch persisted servers and settings once, seed the zustand stores, and keep active server selection in the frontend store.
- The settings route will host the minimal add/remove/update workflows, while the dashboard and detail pages consume the active selection.

Why:
- The command layer already persists data, so the missing piece is frontend hydration and wiring.
- Centralizing selection state in the store avoids prop-drilling across routed pages.

Alternatives considered:
- Re-fetch state independently on every page.
  - Rejected because it duplicates lifecycle logic and makes selection drift easier.
- Keep the settings route read-only and ask users to manage targets elsewhere.
  - Rejected because the dashboard empty state already directs users to settings.

### 3. Refresh the shell with a monitoring-first visual language instead of generic placeholders

Decision:
- Use a ServerCat-inspired dark operations-console look with layered charcoal panels, bright health accents, frosted overlays, and dense metric cards while preserving the current route map.
- Add one application entry animation and a small set of SVG-driven ambient accents instead of generic micro-animations on every element.

Why:
- The audit showed the current shell is truthful only at the data-contract level; visually it still feels like placeholder scaffolding.
- Server monitoring products benefit from a stronger “instrument panel” identity than a generic admin layout.
- One cohesive motion system is easier to maintain than many disconnected transitions.

Alternatives considered:
- Keep the shell structurally identical and only restore missing CSS.
  - Rejected because the user explicitly wants the repaired MVP to ship with a stronger visual direction.
- Add animation to every card and button.
  - Rejected because constant motion would compete with monitoring data and make the shell feel noisy.

### 4. Scope derived-rate continuity to the scheduler-owned poll loop

Decision:
- Keep one `DerivedMetricsEngine` instance per server polling loop so counter baselines survive between polls for that target.
- Preserve the current reset and stale-cleanup behavior inside that long-lived engine.

Why:
- Rate continuity is only meaningful across polls of the same target.
- The scheduler already owns the per-server poll loop, making it the natural lifetime boundary.

Alternatives considered:
- Persist counter baselines in SQLite.
  - Rejected because it adds schema churn for a gap that can be solved in-memory at the poll-loop boundary.
- Keep the engine short-lived and derive rates from SQL history during queries.
  - Rejected because the product already expects derived metrics to be stored like other raw samples.

### 5. Treat route states as explicit product behavior, not placeholders

Decision:
- Dashboard and detail pages will distinguish no-server, no-selection, loading, error, and data-ready states from typed store state.
- Minimal cards/charts are sufficient; hiding behind perpetual loading placeholders is not.

Why:
- The audit showed that some pages compile but do not communicate real runtime state.
- Explicit state rendering is the cheapest way to make the shell truthful without overbuilding UX.

Alternatives considered:
- Leave detail pages as static loading views until a later polish pass.
  - Rejected because this directly conflicts with the audited MVP requirement.

## Risks / Trade-offs

- [Custom CSS build step] → A repository-owned Tailwind build script adds maintenance burden. Mitigation: keep it small, deterministic, and covered by build validation.
- [Visual scope creep] → A visual refresh can turn into a redesign project. Mitigation: constrain the work to shell surfaces, metric cards, SVG accents, and one launch animation only.
- [Frontend scope creep] → Wiring full CRUD UI could expand. Mitigation: keep the workflow minimal to add, remove, select, and update settings only.
- [In-memory rate state] → Derived continuity resets when the poll task restarts. Mitigation: accept that restart boundary for MVP and preserve continuity during normal polling operation.
- [State coupling] → Startup hydration can create UI timing edge cases. Mitigation: make loading/error states first-class in the stores.

## Migration Plan

1. Repair CSS generation so build/preview outputs include compiled shell styling.
2. Reframe the shell surfaces, cards, SVG accents, and startup motion around the chosen monitoring visual language.
3. Hydrate servers and settings during frontend startup and wire minimal settings/dashboard workflows.
4. Move derived-rate engine lifetime to the server poll loop and keep reset protection intact.
5. Update page states and add regression checks for the repaired flows.
6. Re-run baseline validation plus targeted route/build checks.

Rollback strategy:

- If the custom CSS generation step proves unstable, keep the generated stylesheet checked in temporarily and disable the broken raw-directive path.
- If frontend hydration causes regressions, fall back to read-only rendering while keeping server/settings fetches and selection logic isolated for fast rollback.

## Open Questions

- None for this repair slice. The audited gaps are concrete enough to implement directly.
