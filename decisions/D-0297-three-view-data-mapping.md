# D-0297 · Decide how user data maps across native, Linux home and Windows profile views
- Status: proposed
- Task: STO-068
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §46, §48
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
D-0295 fixed the live mapping: one object graph with two path facades. This decision fixes how INS migration imports an existing Linux home directory or Windows profile into that graph without producing three diverging copies (§25, §46, §48): import as copy, adopt in place at first personality launch, or dual-write during the import. It sits on the live mapping (STO-042), the view API (STO-047) and the personality views (STO-036).

## Options

### Option A · Import-as-copy into native Collections
Summary: Migration copies the existing home or profile into native Collections; the personalities then see it through the facades; the original volume is left untouched for the user to remove.
Consequences: One truth after import and the original is a safe fallback. Import takes as long as the data is large, needs the space twice until the original is removed, and the user must trust the copy before deleting.
Evidence: none

### Option B · Adopt-in-place on first personality launch
Summary: The existing home or profile is adopted in place as a foreign volume (D-0286) and indexed into Collections at first personality launch.
Consequences: Instant migration with no extra space. The data stays on a foreign filesystem with its degradation rules, and the personality's view and the native Collection can diverge if the foreign OS in a dual-boot writes to it.
Evidence: none

### Option C · Dual-write during import
Summary: During a transition period writes go to both the original tree and the native graph until the user completes the import.
Consequences: Neither side is stale during migration. Two writers, conflict resolution and a transitional state that can persist indefinitely; rejected unless a dual-boot requirement makes it unavoidable.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
