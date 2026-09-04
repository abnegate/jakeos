# D-0231 · Decide hibernation policy for 1.0
- Status: proposed
- Task: PWR-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §61, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Shipping a hibernation image without a Decision leaves T-010 and T-009 unaddressed (§61, §62); SEC owns key eviction and STO owns swap layout.

## Options

### Option A · No hibernate in 1.0
Summary: Hibernate is not delivered.
Consequences: No image threats; battery drain on long suspend.
Evidence: none

### Option B · Suspend-then-hibernate after idle
Summary: Suspend, then hibernate after idle.
Consequences: Battery safety; image threats apply.
Evidence: none

### Option C · Full hibernate with an encrypted authenticated image
Summary: Full hibernate.
Consequences: Complete; lockdown and key handling.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
