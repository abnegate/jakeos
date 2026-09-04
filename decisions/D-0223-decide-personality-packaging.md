# D-0223 · Decide how Linux and Windows compatibility applications are packaged immutably
- Status: proposed
- Task: PKG-047
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §28, §36
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How Linux and Windows compatibility applications are packaged immutably must be decided for the V1 L2 corpus (§3, §28, §36), answering Q-020.

## Options

### Option A · OCI layers as content-addressed objects
Summary: OCI layers become objects.
Consequences: Reuse; layer mapping.
Evidence: none

### Option B · Flatpak bundles wrapped as Packages
Summary: Flatpaks become Packages.
Consequences: Sandboxed; wrapping.
Evidence: none

### Option C · Wine prefixes as immutable base layers plus ApplicationData overlays
Summary: Prefixes as layers.
Consequences: Wine fit; overlay complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
