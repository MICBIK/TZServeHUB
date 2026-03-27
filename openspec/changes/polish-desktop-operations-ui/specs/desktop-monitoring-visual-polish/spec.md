## ADDED Requirements

### Requirement: Desktop shell SHALL present calmer monitoring-first chrome
The desktop client SHALL keep the ServerCat-inspired monitoring identity while reducing decorative noise, improving icon clarity, and showing a visible startup SVG animation.

#### Scenario: App launch presents a purposeful intro
- **WHEN** the desktop app first renders
- **THEN** the user sees a bounded SVG startup animation that resolves into the final shell instead of only a faint overlay flash

### Requirement: Desktop shell SHALL avoid content overlap in dense layouts
The desktop client SHALL keep long labels, stat chips, and server cards readable without collisions or clipped content in supported desktop widths.

#### Scenario: Long labels appear in dashboard panels
- **WHEN** long mount paths, interface names, or target names are rendered in the dashboard or detail pages
- **THEN** the shell truncates or reflows content cleanly instead of overlapping adjacent values
