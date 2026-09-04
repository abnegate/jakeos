# D-0307 · Decide whether Operations may complete inline at submit and how the ABI signals it
- Status: proposed
- Task: TSK-005
- Surfaces: none
- Layer: none
- Spikes: TSK-014
- Supersedes: none
- Superseded by: none
- Baseline: §18, §19, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Cached reads and signalled Waits can finish before submit returns; whether inline completion is allowed and how the ABI signals it is a Layer 1 choice on S-005 (§18, §19, §65).

## Options

### Option A · Never complete inline
Summary: Every completion is delivered later.
Consequences: One completion path; latency for already-ready work.
Evidence: none

### Option B · Inline with an ABI-visible flag
Summary: Submit returns a flag when the Operation completed inline.
Consequences: Fast path for cached reads; two completion paths for callers.
Evidence: none

### Option C · Inline with a distinct submit return code
Summary: Submit returns a distinct code.
Consequences: Explicit; two paths for callers.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
