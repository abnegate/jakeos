# D-0265 · Define administrator versus standard user
- Status: proposed
- Task: SEC-013
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The identity service (D-0277) creates the first account and Authorization must know which operations require elevation (§9.1, §63). sudo and polkit are replaced by scoped Capabilities minted after re-authentication; elevation never mints a wildcard Capability (T-002). This decision fixes what makes an account an administrator, which operations elevate, and that the first created account is one.

## Options

### Option A · First account is administrator with an elevation Capability
Summary: The first account is created holding an `Capability<Elevate>`; elevation re-authenticates and mints a scoped Capability for the named operation; further administrators are made by an administrator granting the same Capability.
Consequences: Administrator is a Capability like everything else, visible and revocable in `os inspect`, and there is no ambient role. A single privileged account at first boot must be protected by the installer's account-creation flow, and losing the only holder needs a recovery path (INS).
Evidence: none

### Option B · All local accounts equivalent until first elevation
Summary: All local accounts are equal; the first one to perform an elevating operation is asked to become administrator.
Consequences: No special first account. Ambiguity about who administers a shared machine, and a race at first use; the recovery story is the same as A with a worse bootstrap.
Evidence: none

### Option C · Role bit on the identity object
Summary: A role bit on the identity object marks administrators; services check the bit.
Consequences: Explicit and easy to display. A flag that services compare is uid-like authority by another name (T-002, I-021) and invites the ambient checks the model removes; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
