# D-0071 · Decide the documentation toolchain, search and snapshots
- Status: proposed
- Task: DOC-009
- Surfaces: none
- Layer: none
- Spikes: DOC-002
- Supersedes: none
- Superseded by: none
- Baseline: §12, §52, §56.5
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V1 IDL-to-docs site needs a recorded generator, site builder, search and per-release snapshot scheme before those builds start (§12, §52, §56.5); the spike report is an input.

## Options

### Option A · Static site generator plus custom IDL-to-pages compiler
Summary: A static site with a project compiler from IDL, client-side search and versioned snapshot trees.
Consequences: Full control and no server; search scales poorly with corpus size.
Evidence: none

### Option B · Unified rustdoc or Sphinx pipeline
Summary: rustdoc or Sphinx renders everything with server-side search and tagged snapshot aliases.
Consequences: Mature tooling; IDL pages must be bent into a tool made for another language.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
