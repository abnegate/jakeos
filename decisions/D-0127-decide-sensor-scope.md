# D-0127 · Decide 1.0 sensor support per device class in or out of scope
- Status: accepted
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
Option C. Sensor support in scope for 1.0 is the lid switch (suspend and lock behaviour) and the ambient light sensor (automatic brightness on laptops). Accelerometer rotation and tablet-mode switching are LATER; convertibles are not reference hardware.

## Consequences
- PWR consumes lid events; APP and PWR consume ambient light for brightness.
- The IIO sensor framework is retained in the kernel; only these two classes get native service Components.
- Rotation and tablet mode return as a decision when a convertible enters Tier 1.

## Rejected options and why
- Option A (all three classes) rejected: rotation and tablet mode need convertible hardware that is out of scope.
- Option B (lid only) rejected: laptops at fixed brightness fail the battery and UX expectations of the V2 laptop gates.

## Follow-ups
none
