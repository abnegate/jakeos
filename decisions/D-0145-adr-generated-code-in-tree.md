# D-0145 · Decide whether IDL-generated code is committed or generated at build time
- Status: proposed
- Task: IPC-004
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §14
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Typed IPC across a multi-repo or multi-language ecosystem breaks when generated stubs drift from their IDL, so committed versus build-time generation is decided before the first Rust stubs (§14).

## Options

### Option A · Commit generated stubs next to the IDL
Summary: Stubs are checked in.
Consequences: Reviewable diffs and no generator at build; drift possible without CI checks.
Evidence: none

### Option B · Emit stubs at build time from the IDL
Summary: Stubs are generated during build.
Consequences: No drift by construction; every build needs the generator and diffs are invisible.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
