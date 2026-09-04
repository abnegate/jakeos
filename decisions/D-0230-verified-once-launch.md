# D-0230 · Decide verified-once launch trust for cached Package objects
- Status: proposed
- Task: PKG-045
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §34
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Launch must trust cached Package objects so mapping can skip re-hash before V0.5 immutable Package mapping (§34).

## Options

### Option A · dm-verity-style block verification
Summary: Verity blocks.
Consequences: Kernel-enforced; setup.
Evidence: none

### Option B · Signed content-store index
Summary: A signed index.
Consequences: Flexible; index trust.
Evidence: none

### Option C · Per-launch hash of a small manifest
Summary: A manifest hash per launch.
Consequences: Cheap; coverage.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
