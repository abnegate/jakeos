# D-0198 · Decide the MemoryObject locality and placement attribute model
- Status: proposed
- Task: MEM-034
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §17, §37, §38
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The locality and placement attribute vocabulary exposed to placement and dispatch must be fixed jointly with HET before V2 (§17, §37, §38).

## Options

### Option A · Query-only attributes
Summary: Locality is readable but not requestable.
Consequences: Simple; callers cannot steer placement.
Evidence: none

### Option B · Requestable placement at allocation and migrate
Summary: Callers request placement and migration.
Consequences: Control for dispatch; complexity in the allocator.
Evidence: none

### Option C · Placement as a Capability right
Summary: Placement is governed by rights.
Consequences: Governed sharing of scarce memory; heavy.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
