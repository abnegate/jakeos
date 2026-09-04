# D-0035 · Decide linker, LTO scope and PGO policy for kernel and userspace
- Status: proposed
- Task: BLD-039
- Surfaces: none
- Layer: none
- Spikes: BLD-051
- Supersedes: none
- Superseded by: none
- Baseline: §27, §50, §54
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Linker, LTO scope and PGO policy affect size, startup and reproducibility, and PGO threatens bit-for-bit SystemGeneration identity (§27, §50, §54); the spike report is required evidence.

## Options

### Option A · lld with thin LTO, PGO refused where identity breaks
Summary: lld links everything, thin LTO is default, and PGO is refused for profiles that must be bit-for-bit identical.
Consequences: Reproducible builds and reasonable link times; some peak performance left unused.
Evidence: none

### Option B · mold with full LTO and PGO on
Summary: mold links, full LTO everywhere, PGO enabled.
Consequences: Fastest binaries and links; PGO profiles break identity and full LTO lengthens kernel builds.
Evidence: none

### Option C · Status-quo linker with LTO off
Summary: The default linker with no LTO or PGO.
Consequences: Nothing to tune; larger, slower binaries.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
