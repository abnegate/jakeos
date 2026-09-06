# D-0293 · Decide how the content store maps onto the chosen filesystem without double storage
- Status: proposed
- Task: STO-017
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The content store holds every Package object once; SystemGenerations and Packages reference those objects (§27). If materialising a generation or an application tree copied bytes out of the store, disk use would double. This decision picks the filesystem mechanism that lets a Blob appear in several trees while stored once, on the filesystem chosen by STO-016 and the identifier scheme of STO-013, for the store layout of PKG-014.

## Options

### Option A · Hardlinked object directory
Summary: Objects live in a hashed object directory and every tree that needs one hardlinks to it.
Consequences: Works on any filesystem, costs one directory entry per reference, and reads are direct. Hardlinked files share one inode, so metadata (mode, ownership, timestamps) cannot differ per tree, a write through any link would corrupt the object unless the store is mounted read-only or files are immutable, and tools that count links see odd numbers.
Evidence: none

### Option B · Reflinked files
Summary: Objects are stored once and every tree gets a reflinked copy that shares extents copy-on-write.
Consequences: Each tree has an independent inode with its own metadata and a write in one tree never affects another, which suits personality software that expects to chmod or touch files. Requires a reflink-capable filesystem (btrfs, XFS, bcachefs), and metadata duplication plus extent bookkeeping cost more than a link.
Evidence: none

### Option C · Filesystem-native dedup
Summary: Trees are plain copies and the filesystem's offline or inline deduplication merges identical extents.
Consequences: Nothing special at store or materialise time. Deduplication is asynchronous and best-effort, so disk use spikes until it runs, it costs CPU and I/O on a laptop, and no candidate filesystem does inline deduplication maturely.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
