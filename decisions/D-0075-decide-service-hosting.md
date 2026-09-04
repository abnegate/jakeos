# D-0075 · Decide how environment services are hosted and packaged
- Status: proposed
- Task: ENV-009
- Surfaces: none
- Layer: none
- Spikes: ENV-022
- Supersedes: none
- Superseded by: none
- Baseline: §3, §35, §36
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Most developer services exist only as Linux software, so how postgres and redis are hosted decides the native environment story (§3, §35, §36); options come from the spike report.

## Options

### Option A · Linux-personality OCI images inside native isolation
Summary: Services run as OCI images inside the personality.
Consequences: Every service available today; startup and footprint are those of a container.
Evidence: none

### Option B · Native Packages
Summary: Services are packaged natively.
Consequences: Best integration and startup; each service needs a native port.
Evidence: none

### Option C · Both with a documented default
Summary: Both paths exist and one is default for postgres and redis.
Consequences: Coverage plus a native path; two hosting modes to support.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
