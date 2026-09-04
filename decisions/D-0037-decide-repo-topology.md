# D-0037 · Decide repository topology before a second repository exists
- Status: proposed
- Task: BLD-005
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50, §56.4
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every CI, bisection and promotion design depends on whether one commit identifies the whole system (§50, §56.4), so topology must be recorded before a second repository exists.

## Options

### Option A · Single monorepo
Summary: Kernel fork, native userspace, SDK and tooling live in one repository.
Consequences: One commit identifies the system; the kernel fork's upstream merges churn the whole tree.
Evidence: none

### Option B · Pinned-manifest multi-repo
Summary: Separate repositories tied together by a pinned manifest.
Consequences: Independent histories; system identity is the manifest commit and cross-repo changes are two steps.
Evidence: none

### Option C · Separate kernel repository with a userspace monorepo
Summary: The kernel fork is its own repository and everything else is one monorepo.
Consequences: Upstream merges stay isolated; two identities to pin for bisection.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
