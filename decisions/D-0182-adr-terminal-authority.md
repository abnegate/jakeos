# D-0182 · Decide terminal-session authority for Linux programs
- Status: proposed
- Task: LNX-022
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §35, §46
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The authority a terminal session confers on Linux programs and how a developer escalates or attenuates it must be recorded so §9 does not erode (§9.1, §35, §46).

## Options

### Option A · Ambient Linux environment scoped by the terminal's own Capabilities
Summary: The terminal's Capabilities bound what programs inherit.
Consequences: Usable and bounded; scope visible in os inspect.
Evidence: none

### Option B · Per-command grant prompt
Summary: Every command is prompted.
Consequences: Precise; unusable.
Evidence: none

### Option C · Unbounded uid-0 shell
Summary: A root shell.
Consequences: Familiar; rejected against I-021.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
