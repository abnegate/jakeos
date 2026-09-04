# D-0331 · Decide guest-window integration depth and agent protocol
- Status: proposed
- Task: VIRT-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §40, §49
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Seamless mode fixes the guest-agent protocol V2 guest tools ship against (§40, §49).

## Options

### Option A · One virtio-gpu Surface per VM
Summary: One Surface.
Consequences: Simple; a VM in a window.
Evidence: none

### Option B · Per-application guest windows as native Surfaces with bridging
Summary: Seamless windows.
Consequences: Native feel; a protocol.
Evidence: none

### Option C · Single-display default with opt-in seamless
Summary: Opt-in seamless.
Consequences: Balance; two modes.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
