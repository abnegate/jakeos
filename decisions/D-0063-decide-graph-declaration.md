# D-0063 · Decide static manifest graphs versus dynamic child instantiation
- Status: proposed
- Task: CMP-022
- Surfaces: S-019
- Layer: none
- Spikes: CMP-031
- Supersedes: none
- Superseded by: none
- Baseline: §11
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether component graphs are declared in the Package manifest, instantiated dynamically by a parent, or both with constraints must precede CMP-024 (§11), with S-019 prototyped.

## Options

### Option A · Static manifest graphs only
Summary: Every graph is declared in the manifest.
Consequences: Fully auditable before launch; no dynamic plugins or per-document workers.
Evidence: none

### Option B · Dynamic child instantiation only
Summary: Parents create children at runtime.
Consequences: Flexible; the graph is not knowable from the Package.
Evidence: none

### Option C · Static graphs plus constrained dynamic children
Summary: Manifest graphs with declared slots for dynamic children under attenuated Capabilities.
Consequences: Auditable shape with runtime flexibility; slot semantics to specify.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
