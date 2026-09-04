# D-0011 · Decide whether Layer 2 Interface stability applies at V1 or only at 1.0
- Status: proposed
- Task: ABI-037
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §66, §12
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
SDK v1 developers must know whether Layer 2 interfaces are stability-constrained at V1 or only at 1.0 (§66, §12); IPC freezes the evolution rules and this Decision sets the timing developers are told.

## Options

### Option A · Stability from V1
Summary: Layer 2 core interfaces are stable from V1 in the Wayland style.
Consequences: Early confidence for SDK v1 applications; interface mistakes found between V1 and 1.0 cannot be fixed without versioning.
Evidence: none

### Option B · Stability only at 1.0
Summary: Layer 2 may break freely until 1.0.
Consequences: Maximum freedom to fix interfaces; SDK v1 applications may need rework before 1.0.
Evidence: none

### Option C · Evolution rules at V1, versions unlocked until V4
Summary: The evolution rules freeze at V1 while individual interface versions may still bump until V4.
Consequences: Developers know how breakage will happen even when it does; applications must implement version negotiation from day one.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
