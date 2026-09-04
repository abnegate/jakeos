# D-0213 · Decide the content hash algorithm and chunking strategy for the store
- Status: proposed
- Task: PKG-005
- Surfaces: none
- Layer: none
- Spikes: PKG-040
- Supersedes: none
- Superseded by: none
- Baseline: §27, §28
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Hash identity cannot change later without re-identifying every object (§27, §28), so algorithm and chunking are chosen from the dedup spike.

## Options

### Option A · sha256 whole-object
Summary: Whole objects hashed with sha256.
Consequences: Ubiquitous; slower and no partial dedup.
Evidence: none

### Option B · BLAKE3 whole-object
Summary: Whole objects hashed with BLAKE3.
Consequences: Fast; less ubiquitous.
Evidence: none

### Option C · sha256 content-defined chunking
Summary: Chunked with sha256.
Consequences: Partial dedup; complexity.
Evidence: none

### Option D · BLAKE3 content-defined chunking
Summary: Chunked with BLAKE3.
Consequences: Fast partial dedup; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
