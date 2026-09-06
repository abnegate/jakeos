# D-0238 · Declare fleet-management and paid-app non-goals for 1.0
- Status: proposed
- Task: REL-026
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Before the public repository opens, REL's 1.0 non-goals are collapsed into one decision so release engineering does not grow a commerce or fleet stack (§63, I-092): no mobile-device management or fleet provisioning, no paid applications and no payouts. SEC already scopes directory join out of 1.0. The accepted option states the non-goals in language a V3 reviewer can lint new REL tasks against.

## Options

### Option A · Defer MDM, fleet provisioning, paid applications and payouts past 1.0
Summary: Defer MDM, fleet provisioning, paid applications and payouts past 1.0; the store is free-of-charge software only and machines are individually administered.
Consequences: Release engineering stays focused on signing, channels, repository and response. No revenue path for third-party developers at 1.0, which the 1.0 non-promises must say plainly.
Evidence: none

### Option B · Build a store commerce and fleet-provisioning stack for 1.0
Summary: Build store commerce (payments, payouts, refunds, tax) and fleet provisioning for 1.0.
Consequences: A developer revenue path and enterprise appeal. Payments and tax handling need a legal entity (Q-049), payment processors and compliance work that would consume REL for a rung; rejected for 1.0.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
