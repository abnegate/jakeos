# D-0290 · Decide how configuration and application state become versioned restorable objects
- Status: proposed
- Task: STO-056
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V2 exit restores OS, Packages and configuration from the UI, so how configuration becomes restorable objects must be decided (§31).

## Options

### Option A · Structured settings store
Summary: A structured store.
Consequences: Precise restore; migration.
Evidence: none

### Option B · Snapshotting ApplicationData
Summary: Snapshots of ApplicationData.
Consequences: General; coarse.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
