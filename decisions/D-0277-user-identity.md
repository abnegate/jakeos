# D-0277 · Decide user identity versus Capability roots
- Status: proposed
- Task: SEC-012
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §7, §9.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The native model has no ambient identity: authority is a Capability, not a uid check (§7, §9.1, T-001). Something must still say who is logged in, hold the session's root Capabilities, and let the Linux personality derive uid and gid for its processes without exporting them natively. Pulled to V0.5 so V1 login, Session and multi-user work do not wait; it also fixes how login becomes the session Capability root holder.

## Options

### Option A · Kernel uid-like identity
Summary: The kernel keeps a uid-like identity on every Component and services check it.
Consequences: Familiar to every Linux developer and the personality mapping is the identity itself. The identifier becomes authority the moment any service compares it, which reintroduces ambient permission and confused deputies (T-001); it also puts user policy in the kernel against D-0157.
Evidence: none

### Option B · Userspace identity-service mapped to Capability roots
Summary: A user-space identity service maps an authenticated user to a bundle of root Capabilities; login mints the session's roots and hands them to the session Component; the kernel knows nothing about users.
Consequences: Capability-pure: no service can grant by identity, only by holding a Capability, and multi-user is one more identity record. The identity service is on the path of every login and must be supervised; uid and gid for the personality are attributes the personality derives from the bundle, never seen natively.
Evidence: none

### Option C · Hybrid kernel identifier plus userspace root holder
Summary: The kernel carries an opaque per-Component identifier for accounting and audit; a user-space root holder owns all authority as in option B.
Consequences: Audit, quotas and `os inspect` can group Components by user without any service, while authority stays capability-only. Two notions of user (kernel tag, service record) must be kept consistent, and the tag is one more field the ABI carries and can never remove.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
