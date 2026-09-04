# D-0224 · Decide whether SystemGeneration switches may apply without reboot
- Status: proposed
- Task: PKG-070
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether SystemGeneration switches may apply without reboot must be decided given lockdown and PCR constraints (§30, I-086), answering Q-052.

## Options

### Option A · kexec into the new kernel
Summary: kexec replaces reboot.
Consequences: Fast; lockdown and PCR issues.
Evidence: none

### Option B · Userspace-only live switch
Summary: Userspace switches live.
Consequences: Fewer reboots; mixed-version risk.
Evidence: none

### Option C · Reboot-only apply
Summary: Always reboot.
Consequences: Simple; reboots.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
