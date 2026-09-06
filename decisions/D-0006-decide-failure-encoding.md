# D-0006 · Decide the Operation result error model: typed enum per kind or uniform error Object
- Status: proposed
- Task: ABI-009
- Surfaces: S-004
- Layer: L1
- Spikes: ABI-022
- Supersedes: none
- Superseded by: none
- Baseline: §19, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Operation results need a failure encoding that keeps forged handles, denied derivation and Timeout stable across the ABI (§19), and native errors are not errno values (S-004).

## Options

### Option A · Typed enum per Operation kind
Summary: Each Operation kind returns its own error enum.
Consequences: Errors are precise and exhaustive per kind; generic code and IDL backends must handle many enums and cross-kind errors are duplicated.
Evidence: none

### Option B · Uniform error Object
Summary: Every Operation returns the same error type covering forged handle, wrong type, denied rights, timeout, cancellation and exhaustion.
Consequences: One encoding for every tool and binding; kind-specific detail must be carried out of band or lost.
Evidence: none

### Option C · Hybrid class plus per-kind payload
Summary: A uniform error class word is paired with an optional per-kind payload.
Consequences: Generic handling on the class and precision in the payload; two-level encoding is more to freeze and to document.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
