# D-0052 · Decide Capability persistence across Component restart and reboot
- Status: proposed
- Task: CAP-020
- Surfaces: none
- Layer: none
- Spikes: CAP-026
- Supersedes: none
- Superseded by: none
- Baseline: §7, §9.1, §25
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A Component must retain access to a user-selected object across restart and reboot without weakening §7 revocability (§7, §9.1, §25); the Decision also names which object types may ever be persisted.

## Options

### Option A · Sturdy references
Summary: A persisted grant is a sturdy reference the Component can redeem later.
Consequences: Well-understood capability-system pattern; revocation of a sturdy reference needs a registry to invalidate against.
Evidence: none

### Option B · Revocable persistent grant store
Summary: The system keeps a per-Component store of persisted grants the user can inspect and revoke.
Consequences: User-visible and revocable by construction; the store is a privileged service with its own attack surface.
Evidence: none

### Option C · Re-prompt on every launch
Summary: Nothing persists; every launch re-asks the user.
Consequences: No persistent authority at all; a photo editor that forgets every file is unusable.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
