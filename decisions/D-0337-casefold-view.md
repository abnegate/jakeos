# D-0337 · Decide the case-insensitive view for Windows Personality storage
- Status: proposed
- Task: WIN-019
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §48
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Case-insensitive, case-preserving semantics for the Windows personality view must not leak into native or Linux views (§25, §48).

## Options

### Option A · Wine path lookup
Summary: Wine's lookup.
Consequences: Existing; slow.
Evidence: none

### Option B · Per-prefix casefold overlay
Summary: An overlay per prefix.
Consequences: Fast; overlay.
Evidence: none

### Option C · Filesystem casefold confined to the personality view API
Summary: Confined casefold.
Consequences: Native speed; confinement.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
