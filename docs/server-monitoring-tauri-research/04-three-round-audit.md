# Three-Round Audit Summary

Audit date: 2026-03-19

## Scope

Audited files:

- `01-analysis-report.md`
- `02-comparison-table.md`
- `03-initial-implementation-plan.md`

## Round 1: Factual and Source Audit

### Findings

- The report contained one internal inconsistency: it said the problem should be split into "two products" while the actual analysis already used three domains.
- Several capability descriptions were too optimistic for exporter-style tools:
  - `node_exporter` was close to being read as if it emitted ready-to-use traffic and disk speeds, while those are often derived from counters.
  - `ntopng` needed a caveat that some higher-end capabilities vary by edition.
- The original wording underplayed a security concern: scraping exporter endpoints directly over the public internet is a weak default pattern.

### Fixes Applied

- Corrected the domain split to `host metrics`, `network quality`, and `traffic analytics`.
- Added explicit caveats on derived-rate metrics for `node_exporter`.
- Added an edition caveat for `ntopng`.
- Added security corrections to the analysis report about private connectivity and authenticated gateways.

## Round 2: Comparison Logic Audit

### Findings

- The original table merged `History / Alerts` into one column, which hid meaningful differences.
- The original table merged `network fluctuation` and `active probing`, which made passive traffic tools and active probe tools look more equivalent than they are.
- A few ratings were too broad:
  - `Glances` per-core support was understated.
  - `node_exporter` traffic and disk-speed coverage needed to be downgraded from "ready feature" to "derived capability".
  - `Cockpit` alerting needed to be treated more conservatively.
  - `Nagstamon` history needed to be treated as absent at the client layer.

### Fixes Applied

- Split `History / Alerts` into separate columns.
- Split probing from traffic statistics.
- Rebalanced several project ratings to be more conservative and implementation-aware.
- Added audit notes clarifying that some columns describe derived capability, not out-of-the-box charts.

## Round 3: Architecture and Security Audit

### Findings

- The implementation plan did not make exporter exposure safety explicit enough.
- The plan did not explain that desktop-origin probes measure the operator path, not the server-origin path.
- The plan under-specified data correctness rules for counters, resets, staleness, and resource identity drift.
- The plan under-specified operational safeguards for bounded concurrency, retention jobs, and batching.

### Fixes Applied

- Added a `Secure Connectivity Baseline` section.
- Added a `Probe Vantage Model` section.
- Added `Metric Semantics Guardrails` for counters, gauges, resets, and staleness.
- Added `Operational Safeguards` for scrape concurrency, batching, and storage maintenance.
- Tightened Phase 1 so the first deployment assumes `private network or tunnel`, not raw public exporter access.

## Current Residual Risks

- The documents still assume Linux-first remote targets for the MVP.
- The traffic-analysis module remains intentionally light in v1; it is not a substitute for full network observability tooling.
- A future design review is still needed if you want multi-user sync or shared history across multiple desktop clients.

## Audit Outcome

- Status: `Improved`
- Result: the research set is now materially more accurate, more conservative, and safer as a basis for implementation decisions.
