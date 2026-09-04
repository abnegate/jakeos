# D-0304 · Decide whether to retain chrony or build a native NTP/NTS client
- Status: proposed
- Task: SVC-018
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
TLS, signatures and deadlines fail with a wrong clock, so the V1 time-sync client must be chosen.

## Options

### Option A · chrony hosted as a supervised personality service
Summary: chrony runs in the personality under supervision.
Consequences: Mature and accurate; a personality-hosted service on a native path.
Evidence: none

### Option B · Native Rust NTS client Component
Summary: A native NTS client.
Consequences: Native and NTS by default; implementation effort.
Evidence: none

### Option C · systemd-timesyncd-class minimal client
Summary: A minimal SNTP client.
Consequences: Small; fewer features and no NTS.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
