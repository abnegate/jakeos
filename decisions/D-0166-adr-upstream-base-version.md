# D-0166 · Decide the upstream Linux tree and LTS series the fork is cut from
- Status: accepted
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
Option A. The fork is cut from the most recent mainline release tag, v7.2 at the time of this Decision. Release candidates are never bases. Each subsequent mainline release tag (v7.3, v7.4, and so on) is merged as it is released (see D-0168), so the fork always sits on the newest Rust-for-Linux APIs and drivers.

## Consequences
- The fork carries no LTS backport burden; security fixes arrive with each mainline merge and, between releases, from the matching stable branch (7.2.y) merged as point releases.
- Every mainline merge may raise the minimum Rust version; KRN-004 bounds how far the pinned toolchain may lag.
- The divergence policy (KRN divergence ledger) measures merge cost per release; if two consecutive merges exceed the budget recorded there, this Decision is revisited in favour of an LTS series.

## Rejected options and why
- Option B (named LTS series) rejected: the native model needs the newest Rust-for-Linux abstractions, and an LTS base would freeze them for years while the fork is still shaped by upstream.
- Option C (non-LTS stable branch) rejected: same short support window as mainline with none of the freshness.

## Follow-ups
KRN-007 (accepted). Revisit when two consecutive mainline merges exceed the divergence budget.
