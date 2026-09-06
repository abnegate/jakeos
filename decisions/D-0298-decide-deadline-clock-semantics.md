# D-0298 · Decide monotonic versus wall-clock semantics for Operation deadlines across suspend
- Status: proposed
- Task: SVC-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
D-0306 fixed how a deadline is encoded on the Operation ABI; V1 brings suspend and resume on the reference laptop (PWR-002) and so forces the semantic question: which clock deadlines and timers use, whether suspended time counts, and how subscribers learn that the wall clock stepped (TLS validity, signature verification and the time service depend on that). TSK-041 implements the chosen semantics; this decision names them.

## Options

### Option A · CLOCK_MONOTONIC-like clock that does not advance during suspend
Summary: Deadlines use a monotonic clock that pauses in suspend; a deadline that would have expired during suspend is effectively extended by the suspended time.
Consequences: No burst of expiries at resume and I/O timeouts mean active time. Leases and anything meant as real elapsed time silently stretch, so those users must consult the wall clock explicitly.
Evidence: none

### Option B · CLOCK_BOOTTIME-like clock that does
Summary: Deadlines use a boot-time clock that advances in suspend; every deadline that passed during suspend completes as `DeadlineExceeded` immediately at resume.
Consequences: Real elapsed time is honoured everywhere. Resume delivers a burst of expiries that runtimes must absorb without treating them as failures, and UI timers fire all at once.
Evidence: none

### Option C · Separate clocks with explicit step notification
Summary: Operation deadlines use the paused monotonic clock; a separate wall-clock Object emits a step notification Operation whenever the clock jumps or resumes from suspend, and subscribers (TLS, signature verification, time service) re-evaluate on it.
Consequences: Each consumer gets the clock its semantics need and nobody polls for steps. Two clocks in the ABI vocabulary and a notification contract to freeze; the SDK must make the choice obvious per API.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
