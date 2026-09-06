# D-0282 · Decide replacing the global namespace with Capability-scoped storage objects
- Status: proposed
- Task: STO-012
- Surfaces: S-027
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§25 replaces the global filesystem namespace for applications: a Component sees typed storage objects it was granted, while humans keep files and folders through privileged holders such as the File Browser and the chooser (§67). This decision fixes the storage model on S-027 (Layer 2): what a Component's storage world is, how granted objects relate to the folders users see, and what the personalities are given so Linux and Windows software still find a path-shaped view.

## Options

### Option A · Per-component roots
Summary: Each Component receives one or more root directory objects (its ApplicationData, granted folders) and addresses everything relative to a root.
Consequences: Simplest mental model and closest to what personality software expects, so the compatibility facade is thin. Hierarchy and path strings remain the primary abstraction, so typed objects (Image, Document) are layered on top rather than primary, and a grant is always a subtree.
Evidence: none

### Option B · Object graphs
Summary: Storage is a graph of typed objects with identities; a Component holds Capabilities to objects and to collections, and folders are one collection type among others.
Consequences: Grants are exact (this Image, this Document set), typed objects and `file.type` (Q-038) are primary, and search, history and automation (SEM) work on objects rather than paths. Every existing tool and format assumes paths, the File Browser must render a graph as folders, and the personalities need a full synthetic filesystem over the graph.
Evidence: none

### Option C · Hybrid path facades for personalities
Summary: Native Components use the object graph of option B; each personality receives a synthesised path namespace (a facade) over the objects its processes were granted.
Consequences: Native software gets the typed model and compatibility software gets paths, each without compromising the other, and the facade is the single place path semantics live. Two models to keep consistent, facade behaviour for rename, hard links and cross-grant moves must be specified, and the facade is a Component on the personality's hot path.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
