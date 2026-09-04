# D-0050 · Decide whether each SystemGeneration boots as one signed UKI or separately verified parts
- Status: proposed
- Task: BOOT-011
- Surfaces: none
- Layer: none
- Spikes: BOOT-015
- Supersedes: none
- Superseded by: none
- Baseline: §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The image format must be fixed before PKG store layout and the boot-entry format harden (§30); a single signed UKI changes Secure Boot signing units and PCR prediction.

## Options

### Option A · Single signed UKI
Summary: Kernel, initramfs and command line are one signed PE image.
Consequences: One signing unit and predictable PCRs; any change re-signs the whole image.
Evidence: none

### Option B · Kernel plus separately verified initramfs and command line
Summary: Each part is verified independently.
Consequences: Parts update independently; PCR prediction and verification are more complex.
Evidence: none

### Option C · UKI with detached add-ons
Summary: A signed UKI with signed add-on sections for command line or microcode.
Consequences: Flexibility without losing one signing unit; add-on measurement policy must be defined.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
