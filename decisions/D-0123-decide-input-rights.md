# D-0123 · Decide Capability<InputDevice> rights with no ambient device nodes
- Status: proposed
- Task: HW-007
- Surfaces: none
- Layer: none
- Spikes: HW-013
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §7
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Native applications must not receive ambient HID streams, so how Capability<InputDevice> is minted and attenuated must be named for the V0.5 HID service (§9.1, §7).

## Options

### Option A · Per-seat focused-surface only
Summary: Input reaches only the focused Surface and applications never hold device Capabilities.
Consequences: No device authority in applications; games and kiosks lack raw device access.
Evidence: none

### Option B · Per-device Capability with attenuable grab rights
Summary: A Capability per device with attenuable rights for grabs.
Consequences: Fine-grained control for raw-input users; more grants to manage and audit.
Evidence: none

### Option C · Privileged input broker
Summary: A broker Component mediates every input stream.
Consequences: Central policy point; a confused-deputy risk and an extra hop.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
