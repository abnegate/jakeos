# D-0117 · Decide ComputeDevice enumeration ABI and open-ended class taxonomy
- Status: proposed
- Task: HET-001
- Surfaces: S-028
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §1, §37, §38, §65, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0 scopes HET to the enumeration Decision so later device classes do not require an ABI break (§1, §37, §38, §65, §67); nothing Layer 1 is frozen.

## Options

### Option A · Closed class enum
Summary: CPU, GPU, NPU, DSP, FPGA and accelerator as a fixed enum.
Consequences: Exhaustive matching; a new class is an ABI change.
Evidence: none

### Option B · Extensible class id
Summary: Classes are extensible identifiers with reserved ranges.
Consequences: New hardware without ABI change; less static checking.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
