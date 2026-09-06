# D-0308 · Decide which Operation ABI surfaces become Layer 1 freeze candidates
- Status: proposed
- Task: TSK-042
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §65, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Layer 1 freeze candidates are named at V1 with SDK v1 and frozen at V4 (§65, §66, I-040); the V4 snapshot in V4-G01 covers only what is named here. For the Operation ABI that means the submission entry, the completion record layout, the result encoding, the deadline representation (D-0306) and the kind set. This decision lists each candidate with its spike and adr and records that S-005 and S-008 stay prototyped.

## Options

### Option A · Full Operation candidate set
Summary: Every Operation surface is a candidate: submission entry, completion record, result encoding, deadline and timestamp fields, cancellation, and every kind that exists at V1.
Consequences: SDK v1 gets a complete kernel contract for asynchronous work. Every kind needs spike and benchmark evidence now, including kinds added late in V0.5, and a kind that later proves wrong is frozen at V4 with it.
Evidence: none

### Option B · Reduced core: submit, complete, cancel, six V0 kinds
Summary: The core is a candidate (submit, complete, cancel, the completion record and result encoding, and the six V0 kinds); later kinds are added to the candidate list by their own adrs.
Consequences: The parts with two rungs of evidence are committed and the newer kinds keep evolving. SDK v1 code that uses a non-candidate kind may change before V4, which the SDK must mark, and V4-G01 must gather the additional adrs.
Evidence: none

### Option C · Defer naming to V4
Summary: No Operation candidates are named at V1; naming waits for V4.
Consequences: Maximum freedom. SDK v1 has no stable asynchronous contract and V4 must name and freeze in one rung, which is the failure mode the candidate step exists to prevent; listed to be rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
