## ADDED Requirements

### Requirement: Desktop shell SHALL apply runtime theme preferences
The desktop client SHALL apply the persisted theme setting to the live shell and SHALL allow switching between dark and light modes without restart.

#### Scenario: User toggles theme from compact shell chrome control
- **WHEN** the user switches the compact theme button while the desktop app is open
- **THEN** the shell updates its surfaces, typography contrast, and chart styling immediately and persists the selected theme

### Requirement: Desktop shell SHALL apply runtime language preferences
The desktop client SHALL apply the persisted language setting to core shell chrome and monitoring page copy and SHALL allow switching between zh-CN and en-US without restart.

#### Scenario: User toggles language from settings workspace
- **WHEN** the user switches the language control in the settings page while the app is open
- **THEN** the sidebar labels, monitoring headers, and primary state copy update immediately and persist the selected language without requiring restart
