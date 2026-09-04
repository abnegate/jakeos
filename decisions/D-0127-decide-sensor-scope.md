# D-0127 · Decide 1.0 sensor support per device class in or out of scope
- Status: proposed
- Task: HW-042
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Ambient-light, lid and tablet-mode switches, and accelerometer rotation must be declared in or out per device class for 1.0 (§62).

## Options

### Option A · All three classes in
Summary: ALS auto-brightness, lid and tablet-mode switches, and accelerometer rotation are all supported.
Consequences: Complete laptop experience; three sensor drivers and their user-space services.
Evidence: none

### Option B · Lid-only
Summary: Only the lid switch is supported.
Consequences: Minimal driver work; no auto-brightness or rotation.
Evidence: none

### Option C · Lid plus ALS
Summary: Lid switch and ambient-light auto-brightness are supported; rotation is out.
Consequences: Covers the common laptop needs; convertibles get no rotation.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
