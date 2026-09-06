# D-0268 · Decide disk encryption layer and store interaction
- Status: proposed
- Task: SEC-005
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §26, §27, §51, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Full-disk encryption (V3 installer) interacts with everything the STO filesystem decision wants: snapshots, content-addressed deduplication and the store layout (§26, §27). The encryption layer is chosen first so the filesystem decision knows what it sits on, it reuses a mature mechanism rather than inventing a storage layer (§57, I-044), and implementation stays in V1. Threats T-008 (stolen device) and T-010 (evil-maid) frame every option; each must say how snapshots and deduplication behave against ciphertext.

## Options

### Option A · LUKS2/dm-crypt block
Summary: LUKS2 over dm-crypt encrypts the block device; the filesystem, snapshots and store live inside it and never see ciphertext.
Consequences: Mature, TPM-sealable and measured-boot friendly; deduplication and snapshots work unchanged because they operate on plaintext inside the volume. One key per volume, so per-user separation on a shared machine needs a second mechanism, and unlock happens before any user identity exists (T-008 is covered, per-user data at rest against another user is not).
Evidence: none

### Option B · fscrypt file
Summary: fscrypt encrypts directories per key inside the filesystem; the store and system volume are cleartext.
Consequences: Per-user keys and unlock at login, so one user's data is protected from another and from a stolen device. Filenames and sizes are partially visible, deduplication across users is impossible by construction, snapshots contain ciphertext under different keys, and the system volume needs option A anyway to protect the store against T-010.
Evidence: none

### Option C · Filesystem-native
Summary: The filesystem's own encryption (bcachefs, or btrfs when it lands) encrypts extents with filesystem-aware keys.
Consequences: Integrated with snapshots and deduplication by design, one mechanism for everything. Ties encryption to the filesystem choice for the life of 1.0 and to the maturity of that filesystem's crypto implementation, which for the candidates is younger than dm-crypt.
Evidence: none

### Option D · Both block and file
Summary: dm-crypt for the whole device plus fscrypt for per-user home directories.
Consequences: Covers T-008 and T-010 and separates users at rest, with the system volume and store deduplicating normally. Two key hierarchies to manage, two unlock moments in the boot and login flow, and home directories lose deduplication across users.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
