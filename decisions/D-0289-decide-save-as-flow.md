# D-0289 · Decide how an application gains authority to create one new file in a user-chosen place
- Status: proposed
- Task: STO-015
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §25
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Text Editor at V0.5 must save without Directory access (§9.1, §25).

## Options

### Option A · Chooser-created File Capability
Summary: The chooser creates the File and hands back a Capability.
Consequences: Simple; chooser semantics.
Evidence: none

### Option B · Single-use Capability<Directory, CreateOne>
Summary: A single-use create right.
Consequences: Flexible; new rights.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
