# D-0132 · Decide desktop-usable boot-success health criteria
- Status: proposed
- Task: INS-006
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §32, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A new SystemGeneration must be marked good only when the desktop is usable so automatic rollback is not a false success (§30, §32, §62); BOOT owns who clears the counter.

## Options

### Option A · Greeter reached
Summary: The generation is good when the greeter appears.
Consequences: Earliest signal; a broken session behind the greeter counts as good.
Evidence: none

### Option B · User authenticated
Summary: The generation is good after a successful login.
Consequences: Proves the login path; headless boots never mark good.
Evidence: none

### Option C · Required-service quorum including display, network and audio
Summary: The generation is good when a named quorum of services reports ready, per boot mode.
Consequences: Honest desktop-usable health; a quorum to define for desktop, recovery and headless.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
