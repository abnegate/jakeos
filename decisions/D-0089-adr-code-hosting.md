# D-0089 · Decide code hosting forge and repository layout
- Status: accepted
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
Option B. Code is hosted on GitHub under the abnegate account: the roadmap at github.com/abnegate/jakeos, the kernel fork at github.com/abnegate/jakeos-kernel with full upstream history, and the userspace monorepo at github.com/abnegate/jakeos-platform (see D-0037). Repositories are public. GitHub Actions run every required check and GitHub Pages publishes the roadmap dashboard.

## Consequences
- registers/repos.md aliases resolve to github.com/abnegate/*; evidence lines use those aliases.
- Branch protection requires the validate check on main; pull requests are the only path to main for non-owners.
- Moving to a self-hosted forge later is a new GOV Decision and a repository migration, not a rewrite.

## Rejected options and why
- Option A (self-hosted forge) rejected for now: operating a forge, CI runners and Pages equivalents is a standing cost with no benefit before external contributors exist.
- Option C (hybrid) rejected: two hosts doubles account, CI and permission surfaces for a one-person team.

## Follow-ups
BLD-005 (accepted). Create github.com/abnegate/jakeos-kernel and github.com/abnegate/jakeos-platform when their first tasks start.
