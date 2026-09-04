# D-0025 · Decide IDE strategy for 1.0 and the criteria for a native port
- Status: proposed
- Task: APP-020
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §56.5, §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 self-hosting needs a usable IDE for kernel and platform development on the OS (§56.5, §61) without promising a native IDE at 1.0, and the criteria for a later native port must be recorded.

## Options

### Option A · VS Code through the Linux personality
Summary: VS Code is the V1 daily-driving IDE.
Consequences: Familiar and extensible; runs as a personality application.
Evidence: none

### Option B · JetBrains IDE through the Linux personality
Summary: A JetBrains IDE is the V1 daily-driving IDE.
Consequences: Strong Rust and C support; heavier and partly proprietary.
Evidence: none

### Option C · Native Text Editor as daily driver
Summary: The native Text Editor grows into the development environment.
Consequences: Native showcase; lacks debugger and language-server integration at V1.
Evidence: none

### Option D · Native IDE port
Summary: An IDE is ported to the native ABI.
Consequences: Best long-term fit; not a 1.0 promise.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
