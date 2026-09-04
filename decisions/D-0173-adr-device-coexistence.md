# D-0173 · Decide /dev, sysfs and udev coexistence with native drivers
- Status: proposed
- Task: LNX-014
- Surfaces: none
- Layer: none
- Spikes: LNX-008
- Supersedes: none
- Superseded by: none
- Baseline: §33, §46, §56.3
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How a user-space native driver coexists with Personality /dev, sysfs and udev for the same device must be decided for GPU and input at V1 (§33, §46, §56.3), answering Q-028.

## Options

### Option A · Personality-only nodes with native Device objects beside them
Summary: Two independent views of the device.
Consequences: Clean separation; duplicated enumeration and state.
Evidence: none

### Option B · Translating /dev over Object<Device>
Summary: A facade translates /dev onto native objects.
Consequences: One source of truth; translation work per device class.
Evidence: none

### Option C · Shared raw nodes for both worlds
Summary: Both worlds use the same raw nodes.
Consequences: Simple; rejected for native software.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
