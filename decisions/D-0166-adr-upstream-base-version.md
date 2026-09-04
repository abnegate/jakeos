# D-0166 · Decide the upstream Linux tree and LTS series the fork is cut from
- Status: proposed
- Task: KRN-005
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §5.1, §6
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Nothing in KRN can start before the base is chosen; mainline versus LTS and the specific series must be selected (§5.1, §6).

## Options

### Option A · Named current mainline tag
Summary: The fork is cut from a named mainline tag.
Consequences: Newest hardware support and Rust features; rapid churn at every merge.
Evidence: none

### Option B · Named LTS series
Summary: The fork is cut from a named LTS series.
Consequences: Stability and long backport support; older features and drivers.
Evidence: none

### Option C · Named stable branch that is not LTS
Summary: The fork is cut from a stable branch.
Consequences: Middle ground; a short support life forces an early rebase.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
