# D-0287 · Decide GPT partition and volume layout for store, generations, user data, swap and recovery
- Status: proposed
- Task: STO-014
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §26, §30, §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
SystemGenerations exist from V0.5, so the disk layout that holds the content store, the generation trees or images, user data, swap or hibernate space and the recovery environment is fixed now (§26, §30, §32). Each JakeOS partition needs a registered GPT type GUID so foreign tools (Windows, macOS, Linux installers) identify it instead of treating it as unknown or free. The layout sits on the filesystem substrate decision (STO-016) and shapes the installer (INS) and dual-boot coexistence.

## Options

### Option A · Separate GPT partitions
Summary: One GPT partition per volume: ESP, store, generations, user data, swap or hibernate, recovery, each with its own type GUID and filesystem.
Consequences: Every volume is visible to any partition tool, can be resized, encrypted or backed up independently, and a damaged volume does not take the others with it. Sizes are fixed at install, so a full store cannot borrow from user data without repartitioning, and the store and generations cannot share extents even when the filesystem could deduplicate them.
Evidence: none

### Option B · Filesystem subvolumes
Summary: Two GPT partitions (ESP and one encrypted system partition); store, generations, user data and recovery are subvolumes of one filesystem.
Consequences: Space is shared and rebalances itself, snapshots and reflinks work across store, generations and user data, and only two GUIDs are needed. Foreign tools see one opaque volume and cannot identify or rescue a sub-tree, recovery lives inside the volume it is meant to repair, and swap must still be a file or a separate partition.
Evidence: none

### Option C · Mixed layout
Summary: Partitions for what must be separable (ESP, recovery, swap or hibernate, user data) and one system partition whose subvolumes hold the store and generations.
Consequences: Recovery is bootable when the system volume is broken, user data can be preserved or encrypted on its own key across reinstalls, and the store and generations share space and deduplicate. Two mechanisms to document, the system and user-data partitions cannot lend each other space, and the GUID table lists both partition types and the subvolume names.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
