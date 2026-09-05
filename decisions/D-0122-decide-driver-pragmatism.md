# D-0122 · Decide pragmatic driver residency over microkernel purity
- Status: accepted
- Task: HW-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §33, §55, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§33 and §55 forbid microkernel purity and flag-day driver replacement, so pragmatic residency must be recorded as a standing non-goal (§33, §55, §57).

## Options

### Option A · All drivers in user space
Summary: Every driver class is moved out of the kernel.
Consequences: Maximum isolation; unmeasured cost and a flag day per class that §55 forbids.
Evidence: none

### Option B · All drivers in-kernel
Summary: No driver class ever moves.
Consequences: Hardware support is never at risk; no isolation progress and no evidence gathered.
Evidence: none

### Option C · Pragmatic residency by measured cost per class
Summary: Each class moves only when a measured Decision names it, with a dual-path period.
Consequences: Evidence-driven and safe for hardware support; ongoing classification work per class.
Evidence: none

## Decision
Option C. Driver residency is decided per device class by measured cost, never by principle. Inherited Linux drivers stay in the kernel by default. A class moves to a user-space driver Component (SVC hosting) only after a spike shows the latency, DMA-safety and interrupt cost are acceptable for that class; Bluetooth, audio, sensors and printing are the first candidates (§33).

## Consequences
- Every residency move is an adr task citing a spike report and a benchmark report.
- SVC provides the user-space driver hosting framework and device-access Capabilities; HW decides per class.
- GPU, NVMe, network and USB host controllers remain in-kernel through 1.0 unless a spike says otherwise.

## Rejected options and why
- Option A (all drivers in user space) rejected: microkernel purity is a stated non-goal (§33, §57) and would stall the hardware layer for years.
- Option B (all drivers in-kernel) rejected: it gives up the failure isolation §32 wants for the classes that crash most.

## Follow-ups
none
