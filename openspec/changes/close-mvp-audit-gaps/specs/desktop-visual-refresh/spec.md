## ADDED Requirements

### Requirement: Desktop shell SHALL present a monitoring-first visual identity
The desktop client SHALL render a cohesive monitoring-focused shell inspired by verified ServerCat references, using dark layered surfaces, dense status cards, and high-contrast health accents instead of a generic placeholder layout.

#### Scenario: Dashboard loads the refreshed shell
- **WHEN** the user opens the dashboard in a build or preview environment
- **THEN** the shell shows a visually distinct monitoring layout with layered panels, clear metric grouping, and status-oriented visual hierarchy

### Requirement: Desktop shell SHALL use glass and light effects intentionally
The desktop client SHALL apply frosted glass treatments and highlight effects selectively to major shell surfaces so the interface feels modern without obscuring monitoring content.

#### Scenario: Frosted surfaces remain readable
- **WHEN** the user views the sidebar, hero surfaces, or major status panels
- **THEN** any glassmorphism treatment preserves text readability and does not reduce monitoring contrast below usable levels

### Requirement: Desktop shell SHALL include restrained SVG and launch motion
The desktop client SHALL include visible SVG-driven ambient accents and a startup animation that reinforce the product identity without making the monitoring UI distracting.

#### Scenario: Application entry animation completes cleanly
- **WHEN** the app first renders
- **THEN** the shell plays a bounded startup animation and settles into the final layout without blocking interaction or leaving elements hidden

#### Scenario: Ambient SVG accents do not overwhelm content
- **WHEN** the user navigates between primary monitoring routes
- **THEN** SVG accents and motion remain secondary to the metric content and do not interfere with route legibility
