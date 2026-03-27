# OpenSpec: ServerHUB UI Redesign

**Version**: 1.0
**Date**: 2026-03-19
**Status**: Draft

## Executive Summary

Consolidate ServerHUB from a 7-page navigation structure to a **single unified dashboard** with high-density visualizations inspired by ServerCat, GitHub contribution heatmaps, and Apple Health activity rings. Reduce window size, modernize theme controls, and enhance server management with SSH/password authentication.

## Goals

1. **Simplify Navigation**: Single dashboard replaces Dashboard/CPU/Network/Disk/Probes/Alerts pages
2. **Visual Density**: Compact, information-rich visualizations over sparse card layouts
3. **Better UX**: Icon-based theme toggle, smaller window, improved settings
4. **Enhanced Auth**: SSH key + password authentication for server connections

## Current State Analysis

### Existing Structure
- **7 routes**: `/`, `/cpu`, `/network`, `/disk`, `/probes`, `/alerts`, `/settings`
- **Window**: 1400x900px (too large, bottom content hidden)
- **Theme Toggle**: Text-based segment control ("黑色"/"白色")
- **Auth**: Bearer token only (`auth_token` field in ServerConfig)
- **Sidebar**: 320px fixed width with server cards + navigation

### Files to Modify
```
src/App.tsx                          # Remove 5 routes, keep / and /settings
src/components/layout/AppLayout.tsx  # Remove theme segment, add icon hover menu
src/components/layout/Sidebar.tsx    # Remove nav items, keep server list
src/pages/DashboardPage.tsx          # Complete redesign with new visualizations
src/pages/SettingsPage.tsx           # Add SSH/password auth fields
src/types/server.ts                  # Add ssh_key, password, port fields
src/services/tauri.ts                # Update addServer signature
src-tauri/tauri.conf.json            # Reduce window to 1200x800
src-tauri/src/models/server.rs       # Add auth fields to ServerConfig
src-tauri/src/commands/server.rs     # Update add_server command
```

## Design Specifications

### 1. Window Size
**Current**: 1400x900
**Target**: 1200x800 (or 1280x820 for 16:10 aspect ratio)

**Rationale**: Fits standard laptop screens without scrolling, reduces wasted space.

### 2. Single Dashboard Layout

#### Grid Structure (12-column)
```
┌─────────────────────────────────────────────────────────────┐
│ Sidebar (280px)  │  Main Dashboard (flex-1)                 │
│                  │                                           │
│ [Server Cards]   │  ┌─────────────────────────────────────┐ │
│                  │  │ Server Overview Header              │ │
│ [Theme Icon]     │  │ • Server count: 3 / 3 online        │ │
│ [Settings Link]  │  │ • Active: server-01                 │ │
│                  │  └─────────────────────────────────────┘ │
│                  │                                           │
│                  │  ┌─────────────────────────────────────┐ │
│                  │  │ CPU Status (Activity Rings)         │ │
│                  │  │ • Total usage ring (0-100%)         │ │
│                  │  │ • Per-core heatmap (GitHub style)   │ │
│                  │  └─────────────────────────────────────┘ │
│                  │                                           │
│                  │  ┌─────────────────────────────────────┐ │
│                  │  │ Memory & Disk (Dual Rings)          │ │
│                  │  │ • Memory usage ring                 │ │
│                  │  │ • Disk usage ring                   │ │
│                  │  └─────────────────────────────────────┘ │
│                  │                                           │
│                  │  ┌─────────────────────────────────────┐ │
│                  │  │ Network Traffic (ServerCat Style)   │ │
│                  │  │ • Real-time speed: ↓ 12.3 MB/s      │ │
│                  │  │ • Total consumed: 1.2 TB            │ │
│                  │  │ • Read/Write speeds: R 45 MB/s      │ │
│                  │  └─────────────────────────────────────┘ │
│                  │                                           │
│                  │  ┌─────────────────────────────────────┐ │
│                  │  │ Historical Trends (Mini Charts)     │ │
│                  │  │ • CPU 24h sparkline                 │ │
│                  │  │ • Network 24h sparkline             │ │
│                  │  └─────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### 3. Visualization Components

#### A. Activity Rings (Apple Health Style)
**Purpose**: Show CPU/Memory/Disk usage as concentric rings

**Implementation**:
- SVG-based circular progress indicators
- 3 concentric rings: outer (CPU), middle (Memory), inner (Disk)
- Color gradient: green (0-60%) → yellow (60-80%) → red (80-100%)
- Animated transitions on value changes

**Data Source**:
- `cpu_usage_percent` (gauge)
- `memory_usage_percent` (gauge)
- `disk_usage_percent` (gauge, averaged across mounts)

#### B. Per-Core Heatmap (GitHub Contribution Style)
**Purpose**: Show per-core CPU usage as a grid of colored squares

**Implementation**:
- Grid layout: N squares (N = core count)
- Color scale: `rgba(156, 214, 255, 0.1)` (idle) → `rgba(156, 214, 255, 1.0)` (100%)
- Hover tooltip: "Core 3: 87%"
- Update frequency: 2s

**Data Source**:
- `cpu_usage_percent{core="0"}`, `cpu_usage_percent{core="1"}`, etc.

#### C. Network Traffic Panel (ServerCat Style)
**Purpose**: Compact display of network metrics with large numbers

**Layout**:
```
┌─────────────────────────────────────┐
│  Network Traffic                    │
│                                     │
│  ↓ 12.3 MB/s    ↑ 8.7 MB/s         │
│  Ingress        Egress              │
│                                     │
│  Total Consumed: 1.2 TB             │
│                                     │
│  Disk I/O                           │
│  Read: 45 MB/s   Write: 23 MB/s    │
└─────────────────────────────────────┘
```

**Data Source**:
- `network_receive_bytes_total` (counter → rate)
- `network_transmit_bytes_total` (counter → rate)
- Sum of all `network_*_bytes_total` (cumulative)
- `disk_read_bytes_total` (counter → rate)
- `disk_write_bytes_total` (counter → rate)

#### D. Historical Sparklines
**Purpose**: 24-hour trend lines for CPU and network

**Implementation**:
- Recharts `<LineChart>` with minimal styling
- Height: 60px, no axes, no grid
- Gradient fill under line
- Query: `get_metric_history` with `resolution: '15m'`, `from: now-24h`, `to: now`

### 4. Theme Toggle Redesign

**Current**: Segment control with text labels
**Target**: Icon-based hover menu

**Behavior**:
1. Default state: Sun icon (light) or Moon icon (dark) visible in sidebar footer
2. On hover: Menu expands to show both options
3. Click to switch theme
4. Smooth transition (300ms ease)

**Implementation**:
```tsx
<div className="theme-toggle-container">
  <button className="theme-icon" onMouseEnter={handleExpand}>
    {theme === 'dark' ? <MoonIcon /> : <SunIcon />}
  </button>
  {expanded && (
    <div className="theme-menu">
      <button onClick={() => setTheme('light')}><SunIcon /></button>
      <button onClick={() => setTheme('dark')}><MoonIcon /></button>
    </div>
  )}
