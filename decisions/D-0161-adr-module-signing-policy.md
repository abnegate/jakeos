# D-0161 · Decide module signing under Secure Boot for out-of-tree, GPU and local modules
- Status: proposed
- Task: KRN-027
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §51, §56.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Module signing under Secure Boot for out-of-tree, GPU and local modules must be decided at V1 because HW's NVIDIA Decision depends on it (§51, §56.1).

## Options

### Option A · Project key only
Summary: Only project-signed modules load.
Consequences: Strongest chain; no third-party or locally built modules.
Evidence: none

### Option B · MOK enrolment of third-party keys
Summary: Owners enrol third-party keys via MOK.
Consequences: Vendors and users can add modules; a manual enrolment step.
Evidence: none

### Option C · Local developer keys that taint the kernel
Summary: Locally built modules load with a local key and taint the kernel.
Consequences: Developer freedom; tainted kernels and unsupported bug reports.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
