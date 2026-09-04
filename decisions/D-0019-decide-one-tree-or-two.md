# D-0019 · Decide whether semantic actions and accessibility actions share one tree
- Status: proposed
- Task: ACC-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §42, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Q-035 asks whether semantic actions and accessibility actions are one tree or two and how they stay consistent (§41, §42, §65), before action dispatch and the V1 Layer 2 freeze candidates.

## Options

### Option A · Single tree with actions typed by the semantic registry
Summary: One tree carries both accessibility nodes and semantic verbs.
Consequences: No consistency problem by construction; the tree schema must serve two audiences and grows large.
Evidence: none

### Option B · Two trees with a consistency contract
Summary: Separate trees with a published contract that keeps them aligned.
Consequences: Each tree fits its audience; the contract must be tested and drift is a live risk.
Evidence: none

### Option C · Accessibility tree as a projection of the semantic tree
Summary: The accessibility tree is derived mechanically from the semantic tree.
Consequences: One source of truth; the projection must express every accessibility concept the semantic tree does not naturally carry.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
