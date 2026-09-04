# D-0049 · Decide UEFI-only boot on x86-64 with no legacy BIOS/CSM support through 1.0
- Status: proposed
- Task: BOOT-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
BASELINE.md has no boot section, so the first BOOT Decision records whether x86-64 boot through 1.0 is UEFI-only so later work never grows a BIOS matrix (I-079).

## Options

### Option A · UEFI-only through 1.0
Summary: No BIOS or CSM image, test job or installer path exists before 1.0.
Consequences: One boot path; pre-UEFI machines are unsupported.
Evidence: none

### Option B · UEFI plus CSM
Summary: Legacy BIOS boot is supported alongside UEFI.
Consequences: Older hardware boots; a second bootloader path without Secure Boot or measured boot.
Evidence: none

### Option C · UEFI now with a BIOS revisit at V3
Summary: UEFI-only now with a scheduled reconsideration.
Consequences: Deferred cost; the matrix question lingers.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
