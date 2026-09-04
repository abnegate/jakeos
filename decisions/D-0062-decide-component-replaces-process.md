# D-0062 · Decide what replaces PID, parent/child, exit status and process groups
- Status: proposed
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
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
