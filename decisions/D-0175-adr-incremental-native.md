# D-0175 · Decide incremental native-Interface adoption
- Status: proposed
- Task: LNX-062
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §42, §46
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether a Linux-personality app may adopt native file chooser, semantic interfaces and Capabilities incrementally without eroding the §3 firewall must be decided (§3, §42, §46).

## Options

### Option A · Explicit incremental bridge
Summary: A defined bridge lists which native interfaces a Linux app may call.
Consequences: Gradual adoption; a bridge to maintain.
Evidence: none

### Option B · All-or-nothing rewrite
Summary: Apps must be fully native or fully Linux.
Consequences: Firewall purity; no migration path.
Evidence: none

### Option C · Silent mixing of POSIX and native APIs
Summary: Apps mix freely in one Component.
Consequences: Easy; rejected against I-025.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
