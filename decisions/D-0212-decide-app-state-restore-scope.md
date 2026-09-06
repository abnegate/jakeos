# D-0212 · Decide whether application-state restore is a 1.0 goal or non-goal
- Status: proposed
- Task: PKG-069
- Surfaces: none
- Layer: none
- Spikes: PKG-079
- Supersedes: none
- Superseded by: none
- Baseline: §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§31 names application-state restore as a long-term ambition; V4 feature freeze needs its 1.0 scope settled. PKG-079's spike measures checkpointing and cooperative approaches; this decision answers Q-056. If the non-goal is accepted, SDK-093 (the 1.0 restore task) is dropped per its gate rule and the decision names the owning later work so 1.0 restore has no silent gap; it sits on the restorable-class ladder (D-0225).

## Options

### Option A · In-scope via checkpointing
Summary: Application state is restored by checkpointing Component memory and Objects transparently.
Consequences: Works for every application with no developer effort. GPU state, open network connections, Channels to other Components and hardware-committed Operations do not checkpoint, so the restored state is inconsistent for exactly the applications people use; the spike will show the failure set.
Evidence: `reports/spikes/PKG-079.md`

### Option B · In-scope via cooperative state interfaces
Summary: Applications implement a cooperative state Interface (save and restore typed state on request); only opted-in applications participate and `os restore` reports the rest.
Consequences: Feasible, honest about coverage, and the interface doubles as session restore (APP-056). First-party applications must implement it and third parties may not, so the 1.0 promise is "restore for applications that support it".
Evidence: `reports/spikes/PKG-079.md`

### Option C · Explicit 1.0 non-goal
Summary: Application-state restore is an explicit 1.0 non-goal; history restores OS, Packages, configuration and user data only.
Consequences: Focus for V4 and no half-working feature. The gap is named in the 1.0 non-promises and the cooperative interface becomes LATER work; SDK-093 is dropped.
Evidence: `reports/spikes/PKG-079.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
