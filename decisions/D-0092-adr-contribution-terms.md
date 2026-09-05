# D-0092 · Decide contributor licensing, copyright holder and DCO or CLA
- Status: accepted
- Task: GOV-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Provenance cannot be reconstructed after the first external commit, so contributor licensing, copyright holder and DCO versus CLA must be recorded so BLD can enforce headers and sign-off from V0 (§50, §57).

## Options

### Option A · DCO-only on all trees
Summary: Every commit carries Signed-off-by and contributors keep their copyright.
Consequences: Lowest contributor friction and identical to kernel practice; the project can never relicense without tracking down every contributor.
Evidence: none

### Option B · DCO on the kernel and a non-assignment CLA on userspace
Summary: Kernel commits use DCO while userspace contributors sign a CLA that grants patent rights without assigning copyright.
Consequences: Explicit patent grants where permissive licenses need them; two onboarding processes and a CLA bot to run.
Evidence: none

### Option C · Assignment CLA on all trees
Summary: Contributors assign copyright to the project entity.
Consequences: The entity can relicense freely; assignment deters contributors and conflicts with kernel norms and the GPLv2 fork.
Evidence: none

## Decision
Option B. Kernel contributions use the Developer Certificate of Origin (Signed-off-by), matching Linux. Contributions to the MIT userspace repositories additionally require a non-assignment contributor licence agreement granting the project a perpetual licence to the contribution under the project licences and any future OSI-approved licence, while the contributor keeps copyright.

## Consequences
- The CLA is a one-time signature recorded by a bot on the platform repository; the kernel repository only checks Signed-off-by.
- Relicensing the platform layers later is possible without hunting every contributor; the kernel remains GPLv2-only forever, which the DCO suits.
- GOV publishes both texts and the bot configuration before the first external pull request.

## Rejected options and why
- Option A (DCO-only everywhere) rejected: it would leave the MIT layers unable to adopt a future licence without unanimous consent.
- Option C (assignment CLA everywhere) rejected: copyright assignment on a GPLv2 kernel fork is unusual and deters the driver contributors upstream-first depends on.

## Follow-ups
none
