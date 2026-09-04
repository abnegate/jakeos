# D-0085 · Decide whether RDP/VNC clients and a remote-desktop server are in 1.0 scope
- Status: proposed
- Task: GFX-089
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §40, §43, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether RDP/VNC clients and a remote-desktop server are in 1.0 scope must precede the V4 feature freeze (§40, §43, §57), with distribution not a kernel concern (I-047).

## Options

### Option A · Out of scope for 1.0
Summary: No remote desktop in 1.0.
Consequences: Focus; follow-ups stay in LATER.
Evidence: none

### Option B · Clients only via the Linux personality
Summary: Existing clients run in the personality; no server.
Consequences: Users can reach other machines; nothing native.
Evidence: none

### Option C · Native server over remote Surfaces
Summary: A userspace server exports Surfaces.
Consequences: Native remote access; significant work before feature freeze.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
