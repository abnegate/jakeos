# D-0129 · Decide the V1 through V2 Reference machine list and security criteria
- Status: accepted
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
Option A. Reference machines are three named SKUs recorded in registers/hardware.md: one AMD desktop (H-002), one Intel laptop (H-004) and one AMD laptop (H-005). Each must have an IOMMU, TPM 2.0 and Secure Boot with user key enrolment. Nothing gates on any other machine before V3, when Tier 1 widens per the ladder. The exact models are chosen in a follow-up decision task once procurement starts.

## Consequences
- LAB procurement tasks name the three SKUs; QEMU profiles (H-001, H-003) remain the CI baseline.
- Firmware quirks are tracked per SKU, so suspend, boot and HDR results are reproducible.
- A machine that fails the security criteria cannot become a reference machine even if convenient.

## Rejected options and why
- Option B (families without SKUs) rejected: firmware behaviour varies within a family, making the V1 and V2 laptop gates non-reproducible.
- Option C (unconstrained PC list) rejected: contradicts §62 and would spread the HW workstream across hardware nobody tests.

## Follow-ups
Name the three SKUs in registers/hardware.md when LAB procurement starts (LAB, HW).
