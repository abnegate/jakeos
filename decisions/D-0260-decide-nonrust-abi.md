# D-0260 · Decide how non-Rust bindings map onto Layer 1 and IDL stubs
- Status: proposed
- Task: SDK-072
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Swift, Kotlin, C# and TypeScript at V3 need a recorded mapping onto the C ABI and IDL (§50, §65) without a second Native ABI.

## Options

### Option A · C ABI only
Summary: Languages bind through the C header.
Consequences: Universal; poor ergonomics per language.
Evidence: none

### Option B · Per-language IDL codegen
Summary: An IDL backend per language.
Consequences: Idiomatic bindings; backends to write and maintain.
Evidence: none

### Option C · Both
Summary: C header plus codegen.
Consequences: Coverage; maintenance of both.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
