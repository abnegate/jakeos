# D-0209 · Decide who may trace and inspect which Components
- Status: proposed
- Task: OBS-014
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §24
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§24 requires security-aware tracing; unrestricted tracing would leak Channel payloads and defeat the V0.5 file-chooser demo (§9.1, §24, T-027).

## Options

### Option A · Ambient debug right per session
Summary: Session-wide tracing.
Consequences: Convenient; leaks payloads.
Evidence: none

### Option B · Capability<Trace> per ResourceDomain subtree
Summary: Scoped tracing Capability.
Consequences: Precise; grants to manage.
Evidence: none

### Option C · Owner-only with elevation
Summary: Trace only your own without elevation.
Consequences: Safe default; friction for developers.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
