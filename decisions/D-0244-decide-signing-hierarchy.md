# D-0244 · Decide the signing key hierarchy and custody model
- Status: proposed
- Task: REL-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28, §30, §51, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every update, rollback and later Secure Boot chain rests on who can sign a SystemGeneration, a Package, repository metadata, the kernel and the bootloader (§28, §30, §51, §63). This decision fixes the key hierarchy and custody before V1 devices enrol trust, names which key signs which artifact and who holds quorum shares, and makes rotation and compromise (T-028, T-029) rehearsed follow-ups rather than improvised responses.

## Options

### Option A · Offline root with HSM-backed threshold intermediates per channel plus publisher keys
Summary: An offline root key signs per-channel intermediates held in HSMs under a threshold quorum; publisher keys sign Packages and are certified by an intermediate; bootloader and kernel are signed by a dedicated intermediate enrolled in firmware.
Consequences: A single leaked key compromises one channel or one publisher, not the platform; rotation is per intermediate; the root is touched only at ceremonies. Requires HSMs, ceremony procedures and at least two quorum holders from day one, which is operational work REL must own before V1.
Evidence: none

### Option B · Software-held root with delayed HSM migration
Summary: A software-held root and intermediates on the maintainer's machine, with HSM migration scheduled before V3 public alpha.
Consequences: The fastest start for a one-person project and the same logical hierarchy as option A. Every key is as safe as one laptop until migration, migration is a rotation event every enrolled device must survive, and a compromise before migration is unrecoverable for devices that pinned the root.
Evidence: none

### Option C · Single online root for all artifacts
Summary: One online signing key for every artifact.
Consequences: Simplest possible pipeline. Compromise of the CI host is compromise of every installed machine's boot chain with no scoped rotation, which T-028 rates as the worst outcome; recorded so it is never re-proposed.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
