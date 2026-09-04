# D-0073 · Decide whether DevelopmentEnvironment is kernel or userspace
- Status: proposed
- Task: ENV-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §35, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 composes ResourceDomain, StorageSnapshot, CapabilityNamespace, NetworkNamespace and Components into development isolation (§35, §65); the composition must be placed in userspace, a Package profile or a new Layer 1 object.

## Options

### Option A · Userspace supervisor Component
Summary: A supervisor Component composes existing primitives.
Consequences: No new kernel surface; os inspect shows a userspace object.
Evidence: none

### Option B · Package-profile instantiation with no extra object
Summary: The environment is a Package manifest profile instantiated by existing machinery.
Consequences: Nothing new to build or inspect; environment-specific operations have no object to hang on.
Evidence: none

### Option C · New Layer 1 kernel object
Summary: DevelopmentEnvironment is a kernel object.
Consequences: First-class and enforceable; a new L1 surface needing a spike that cannot freeze before V4.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
