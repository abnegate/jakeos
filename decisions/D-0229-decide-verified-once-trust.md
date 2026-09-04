# D-0229 · Decide the verified-once launch trust mechanism
- Status: proposed
- Task: PKG-050
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §34
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Verified-once launch trust is revisited for V1 signed activation, composing with the verification cache and store verity (§34).

## Options

### Option A · dm-verity-style block verification
Summary: Verity blocks.
Consequences: Kernel-enforced; block granularity.
Evidence: none

### Option B · Signed content-store index
Summary: A signed index.
Consequences: Flexible; index trust.
Evidence: none

### Option C · Per-launch hash of a small manifest
Summary: A manifest hash per launch.
Consequences: Cheap; less coverage.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
