# D-0329 · Decide server-side versus client-side decorations for native and compat windows
- Status: proposed
- Task: UIP-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §47, §49
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Server-side versus client-side decorations fixes window Surface roles in protocol v0 before compat windows at V0.5 (§41, §47, §49).

## Options

### Option A · Server-side for native and compatibility windows
Summary: SSD everywhere.
Consequences: Consistent; less app control.
Evidence: none

### Option B · Client-side for native and server-side for compatibility
Summary: Split.
Consequences: Native flexibility; two paths.
Evidence: none

### Option C · Client-side for all
Summary: CSD everywhere.
Consequences: App control; inconsistency.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
