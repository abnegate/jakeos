# D-0306 · Decide deadline and timestamp representation in the Operation ABI
- Status: proposed
- Task: TSK-004
- Surfaces: none
- Layer: none
- Spikes: TSK-015
- Supersedes: none
- Superseded by: none
- Baseline: §18, §19, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every Operation carries a deadline (§18, §19), so the clock domain, resolution and overflow horizon are part of the Layer 1 Operation ABI on S-005 while it stays prototyped (§65). TSK-015 measures in-kernel deadline enforcement. The choice fixes what a Timer or Wait observes across suspend and resume (implemented against PWR later), what `Error::DeadlineExceeded` means, and how completions are timestamped for tracing. The representation is architecture-neutral: a 64-bit nanosecond count with a stated horizon.

## Options

### Option A · Monotonic clock that does not advance during suspend
Summary: Deadlines and timestamps are nanoseconds on a monotonic clock that pauses while the system is suspended.
Consequences: Timers never fire spuriously at resume and a relative deadline means active time, which is the common case for I/O and UI deadlines. A deadline meant as elapsed real time (a lease, a network timeout, a certificate) silently stretches across suspend, so either a second clock kind is added to S-005 later or PWR must compensate at resume.
Evidence: `reports/spikes/TSK-015.md`

### Option B · Boot-time clock that does
Summary: Deadlines and timestamps are nanoseconds since boot and keep advancing through suspend.
Consequences: Deadlines mean real elapsed time, so leases and network timeouts behave without help. Everything queued across a suspend expires at resume, producing a burst of `DeadlineExceeded` completions the runtime must absorb without treating them as failures. Trace timestamps line up with power events, which OBS and PWR both want.
Evidence: `reports/spikes/TSK-015.md`

### Option C · Wall clock
Summary: Deadlines are UTC instants.
Consequences: Human-readable and comparable across machines. NTP steps, leap seconds and a user changing the clock move every outstanding deadline, so the kernel cannot promise ordering or a bounded overshoot; this fails the requirement that a deadline be unaffected by policy and is recorded so it is not proposed again.
Evidence: `reports/spikes/TSK-015.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
