# D-0271 · Declare formal certifications out of scope for 1.0
- Status: proposed
- Task: SEC-074
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Common Criteria and FIPS 140 certifications open government and regulated markets but cost money, process and design constraints that would divert effort from the 1.0 stability contract (§51). This non-goal decision records the 1.0 position as I-091 and lists it in the published non-promises; it sits on the threat model (SEC-002) and the audit closure (SEC-070).

## Options

### Option A · Pursue Common Criteria and FIPS 140 for 1.0
Summary: Pursue Common Criteria and FIPS 140 validation for 1.0.
Consequences: Access to procurement that requires them. Both take years, external labs and a legal entity, and FIPS constrains the cryptographic implementations the platform may use (rustls is not validated); impossible before 1.0.
Evidence: none

### Option B · Declare both out of scope for 1.0
Summary: Declare both out of scope for 1.0, recorded as I-091 and in the non-promises; design nothing that precludes a later attempt.
Consequences: Focus on the stability contract and the external audit. Regulated buyers are excluded at 1.0, which the non-promises state.
Evidence: none

### Option C · Pursue only one
Summary: Pursue FIPS 140 for the cryptographic module only.
Consequences: Narrower and cheaper than both. Still requires a validated module and a vendor relationship, and it constrains the TLS and disk-encryption choices already made; rejected for 1.0.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
