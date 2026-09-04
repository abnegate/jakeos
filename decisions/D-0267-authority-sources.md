# D-0267 · Decide authority sources and precedence
- Status: proposed
- Task: SEC-004
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§9.1 needs a single precedence among manifest request, launcher policy, user chooser, permissions UI and delegation before Packages request Capabilities.

## Options

### Option A · User chooser always wins over manifest and launcher
Summary: The user's explicit choice is final.
Consequences: User sovereignty; policy cannot forbid a choice.
Evidence: none

### Option B · Launcher policy can deny a chooser grant
Summary: Launcher policy overrides user choice.
Consequences: Enterprise-style control; user frustration and a policy surface.
Evidence: none

### Option C · Manifest request is sufficient without a user step
Summary: A manifest request grants.
Consequences: Convenient; ambient authority returns.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
