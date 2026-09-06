# D-0225 · Decide which state classes are restorable at each Milestone and in scope for 1.0
- Status: proposed
- Task: PKG-048
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§31 lists state classes (OS, applications, packages, configuration, user data, workspaces, application state) that history can restore. This decision fixes which classes are restorable at V1, V2, V3, V4 and 1.0 and which are non-goals, so the V2 restore UI and the 1.0 rollback guarantee promise only what exists; application state is marked pending PKG-069 rather than silently included. It sits on the generation boundary (D-0216) and the history model (PKG-022).

## Options

### Option A · OS and Packages only through 1.0
Summary: Only the OS and installed Packages are restorable through 1.0; configuration, user data and application state are non-goals for restore.
Consequences: Achievable with the generation mechanism alone and the rollback guarantee is exactly the generation switch. Configuration changes are not undoable, which makes the settings service's history events (D-0303) purposeless, and the §31 story shrinks to what every immutable distribution already offers.
Evidence: none

### Option B · Adding configuration at V3
Summary: OS and Packages at V1, configuration (settings service objects) at V3, user data and workspaces via snapshots at V4, application state pending PKG-069.
Consequences: Each class arrives with the mechanism that makes it restorable and the 1.0 guarantee covers configuration, which users notice most. Four restore paths must present as one `os restore` experience, and the V2 restore UI ships with a subset that later grows.
Evidence: none

### Option C · Every §31 class into 1.0
Summary: Every §31 class including application state is restorable at 1.0.
Consequences: The full §31 promise. Application state restore requires every application to opt in through the SDK (Q-056, SDK-093), which the project cannot guarantee for third parties, so the gate would be failed by software the project does not control.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
