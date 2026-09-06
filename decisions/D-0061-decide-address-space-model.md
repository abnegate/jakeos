# D-0061 · Decide whether every Component owns a hardware address space
- Status: proposed
- Task: CMP-021
- Surfaces: S-007
- Layer: L1
- Spikes: CMP-032
- Supersedes: none
- Superseded by: none
- Baseline: §10, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether every Component owns a hardware address space bounds isolation cost before applications are architected as component graphs (§10, §51); the spike report is the evidence.

## Options

### Option A · Hardware address space for every Component
Summary: Every Component is its own address space.
Consequences: Strongest isolation and one model; plugins and decoders pay full address-space cost.
Evidence: none

### Option B · In-address-space Component class
Summary: A Component class exists that shares an address space with its host.
Consequences: Cheap fine-grained sandboxes; isolation depends on language or software fault isolation.
Evidence: none

### Option C · Hardware address space by default with opt-in in-address-space class
Summary: Default is hardware isolation and hosts may opt plugins into shared space.
Consequences: Both models available; two isolation stories to document and test.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
