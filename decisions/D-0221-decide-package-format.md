# D-0221 · Decide the on-disk and on-wire Package format and its relation to the store
- Status: proposed
- Task: PKG-012
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27, §28
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A Package must exist in three places with one identity: on the publisher's disk, on the wire from the repository, and in the content store (§27, §28). The format decides whether a download is one object or many, what is signed, and how the on-wire shape maps onto the store layout chosen by PKG-014. It is fixed before the first immutable install because two builds of identical content must produce the same Package identity from then on.

## Options

### Option A · Content-addressed tree with a manifest
Summary: A Package is a content-addressed tree: a signed manifest that names every object by identity, and the objects themselves are fetched individually into the store.
Consequences: Downloads fetch only objects the store lacks, so updates and shared dependencies cost their delta, and the store and the wire format are the same thing. A Package is many small transfers, repository serving must handle object-granular requests efficiently, and offline distribution of one Package needs a bundling step.
Evidence: none

### Option B · Single signed archive unpacking into the store
Summary: A Package is one signed archive whose payload is unpacked into the store on install.
Consequences: One file to sign, mirror, download and hand around, and the repository is a plain file server. No deduplication on the wire, so every update re-downloads unchanged objects, and identity must be computed from the unpacked content, not the archive bytes, to keep builds reproducible.
Evidence: none

### Option C · Hybrid archive that is also a store object
Summary: A Package archive is itself a store object that indexes the content-addressed objects inside it; the store can serve either the archive or its parts.
Consequences: Offline bundling and object-granular fetching both work, and the archive identity is the Package identity. Two representations of one Package must stay consistent, the store tracks which objects arrived via which archive, and garbage collection (Q-019) has two kinds of root.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
