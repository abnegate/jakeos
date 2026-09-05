# D-0037 · Decide repository topology before a second repository exists
- Status: accepted
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
Option C. Two code repositories plus this roadmap: github.com/abnegate/jakeos-kernel holds the Linux fork with full upstream history; github.com/abnegate/jakeos-platform is a monorepo for everything above the kernel ABI (runtime, IDL compiler, SDK, compositor, shell, applications, personalities, installer, benchmark harnesses, documentation). The roadmap stays standalone at github.com/abnegate/jakeos. The registers/repos.md aliases kernel, platform and roadmap are the evidence targets; the earlier per-component aliases resolve to paths inside jakeos-platform.

## Consequences
- One CI pipeline and one licence (MIT) for the platform; one pipeline and GPLv2 for the kernel.
- Cross-repo changes (an ABI change with its SDK binding) are two pull requests linked by the Roadmap: trailer; the roadmap task is done only when both land.
- Kernel clones are large; CI uses shallow clones and the platform repository never vendors kernel sources.

## Rejected options and why
- Option A (single monorepo including the kernel) rejected: kernel history and upstream merges would dominate every clone and CI run of userspace work.
- Option B (pinned-manifest multi-repo) rejected: for a small team, a manifest across a dozen repositories is coordination overhead with no isolation benefit that the two-repo split does not already provide.

## Follow-ups
Rewrite registers/repos.md aliases to kernel, platform and roadmap with per-component paths (GOV).
