# D-0137 · Decide dual-boot shared data partition format
- Status: proposed
- Task: INS-024
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The recommended dual-boot shared data partition format and whether the installer offers to create one must be decided (§25, §63).

## Options

### Option A · exFAT
Summary: The installer recommends and creates exFAT.
Consequences: Readable by every OS; no permissions or extended metadata.
Evidence: none

### Option B · NTFS
Summary: The installer recommends NTFS.
Consequences: Windows-native with metadata; Fast Startup and hibernation lock the volume.
Evidence: none

### Option C · No shared partition by default
Summary: The installer offers none.
Consequences: Simplest; no cross-OS sharing.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
