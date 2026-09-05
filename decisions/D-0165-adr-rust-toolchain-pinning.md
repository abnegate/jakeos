# D-0165 · Decide kernel Rust toolchain pinning relative to the Rust-for-Linux minimum
- Status: accepted
- Task: KRN-004
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The fork and CI need a Rust toolchain policy relative to the Rust-for-Linux minimum (§50), answering Q-051.

## Options

### Option A · Track upstream's minimum
Summary: The fork uses upstream Linux's minimum Rust version.
Consequences: Alignment with upstream patches; slow access to new language features.
Evidence: none

### Option B · Pin independently
Summary: The fork pins its own toolchain.
Consequences: Freedom to use new features; drift from upstream and porting cost at merges.
Evidence: none

### Option C · Pin with a bounded lag
Summary: The fork may lag upstream by a bounded number of releases.
Consequences: Balance; a policy to enforce in CI.
Evidence: none

## Decision
Option C. The kernel Rust toolchain (and the matching LLVM per D-0036) is pinned per release in a single toolchain file, and the pin may lag the Rust-for-Linux minimum of the merged upstream tag by at most two Rust releases. A mainline merge that would exceed the lag bumps the pin in the same pull request. This answers Q-051: the fork tracks upstream's minimum with a bounded lag rather than exactly or independently.

## Consequences
- Builds are reproducible from the pinned toolchain; the platform repository pins the same Rust release for the SDK.
- Unstable Rust features used in the kernel are inventoried so a pin bump cannot silently break them.
- The pin file is the single source read by CI, the pre-merge lint and the reproducible-build verifier.

## Rejected options and why
- Option A (track upstream's minimum exactly) rejected: every mainline merge could force a toolchain bump mid-work with no room to stage it.
- Option B (pin independently) rejected: newly merged mainline Rust code may require a newer compiler than the pin allows, blocking merges.

## Follow-ups
Q-051 answered by this Decision.
