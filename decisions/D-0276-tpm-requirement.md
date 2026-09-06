# D-0276 · Decide TPM 2.0 as requirement versus optional
- Status: proposed
- Task: SEC-050
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §51, §55, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V2 measured boot and sealed disk keys assume a TPM 2.0 (§51, §55). The reference machines all have one (D-0129), but community hardware may not. This decision fixes whether TPM 2.0 is a hard requirement for 1.0, optional with a passphrase-only degraded path, or required for Tier 1 only, states the degraded behaviour and how the HCL records it (§62, I-074), and cites T-008 (stolen device).

## Options

### Option A · TPM 2.0 hard requirement for 1.0
Summary: TPM 2.0 is required; the installer refuses machines without one.
Consequences: Every installed machine gets measured boot, sealed keys and attestation, so the security story has no asterisk. Excludes older and some virtual hardware, and the VIRT guest images need a virtual TPM.
Evidence: none

### Option B · Optional with passphrase-only degraded path
Summary: TPM optional: without one, disk unlock is passphrase-only, measured boot is unavailable, and the HCL entry records `tpm: absent` with the consequences.
Consequences: Broad hardware reach with an honest degraded mode. Two unlock paths to test, and the degraded path's users lose the T-008 protections silently unless the installer explains it.
Evidence: none

### Option C · Required for Tier 1 only
Summary: Required for Tier 1 machines, optional with the degraded path elsewhere.
Consequences: Lab machines always exercise the full path, community machines still install. The same two paths as B with a rule that makes Tier 1 stricter than the product; it is B with a lab policy attached.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
