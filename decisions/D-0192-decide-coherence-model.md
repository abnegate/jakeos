# D-0192 · Decide the MemoryObject sharing coherence model across CPUs and devices
- Status: proposed
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
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
