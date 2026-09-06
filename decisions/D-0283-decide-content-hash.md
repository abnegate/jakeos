# D-0283 · Decide the content-hash algorithm, identifier format, chunking and upgrade path
- Status: proposed
- Task: STO-013
- Surfaces: none
- Layer: none
- Spikes: PKG-040
- Supersedes: none
- Superseded by: none
- Baseline: §27
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The store identifies every object by content hash (§27) and V0.5 stores the first Packages and SystemGenerations under it, so algorithm, identifier form and large-object chunking are fixed here for the store, with PKG-005 recording the Package-identity consequence. Because a hash algorithm is eventually retired, the identifier must carry an algorithm tag from day one so a second algorithm can be introduced without rewriting history. PKG-040 measures hash throughput and deduplication on H-002.

## Options

### Option A · SHA-256
Summary: Identifiers are `sha256:<hex>`; large objects are hashed whole, with the algorithm tag reserving room for a later `sha256+chunked` or successor tag.
Consequences: The algorithm every signing, transparency and forensic tool understands, hardware-accelerated on the reference machines, with an audited history. Slower than BLAKE3 in software, no incremental verification of a large object, and the upgrade path is a dual-identity period in which every object carries two tags.
Evidence: `reports/spikes/PKG-040.md`

### Option B · BLAKE3
Summary: Identifiers are `blake3:<hex>`; BLAKE3's tree mode gives verified streaming and chunk-level identity for large objects with the same root hash.
Consequences: Fastest hashing, parallel by construction, and streaming verification of large objects with no separate chunk-list format. Less accepted by external transparency logs and signing ecosystems, so REL may sign a SHA-256 digest of the manifest alongside; the upgrade path is the same tagged dual-identity period.
Evidence: `reports/spikes/PKG-040.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
