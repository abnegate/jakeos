# D-0200 · Decide whether to ship a native typed remote shell after 1.0
- Status: proposed
- Task: NET-040
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §43, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
sshd via the personality is V1; a native typed remote shell is post-1.0 and stays in userspace (§43, §57, I-047).

## Options

### Option A · Keep sshd in the Linux personality only
Summary: No native remote shell.
Consequences: No work; remote access is Linux-shaped.
Evidence: none

### Option B · Channel-based remote shell honouring Capabilities
Summary: A native shell over Channels.
Consequences: Capability-honouring remote access; a new protocol.
Evidence: none

### Option C · SSH-compatible native daemon
Summary: A native daemon speaking SSH.
Consequences: Client compatibility; a large protocol surface.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
