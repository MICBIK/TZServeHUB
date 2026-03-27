## ADDED Requirements

### Requirement: Counter-derived rates SHALL persist continuity across polling cycles
The metrics pipeline SHALL preserve the prior sample state for each counter series across consecutive polling cycles so derived rates are computed from real sequential observations.

#### Scenario: Consecutive polls emit a derived rate
- **WHEN** two polls for the same counter series arrive in increasing timestamp order
- **THEN** the pipeline emits a non-negative derived rate for that series instead of treating every poll as a first sample

#### Scenario: Series identity remains distinct
- **WHEN** multiple labeled counter series for the same key are collected across polling cycles
- **THEN** the pipeline preserves continuity per series identity and does not mix prior state across sibling label sets

### Requirement: Counter resets SHALL remain safe after continuity repair
The metrics pipeline SHALL continue suppressing negative derived rates when a counter resets or when timestamps are invalid even after rate continuity is preserved across polls.

#### Scenario: Counter reset after prior samples
- **WHEN** a later poll reports a lower counter value for the same series identity
- **THEN** the pipeline updates its stored baseline and does not emit a negative derived rate

#### Scenario: Invalid or non-increasing timestamps
- **WHEN** a later poll for the same counter series has a non-increasing timestamp
- **THEN** the pipeline does not emit a derived rate for that sample
