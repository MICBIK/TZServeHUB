## ADDED Requirements

### Requirement: Enabled servers SHALL produce normalized metric samples

The system SHALL collect metrics from enabled servers through supported adapters and SHALL normalize each sample into a canonical internal representation containing server identity, metric key, metric type, timestamp, vantage point, and labels.

#### Scenario: Node exporter sample is normalized

- **WHEN** the system ingests a node_exporter sample with labels
- **THEN** the metric is stored with a stable internal key and structured labels instead of an unparsed raw sample string

#### Scenario: Go agent sample is normalized

- **WHEN** the system ingests a Go agent payload
- **THEN** the payload is converted into the same canonical metric representation used by other adapters

### Requirement: Counter-based rates SHALL be derived safely

The system SHALL derive traffic and disk I/O rates from counter metrics over time and SHALL guard against negative rates caused by counter resets or invalid timestamps.

#### Scenario: Counter advances normally

- **WHEN** two counter samples for the same metric arrive in increasing timestamp order
- **THEN** the system computes a non-negative derived rate from the delta over time

#### Scenario: Counter resets

- **WHEN** a counter sample is lower than the previously stored value for the same metric identity
- **THEN** the system treats it as a reset and does not emit a negative derived rate

### Requirement: Metric history SHALL be queryable by server, key, and time range

The system SHALL persist raw and rolled-up metric history in SQLite and SHALL return bounded history for a given server, metric key, and requested time range.

#### Scenario: Query raw history

- **WHEN** the caller requests recent history within the raw retention window
- **THEN** the system returns metric points for that server and key within the requested bounds

#### Scenario: Query rolled-up history

- **WHEN** the caller requests older history outside the raw retention window but inside a configured rollup window
- **THEN** the system returns aggregated history from the appropriate rollup table
