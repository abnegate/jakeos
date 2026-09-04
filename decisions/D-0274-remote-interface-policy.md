# D-0274 · Decide remote-Interface Capability, identity, and encryption rules
- Status: proposed
- Task: SEC-079
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §43, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Remote transports, when they exist, must honour Capabilities, identity, encryption and user policy without becoming a kernel concern (§43, §57).

## Options

### Option A · Capability-honouring encrypted transport with explicit user policy in userspace
Summary: A userspace transport.
Consequences: Correct model; later work.
Evidence: none

### Option B · Keep remote out until a later major version
Summary: No remote at all.
Consequences: Focus; no remote.
Evidence: none

### Option C · Kernel-mediated remote
Summary: The kernel handles remote.
Consequences: Performance; rejected under §57.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
