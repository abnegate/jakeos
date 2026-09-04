# D-0119 · Decide Bluetooth host placement and required profiles
- Status: proposed
- Task: HW-040
- Surfaces: none
- Layer: none
- Spikes: HW-028
- Supersedes: none
- Superseded by: none
- Baseline: §33, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V2 Bluetooth gate depends on host placement and required profiles (§33, §62), fed by HW-028.

## Options

### Option A · Retain BlueZ in-kernel/host
Summary: BlueZ stays as the host with its existing profile set.
Consequences: Mature profiles including A2DP, HFP, HID and GATT; a Linux-shaped host outside the native model.
Evidence: none

### Option B · Move the host to a native Component
Summary: A native Bluetooth host Component per §33.
Consequences: Isolation and native objects; profile parity must be rebuilt.
Evidence: none

### Option C · Hybrid
Summary: Kernel HCI with a native host for selected profiles and BlueZ for the rest.
Consequences: Incremental migration; two hosts to keep consistent.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
