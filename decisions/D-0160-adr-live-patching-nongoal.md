# D-0160 · Decide that kernel live-patching is a non-goal in favour of generations plus reboot
- Status: proposed
- Task: KRN-026
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §56.4
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
SystemGenerations plus reboot is the update model and live-patching a non-goal (I-086), recorded before the V1 update channel (§30, §56.4).

## Options

### Option A · Kernel live-patching as a supported path
Summary: Livepatch series are supported.
Consequences: Fixes without reboot; complexity and a violation of I-086.
Evidence: none

### Option B · SystemGenerations plus reboot only
Summary: Every kernel update is a new generation and a reboot.
Consequences: Simple and consistent; reboots for every fix.
Evidence: none

### Option C · kexec into a new kernel without livepatch
Summary: kexec replaces the reboot.
Consequences: Faster switch; lockdown and PCR re-derivation issues.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
