## ADDED Requirements

### Requirement: Control console SHALL combine fleet comparison with active-server history
The desktop control console SHALL show all registered servers in a comparison-first status surface and SHALL pair the selected server with chart-backed detail panels derived from real metric history.

#### Scenario: Console loads with recent telemetry
- **WHEN** registered servers have current and historical metric data
- **THEN** the console shows a fleet status list for cross-server comparison
- **AND** the selected server shows chart-backed CPU, memory, network, or disk panels in the detail area

### Requirement: Active-server detail SHALL stay dense without losing clarity
The selected-server detail area SHALL combine small charts, bars, and summary stats so operators can inspect one server quickly without leaving the main console.

#### Scenario: Operator selects a server from the fleet list
- **WHEN** the operator changes the active server in the sidebar or status list
- **THEN** the detail stack updates to the newly selected target
- **AND** long labels, rates, and percentages remain readable without overlap

#### Scenario: Operator inspects the memory module
- **WHEN** the selected server has current memory metrics
- **THEN** the memory module shows total memory alongside used, cached, and free breakdown values
- **AND** the breakdown remains readable in dense desktop layouts without collapsing cached memory into a generic idle bucket
