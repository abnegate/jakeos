# D-0165 · Decide kernel Rust toolchain pinning relative to the Rust-for-Linux minimum
- Status: proposed
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
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
