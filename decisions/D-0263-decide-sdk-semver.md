# D-0263 · Decide the Layer 3 SDK semver and deprecation policy
- Status: proposed
- Task: SDK-054
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §52, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V1 gate requires an accepted SDK stability policy. Layer 3 (the SDK crates) evolves by semantic versioning with deprecation windows rather than by a Layer 1 freeze (§52, §66, I-040, R-028). This decision fixes the policy, names S-031 as a V1 freeze candidate that is not frozen, and sits on the SDK v1 candidate (SDK-018) and ABI's layering decisions (ABI-037, ABI-039).

## Options

### Option A · Semver with recorded deprecation windows
Summary: Semantic versioning per crate with a recorded deprecation window (deprecated items remain for two minor releases, removal only at a major), and a compatibility lint in CI.
Consequences: Third-party code has a predictable upgrade path and the SDK can still fix mistakes. Every release must honour the window, deprecations accumulate until a major, and the lint (cargo-semver-checks class) is a required check.
Evidence: none

### Option B · Lockstep with Layer 2 interface versions
Summary: SDK crate versions move in lockstep with the Layer 2 interface versions they wrap.
Consequences: One version number to explain and interface and SDK changes ship together. Unrelated changes are coupled, a pure-Rust ergonomics change forces an interface version, and crates that wrap several interfaces have no single lockstep partner.
Evidence: none

### Option C · Freeze Layer 3 at V1
Summary: Layer 3 freezes at V1.
Consequences: Maximum stability for early adopters. It freezes an SDK with one rung of use behind it, contradicts R-028 and I-040 and would force mistakes into 1.0; listed to be rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
