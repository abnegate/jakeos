# D-0217 · Decide how a SystemGeneration is materialised on disk
- Status: proposed
- Task: PKG-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §26, §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0.5 exit requires the mechanism that makes generation N+1 bootable while N stays bootable (§30). This decision sits on the STO filesystem substrate decision (STO-016) and does not invent a native filesystem (§26, I-044); ZFS is rejected on licence grounds before measurement, matching that decision. The choice determines what a generation costs on disk, how fast a switch is, and how boot integrity (BOOT measured boot) verifies the running system.

## Options

### Option A · Profile tree over the content store
Summary: A generation is a directory tree of links into the content store, composed at install time; booting selects a tree.
Consequences: Composition is cheap, generations share every unchanged object, and the mechanism is filesystem-independent. Integrity is per object rather than per image, so measured boot must verify a tree of hashes rather than one root, and the link farm is visible to personality software that expects a conventional layout.
Evidence: none

### Option B · Filesystem snapshot per generation
Summary: Each generation is a copy-on-write filesystem snapshot (btrfs or bcachefs) of the composed system volume.
Consequences: Switching and rollback are native filesystem operations, snapshots are space-efficient, and the layout looks conventional to personality software. The mechanism couples generations to the chosen CoW filesystem for the life of 1.0, and image verification for measured boot needs the filesystem's own checksums or a separate manifest.
Evidence: none

### Option C · Verified image per generation
Summary: Each generation is a read-only verified image (dm-verity over an erofs or squashfs) built from the store; mutable state mounts beside it.
Consequences: One root hash verifies the whole running system, which measured boot and the 1.0 integrity story want, and the image is immutable by construction. Every generation is a full image on disk unless deduplicated at the block layer, composing one takes a full build step, and per-Package updates always produce a new image.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
