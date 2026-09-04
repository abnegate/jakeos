# D-0287 · Decide GPT partition and volume layout for store, generations, user data, swap and recovery
- Status: proposed
- Task: STO-014
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §26, §30, §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Layout must be fixed before V0.5 SystemGenerations with registered GPT type GUIDs (§26, §30, §32).

## Options

### Option A · Separate GPT partitions
Summary: One partition per volume.
Consequences: Tool-visible; rigid.
Evidence: none

### Option B · Filesystem subvolumes
Summary: Subvolumes.
Consequences: Flexible; opaque to foreign tools.
Evidence: none

### Option C · Mixed layout
Summary: Partitions plus subvolumes.
Consequences: Balance; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
