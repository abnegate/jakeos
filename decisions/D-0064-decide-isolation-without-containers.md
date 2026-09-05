# D-0064 · Decide Component plus ResourceDomain as the native isolation model
- Status: accepted
- Task: CMP-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §1, §23, §36, §53
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Container-based isolation is replaced by Component plus ResourceDomain with OCI containers confined to the Linux personality, adopting the §53 non-goals as standing rules (§1, §23, §36, §53).

## Options

### Option A · Component plus ResourceDomain as the only native isolation model
Summary: No native runtime daemon, namespace step, overlay mount, image layers or fork/exec creation.
Consequences: One isolation model; Linux container workflows exist only in the personality.
Evidence: none

### Option B · Retain a native container runtime
Summary: A native OCI runtime exists beside Components.
Consequences: Familiar developer tooling; duplicates isolation and contradicts §53.
Evidence: none

## Decision
Option A. Component plus ResourceDomain is the only native isolation model. There is no native container runtime; OCI containers are a Linux-personality compatibility feature (§36). Native development environments (§35) compose ResourceDomains, storage snapshots and capability namespaces directly.

## Consequences
- ENV builds on ResourceDomain and CapabilityNamespace primitives, never on a container engine.
- LNX owns OCI support entirely; no native API exposes container concepts.
- Component creation cost is therefore the isolation cost the project is judged on (B-001).

## Rejected options and why
- Option B (retain a native container runtime) rejected: it contradicts §36 and would give developers a second, more familiar isolation path that pulls the ecosystem back toward Linux semantics.

## Follow-ups
none
