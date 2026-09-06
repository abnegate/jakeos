# D-0249 · Decide behaviour on ResourceDomain budget exhaustion and owner reporting
- Status: proposed
- Task: SCH-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §23, §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Packages and system services run under real ResourceDomain budgets from V0.5 (§23), so exhaustion must have one typed, observable behaviour per budget kind (memory, CPU share, kernel-object count) and the owner of the domain must learn about it (§32, T-016). This answers Q-015. The accepted option names the error type, forbids untyped SIGKILL-style death on the native path, and states the Channel or inspect signal the owner sees.

## Options

### Option A · Fail-closed typed Operation errors with an owner event
Summary: The Operation that would exceed the budget fails closed with `Error::Exhausted` naming the budget kind, and the domain owner receives a typed event on its supervision Channel.
Consequences: Applications can recover, `os inspect` shows the exhaustion, and no Component dies for exceeding a limit. Every allocation path in the SDK must handle `Exhausted`, CPU share cannot fail an Operation (it throttles), and a Component that ignores the error can spin on retries.
Evidence: none

### Option B · Component termination after reclaim
Summary: The kernel reclaims what it can (caches, unpinned pages) and if the budget is still exceeded terminates the Component with a typed exit cause the supervisor sees.
Consequences: One rule, no error handling burden on applications, and the supervisor's restart policy (SVC) already covers it. Users lose unsaved state, work in flight on Channels is dropped, and the behaviour is indistinguishable from a crash unless the exit cause is surfaced.
Evidence: none

### Option C · Per-kind mix
Summary: Per kind: memory allocations fail with `Error::Exhausted` after reclaim, CPU share throttles with no error, kernel-object creation fails with `Error::Exhausted`; termination happens only if the domain owner asks for it.
Consequences: Each budget kind gets the behaviour that makes sense for it and the owner keeps the kill decision. Three behaviours to specify, test and document, and applications must know which kind they hit.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
