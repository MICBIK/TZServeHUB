## Why

The repaired MVP is now functional, but the live desktop shell still misses the product shape the user expects: runtime theme and language preferences do not fully drive the UI, the information architecture is spread across too many implied views, and the main monitoring surface still lacks the dense dark operations-console feel shown in the provided prototypes. This change is needed now because the product bar is no longer just “works in Tauri”; it must also feel like a credible server operations desktop.

## What Changes

- Add real runtime preference switching for dark/light theme and zh-CN/en-US language so the desktop shell responds immediately to persisted settings.
- Collapse the frontend information architecture into two clear work surfaces: a control console and a settings workspace.
- Reframe the desktop shell around a denser dark operations-console visual system with less decorative noise, stronger layout constraints, and a visible startup SVG animation.
- Replace the loose dashboard card grid with a fleet status table plus a chart-backed active-server detail stack that better matches the provided references.
- Fix content overlap and unstable sizing in the console, sidebar, server cards, and settings workspace so dense content remains readable.
- Expand the active-server memory module so it can show used, cached, and free memory states instead of collapsing everything into only used versus idle.

## Capabilities

### New Capabilities
- `desktop-runtime-preferences`: Covers applying and switching persisted theme and language preferences across the live desktop shell.
- `desktop-monitoring-visual-polish`: Covers calmer shell styling, startup SVG motion, better iconography, and layout rules that prevent overlap.
- `desktop-monitoring-charts`: Covers the control console's comparison table and chart-backed active-server detail panels using real history data instead of static-only cards.

### Modified Capabilities

## Impact

- Affected frontend code: `src/App.tsx`, `src/index.css`, `src/components/layout/*`, `src/components/common/*`, `src/components/charts/*`, `src/pages/*`, `src/hooks/*`, `src/lib/*`, `src/stores/settingsStore.ts`
- Affected Tauri/frontend contract: `src/services/tauri.ts`, metric history consumption on existing commands
- Affected docs/spec tracking: `openspec/changes/polish-desktop-operations-ui/*`
