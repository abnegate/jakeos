# D-0133 · Decide installer encryption default with opt-out
- Status: proposed
- Task: INS-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether the installer encrypts user data by default must be decided before the V2 first-boot flow and V3 installer (§9, §63, I-073).

## Options

### Option A · Default-on with explicit opt-out
Summary: Encryption is on unless the user records an opt-out.
Consequences: Safe default with an escape hatch; users who opt out lose protection.
Evidence: none

### Option B · Default-on with no opt-out on Tier 1
Summary: Tier 1 images always encrypt.
Consequences: Guaranteed protection; losing the recovery key means losing data.
Evidence: none

### Option C · Default-off
Summary: Encryption is offered but off by default.
Consequences: Simplest install; violates I-073.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
