# D-0041 · Decide what 'boot succeeded' means and which Component may clear the boot counter
- Status: proposed
- Task: BOOT-020
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Marking boot success too early lets a broken desktop count as good and too late makes headless and recovery boots impossible (§30, §32); this picks the event and the Component authorized to clear the counter.

## Options

### Option A · Session manager reached greeter
Summary: The counter clears when the greeter is shown.
Consequences: Early and simple; a login screen over a broken session counts as success.
Evidence: none

### Option B · User authenticated
Summary: The counter clears after a successful login.
Consequences: Proves the desktop path works; headless machines never clear.
Evidence: none

### Option C · Per-boot-mode profile
Summary: Desktop, headless and recovery each have their own success event.
Consequences: Every mode has an honest rule; three rules to maintain.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
