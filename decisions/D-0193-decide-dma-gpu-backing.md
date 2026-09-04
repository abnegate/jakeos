# D-0193 · Decide whether dma-buf backs DMA- and GPU-compatible MemoryObjects
- Status: proposed
- Task: MEM-019
- Surfaces: none
- Layer: none
- Spikes: MEM-030
- Supersedes: none
- Superseded by: none
- Baseline: §16, §17, §39
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V0.5 compositor needs GPU buffers on inherited DRM drivers, so whether dma-buf backs DMA and GPU MemoryObjects is decided from the spike (§16, §17, §39).

## Options

### Option A · dma-buf is the backing
Summary: GPU-compatible MemoryObjects are dma-bufs internally.
Consequences: Direct Mesa interop; a Linux object as the backing.
Evidence: none

### Option B · Native object exported as dma-buf on demand
Summary: A native object exports a dma-buf when a personality or driver needs one.
Consequences: Native first; an export path to maintain.
Evidence: none

### Option C · Native object only, no dma-buf
Summary: No dma-buf anywhere.
Consequences: Purity; no Mesa or DRM interop.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
