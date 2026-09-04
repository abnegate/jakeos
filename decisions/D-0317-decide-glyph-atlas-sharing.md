# D-0317 · Decide the cross-Component glyph atlas and shaped-text cache sharing model
- Status: proposed
- Task: TXT-015
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §51, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 runs many text-rendering Components, so glyph atlas and shaped-text sharing must be decided under I-083 and T-030 (§41, §51, §67).

## Options

### Option A · Per-Component caches
Summary: Each Component caches.
Consequences: Isolation; memory duplication.
Evidence: none

### Option B · Read-only atlas minted by the text service
Summary: A read-only shared atlas.
Consequences: Sharing without writes; a service.
Evidence: none

### Option C · Shared writable atlas
Summary: A writable shared atlas.
Consequences: Efficient; a cross-domain side channel.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
