# D-0292 · Decide the initial Linux filesystem under the native storage layer
- Status: proposed
- Task: STO-016
- Surfaces: none
- Layer: none
- Spikes: STO-026
- Supersedes: none
- Superseded by: none
- Baseline: §26, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The native storage layer sits on a retained Linux filesystem through 1.0; no native filesystem or object store is built before then (§26, §57). V0.5 exit requires the choice, made against the nine §26 properties (copy-on-write, snapshots, reflink, checksums, compression, deduplication, quotas, encryption interaction, repair). STO-026 measures the candidates; ZFS is rejected on CDDL licence grounds (I-067, GOV-003) before any measurement row so that no benchmark reopens it.

## Options

### Option A · btrfs
Summary: btrfs is the system filesystem for store, generations and user data.
Consequences: Copy-on-write, snapshots, reflink, checksums, compression and send/receive are in-tree and mature on the workloads the reference machines run; generations as snapshots (D-0217) and store deduplication by reflink (D-0293) are native operations. Its RAID 5/6 remains unsafe, free-space accounting surprises users, and quota groups are slow, so quotas need a design that avoids qgroups.
Evidence: `reports/spikes/STO-026.md`

### Option B · bcachefs
Summary: bcachefs is the system filesystem.
Consequences: A modern design with tiering, native encryption, checksums and snapshots, and the cleanest fit for the §26 property list on paper. It is the youngest candidate, its upstream status has been unstable, and betting 1.0 on it means the project carries its bugs and possibly its maintenance.
Evidence: `reports/spikes/STO-026.md`

### Option C · XFS with reflink
Summary: XFS with reflink is the system filesystem.
Consequences: The most mature and fastest candidate for large files and metadata, with reflink for store deduplication. No snapshots, no checksums of data and no compression, so generations need dm-verity images (D-0217 option C) and integrity comes from the store rather than the filesystem.
Evidence: `reports/spikes/STO-026.md`

### Option D · ZFS
Summary: ZFS via the out-of-tree module.
Consequences: The strongest feature set of any candidate. The CDDL is incompatible with GPLv2 for in-tree shipping, so the kernel fork could not carry it and every image would depend on an out-of-tree module (I-067, GOV-003); rejected before measurement and recorded so it is not re-proposed.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
