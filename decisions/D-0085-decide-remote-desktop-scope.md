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
Distributed interfaces are not a kernel concern (§43, §57, I-047), but whether 1.0 ships remote-desktop clients or a server over remote Surfaces (GFX-092) is a product question that must precede the V4 feature freeze, so it moved from V4 to V3 (§40). The accepted option lists the follow-up tasks that stay in LATER if 1.0 is out of scope.

## Options

### Option A · Out of scope for 1.0
Summary: No remote desktop in 1.0; clients and server are LATER.
Consequences: Focus for V4 and no new network-facing service before the audit. Remote support and headless use cases are unaddressed at 1.0, and the 1.0 non-promises must say so.
Evidence: none

### Option B · Clients only via the Linux personality
Summary: Existing RDP and VNC clients run through the Linux personality; no server ships.
Consequences: Users reach other machines with familiar tools at no native cost. Nothing native and no way to reach a JakeOS machine remotely, which limits the lab's own remote-console story to serial and capture.
Evidence: none

### Option C · Native server over remote Surfaces
Summary: A native user-space server exports Surfaces over remote Surfaces with a Capability per exported window and a session grant.
Consequences: Native remote access under the grant model and the first consumer of remote Surfaces. A network-facing service with a large attack surface must be built and audited before V4 feature freeze; the compositor's remote path (GFX-092) is on its critical path.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
