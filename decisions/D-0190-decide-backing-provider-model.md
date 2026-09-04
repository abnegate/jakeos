# D-0190 · Decide the MemoryObject backing-provider abstraction for future memory media
- Status: proposed
- Task: MEM-017
- Surfaces: S-006
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §16, §38, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A MemoryObject is properties plus a backing provider, so how DRAM, CXL, persistent, disaggregated, unified and accelerator-local media plug in without changing Layer 1 mapping must be decided (§16, §38, §65).

## Options

### Option A · DRAM object now, retrofit providers later
Summary: Build a DRAM-only MemoryObject and add providers later.
Consequences: Fast V0; a retrofit risks changing mapping operations.
Evidence: none

### Option B · Provider interface from V0
Summary: A backing-provider interface exists from the first implementation.
Consequences: Future media plug in without ABI change; upfront design cost.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
