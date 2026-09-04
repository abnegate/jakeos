# D-0018 · Decide the assistive-technology protocol: AT-SPI compatible, native, or both
- Status: proposed
- Task: ACC-001
- Surfaces: none
- Layer: none
- Spikes: ACC-004
- Supersedes: none
- Superseded by: none
- Baseline: §9, §41, §42, §57, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The transport and client-facing protocol for assistive technology must be fixed before the V1 UI protocol freeze candidate so S-017 is not an incompatible surprise on S-015 (§9, §41, §42, §57, §65).

## Options

### Option A · Native typed Channel only
Summary: AT clients use a native typed Channel over IDL.
Consequences: One protocol with Capability semantics from the start; existing Linux AT tooling cannot connect natively.
Evidence: none

### Option B · AT-SPI2 export of the native tree
Summary: The native tree is exported over AT-SPI2.
Consequences: Existing screen readers work immediately; an ambient bus becomes the native path, which §9 rejects.
Evidence: none

### Option C · Both, AT-SPI2 confined to the Linux personality
Summary: Native clients use the typed Channel and AT-SPI2 exists only inside the Linux personality.
Consequences: Native purity and Linux compatibility; two protocols to keep in step.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
