# D-0256 · Decide POSIX-Personality shell versus a native Object-aware shell
- Status: proposed
- Task: SDK-025
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §21, §42, §64
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How the os CLI, PTYs and job control map onto Tasks and Channels must be fixed before V1 daily-driving (§3, §21, §42, §64, I-006).

## Options

### Option A · POSIX shell only in the Linux personality
Summary: No native shell exists.
Consequences: Nothing new to build; every command-line task goes through Linux.
Evidence: none

### Option B · Native object-aware shell over Tasks and Channels
Summary: A native shell speaks typed Interfaces.
Consequences: Typed pipelines and Capability-aware jobs; a new shell to design.
Evidence: none

### Option C · Both with os as the native surface
Summary: POSIX shell in the personality and os as the native command surface.
Consequences: Coverage for both audiences; two shells to document.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
