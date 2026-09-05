# D-0168 · Decide upstream tracking: rebase vs merge and cadence per divergence phase
- Status: accepted
- Task: KRN-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §6, §56.4
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Rebase versus merge and cadence for phases A through C must be fixed for the merge bot and V1 rebase gate (§6, §56.4).

## Options

### Option A · Periodic rebase onto the chosen series
Summary: The fork rebases regularly.
Consequences: Clean patch series and easy delta reports; conflict pain concentrated at each rebase.
Evidence: none

### Option B · Merge each upstream tag
Summary: Upstream tags are merged.
Consequences: Simple and continuous; a messy history that hides the delta.
Evidence: none

### Option C · Rebase for LTS plus merge for mainline
Summary: Hybrid per series.
Consequences: Best of both; two workflows for the merge bot.
Evidence: none

## Decision
Option B. The fork merges each upstream mainline release tag (and the matching stable point releases between them) into its main branch as merge commits. History is never rewritten. Fork-only patches are kept mergeable by living in their own directories and behind clearly named hooks in shared files, so conflicts are confined and recorded in the divergence ledger. From phase D onward, merges may be replaced by selective adaptation per D-0166 revisit conditions.

## Consequences
- Fork history is preserved; git log shows exactly when each upstream release entered the tree.
- Sending patches upstream uses the upstream-candidate series from the divergence ledger, re-based for submission only, never on main.
- Merge conflicts are a tracked KRN metric; each merge produces a ledger entry with conflict count and resolution time.

## Rejected options and why
- Option A (periodic rebase) rejected by the maintainer as unacceptable history rewriting of a public repository that other repositories pin by commit.
- Option C (rebase for LTS, merge for mainline) rejected: with a mainline base (D-0166) there is only one track.

## Follow-ups
none
