# D-0286 · Decide how the storage model degrades on foreign filesystems lacking its metadata
- Status: proposed
- Task: STO-055
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §26
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Native Collections carry metadata (typed kind, history, Capability annotations) that NTFS and exFAT cannot store, and those filesystems differ in case sensitivity, forbidden characters, timestamp precision and permission bits (§25, §26). Before foreign-filesystem support ships, this decision fixes how the storage model degrades on such volumes so metadata is never lost silently: xattr fallback, sidecar metadata or refusing unsupported volumes. It sits on the storage model (D-0282) and the personality view API (STO-036).

## Options

### Option A · xattr fallback
Summary: Metadata is stored in extended attributes where the filesystem supports them (NTFS via the ntfs3 driver), and the volume is marked read-mostly where it does not.
Consequences: Metadata travels with the file on NTFS and looks native to Windows tools that ignore unknown streams. exFAT and FAT have no xattrs, so they need a second answer anyway, and Windows itself may strip alternate data streams on copy.
Evidence: none

### Option B · Sidecar metadata
Summary: Metadata lives in a hidden sidecar database at the volume root keyed by file identity; the filesystem holds only the bytes.
Consequences: Works on every filesystem including exFAT media, and a whole volume's metadata is one object to back up. Renames and moves by foreign tools orphan sidecar entries, the database is visible clutter on removable media, and two writers (native and foreign) race.
Evidence: none

### Option C · Refuse-unsupported
Summary: Volumes that cannot hold the metadata are mounted as plain byte stores: files appear in the File Browser but not as native Collections, and every degradation (case folding, dropped timestamps, lost kinds) is shown in the volume's status.
Consequences: Nothing is lost silently and the user sees exactly what the volume can do. Removable media and dual-boot data partitions are second-class, and applications that want typed objects from such volumes get `Unknown` kinds.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
