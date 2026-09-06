# D-0237 · Decide publisher identity and Package naming
- Status: proposed
- Task: REL-025
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A public repository without namespace ownership, key attestation and a name-squatting rule cannot stop impersonation (§28, §63). Persistent grants are keyed on Package identity plus publisher, not content hash (T-033), so the identity scheme is what grants survive updates by. Trademark policy stays with GOV (GOV-056); this decision fixes the operational scheme: how a publisher proves a name, how a key change is attested, and how a squatting dispute is resolved.

## Options

### Option A · Reverse-DNS namespace with key attestation at first publish
Summary: Package names are reverse-DNS under a domain the publisher proves control of at first publish; the publisher's signing key is attested against that proof and rotated by a signed statement from the old key or a fresh domain proof.
Consequences: Ownership is verifiable, squatting requires owning the domain, and key rotation has a defined path. Depends on DNS and on publishers having a domain; individuals without one need a project-provided namespace, and domain expiry becomes an identity risk.
Evidence: none

### Option B · Flat names with first-come ownership
Summary: Flat names, first come first served.
Consequences: Simplest to run and to type. Squatting is trivial, disputes are manual, and a popular name taken by a squatter misleads users; rejected.
Evidence: none

### Option C · Publisher IDs without package-name ownership
Summary: Stable publisher identifiers with no ownership of Package names; two publishers may ship the same name.
Consequences: Flexible and no naming disputes. Users must read the publisher on every install to tell two same-named Packages apart, which is the confusion T-033 warns about; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
