# D-0222 · Decide that global dependency installation is replaced by per-Package dependency objects
- Status: proposed
- Task: PKG-013
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §2, §29
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The §2 replacement of global dependency installation by per-Package dependency objects must be recorded (§2, §29, I-036).

## Options

### Option A · Per-Package content-identity dependency objects
Summary: Each Package references its own objects.
Consequences: No conflicts; duplication.
Evidence: none

### Option B · Global shared library directory
Summary: A global directory.
Consequences: Familiar; conflicts.
Evidence: none

### Option C · Generation-wide dependency set with conflict resolution
Summary: A per-generation set.
Consequences: Middle; resolution.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
