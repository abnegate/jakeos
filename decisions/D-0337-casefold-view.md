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
Windows software expects case-insensitive, case-preserving paths. Native and Linux views are case-sensitive and must stay so (I-007, §25, §48). This decision fixes where the Windows personality's case-insensitive view lives: Wine's own path lookup, a per-prefix casefold overlay, or filesystem casefolding confined to the personality view API (STO-047), so no case-insensitive lookup leaks into native or Linux views.

## Options

### Option A · Wine path lookup
Summary: Wine's user-space case-insensitive path lookup, scanning directories on miss.
Consequences: Exists and is correct today. Directory scans on every miss make it the slowest option for large trees (game installs), which the W corpora's load-time metrics will show.
Evidence: none

### Option B · Per-prefix casefold overlay
Summary: A per-prefix casefold overlay filesystem presents the prefix and the user's granted objects case-insensitively to the personality only.
Consequences: Fast lookups and confined to the prefix. One more overlay in the personality's mount graph, and objects granted from outside the prefix must be projected into it.
Evidence: none

### Option C · Filesystem casefold confined to the personality view API
Summary: The personality view API returns a casefolded view (using the filesystem's casefold feature where present) for Windows-personality Components only; native and Linux views never request it.
Consequences: Native speed with the rule enforced at the one API that builds views. Depends on the filesystem's casefold support (btrfs and ext4 have it, others do not), and fallback to A is needed on volumes without it.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
