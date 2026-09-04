# D-0015 · Decide how user space identifies an Operation: Capability, ring index or opaque handle
- Status: proposed
- Task: ABI-015
- Surfaces: none
- Layer: none
- Spikes: TSK-014
- Supersedes: none
- Superseded by: none
- Baseline: §19, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
User space must identify an in-flight Operation for cancel and deadline without a blocking syscall other than wait-for-completion (§19), and V0 cancellation tests need the reference stable.

## Options

### Option A · Capability to Operation
Summary: An in-flight Operation is a Capability<Operation> handle.
Consequences: Cancel and inspect are ordinary Capability operations with rights; each submission mints a handle that must be released.
Evidence: none

### Option B · Ring index
Summary: The Operation is named by its index in the submission ring.
Consequences: No allocation per Operation; index reuse and ring wrap make stale references a hazard.
Evidence: none

### Option C · Opaque handle plus cancellation token
Summary: The Operation is an opaque handle paired with a separate cancellation token.
Consequences: Cancellation can be delegated without exposing the Operation; two identifiers per Operation to manage.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
