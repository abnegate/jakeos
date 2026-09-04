# D-0042 · Decide the bootloader: systemd-boot, GRUB or a native Rust UEFI stub
- Status: proposed
- Task: BOOT-008
- Surfaces: none
- Layer: none
- Spikes: BOOT-015
- Supersedes: none
- Superseded by: none
- Baseline: §2, §30, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Generation selection, boot counting and PCR policy hang off the bootloader, and §57 forbids rewriting a mature loader without demonstrated benefit (§2, §30, §57); the spike report is the evidence.

## Options

### Option A · systemd-boot
Summary: systemd-boot with boot-counting and UKI support.
Consequences: Small, UEFI-native, boot counting built in; GPL-2.0-or-later with LGPL parts and tied to systemd releases.
Evidence: none

### Option B · GRUB
Summary: GRUB with generation menu entries.
Consequences: Universal and scriptable; GPLv3 triggers anti-tivoization duties under Secure Boot and the ESP footprint is large.
Evidence: none

### Option C · Native Rust UEFI stub
Summary: A purpose-built UEFI stub selects and measures generations.
Consequences: Exact fit and Rust safety; a rewrite that must justify itself under §57 and I-009.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
