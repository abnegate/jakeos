# D-0303 · Decide the settings storage model: typed schema-versioned objects with history events
- Status: proposed
- Task: SVC-006
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Without a Decision every V0.5 application invents a config file and §31 history cannot restore configuration.

## Options

### Option A · Typed schema-versioned objects in a settings service
Summary: Settings are typed, versioned objects in a service.
Consequences: Restorable history and Capability scoping; a service to run.
Evidence: none

### Option B · Per-application files in ApplicationData
Summary: Each app keeps files.
Consequences: Simple; no history or restore.
Evidence: none

### Option C · Registry-style store
Summary: A central registry.
Consequences: One place for everything; rejected by I-020 and PKG INV-0535.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
