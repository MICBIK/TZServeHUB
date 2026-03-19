## ADDED Requirements

### Requirement: Go agent and desktop adapter SHALL share a compatible metrics contract

The Go agent metrics payload and the Rust adapter deserialization model SHALL use one compatible field contract for host metrics, disk metrics, disk I/O counters, and network counters.

#### Scenario: Agent payload deserializes successfully

- **WHEN** the desktop adapter requests `/api/metrics` from a supported Go agent version
- **THEN** the response deserializes successfully without field-name translation errors or dropped required fields

#### Scenario: Contract change is explicit

- **WHEN** the payload shape changes for the Go agent
- **THEN** the corresponding Rust adapter contract and documentation are updated in the same change

### Requirement: Agent auth behavior SHALL be explicit and consistent

The Go agent SHALL apply documented auth behavior consistently across health and metrics endpoints so desktop integration can perform liveness checks and secured metric fetches predictably.

#### Scenario: Metrics endpoint requires token

- **WHEN** agent token authentication is configured and the client requests `/api/metrics` without a valid bearer token
- **THEN** the agent rejects the request with an unauthorized response

#### Scenario: Health endpoint semantics are documented and implemented consistently

- **WHEN** the client requests `/api/health`
- **THEN** the endpoint behavior matches the documented auth expectation used by the desktop integration
