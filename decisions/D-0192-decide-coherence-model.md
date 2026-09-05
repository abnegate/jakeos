# D-0192 · Decide the MemoryObject sharing coherence model across CPUs and devices
- Status: accepted
- Task: MEM-001
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §16, §17, §38
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Shared mappings need a written coherence contract before the shared property lands, and the platform does not assume coherent memory between all devices (§16, §17, §38).

## Options

### Option A · CPU-coherent mappings plus explicit device sync Operations
Summary: CPU mappings are coherent and devices need explicit sync.
Consequences: Simple CPU model; device users carry the sync burden.
Evidence: none

### Option B · Per-mapping coherence attribute
Summary: Each mapping declares its coherence.
Consequences: Precise per device; more complex API.
Evidence: none

## Decision
Option A. Every CPU mapping of a MemoryObject is coherent. Device visibility (DMA, GPU, accelerators) is made explicit through typed synchronisation and ownership-handoff Operations rather than through non-coherent mappings. The model leaves room for unified and CXL-attached memory by treating them as coherent domains with explicit handoff at their edges.

## Consequences
- Zero-copy pipelines (§17) express device handoff as Operations that MEM benchmarks measure.
- DMA-compatible and GPU-compatible MemoryObject properties describe placement, not coherence.
- Personalities map dma-buf semantics onto these Operations (GFX, LNX).

## Rejected options and why
- Option B (per-mapping coherence attribute) rejected: every consumer would need to handle both modes correctly, and the failure mode (stale data) is silent.

## Follow-ups
none
