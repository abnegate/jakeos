# D-0172 · Decide the default Capability bundle for Linux apps
- Status: proposed
- Task: LNX-013
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §46, §47
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Unmanifested Linux apps need a default directory, network and device bundle plus portal upgrades (§9.1, §46, §47), answering Q-039.

## Options

### Option A · Narrow bundle upgraded only via portals
Summary: Minimal defaults, upgraded through portals.
Consequences: Safe by default; some unported apps break.
Evidence: none

### Option B · Home-and-network bundle with user-visible grants
Summary: Broader defaults shown to the user.
Consequences: Better compatibility; larger authority per app.
Evidence: none

### Option C · Ambient-home bundle
Summary: Full home access by default.
Consequences: Everything works; rejected against I-021.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
