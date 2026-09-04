# D-0084 · Decide the proprietary GPU kernel driver policy
- Status: proposed
- Task: GFX-047
- Surfaces: none
- Layer: none
- Spikes: GFX-035
- Supersedes: none
- Superseded by: none
- Baseline: §51, §56.1, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
GPL-only symbol exposure, tainting and support commitments for proprietary GPU kernel modules versus open modules, Nouveau and NVK must be decided (§51, §56.1, §57), coordinated with HW's NVIDIA Secure Boot Decision.

## Options

### Option A · Open kernel modules plus NVK
Summary: Only open-source GPU kernel modules with NVK and Mesa are supported.
Consequences: Clean licensing and a supportable stack; NVIDIA users depend on the maturity of the open modules and NVK.
Evidence: none

### Option B · Proprietary module tolerated with taint and no support
Summary: Proprietary modules may load, taint the kernel and receive no project support.
Consequences: Users keep their choice; GPL-only symbol exports must be settled and every tainted bug report is closed unsupported.
Evidence: none

### Option C · Unsupported
Summary: Proprietary GPU kernel modules are refused.
Consequences: Simplest policy and no taint handling; a large share of desktops is excluded.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
