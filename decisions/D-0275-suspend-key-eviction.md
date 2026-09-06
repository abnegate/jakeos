# D-0275 · Decide disk-key eviction on suspend
- Status: proposed
- Task: SEC-031
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §51, §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 ships suspend and resume on the reference laptop, so a suspended machine with unlocked disk keys in RAM is the cold-boot and DMA attack case (T-009) and a hibernation image written under lockdown is the unsigned-image case (T-041) (§51, §61). This decision fixes what happens to disk keys on suspend, whether re-unlock is required at resume, how suspend-then-hibernate is treated and whether hibernation is permitted under lockdown; SEC-017 owns the key store, PWR-002 the suspend state.

## Options

### Option A · Evict keys from RAM on suspend and re-unlock on resume
Summary: On suspend the kernel evicts volume keys from RAM (dm-crypt key wipe) after flushing; resume requires re-authentication (or TPM-sealed release when the boot chain measures clean) before user data is readable again.
Consequences: A suspended laptop yields no keys to cold-boot or DMA attacks, which is the property a stolen suspended device needs. Every resume shows an unlock prompt or depends on the TPM policy, in-flight I/O to encrypted volumes must be quiesced before suspend, and applications with mapped files see a pause rather than an error.
Evidence: none

### Option B · Keep keys in RAM while locked
Summary: Keys stay in RAM while the session is locked; only the screen lock protects a suspended machine.
Consequences: Instant resume with no prompt, which is what users expect from other platforms. T-009 is unmitigated: memory remanence and DMA attacks on a suspended device recover the keys; acceptable only if the TPM-backed variant of A is unavailable on a machine.
Evidence: none

### Option C · Suspend-then-hibernate
Summary: After a bounded idle period in suspend the system hibernates to an encrypted authenticated image and powers off; keys are wiped when the image is written.
Consequences: Bounded exposure window and battery safety together, matching D-0231 option B. The image key handling under lockdown is exactly T-041, so this option requires the authenticated-image work and is not available before it.
Evidence: none

### Option D · Forbid hibernate under lockdown
Summary: Hibernation is disabled whenever kernel lockdown is active, so no hibernation image ever exists on a locked-down system.
Consequences: T-041 is closed by construction and lockdown integrity is simple to state. No hibernation on any Secure Boot machine, which is every reference machine, so the power product (D-0231) can only choose its option A.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
