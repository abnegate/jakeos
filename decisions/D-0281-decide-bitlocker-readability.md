# D-0281 · Decide whether BitLocker volumes are readable via user-space dislocker-style support
- Status: proposed
- Task: STO-072
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Migrating from Windows means reading the previous OS's disk, which is usually BitLocker-encrypted (§25). A user-space implementation in the dislocker family can read it given the recovery key, but its licence must pass GOV-016's userspace allowlist and the parser is untrusted-input code (§51, D-0349). This decision is the read half; STO-074 builds the mount if it is accepted.

## Options

### Option A · User-space dislocker-style support
Summary: Ship a user-space BitLocker reader (dislocker-style, sandboxed as a filesystem-parsing helper per D-0349) that mounts a volume read-only given the recovery key or password.
Consequences: Migration from a typical Windows laptop works without a second machine. The implementation's licence must be on the allowlist, the parser runs on hostile input and is confined accordingly, and newer BitLocker modes may be unsupported.
Evidence: none

### Option B · No BitLocker support
Summary: No BitLocker support: the migration assistant asks the user to decrypt in Windows first or to copy data another way.
Consequences: Nothing to license or sandbox. Most Windows laptops ship encrypted, so migration becomes a manual process most users abandon.
Evidence: none

### Option C · Deferral past 1.0
Summary: Defer past 1.0.
Consequences: Focus for V3 and V4. The migration assistant (INS) ships without its most common case at 1.0, which the non-promises must state.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
