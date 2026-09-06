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
Remote administration at V1 is sshd inside the Linux personality (NET-017). Whether a native typed remote shell ships after 1.0 is parked at LATER (§43, §57): the implementation stays in user space (I-047) and listening remains an explicit Capability (T-040). This decision names the direction so post-1.0 NET work has a recorded starting point.

## Options

### Option A · Keep sshd in the Linux personality only
Summary: sshd in the personality remains the only remote shell.
Consequences: No new work and every SSH client works. Remote access is Linux-shaped: it gives a personality shell with personality authority, not typed access to native Objects.
Evidence: none

### Option B · Channel-based remote shell honouring Capabilities
Summary: A native remote shell over an encrypted Channel transport that carries Capabilities: the remote session is a Component with exactly the grants the user delegated.
Consequences: Remote access honours the capability model and `os inspect` shows what a remote session may do. A new protocol and client, and it depends on the remote-Interface rules of D-0274.
Evidence: none

### Option C · SSH-compatible native daemon
Summary: A native daemon speaking the SSH protocol that maps sessions onto native Components.
Consequences: Existing clients connect to a native session. SSH's authority model (a user account) must be mapped onto Capabilities, and the protocol surface is large and security-critical to reimplement.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
