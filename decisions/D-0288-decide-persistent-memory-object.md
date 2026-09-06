# D-0288 · Decide persistent MemoryObject semantics: storage backing, crash consistency, content addressing
- Status: proposed
- Task: STO-040
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §16, §26, §27
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§16 leaves open what a persistent MemoryObject is: memory whose contents survive the Component and the boot. Before the persistent property ships, this decision fixes its storage backing, its crash consistency relative to the durability contract (D-0284) and whether it is content-addressed (§26, §27). It answers Q-006 and sits on MEM-020 and STO-005.

## Options

### Option A · File-backed CoW
Summary: A persistent MemoryObject is backed by a copy-on-write file in a Capability-scoped storage object; durability follows D-0284 on explicit sync.
Consequences: Mutable in place, mmap-friendly, and crash consistency is the filesystem's. It is not content-addressed, so two identical persistent objects are stored twice and cannot be shared or verified by identity.
Evidence: none

### Option B · Content-addressed Blob
Summary: A persistent MemoryObject is a content-addressed Blob in the store; writing produces a new Blob and rebinding the name is the durable step.
Consequences: Immutable, deduplicated and verifiable like everything else in the store, and a crash leaves either the old or the new Blob. Every write of a large object rewrites or chunks it, so it is a poor fit for databases and append logs.
Evidence: none

### Option C · Hybrid seal-on-durable
Summary: A persistent MemoryObject is a CoW file while open and is sealed into a content-addressed Blob when the holder requests durability.
Consequences: Mutable while working, immutable and deduplicated once saved: the document model most applications want. Two states with different semantics, a seal step that can fail, and the open-state file is not content-addressed until sealed.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
