# D-0171 · Decide the Linux Personality container engine Surface
- Status: proposed
- Task: LNX-012
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §36, §56.3
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Which container engine surface the Personality exposes must be decided so the L2 corpus can require OCI without making containers native (§36, §56.3), answering Q-032.

## Options

### Option A · Docker socket API
Summary: A Docker-compatible socket is exposed.
Consequences: Maximum tooling compatibility; a daemon inside the personality.
Evidence: none

### Option B · podman-compatible tooling without a Docker socket
Summary: podman is the engine.
Consequences: Daemonless and rootless; some tools expecting the socket break.
Evidence: none

### Option C · containerd with a thin CLI
Summary: containerd is the engine.
Consequences: Minimal; less developer tooling.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
