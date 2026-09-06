# D-0272 · Declare multi-seat, guest, kiosk, and enterprise directory out of scope
- Status: proposed
- Task: SEC-075
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Multi-seat, guest accounts, kiosk mode and enterprise directory integration (LDAP, Active Directory, Kerberos, SSO, Group Policy) are large product areas that release engineering and SEC cannot carry to 1.0 (§63). This non-goal decision records the position as I-092 while keeping the authenticator Interface pluggable so a later version can add directory login without redesign; it sits on identity (D-0277) and authentication (SEC-014).

## Options

### Option A · Ship directory login and multi-seat for 1.0
Summary: Ship directory login and multi-seat in 1.0.
Consequences: Enterprise deployability at launch. Kerberos, SSO and policy engines are each a workstream, and multi-seat needs the compositor and input work D-0270 option C describes; impossible in the remaining rungs.
Evidence: none

### Option B · Declare out while keeping authenticators pluggable
Summary: Declare multi-seat, guest, kiosk and directory integration out of scope for 1.0 (I-092); the authenticator Interface stays pluggable and the non-promises list them.
Consequences: Focus, and a later path that needs no redesign. Shared-machine and enterprise scenarios are unaddressed at 1.0.
Evidence: none

### Option C · Ship kiosk only
Summary: Ship kiosk mode only.
Consequences: A niche but self-contained feature (a locked single-application session). It still needs a session policy model and a management path, which is most of the enterprise work in miniature; rejected for 1.0.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
