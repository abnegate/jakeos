# D-0129 · Decide the V1 through V2 Reference machine list and security criteria
- Status: proposed
- Task: HW-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §1, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
GPU, driver, energy and latency spikes need a fixed reference set (§1, §62), naming SKUs with Secure Boot key enrolment, TPM 2.0 and IOMMU as criteria.

## Options

### Option A · Three SKUs now with IOMMU, TPM 2.0 and Secure Boot enrolment required
Summary: One AMD desktop, one Intel laptop and one AMD laptop are named now.
Consequences: Spikes can run on reproducible hardware; results generalise narrowly.
Evidence: none

### Option B · Families without SKUs
Summary: Vendor families are named without exact models.
Consequences: Flexibility in procurement; irreproducible results between machines.
Evidence: none

### Option C · Unconstrained PC list
Summary: Any x86-64 PC is a target.
Consequences: Broad appeal; measurements cannot be compared.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
