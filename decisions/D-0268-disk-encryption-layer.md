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
The encryption choice precedes the STO filesystem adr and must reuse a mature mechanism (§26, §27, §51, §57).

## Options

### Option A · LUKS2/dm-crypt block
Summary: Block-level encryption under the filesystem.
Consequences: Mature; whole-volume keys only.
Evidence: none

### Option B · fscrypt file
Summary: File-level encryption.
Consequences: Per-user keys; metadata leaks.
Evidence: none

### Option C · Filesystem-native
Summary: The filesystem's own encryption.
Consequences: Integrated; filesystem dependence.
Evidence: none

### Option D · Both block and file
Summary: Block plus file.
Consequences: Coverage; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
