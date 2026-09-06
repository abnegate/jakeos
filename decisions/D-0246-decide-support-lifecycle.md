# D-0246 · Decide release cadence, LTS window and support lifecycle
- Status: proposed
- Task: REL-053
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §56.4, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The 1.0 support promise is this decision accepted (§56.4, §66): release cadence, the LTS window, which layers receive backports, the security-only phase and how end-of-life is communicated. The numbers for windows live in the GOV support contract (GOV-075) and the CVE SLA register entry, never in this prose (R-061); roadmap sources carry no calendar dates. It sits on the channel model (D-0248) and the versioning scheme (D-0240).

## Options

### Option A · Time-based releases with an LTS window and a security-only phase
Summary: Time-based releases on a fixed cadence; every Nth release is LTS with a support window and a trailing security-only phase; Layer 1 and Layer 2 receive backports, Layer 3 and applications move with releases; end-of-life is announced in the release notes of the successor and in `os update` status.
Consequences: Predictable for users, vendors and the HCL, and the 24-month support statement has a concrete shape. Backport work for two or more live branches is a standing REL cost, and a fixed cadence ships releases whether or not features are ready.
Evidence: none

### Option B · Feature-based releases with no LTS
Summary: Feature-based releases when the roadmap says a milestone is done; no LTS branch.
Consequences: Releases carry coherent feature sets. Users cannot plan, enterprises and vendors have no stable target, and the 1.0 support statement has nothing to attach to; rejected.
Evidence: none

### Option C · Rolling stable only
Summary: A single rolling stable channel with no versions.
Consequences: No branches and no backports. No LTS means no stability promise, and the roadmap's 1.0 support and CVE SLA commitments cannot be stated; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
