# D-0310 · Decide how Operation priority relates to ResourceDomain Scheduling intent
- Status: proposed
- Task: TSK-027
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §19, §22
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every Operation carries a priority field (§19) and every Component runs under a ResourceDomain whose scheduling intent (Interactive, Background, EnergyEfficient, Realtime) is set by SCH (§22). Before the kernel orders I/O and Channel work by that field, and before the compositor consumes Interactive and Background classes, this decision fixes how the two relate: inherited, bounded override, or independent. Native software expresses intent, never a POSIX nice value (I-032), and each option must say how a compositor Operation and a Background Operation are ordered when both are in flight.

## Options

### Option A · Inherit-from-domain intent
Summary: The priority field is derived from the owning ResourceDomain's intent at submission and cannot be set by the Component.
Consequences: One knob, consistent ordering, and no way for an application to promote itself: a compositor Operation outranks a Background one because the compositor's domain is Interactive. A Component cannot mark one of its own Operations as less urgent (a prefetch beside a frame), so the runtime serialises internally instead.
Evidence: none

### Option B · Per-Operation override bounded by the domain
Summary: A Component may set a per-Operation priority within the range its domain's intent allows; the domain is the ceiling.
Consequences: Applications order their own work (frame before prefetch) while no Operation exceeds what the domain permits, so the compositor still wins over Background. Ordering is two-dimensional (domain ceiling, then Operation value) and the kernel must define it for I/O queues, Channel delivery and timers alike; the SDK must explain the ceiling when a value is clamped.
Evidence: none

### Option C · Independent Operation priority
Summary: Operation priority is a free field unrelated to domain intent.
Consequences: Maximum control for a Component author. A Background application can submit top-priority Operations and starve the compositor, which defeats intent classes entirely and reintroduces nice-style tuning (I-032); recorded as rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
