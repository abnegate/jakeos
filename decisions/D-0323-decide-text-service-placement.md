# D-0323 · Decide whether shaping runs in-Component or in a shared text service Component
- Status: proposed
- Task: TXT-004
- Surfaces: none
- Layer: none
- Spikes: TXT-011
- Supersedes: none
- Superseded by: none
- Baseline: §41, §10, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Placement of shaping changes isolation and latency for every UI Component (§41, §10, §51); I-083 and T-030 constrain writable sharing.

## Options

### Option A · In-Component library
Summary: Shaping in each Component.
Consequences: Isolation; duplication.
Evidence: none

### Option B · Shared system text service
Summary: A shared service.
Consequences: Sharing; a font-parsing attack surface.
Evidence: none

### Option C · Hybrid: library shaping, service-minted read-only caches
Summary: Library plus read-only caches.
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
