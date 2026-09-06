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
Without one model every V0.5 application invents a configuration file, and §31 history cannot restore configuration because nothing records a change as an event. This decision picks how settings are stored, how a change becomes a system history event restorable by `os restore`, and that an application reads only its own Capability-scoped settings. Registry-style global configuration is already forbidden (I-020, PKG INV-0535).

## Options

### Option A · Typed schema-versioned objects in a settings service
Summary: A settings service stores typed, schema-versioned setting objects per application and per system area; a write is a Capability-scoped Operation that records a history event with the previous value.
Consequences: Restore, migration between schema versions and inspection come from one place, and an application cannot read another's settings. The service is on the path of every settings read at startup (cache in the SDK), every application declares a schema, and system settings the shell shows are the same objects.
Evidence: none

### Option B · Per-application files in ApplicationData
Summary: Each application keeps its own files in its ApplicationData directory in whatever format it chooses.
Consequences: Nothing to build and every framework's existing configuration code works. No history events, no restore, no schema migration, and the shell's system settings need a separate mechanism anyway.
Evidence: none

### Option C · Registry-style store
Summary: A registry-style hierarchical store with global keys any application can read.
Consequences: One place for everything and trivial discovery. Global readable keys are ambient authority and cross-application coupling, which I-020 and PKG INV-0535 already forbid; recorded as rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
