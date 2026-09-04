# D-0089 · Decide code hosting forge and repository layout
- Status: proposed
- Task: GOV-001
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every V0 CI and review path needs a recorded home for git: which forge hosts the trees, whether the Markdown roadmap is a standalone repository, and how Evidence aliases resolve (§50, §65).

## Options

### Option A · Self-hosted forge
Summary: The project runs its own forge for roadmap and product trees.
Consequences: Full control and no vendor lock-in; operational load from day one.
Evidence: none

### Option B · Public hosted forge
Summary: A hosted forge holds everything.
Consequences: Zero operations and discoverability; dependence on a vendor's terms and tooling.
Evidence: none

### Option C · Hybrid with the roadmap standalone
Summary: The roadmap is a standalone repository and product trees may live elsewhere.
Consequences: Planning corpus stays independent; Evidence aliases must resolve across forges.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
