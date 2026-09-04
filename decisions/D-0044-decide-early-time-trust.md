# D-0044 · Decide the trusted time source policy before network time is available
- Status: proposed
- Task: BOOT-021
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Certificate and signature checks at early boot need a clock-trust policy before SVC time sync exists.

## Options

### Option A · Bootloader-persisted monotonic floor
Summary: The bootloader stores the last good time on the ESP and refuses to go below it.
Consequences: Simple and firmware-independent; the ESP write must be protected and refreshed.
Evidence: none

### Option B · Build-timestamp floor of the booted generation
Summary: Time can never be earlier than the generation's build timestamp.
Consequences: No mutable state at all; a stale generation has a stale floor.
Evidence: none

### Option C · RTC trusted with TPM-backed clock attestation
Summary: The RTC is trusted when TPM attestation confirms it.
Consequences: Best accuracy; depends on TPM presence and a more complex trust chain.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
