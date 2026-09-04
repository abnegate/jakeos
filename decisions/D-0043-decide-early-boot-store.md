# D-0043 · Decide how early boot locates the content store and the selected SystemGeneration
- Status: proposed
- Task: BOOT-009
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27, §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Generation booting only works if early boot finds immutable objects before trusting mutable state (§27, §30), paired with SVC's early-userspace Decision.

## Options

### Option A · Initramfs-embedded locator
Summary: The initramfs carries the store location and generation id.
Consequences: Everything needed is inside the signed image; changing the generation means a new initramfs.
Evidence: none

### Option B · Kernel command line parameters
Summary: The bootloader passes store and generation on the command line.
Consequences: Flexible per entry; the command line must be inside the signed unit or it is untrusted.
Evidence: none

### Option C · Bootloader-passed manifest
Summary: The bootloader hands a signed manifest to early userspace.
Consequences: Rich, verifiable metadata; a bootloader-to-userspace protocol to define.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
