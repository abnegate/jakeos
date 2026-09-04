# D-0258 · Decide the userspace executor shape for the native runtime
- Status: proposed
- Task: SDK-010
- Surfaces: none
- Layer: none
- Spikes: SDK-011
- Supersedes: none
- Superseded by: none
- Baseline: §18, §20, §52, §58
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V0 tiny-runtime gate needs an executor shape before the runtime is built (§18, §20, §52, §58).

## Options

### Option A · Custom executor over Operation completions
Summary: A purpose-built executor drives Operation completions.
Consequences: Exact fit and tiny; no ecosystem reuse.
Evidence: none

### Option B · Tokio subset with an Operation reactor
Summary: A Tokio subset with an Operation-based reactor.
Consequences: Ecosystem crates work; temptation to wrap blocking syscalls.
Evidence: none

### Option C · embassy-style executor
Summary: An embassy-style static executor.
Consequences: Small and predictable; embedded shape for a desktop runtime.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
