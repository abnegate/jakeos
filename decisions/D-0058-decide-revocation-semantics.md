# D-0058 · Decide revocation semantics: eager vs lazy, in-flight Operations, cost bounds
- Status: proposed
- Task: CAP-009
- Surfaces: none
- Layer: none
- Spikes: CAP-014, CAP-015
- Supersedes: none
- Superseded by: none
- Baseline: §7
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How revoke walks derived Capabilities, what happens to in-flight Operations and which cost bounds apply must be fixed before the V0 depth-8 revocation gate (§7), coordinated with TSK cancellation.

## Options

### Option A · seL4-style derivation-tree walk
Summary: Revoke walks the derivation tree and deletes descendants eagerly.
Consequences: Immediate and complete; walk cost grows with tree size and must be bounded.
Evidence: none

### Option B · Indirection with epoch invalidation
Summary: Handles point through an indirection slot whose epoch is bumped on revoke.
Consequences: Constant-time revoke; every use pays an indirection and epoch check.
Evidence: none

### Option C · Lazy check-on-use
Summary: Revocation is recorded and checked at the next use.
Consequences: Cheap revoke; a revoked handle remains apparently valid until used.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
