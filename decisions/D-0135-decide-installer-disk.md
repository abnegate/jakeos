# D-0135 · Decide installer disk layout, wipe and dual-boot policy
- Status: proposed
- Task: INS-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §30, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V3 graphical installer must not invent disk policy at the moment of install. STO-014 decided the GPT layout and type GUIDs; BOOT-029 decided ESP policy. This decision is the installer's offer: wipe versus install alongside, whether an existing ESP is reused, where the signed recovery SystemGeneration lives, how an existing Windows, Linux or Intel Mac install is detected, and what the installer refuses to destroy without an explicit wipe confirmation (§25, §30, §63).

## Options

### Option A · Wipe-only
Summary: The installer offers only a whole-disk wipe; the recovery generation lives on a dedicated partition; an existing ESP is replaced.
Consequences: One path to test on every Tier 1 machine and the layout is always the canonical one. No dual boot at all, so migrating users must keep a second machine or a VM, which INS migration and the V3 alpha audience will feel.
Evidence: none

### Option B · Shrink-and-install-alongside as a first-class offer
Summary: Shrink-and-install-alongside is a first-class offer: the installer shrinks the largest resizable foreign volume, reuses the existing ESP when it has room, adds boot entries for detected systems, and places recovery on the ESP or a dedicated partition by size.
Consequences: Migrating users keep their existing OS, which is what most early adopters need. Resizing NTFS and APFS-adjacent layouts is the riskiest operation an installer performs, Windows BitLocker and fast-startup states must be detected and refused, and the ESP-sharing rules are firmware-specific.
Evidence: none

### Option C · Refuse-to-install when space cannot hold recovery plus retained kernels
Summary: Whenever free space or the ESP cannot hold the recovery generation plus the retained kernels, the installer refuses with a typed reason instead of degrading the layout.
Consequences: Never produces a machine that cannot recover, and the refusal is honest. Some machines are refused outright; this is a rule that accompanies A or B rather than an alternative to them.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
