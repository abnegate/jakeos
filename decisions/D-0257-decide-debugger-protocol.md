# D-0257 · Decide DAP versus a native debugger protocol
- Status: proposed
- Task: SDK-052
- Surfaces: none
- Layer: none
- Spikes: SDK-060
- Supersedes: none
- Superseded by: none
- Baseline: §52, §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V1 debugger gate and editor integration need a protocol choice before the adapter (§52, §61); attach is a Capability.

## Options

### Option A · DAP
Summary: The Debug Adapter Protocol is used directly.
Consequences: Immediate editor support; thread-shaped model for Tasks.
Evidence: none

### Option B · Native protocol
Summary: A native debugger protocol is designed.
Consequences: Task-shaped; no editor support without adapters.
Evidence: none

### Option C · DAP over a native control Channel
Summary: DAP messages are carried over a Capability-gated Channel.
Consequences: Editor support with native transport; adaptation layer.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
