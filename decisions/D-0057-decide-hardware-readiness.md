# D-0057 · Decide the ABI invariants required for hardware-assisted Capability enforcement
- Status: proposed
- Task: CAP-021
- Surfaces: none
- Layer: none
- Spikes: CAP-027, CAP-012
- Supersedes: none
- Superseded by: none
- Baseline: §8, §38, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The ABI invariants that keep application-visible Capabilities stable across kernel-metadata, page-table, CPU-tag and CHERI enforcement must be published (§8, §38, §65) as the hardware-assisted readiness ADR.

## Options

### Option A · Kernel-metadata enforcement with a published invariant list and reserved handle bits
Summary: Enforcement stays in kernel metadata with the invariants and reserved bits written down.
Consequences: Works on all shipping hardware; readiness rests on discipline rather than tests.
Evidence: none

### Option B · Dual representation with a sealed-pointer-shaped handle on x86-64
Summary: Handles already take the sealed-pointer shape on x86-64.
Consequences: Hardware transition is a backend swap; wider handles today for no runtime benefit.
Evidence: none

### Option C · CHERI-shaped handles from V0.5
Summary: Handles are CHERI capabilities from V0.5.
Consequences: Maximum readiness; rejected unless the spike supports it because no shipping target has CHERI.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
