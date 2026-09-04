# D-0066 · Decide Component panic, abort and typed exit-cause semantics
- Status: proposed
- Task: CMP-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §10, §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A Rust panic must abort only its Component, stack overflow and OOM must be typed exit causes, and no unwinding crosses the Native ABI (§10, §32); V0-D03 requires this Decision.

## Options

### Option A · Abort-only
Summary: Any panic aborts the Component with a typed exit cause.
Consequences: Simple and predictable; no in-Component recovery from panics.
Evidence: none

### Option B · Unwind to Component boundary
Summary: Panics unwind within the Component and abort at its boundary.
Consequences: Destructors run and some recovery is possible; unwinding tables and cost in every Component.
Evidence: none

### Option C · Per-Component policy
Summary: Each Component declares abort or unwind.
Consequences: Flexibility; two behaviours for supervisors to reason about.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
