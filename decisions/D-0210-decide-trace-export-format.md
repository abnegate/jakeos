# D-0210 · Decide the trace event schema and export format
- Status: proposed
- Task: OBS-015
- Surfaces: S-035
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §24
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The trace event schema and export format must let existing tooling be reused; V0.5 is the last rung before V1 requires offline export (§24).

## Options

### Option A · Perfetto/CTF-compatible
Summary: Perfetto format.
Consequences: Existing viewers; format constraints.
Evidence: none

### Option B · OpenTelemetry mapping
Summary: OTel mapping.
Consequences: Ecosystem; overhead.
Evidence: none

### Option C · Native binary with schema registry
Summary: Native format with a registry.
Consequences: Exact fit; tooling to build.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
