# D-0067 · Decide how Personality processes map onto Components
- Status: proposed
- Task: CMP-036
- Surfaces: none
- Layer: none
- Spikes: LNX-060
- Supersedes: none
- Superseded by: none
- Baseline: §3, §10, §46, §48
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 daily-driving through the Linux personality and Wine bring-up need a decided mapping of Linux and Windows processes onto Components (§3, §10, §46, §48); the spike report is the evidence.

## Options

### Option A · One Component per process
Summary: Every personality process is a Component.
Consequences: Strong isolation and clear inspect; process-heavy workloads pay Component creation cost.
Evidence: none

### Option B · One Component per process tree
Summary: A process tree shares one Component.
Consequences: Cheaper forks; isolation between processes in a tree is Linux-shaped only.
Evidence: none

### Option C · Personality ResourceDomain hosting plain tasks
Summary: One ResourceDomain hosts personality processes as plain tasks.
Consequences: Cheapest; PID and exit status surface only through the personality.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