</div>
```

**Icons**: Use inline SVG (sun: circle with rays, moon: crescent)

### 5. Settings Page Enhancement

#### Current Fields
- Name, Host, Port, Adapter Type, Access Method, Polling Interval, Auth Token

#### New Fields
```typescript
interface ServerFormData {
  name: string;
  host: string;
  port: number;
  adapter_type: 'node_exporter' | 'glances' | 'go_agent';
  access_method: 'private' | 'tunnel' | 'gateway';
  polling_interval_sec: number;

  // New authentication fields
  auth_type: 'token' | 'ssh_key' | 'password';
  auth_token?: string;
  ssh_key?: string;        // Path to private key file
  ssh_passphrase?: string; // Optional passphrase for encrypted keys
  password?: string;       // SSH password
}
```

#### UI Layout
```
┌─────────────────────────────────────────────────────────┐
│ Authentication Method                                   │
│ ○ Bearer Token   ● SSH Key   ○ Password                │
│                                                         │
│ [Conditional fields based on selection]                │
│                                                         │
│ If SSH Key:                                            │
│   Private Key Path: [/Users/user/.ssh/id_rsa    ]     │
│   Passphrase:       [••••••••••••••••••••••••••]       │
│                                                         │
│ If Password:                                           │
│   Password:         [••••••••••••••••••••••••••]       │
│                                                         │
│ If Token:                                              │
│   Bearer Token:     [abc123...                  ]      │
└─────────────────────────────────────────────────────────┘
```

#### Backend Changes
**Rust**: Update `ServerConfig` struct in `src-tauri/src/models/server.rs`
```rust
pub struct ServerConfig {
    // ... existing fields
    pub auth_type: AuthType,
    pub auth_token: Option<String>,
    pub ssh_key_path: Option<String>,
    pub ssh_passphrase: Option<String>,
    pub password: Option<String>,
}

