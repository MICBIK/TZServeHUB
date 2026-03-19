## ADDED Requirements

### Requirement: Server registry SHALL persist configured targets

The system SHALL store configured monitoring targets in SQLite and SHALL expose them through working Tauri commands for list, create, and remove operations.

#### Scenario: Add server persists

- **WHEN** the user adds a server through the desktop control plane
- **THEN** a persisted server record is created and returned by subsequent `list_servers` calls

#### Scenario: Remove server persists

- **WHEN** the user removes a stored server
- **THEN** the server no longer appears in `list_servers` results after the command completes

#### Scenario: Restart preserves server registry

- **WHEN** the desktop app restarts after servers were configured
- **THEN** the previously stored servers are still available from the command layer

### Requirement: App settings SHALL persist across sessions

The system SHALL persist app settings such as polling defaults, retention settings, theme, and language through the settings command surface.

#### Scenario: Update settings persists

- **WHEN** the user updates settings
- **THEN** a subsequent `get_settings` call returns the updated values

#### Scenario: Restart preserves settings

- **WHEN** the desktop app restarts after settings were updated
- **THEN** the updated settings remain the values returned by `get_settings`
