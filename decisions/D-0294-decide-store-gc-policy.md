# D-0294 · Decide content-store garbage collection: root set, policy and user control
- Status: proposed
- Task: STO-041
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every SystemGeneration and every installed Package keeps objects alive in the content store (§27). Once the V1 signed repository produces daily generations the store grows without bound unless a collector removes unreferenced objects. This decision fixes the root set, the policy and the user control surface, answering Q-019; it sits on store objects (STO-009) and the generation boundary (D-0216), and must never collect an object a running Component still maps (D-0226).

## Options

### Option A · Generation-count roots
Summary: Roots are the last N bootable generations plus every installed Package and every running mapping; anything unreachable is collected.
Consequences: Predictable disk use and rollback depth is a number the user can see. Age is ignored, so a generation from months ago survives if it is within N, and N must be large enough for the rollback guarantee yet small enough for the disk.
Evidence: none

### Option B · Age-based collection
Summary: Objects unreferenced for longer than an age threshold are collected; generations older than the threshold lose bootability.
Consequences: Intuitive ("keep a month") and adaptive to update frequency. A machine that updates rarely may keep few generations, and an object needed by a rarely used but installed Package must still be protected by an installed-Package root.
Evidence: none

### Option C · User-pinned roots
Summary: Users pin generations and Packages; only pinned roots and the current generation survive collection.
Consequences: Full control and zero surprises for users who manage it. Most users never pin, so either nothing is collected or the current generation is the only root and rollback disappears; acceptable only as an addition to A or B.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
