# D-0220 · Decide the Package manifest schema shape and its Layer 2 evolution rules
- Status: proposed
- Task: PKG-011
- Surfaces: S-018
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §11, §12, §28, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The Package manifest names identity, version, Components, Interfaces, RequestedCapabilities, Dependencies and Resources (§11, §28) and is a hard-to-change Layer 2 surface (S-018, §66). Capability requests must match the grant taxonomy and authority sources decided by SEC-007, SEC-004 and CAP-007, reserved signature and trust-policy fields must exist before the first install, multi-Component graphs need per-Component requirements and connections, and evolution follows the S-014 rules IPC-002 prototyped rather than a Package-only dialect (§12).

## Options

### Option A · Single typed manifest document
Summary: One typed manifest document describes the Package and every Component in it.
Consequences: One schema to learn, sign and validate, and one place `os inspect` reads. Component-level fields (Inputs, Outputs, Capabilities) are nested inside, so the document grows with the Component graph and S-019 (the Component manifest) has no independent existence.
Evidence: none

### Option B · Split Package-plus-Component manifest pair
Summary: A Package manifest (S-018) lists identity, Dependencies, Resources and its Components; each Component has its own manifest (S-019) with its Interfaces and RequestedCapabilities.
Consequences: Component manifests are reusable by SVC supervision and `os env` without the Package around them, and each surface evolves on its own schedule. Two schemas to sign and cross-validate, and a connection between Components references across documents.
Evidence: none

### Option C · IDL-defined manifest served as a Layer 2 interface
Summary: The manifest is an IDL-defined type served and validated through the ordinary Layer 2 interface machinery.
Consequences: Evolution, versioning and codegen come from S-014 for free and every language binding reads manifests natively. A manifest must be parsed before any IDL runtime exists at boot and by non-native tools (REL signing, store), so a canonical serialised form is needed anyway.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
