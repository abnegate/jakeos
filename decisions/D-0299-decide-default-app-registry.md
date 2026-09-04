# D-0299 · Decide the default-application registry and open-by-Capability model
- Status: proposed
- Task: SVC-017
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Daily-driving needs a default handler for typed kinds and URL schemes that still mints a Capability to only that object.

## Options

### Option A · User-chosen default per kind or scheme with Package handler candidates
Summary: A user default.
Consequences: Capability-scoped; a registry.
Evidence: none

### Option B · Shared MIME/xdg-data database
Summary: A MIME database.
Consequences: Compatible; rejected as ambient.
Evidence: none

### Option C · Always-ask chooser with remembered last choice
Summary: Always ask.
Consequences: Safe; friction.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
