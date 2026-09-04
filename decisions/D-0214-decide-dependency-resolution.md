# D-0214 · Decide dependency resolution semantics and lockfile location
- Status: proposed
- Task: PKG-006
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §29, §53
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether manifests pin by content hash or version ranges and where lockfiles live must be settled before the install path (§29, §53); launch performs no resolution.

## Options

### Option A · Exact content-hash pins in the manifest
Summary: Manifests pin hashes.
Consequences: Deterministic; brittle updates.
Evidence: none

### Option B · Version ranges resolved at install into a lockfile next to the manifest
Summary: Ranges plus a lockfile.
Consequences: Flexible; lockfile management.
Evidence: none

### Option C · Version ranges resolved into a generation-level lock
Summary: Ranges plus a generation lock.
Consequences: System-wide consistency; coupling.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
