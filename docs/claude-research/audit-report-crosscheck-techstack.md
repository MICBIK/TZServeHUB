# Audit Report: Cross-check & Tech Stack Analysis

Generated: 2026-03-19

---

## Part 1: Cross-Validation — Duplicate Projects

Projects appearing in more than one research report, with consistency check.

### Duplicates Found: 13

| # | Project | Files | Stars Consistent? | Notes |
|---|---------|-------|-------------------|-------|
| 1 | **Abdenasser/neohtop** | research-1, research-2, research-3 | Yes (8,940) | Consistent across all 3 |
| 2 | **vietanhdev/ThinkUtils** | research-1, research-3, research-5 | Yes (83) | Consistent |
| 3 | **Slocean/PulseCoreLite** | research-1, research-3, research-5 | Yes (8) | Consistent description |
| 4 | **WRVbit/glance** | research-2, research-3, research-5 | Yes (2) | Consistent: Rust+Tauri 2.0+Svelte 5 |
| 5 | **raro42/mac-stats** | research-1, research-2, research-3 | research-1: 7 stars, research-2: 7 stars, research-3: 7 stars | Consistent |
| 6 | **GOODBOY008/r-shell** | research-1, research-3 | Yes (19) | Consistent: Tauri+React+Rust |
| 7 | **alexandrosnt/Reach** | research-1, research-3 | Yes (3) | Consistent |
| 8 | **jun0-ds/clance** | research-1, research-3 | Yes (2) | Consistent |
| 9 | **Icarus-afk/DPanel** | research-1, research-3 | Yes (1) | Consistent: Tauri 2+React+Rust |
| 10 | **CoderrsHandbook/tauri2-system-monitor** | research-1, research-5 | Yes (0) | Consistent |
| 11 | **chochy2001/omnimon** | research-2, research-3 | Yes (2) | Consistent: Rust+Tauri |
| 12 | **pacholoamit/pachtop** | research-1, research-3 | Yes (176) | Consistent |
| 13 | **felps-dev/ghtray** | research-2, research-3 | Yes (6) | Consistent: Rust+Tauri v2 |

### Consistency Issues

- **raro42/mac-stats**: research-2 lists 7 stars, research-3 lists 7 stars — consistent. However research-2 omits the "local AI agent" detail present in research-1 and research-3.
- **WRVbit/glance**: research-5 lists it under "Emerging Projects" section but research-2 and research-3 list it with 2 stars — minor categorization difference, data is consistent.
- No star count contradictions found across duplicates.

---

## Part 2: Tech Stack Statistics — Frontend Frameworks

Compiled from all 4 research reports (research-1, research-2, research-3, research-5).

### Tauri + React

| Project | Stars | Source |
|---------|-------|--------|
| GOODBOY008/r-shell | 19 | research-1, research-3 |
| WigoWigo10/SerialHub | 1 | research-5 |
| ssshaise/Hobbit | 0 | research-5 |
| HardBoss07/resource-monitor (Next.js) | 0 | research-5 |
| moFeng123/win-system-monitor | 0 | research-2 |
| Icarus-afk/DPanel | 1 | research-1, research-3 |
| krjordan/PulsePrint-Desktop | 4 | research-3 |
| wwwnakanaka1-lgtm/pc-dashboard | 0 | research-3 |
| boparaiamrit/claude-session-monitor | 0 | research-3 |
| 073145/argus-dashboards | 0 | research-3 |

**Total: 10 projects**

### Tauri + Vue

| Project | Stars | Source |
|---------|-------|--------|
| Slocean/PulseCoreLite (Vue 3) | 8 | research-1, research-3, research-5 |
| starsdaisuki/StarDash | 1 | research-3 |
| xinggaoya/system-monitor (Vue 3) | 1 | research-1 |

**Total: 3 projects**

### Tauri + Svelte

| Project | Stars | Source |
|---------|-------|--------|
| Abdenasser/neohtop (Svelte) | 8,940 | research-1, research-2, research-3 |
| WRVbit/glance (Svelte 5) | 2 | research-2, research-3, research-5 |

**Total: 2 projects**

### Other / Unknown Frontend

| Project | Frontend | Stars | Source |
|---------|----------|-------|--------|
| vietanhdev/ThinkUtils | Unknown (Rust primary) | 83 | research-1, research-3, research-5 |
| pacholoamit/pachtop | Unknown | 176 | research-1, research-3 |
| raro42/mac-stats | Unknown | 7 | research-1, research-2, research-3 |
| felps-dev/ghtray | Unknown | 6 | research-2, research-3 |
| kimdj2/claude-token-monitor | Unknown | 3 | research-2, research-3 |
| alexandrosnt/Reach | Unknown | 3 | research-1, research-3 |
| jun0-ds/clance | Unknown | 2 | research-1, research-3 |
| chochy2001/omnimon | Unknown | 2 | research-2, research-3 |
| CoderrsHandbook/tauri2-system-monitor | TypeScript (framework unspecified) | 0 | research-1, research-5 |
| jbilakhi/pulsehud | Unknown | 0 | research-2 |
| vehler/dev-dashboard | Unknown | 0 | research-3 |
| giorgiabes/system-monitor-dashboard | TypeScript only | 0 | research-3 |
| hazy2go/rem-dashboard | Unknown | 0 | research-3 |
| jbrenner1000/netguard-desktop | Unknown | 0 | research-3 |
| havenscr/desktop-widget-wall | Unknown | 1 | research-3 |
| JackpotMachine777/tauri-system-monitor | JS (framework unspecified) | 1 | research-1 |
| xScherpschutter/ActioWatch | Unknown | 2 | research-1 |
| shubhamkarande/PulseTrack | Unknown | 0 | research-1 |
| Shadow123x/ThinkUtils (fork) | N/A | 0 | research-5 |

---

## Summary

| Metric | Count |
|--------|-------|
| Total duplicate projects found | 13 |
| Tauri + React projects | 10 |
| Tauri + Vue projects | 3 |
| Tauri + Svelte projects | 2 |
| Other/Unknown frontend | 19 |

All duplicate entries are data-consistent (no star count contradictions). Minor omissions in descriptions across reports but no conflicting information.
