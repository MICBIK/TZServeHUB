## ADDED Requirements

### Requirement: Frontend SHALL hydrate persisted control-plane state on startup
The desktop frontend SHALL load persisted servers and application settings during startup and SHALL reflect that state in the routed shell without requiring a manual refresh.

#### Scenario: Saved state appears after launch
- **WHEN** the desktop app starts with previously stored servers and settings
- **THEN** the frontend loads that data into its stores and renders the current control-plane state in the UI

### Requirement: Desktop SHALL expose minimal server management workflows
The desktop frontend SHALL expose the minimal control-plane workflows needed to add, remove, and select monitoring targets using the implemented Tauri command surface.

#### Scenario: User adds a server from the settings route
- **WHEN** the user submits a valid server configuration in the settings route
- **THEN** the desktop invokes the persistent add-server command and the new target appears in the server list without restarting the app

#### Scenario: User removes a server from the settings route
- **WHEN** the user removes an existing server from the settings route
- **THEN** the desktop invokes the persistent remove-server command and the removed target disappears from the active list and selection state

#### Scenario: User selects the active server
- **WHEN** the user selects one of the available monitoring targets
- **THEN** the dashboard and detail pages use that selection for metric polling and stateful page rendering

### Requirement: Monitoring pages SHALL expose state from hydrated data
The dashboard and metric pages SHALL show explicit runtime states derived from hydrated servers, the active selection, and command results.

#### Scenario: No servers are configured
- **WHEN** the frontend hydrates and no monitoring targets exist
- **THEN** the dashboard and settings route show an actionable empty state describing how to add a server

#### Scenario: Server exists but no active selection is available
- **WHEN** monitoring targets exist but no active server is selected
- **THEN** the dashboard and detail pages show a selection-required state instead of a perpetual loading placeholder

#### Scenario: Metric retrieval fails
- **WHEN** the active server metric query fails
- **THEN** the affected page shows an explicit error state that communicates the failed retrieval instead of silently rendering placeholder values

#### Scenario: Metric data is available
- **WHEN** the active server has current or historical metric data
- **THEN** the dashboard and relevant detail views render metric cards or charts from typed results
