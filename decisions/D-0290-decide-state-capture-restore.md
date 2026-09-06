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
V2 exit restores the OS, Packages and configuration from the UI (§31). D-0303 chose the settings storage model; this decision fixes how configuration and application state become versioned restorable objects in history and what `os restore` reverts for configuration versus user files. It answers Q-025 and sits on storage transactions (STO-002) and the settings service (SVC-006).

## Options

### Option A · Structured settings store
Summary: Configuration lives only in the structured settings service; every write is a versioned object and a history event; `os restore` reverts settings objects to a point in history and leaves user files alone.
Consequences: Precise, per-setting restore with schema migration handled by the service, and user documents are never touched by a configuration restore. Applications that keep state outside the settings service (caches, databases in ApplicationData) are not covered, and every application must adopt the service for its restore to work.
Evidence: none

### Option B · Snapshotting ApplicationData
Summary: ApplicationData directories are snapshotted with each generation and `os restore` restores the snapshot.
Consequences: Covers everything an application writes without adoption. Coarse: a restore rolls back documents, caches and settings together, snapshots of live databases may be inconsistent, and disk use grows with application data.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
