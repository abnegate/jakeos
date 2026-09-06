# D-0218 · Decide that Package mutation is replaced by immutable Packages and SystemGenerations
- Status: proposed
- Task: PKG-009
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §2, §28, §30, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§2 replaces package mutation with immutable Packages and SystemGenerations (§28, §30): no in-place mutation of installed contents, no shared-filesystem writes on install, versioned generations and explicit history instead of an accumulation of mutable files (§67). This decision records those standing rules as I-022 and I-036 so the immutability gate can enforce them, and names what is rejected so no later task reintroduces mutation for convenience.

## Options

### Option A · Immutable Packages plus SystemGenerations
Summary: Installed Package contents are never mutated in place; every change is a new Package identity and a new SystemGeneration, and history is explicit.
Consequences: Rollback, reproducibility and content verification hold by construction, and two machines with the same generation identity run the same bytes. Anything that used to edit files under an installed prefix (post-install scripts, plugin drops, configuration under the package tree) must move to ApplicationData or to a new Package.
Evidence: none

### Option B · In-place Package mutation with snapshots
Summary: Packages are mutable trees and the system snapshots the filesystem before each install for rollback.
Consequences: Familiar to Linux packaging and personality software. Two machines drift after the same installs, verification is against a snapshot rather than an identity, and rollback restores whatever else happened to be on the volume; this is the model §2 rejects.
Evidence: none

### Option C · Hybrid writable overlay on immutable bases
Summary: Packages are immutable bases with a per-Package writable overlay for local modification.
Consequences: Local patches and plugins work without republishing. The overlay is unverified and unaccounted state that survives generation switches in undefined ways, and `os inspect` cannot tell a pristine Package from a modified one without diffing.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
