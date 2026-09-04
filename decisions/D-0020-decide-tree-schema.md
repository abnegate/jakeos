# D-0020 · Decide the accessibility tree model shared by the toolkit and semantic interfaces
- Status: proposed
- Task: ACC-002
- Surfaces: S-017
- Layer: none
- Spikes: ACC-004
- Supersedes: none
- Superseded by: none
- Baseline: §41, §42, §60, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The accessibility tree contract shared by toolkit and semantic interfaces must be fixed before V0.5 metadata emission and the UI protocol Layer 2 freeze candidate (§41, §42, §60, §65), with AccessKit the leading candidate measured by ACC-004.

## Options

### Option A · Adopt AccessKit
Summary: AccessKit's schema is the accessibility tree model.
Consequences: Existing platform adapters and tooling apply; the project depends on an external schema's evolution.
Evidence: none

### Option B · Native schema with AccessKit export
Summary: A native schema is defined and exported to AccessKit.
Consequences: Native concepts fit exactly and interoperability is retained; two schemas must be kept mappable.
Evidence: none

### Option C · Native only
Summary: A native schema with no AccessKit relationship.
Consequences: Full control; no reuse of existing AT tooling and a longer path to a working screen reader.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
