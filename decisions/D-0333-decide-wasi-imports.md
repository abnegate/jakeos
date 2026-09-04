# D-0333 · Decide WASI imports bound to native Capability
- Status: proposed
- Task: WASM-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §13, §9, §9.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Which WASI worlds bind to Capability and which ambient Preview1 imports are forbidden must be listed (§13, §9, §9.1); S-029 is listed.

## Options

### Option A · Documented WASI Preview2 subset bound to Capability, Preview1 forbidden
Summary: A Capability-bound subset.
Consequences: Safe; compatibility limits.
Evidence: none

### Option B · Full WASI Preview2 including filesystem and socket worlds
Summary: Full Preview2.
Consequences: Compatibility; ambient shapes.
Evidence: none

### Option C · No WASI worlds; only native IDL imports
Summary: No WASI.
Consequences: Pure; ecosystem loss.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
