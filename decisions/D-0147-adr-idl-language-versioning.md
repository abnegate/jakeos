# D-0147 · Decide how the IDL language itself is versioned
- Status: proposed
- Task: IPC-055
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §14, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V3 opens the repository to third-party packages authoring Interfaces, so a language version pragma and compatibility policy must exist (§14, §66).

## Options

### Option A · Language version pragma with a published compatibility window
Summary: Each IDL file declares a language version and compilers support a published window.
Consequences: Explicit compatibility for third parties; a window to maintain.
Evidence: none

### Option B · Compiler major version as the language version
Summary: The compiler's major version is the language version.
Consequences: Nothing extra to declare; language changes are tied to tooling releases.
Evidence: none

### Option C · Edition flags per file
Summary: Rust-style editions selected per file.
Consequences: Gradual migration; more combinations to test.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
