# D-0264 · Decide that Rust std lives only as a Layer 3 crate
- Status: proposed
- Task: SDK-028
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §52, §65, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
std filesystem, net and process APIs must not justify Layer 1 POSIX shapes (§3, §52, §65, §66, I-013, I-026).

## Options

### Option A · Layer 3 facade over the SDK
Summary: std is implemented as a facade over the SDK.
Consequences: Ecosystem crates compile; POSIX shapes leak through the facade.
Evidence: none

### Option B · No std on the native target
Summary: The native target has no std.
Consequences: Purity; most of the Rust ecosystem is unavailable.
Evidence: none

### Option C · Allowlisted std modules with no new L1 entries
Summary: Only allowlisted modules exist.
Consequences: Balance; an allowlist to maintain.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
