# D-0062 · Decide what replaces PID, parent/child, exit status and process groups
- Status: accepted
- Task: CMP-006
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §1, §2, §10
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Component replaces process, so the native replacements for PID, parent/child, exit status and process groups that every V0 build task assumes must be fixed (§1, §2, §10).

## Options

### Option A · Object<Component> handles only
Summary: Identity and lineage exist only as handles held by the creator.
Consequences: No global identifiers; inspection tooling must follow handle graphs.
Evidence: none

### Option B · Handle plus kernel-visible lineage
Summary: Handles plus a kernel record of who created whom.
Consequences: Supervisors and inspect see lineage; a kernel-side tree to maintain.
Evidence: none

### Option C · Opaque identity with supervisor-held lineage
Summary: The kernel issues opaque ids and supervisors keep lineage in userspace.
Consequences: Kernel stays minimal; lineage is only as reliable as the supervisor.
Evidence: none

## Decision
Option A. Object<Component> handles are the only native identity. There are no PIDs, no kernel parent/child tree, no exit status integers and no process groups in the native ABI. Lineage, exit causes and supervision live in the supervising Component (SVC), which receives typed exit causes (CMP panic and abort semantics) over its Channel.

## Consequences
- os inspect names Components by handle and by the supervisor-assigned name, never by a number.
- The Linux personality synthesises PIDs and process trees for its guests (LNX) without kernel help.
- Debuggers attach through a debug Capability on the Component handle (CAP).

## Rejected options and why
- Option B (kernel-visible lineage) rejected: it reintroduces a process tree the native model does not need and every tool would come to depend on.
- Option C (opaque identity with supervisor-held lineage) rejected as a distinction without a difference: handles are already opaque, and the supervisor already holds lineage under Option A.

## Follow-ups
none
