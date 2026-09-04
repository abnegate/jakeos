# D-0207 · Decide the Component crash capture format
- Status: proposed
- Task: OBS-029
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §10, §24, §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The Component crash capture format must serve the SDK debugger and the INS crash-report client while enforcing I-077 (§10, §24, §61).

## Options

### Option A · Minidump-compatible
Summary: Crashes are minidumps.
Consequences: Existing tooling; no async Task stacks.
Evidence: none

### Option B · Native typed record with async Task stacks
Summary: A native record with Task stacks.
Consequences: Task stacks and typed fields; new tooling.
Evidence: none

### Option C · Core-file plus sidecar
Summary: A core file plus a sidecar.
Consequences: Complete state; large and hard to redact.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
