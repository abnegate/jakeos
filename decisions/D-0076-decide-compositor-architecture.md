# D-0076 · Decide compositor architecture: monolithic or split display/scene/input
- Status: proposed
- Task: GFX-012
- Surfaces: S-024
- Layer: L2
- Spikes: GFX-033
- Supersedes: none
- Superseded by: none
- Baseline: §32, §40
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The compositor's process split determines crash recovery and Input-to-photon latency and must be stable before clients exist (§32, §40); the spike report is the evidence.

## Options

### Option A · Monolithic display server plus shell
Summary: One process holds DRM master, scene and shell.
Consequences: Lowest latency and simplest; a shell crash takes the display.
Evidence: none

### Option B · Split display server / window manager / shell
Summary: Three processes with typed Channels between them.
Consequences: Each crash is contained; extra hops on the input path.
Evidence: none

### Option C · Display-plus-input core with out-of-process shell
Summary: A small core owns DRM and input; the shell is separate.
Consequences: Shell crashes are survivable with one extra hop; the core is still a single point of failure.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
