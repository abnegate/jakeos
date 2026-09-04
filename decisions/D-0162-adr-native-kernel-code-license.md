# D-0162 · Decide the licence for new native kernel code
- Status: proposed
- Task: KRN-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §5.1, §50
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
New code linked into a GPLv2 kernel must be GPLv2-compatible and the choice decides whether kernel abstractions can be shared with permissive userspace (§5.1, §50), before the first native commit.

## Options

### Option A · GPLv2-only
Summary: New kernel code is GPLv2-only.
Consequences: Matches Linux exactly; nothing can be copied to permissive userspace crates.
Evidence: none

### Option B · GPLv2-or-later
Summary: New kernel code is GPLv2-or-later.
Consequences: Future license flexibility; still copyleft and not shareable with userspace.
Evidence: none

### Option C · Dual GPLv2/MIT for reusable abstractions
Summary: Selected reusable files are dual-licensed.
Consequences: Abstractions can be shared with userspace; per-file license tracking.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
