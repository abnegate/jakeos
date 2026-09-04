# D-0124 · Decide whether IOMMU is required for user-space drivers and DMA
- Status: proposed
- Task: HW-017
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §33, §17
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Without IOMMU a user-space driver can DMA over the kernel, so whether IOMMU is required must be recorded (§33, §17).

## Options

### Option A · IOMMU required on all target hardware
Summary: Machines without an IOMMU are unsupported.
Consequences: Isolation is guaranteed; some hardware is excluded.
Evidence: none

### Option B · Required only for user-space DMA drivers
Summary: User-space DMA is refused without an IOMMU and the rest works.
Consequences: Broad hardware support; a degraded mode to document and test.
Evidence: none

### Option C · Software-only isolation with documented degradation
Summary: Drivers run without an IOMMU under documented risk.
Consequences: Broadest hardware; weak isolation that contradicts §33.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
