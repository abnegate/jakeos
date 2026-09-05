# D-0066 · Decide Component panic, abort and typed exit-cause semantics
- Status: accepted
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
Option A. A panic, stack overflow, out-of-memory abort or capability violation in native code terminates the whole Component. The kernel reports a typed exit cause on the supervisor's Channel; nothing unwinds across the ABI and no destructor runs after the fault. Recovery is the supervisor's restart and rebind policy (§32, SVC).

## Consequences
- The native runtime sets panic=abort; the SDK exposes typed exit causes to supervisors.
- Resource release on fault is the kernel's job (ResourceDomain teardown), never the faulting code's.
- Tests for every exit cause live in CMP (the leak test and isolation negative tests).

## Rejected options and why
- Option B (unwind to the Component boundary) rejected: a larger runtime, destructors that panic, and a fault path that runs code inside a possibly compromised Component.
- Option C (per-Component policy) rejected: two fault behaviours for every supervisor, debugger and tracer to handle.

## Follow-ups
none
