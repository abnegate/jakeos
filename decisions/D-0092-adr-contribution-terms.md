# D-0092 · Decide contributor licensing, copyright holder and DCO or CLA
- Status: proposed
- Task: GOV-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Provenance cannot be reconstructed after the first external commit, so contributor licensing, copyright holder and DCO versus CLA must be recorded so BLD can enforce headers and sign-off from V0 (§50, §57).

## Options

### Option A · DCO-only on all trees
Summary: Every commit carries Signed-off-by and contributors keep their copyright.
Consequences: Lowest contributor friction and identical to kernel practice; the project can never relicense without tracking down every contributor.
Evidence: none

### Option B · DCO on the kernel and a non-assignment CLA on userspace
Summary: Kernel commits use DCO while userspace contributors sign a CLA that grants patent rights without assigning copyright.
Consequences: Explicit patent grants where permissive licenses need them; two onboarding processes and a CLA bot to run.
Evidence: none

### Option C · Assignment CLA on all trees
Summary: Contributors assign copyright to the project entity.
Consequences: The entity can relicense freely; assignment deters contributors and conflicts with kernel norms and the GPLv2 fork.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
