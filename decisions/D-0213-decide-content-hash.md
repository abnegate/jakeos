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
Package and store identities are content hashes (§27, §28); changing algorithm or chunking later re-identifies every object and every signature, so this is decided once from the PKG-040 measurements of deduplication ratio, update download size and hash throughput on H-002. STO-013 records the store identifier format; this decision records the Package-identity consequence and that two builds with identical content share one identity.

## Options

### Option A · sha256 whole-object
Summary: Every store object is identified by the SHA-256 of its whole content.
Consequences: Universally available, hardware-accelerated on the reference machines and accepted by every signing and transparency ecosystem. Slower than BLAKE3 per byte and no partial deduplication: a one-byte change re-downloads the whole object.
Evidence: `reports/spikes/PKG-040.md`

### Option B · BLAKE3 whole-object
Summary: Every store object is identified by the BLAKE3 hash of its whole content.
Consequences: Several times faster than SHA-256 in software and parallel by construction, which shows in install and verification time. Less ubiquitous in external tooling and transparency logs, so REL's signing path may need SHA-256 alongside; still no partial deduplication.
Evidence: `reports/spikes/PKG-040.md`

### Option C · sha256 content-defined chunking
Summary: Objects are split by content-defined chunking and each chunk is a SHA-256 store object; a Package identity is the hash of its chunk list.
Consequences: Partial deduplication and small updates, which B-027-class update-size metrics will show. Chunk boundaries, the chunk-list format and the chunking parameters are part of identity forever, and verification touches many small objects.
Evidence: `reports/spikes/PKG-040.md`

### Option D · BLAKE3 content-defined chunking
Summary: Content-defined chunking with BLAKE3 chunk hashes and a BLAKE3 chunk-list identity.
Consequences: Fastest hashing plus partial deduplication. Carries both the chunking commitments of option C and the ecosystem cost of BLAKE3 from option B.
Evidence: `reports/spikes/PKG-040.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
