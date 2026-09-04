# D-0261 · Decide profiler export format and Task attribution
- Status: proposed
- Task: SDK-053
- Surfaces: none
- Layer: none
- Spikes: SDK-061
- Supersedes: none
- Superseded by: none
- Baseline: §24, §64
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Native profiles are attributed to Task, Component and TaskGroup, not threads (§24, §64).

## Options

### Option A · pprof
Summary: pprof is the export format.
Consequences: Existing tooling; a thread-centric model.
Evidence: none

### Option B · Firefox Profiler
Summary: The Firefox Profiler format.
Consequences: A capable UI; format fit for Tasks.
Evidence: none

### Option C · Native format folded into traces
Summary: Profiles are trace events.
Consequences: Unified with tracing; tooling to build.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
