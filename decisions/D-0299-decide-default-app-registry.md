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
V0.5 opens objects only through the UserSelected chooser. Daily driving needs a default handler for a typed kind (D-0296) or URL scheme that still mints a Capability to exactly that object and nothing else. A shared path-based MIME database would reintroduce ambient filesystem authority (T-002). This decision fixes how kinds and schemes map to handler Packages, how a user's default overrides a Package's offer, and that open mints a single-object Capability; it sits on the service registry (SVC-013) and the shell (APP-002).

## Options

### Option A · User-chosen default per kind or scheme with Package handler candidates
Summary: Packages declare the kinds and schemes they handle in the manifest; the user picks one default per kind or scheme in Settings; `open` resolves the default and launches it with a Capability to that one object.
Consequences: Capability-scoped by construction and the handler list is store metadata. A registry of defaults is user state the settings service (D-0303) must hold and restore, and a kind with no default falls back to the chooser.
Evidence: none

### Option B · Shared MIME/xdg-data database
Summary: The shared MIME and xdg-data database of the Linux desktop, consulted by path and extension.
Consequences: Every Linux application's desktop file works unchanged. Handlers receive a path and open it themselves, which is path-based ambient authority (T-002); rejected for native software and confined to the personality's own launcher.
Evidence: none

### Option C · Always-ask chooser with remembered last choice
Summary: Every open shows the chooser, pre-selecting the last handler used for that kind.
Consequences: No registry and no ambient anything. One extra click on every open, which users of every other platform notice immediately; acceptable as the fallback of A, not as the model.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
