# D-0347 · SDK v1 crate API surface
- Status: proposed
- Task: SDK-055
- Surfaces: S-031
- Layer: none
- Spikes: SDK-031
- Supersedes: none
- Superseded by: none
- Baseline: §50, §52, §66
- Revisit when: a V1 Gate cannot be met without adding a crate that the chosen surface omitted, or Layer 3 policy from SDK-054 is superseded

## Context
V1 names S-031 as a freeze candidate: the Rust SDK v1 crate API with semver (§52, §66). Layer 3 evolves; it is not a Layer 1 freeze. This Decision names which crates and which public items constitute that candidate, after SDK-031 reports what a daily-driving SDK actually exports.

## Options

### Option A · Named crate set as freeze candidate
Summary: Enumerate the v1 crates and their public modules; S-031 becomes a freeze candidate at V1 and freezes only when the freeze task lands.
Consequences: SDK authors have a concrete surface; adding a crate later is a recorded Decision; Layer 1 stays unfrozen.
Evidence: none

### Option B · Semver-only with no freeze candidate
Summary: Publish crates with semver and never list S-031 as a freeze candidate.
Consequences: V1 Gates that name an SDK v1 freeze candidate fail; third parties cannot tell which items are the v1 contract.
Evidence: none

### Option C · Freeze Layer 3 with Layer 1
Summary: Treat the SDK crate API as frozen on the same schedule as Layer 1.
Consequences: Layer 3 could not evolve with semver; I-040 and the Layer 3 rule in §66 are violated.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
SDK-057.
