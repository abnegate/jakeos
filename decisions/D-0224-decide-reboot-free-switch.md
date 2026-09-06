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
Whether a SystemGeneration switch may apply without a reboot shapes the V2 update promise and BOOT's PCR policy (§30). kexec into the new kernel is forbidden under lockdown unless the image is signed, and measured-boot values must be re-derived; a user-space-only live switch raises the mixed-version question (T-034); kernel live patching is a non-goal (I-086). This decision answers Q-052 and sits on generation switching (PKG-020) and measured boot (BOOT-008).

## Options

### Option A · kexec into the new kernel
Summary: The updater kexecs into the new generation's signed kernel and user space restarts from native init, skipping firmware.
Consequences: A switch takes seconds rather than a firmware boot and is a full, consistent restart. Lockdown must verify the kexec image signature, PCR values after kexec differ from a cold boot so TPM-sealed secrets need a policy for both, and device drivers that mishandle kexec (GPUs) show up as V2 bugs.
Evidence: none

### Option B · Userspace-only live switch
Summary: User-space Packages switch live under D-0226; the kernel switches only at reboot.
Consequences: Most updates apply without any restart. The two-level state of D-0138 option B, with T-034 argued per Component.
Evidence: none

### Option C · Reboot-only apply
Summary: Every generation switch is a full reboot.
Consequences: One path, one integrity story, no kexec quirks. Every update costs a firmware boot, which on the reference desktop is tens of seconds and on laptops interrupts work; the V2 update UX is then about scheduling rather than speed.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
