## Context

The shell is technically functional, but the current desktop UI still drifts away from the user's target references in four concrete ways:

- Navigation implies more views than are actually needed for this phase.
- The main monitoring surface uses loose cards instead of a dense comparison-first console.
- Visual treatment is too airy and decorative relative to the dark, compact operations feel in the provided prototypes.
- Dense content can still waste space or compete with nearby values instead of forming a clear hierarchy.

This change focuses on reshaping the frontend into a credible two-page server operations desktop without expanding backend scope.

## Goals / Non-Goals

**Goals:**
- Make `theme` and `language` settings affect the live shell immediately.
- Reduce the desktop IA to two surfaces: `Console` and `Settings`.
- Present the console as a fleet comparison table plus an active-server detail stack.
- Keep startup motion explicit but restrained and stop decorative effects from overpowering telemetry.
- Eliminate visible overlap and tighten density so the app resembles the supplied operations-console references.

**Non-Goals:**
- Reintroducing separate CPU / Network / Disk / Alert / Probe routes in this phase.
- Adding new backend commands or changing metric schema.
- Building a full mobile-specific layout beyond keeping the desktop UI responsive and stable.

## Decisions

### 1. Keep only two routes in the shell

Decision:
- The shell exposes only `Console` and `Settings` in navigation for this phase.
- Server selection remains in the sidebar so the active-server context is always visible.

Why:
- The user explicitly wants two interfaces right now.
- The previous nav implied unfinished pages and diluted focus.

### 2. Make the console comparison-first

Decision:
- The console uses a left-hand fleet status table for quick comparison across all servers.
- The currently selected server renders in a right-hand detail stack with CPU, memory, network, and disk panels.
- Real metric history is used where it adds signal, especially in the selected-server detail area.

Why:
- The supplied references combine fleet comparison and single-host inspection, not a decorative landing page.
- This structure works with the existing metric store and history APIs without backend expansion.

### 3. Use a dark dense visual system as the primary mode

Decision:
- Dark theme becomes the primary visual baseline.
- Surfaces shift from airy glass cards toward compact matte panels with restrained glow and sharper information grouping.
- Theme toggle remains available in compact shell chrome, while language moves into the settings workspace so the header does not waste horizontal space.

Why:
- The prototypes are decisively dark and compact.
- The product goal is operational clarity, not decorative novelty.

### 4. Use whitespace for grouping, not emptiness

Decision:
- Tighten card padding, row height, toolbar sizing, and sidebar chrome.
- Replace empty hero space with operational summaries and useful controls.
- Explicitly constrain long labels and repeated percentages so rows stay readable.

Why:
- The problem is not lack of style; it is misallocated space.
- Dense monitoring UI must preserve scanability without becoming cluttered.

## Risks / Trade-offs

- [History fetch volume] More active detail panels can increase history queries. Mitigation: keep history focused on the selected server only.
- [Theme parity] Dark mode is the design baseline; light mode remains supported but is visually secondary. Mitigation: keep contrast and spacing rules shared between both themes.
- [Sidebar density] A persistent target list can become long. Mitigation: keep it scrollable and use truncation plus bounded cards.

## Migration Plan

1. Update proposal/spec/tasks to reflect the two-page console-first desktop direction.
2. Apply runtime theme/language preferences in shell chrome.
3. Rework navigation and sidebar to only expose Console and Settings.
4. Replace the dashboard card grid with a fleet status table and selected-server detail stack.
5. Rework the settings surface into a denser server registry and configuration workspace.
6. Validate visually in local dev and record outcomes for the later audit passes.