pub enum AuthType {
    Token,
    SshKey,
    Password,
}
```

**Migration**: Add columns to `servers` table
```sql
ALTER TABLE servers ADD COLUMN auth_type TEXT NOT NULL DEFAULT 'token';
ALTER TABLE servers ADD COLUMN ssh_key_path TEXT;
ALTER TABLE servers ADD COLUMN ssh_passphrase TEXT;
ALTER TABLE servers ADD COLUMN password TEXT;
```

### 6. Server Count Display

**Location**: Dashboard header (top of main content area)

**Content**:
```
Servers: 3 total / 3 online / 0 offline
Active: server-01 (192.168.1.100:9100)
```

**Data Source**:
- `useServerStore` → `servers.length` (total)
- Count servers with recent metrics (< 60s old) as online
- Subtract to get offline count

## Implementation Plan

### Phase 1: Window & Navigation (2 agents)
**Agent 1**: Update window size in `tauri.conf.json`
**Agent 2**: Remove routes from `App.tsx`, update `Sidebar.tsx` to remove nav items

### Phase 2: Theme Toggle (2 agents)
**Agent 3**: Create `ThemeToggle.tsx` component with icon hover menu
**Agent 4**: Update `AppLayout.tsx` to use new component, remove old segment control

### Phase 3: Dashboard Visualizations (8 agents)
**Agent 5**: Create `ActivityRings.tsx` (CPU/Memory/Disk rings)
**Agent 6**: Create `CoreHeatmap.tsx` (per-core CPU grid)
**Agent 7**: Create `NetworkPanel.tsx` (ServerCat-style traffic display)
**Agent 8**: Create `DiskIOPanel.tsx` (read/write speeds)
**Agent 9**: Create `HistoricalSparkline.tsx` (24h trend charts)
**Agent 10**: Create `ServerOverviewHeader.tsx` (server count + active server)
**Agent 11**: Update `DashboardPage.tsx` to use all new components
**Agent 12**: Add CSS for new components in `index.css`

### Phase 4: Settings Enhancement (4 agents)
**Agent 13**: Update `ServerFormData` type in `types/server.ts`
**Agent 14**: Update `SettingsPage.tsx` with auth type selector + conditional fields
**Agent 15**: Update Rust `ServerConfig` struct + migration
**Agent 16**: Update `add_server` Tauri command to handle new auth fields

### Phase 5: Integration & Polish (4 agents)
**Agent 17**: Update `uiCopy.ts` with new translation keys
**Agent 18**: Test data flow from Tauri → React for new auth fields
**Agent 19**: Verify all visualizations render correctly with real data
**Agent 20**: Final QA pass + fix any layout issues

## Data Requirements

### Metrics Needed
```typescript
// From get_metrics() response
interface RequiredMetrics {
  // CPU
  cpu_usage_percent: number;                    // Total
  'cpu_usage_percent{core="N"}': number[];      // Per-core

  // Memory
  memory_usage_percent: number;
  memory_used_bytes: number;
  memory_total_bytes: number;

  // Disk
  'disk_usage_percent{mount="/"}': number;      // Per mount
  disk_read_bytes_total: number;                // Counter
  disk_write_bytes_total: number;               // Counter

  // Network
  network_receive_bytes_total: number;          // Counter
  network_transmit_bytes_total: number;         // Counter
  'network_receive_bytes_total{interface="eth0"}': number; // Per interface
}
```

### Historical Queries
```typescript
// 24-hour CPU trend
getMetricHistory(serverId, 'cpu_usage_percent', now-24h, now, { resolution: '15m' })

// 24-hour network trend (sum of all interfaces)
getMetricHistory(serverId, 'network_receive_bytes_total', now-24h, now, { resolution: '15m' })
```

## Success Criteria

1. ✅ Single dashboard page displays all key metrics
2. ✅ Window size reduced to 1200x800 (or 1280x820)
3. ✅ Theme toggle uses icon hover menu (no text labels)
4. ✅ Settings page supports SSH key + password auth
5. ✅ Server count and online status visible in dashboard header
6. ✅ Activity rings animate smoothly (CPU/Memory/Disk)
7. ✅ Per-core heatmap updates every 2s
8. ✅ Network panel shows real-time speeds + total consumption
9. ✅ Historical sparklines render 24h trends
10. ✅ All existing functionality preserved (polling, storage, alerts)

## Non-Goals

- **Not changing**: Backend polling logic, adapter system, SQLite schema (except auth fields)
- **Not removing**: Probe and Alert functionality (deferred to future, but keep backend intact)
- **Not adding**: New metrics beyond what adapters already provide

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Per-core metrics missing for some adapters | Fallback to total CPU if per-core unavailable |
| Window too small on low-res displays | Add responsive breakpoints, allow window resize |
| SSH auth requires new Rust dependencies | Use existing `ssh2` crate, add to Cargo.toml |
| Translation keys incomplete | Add placeholders, mark for i18n review |

## Appendix: Icon SVG Code

### Sun Icon (Light Theme)
```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
  <circle cx="12" cy="12" r="4"/>
  <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"/>
</svg>
```

### Moon Icon (Dark Theme)
```svg
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
  <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
</svg>
```

---

**Next Steps**: Deploy 20-agent team to execute implementation plan in parallel.
