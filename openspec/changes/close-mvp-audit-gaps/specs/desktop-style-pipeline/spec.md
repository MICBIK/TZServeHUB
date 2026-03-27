## ADDED Requirements

### Requirement: Build output SHALL include compiled desktop shell styling
The desktop client SHALL ship compiled CSS for the utility classes used by the routed shell so preview and production builds do not render as unstyled HTML.

#### Scenario: Preview build renders styled shell
- **WHEN** maintainers run `pnpm build` and serve the build output
- **THEN** the main shell renders with the intended layout, spacing, colors, and typography instead of browser-default styling

#### Scenario: Utility CSS is emitted for shipped routes
- **WHEN** maintainers inspect the emitted CSS for the built desktop client
- **THEN** it contains compiled rules for the utility classes used by the shipped shell routes or an equivalent generated stylesheet that produces the same visual result

### Requirement: Primary desktop routes SHALL remain visibly rendered
The dashboard, CPU, network, disk, probe, alert, and settings routes SHALL remain visibly usable in build and preview outputs after the styling pipeline is repaired.

#### Scenario: Routed page keeps shell layout
- **WHEN** the user navigates between primary desktop routes in a build or preview environment
- **THEN** the sidebar, content pane, and route-specific sections remain visibly laid out and readable without CSS regressions
