# D-0180 · Decide the POSIX path view of native storage
- Status: proposed
- Task: LNX-019
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §46
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Linux programs address storage by path; native storage is Capability-scoped typed objects (§25, D-0282). The personality must present the objects a process was granted as a POSIX tree without turning a path check into an authority check (§46). STO owns the objects and the view API (STO-042, STO-047); LNX owns the facade. This answers Q-018 and rejects a global FHS as native storage (I-016).

## Options

### Option A · Capability-scoped path facade
Summary: A per-process synthetic filesystem presents only the objects the process's Component holds, mounted at conventional paths; every open resolves the path to a Capability the facade already holds.
Consequences: Authority is exactly the Component's grants, `os inspect` shows the same objects for the Linux process as for a native one, and a path that is not granted does not exist rather than being forbidden. The facade implements rename, hard links, `..` traversal and directory listing over an object graph, and every path operation is a Component boundary crossing unless the facade runs in the personality kernel path (D-0178).
Evidence: none

### Option B · Copy-on-first-use tree
Summary: Granted objects are copied into a private POSIX tree when first opened and copied back on close or exit.
Consequences: The simplest facade: real files on a real filesystem. Two copies diverge while the program runs, concurrent native and Linux access to one object sees different bytes, large files are copied in full, and copy-back on crash loses data.
Evidence: none

### Option C · Global FHS as native storage
Summary: The native store is a conventional FHS tree and native storage objects are paths into it.
Consequences: Every Linux program works unchanged. Native storage becomes a global namespace with path-based authority, which is what §25 replaces and I-016 forbids; recorded as rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
