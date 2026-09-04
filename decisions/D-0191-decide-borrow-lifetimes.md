# D-0191 · Decide how borrowing lifetimes are enforced across Component boundaries
- Status: proposed
- Task: MEM-018
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §17
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Borrowing is temporary access that returns to the owner (§17); the compositor borrows application buffers in V0.5 so enforcement must be fixed, answering Q-008.

## Options

### Option A · Revocation on return
Summary: The borrower's mapping is revoked when the borrow ends.
Consequences: Exact enforcement; revocation cost on every return.
Evidence: none

### Option B · Deadline-bound borrow with forced unmap
Summary: Borrows carry a deadline and are force-unmapped when it passes.
Consequences: Bounded exposure; deadline tuning per use.
Evidence: none

### Option C · Trust in the borrower with audit
Summary: The borrower promises to stop and audit catches violations.
Consequences: Cheapest; weak enforcement.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
