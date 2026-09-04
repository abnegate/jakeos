# D-0051 · Decide V0 boots Linux init from a retained initramfs with native Components beside it
- Status: proposed
- Task: BOOT-004
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §5.1, §59
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0 exit text demanding a native init conflicts with §5.1 Phase A, which preserves boot (§5.1, §59), so V0 boot is fixed as retained Linux boot with native Components started beside Linux init and native init deferred to SVC at V0.5.

## Options

### Option A · Retained initramfs plus Linux init
Summary: V0 boots a retained initramfs and Linux init, launching native Components beside it.
Consequences: Boot keeps working while native pieces land; Linux init semantics persist into V0.
Evidence: none

### Option B · Native init at V0
Summary: A native init replaces Linux init from the start.
Consequences: Native boot path from day one; V0 slips on init work that SVC owns.
Evidence: none

### Option C · Hybrid stub init
Summary: A tiny stub starts both Linux services and native Components.
Consequences: Middle ground; a third init to throw away.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
