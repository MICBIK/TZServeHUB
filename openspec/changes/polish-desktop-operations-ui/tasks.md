## 1. Runtime Preferences

- [x] 1.1 Apply persisted `theme` and `language` settings to the live desktop shell at startup and during runtime updates.
- [x] 1.2 Add a compact shell-level theme toggle plus settings-page language controls so runtime preferences can be tested without restart.

## 2. Visual Polish and Layout Stability

- [x] 2.1 Rework the shell chrome into a denser dark operations-console style with a visible startup SVG animation.
- [x] 2.2 Fix overlap-prone layouts in the console, sidebar, status rows, and settings workspace with bounded sizing, truncation, and tighter spacing.

## 3. Console Restructure

- [x] 3.1 Collapse the desktop IA to two visible surfaces: `Console` and `Settings`.
- [x] 3.2 Replace the loose dashboard grid with a fleet status table plus chart-backed active-server detail stack.
- [x] 3.3 Rework the settings page into a denser server registry and configuration workspace.
- [x] 3.4 Extend the active-server memory module and metric contract to expose `used / cached / free` memory states in the console.

## 4. Verification

- [x] 4.1 Re-run frontend validation and verify the improved shell in local dev.
- [x] 4.2 Record outcomes in this task list.

## Validation Notes

- `pnpm lint` passed after the shell/navigation refactor and console redesign.
- `pnpm build` passed after the multi-server browser demo, full-fleet polling, and dark console polish landed.
- Headless Chrome verification confirmed:
  - sidebar navigation now exposes only `控制台 / 设置`
  - the main console now uses a fleet comparison table plus active-server detail stack
  - the shell keeps a compact theme toggle in the header while language preference lives in settings and still updates the UI immediately
  - settings now reads as a denser registry/configuration workspace instead of a sparse showcase page
- Browser fallback now seeds multiple demo servers and richer memory metrics so non-Tauri preview matches the intended operations-console direction more closely.
- The active-server memory module now reads from an expanded memory metric contract and shows `used / cached / free` breakdown values instead of only `used / idle`.
