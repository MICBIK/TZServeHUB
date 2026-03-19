## ADDED Requirements

### Requirement: Desktop shell MUST build and load cleanly

The desktop client SHALL pass the repository frontend validation commands and SHALL load the routed monitoring shell without broken imports, invalid type-only imports, or placeholder compile failures.

#### Scenario: Frontend validation passes

- **WHEN** maintainers run `pnpm lint` and `pnpm build`
- **THEN** both commands complete successfully without TypeScript or ESLint errors

#### Scenario: Application shell loads

- **WHEN** the desktop application starts
- **THEN** the main layout and configured routes load without module resolution or syntax errors

### Requirement: Monitoring pages MUST expose explicit runtime states

Each monitoring page in the MVP SHALL render an explicit `loading`, `empty`, `error`, or `data` state that is backed by typed command results rather than user-facing TODO placeholders.

#### Scenario: No configured servers

- **WHEN** the app has no configured servers
- **THEN** the dashboard and relevant detail pages show an actionable empty state explaining how to add a server

#### Scenario: Data retrieval fails

- **WHEN** a command or data fetch fails for the selected server
- **THEN** the page shows an error state instead of silently rendering empty placeholders

#### Scenario: Data is available

- **WHEN** the selected server has current or historical metric data
- **THEN** the relevant page renders charts or metric cards from typed results
