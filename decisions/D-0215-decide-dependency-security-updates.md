# D-0215 · Decide how security fixes reach a library pinned by many Packages
- Status: proposed
- Task: PKG-046
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §29
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Dependencies are content-identified per Package (D-0222) and resolved per the D-0214 rule, so a security fix to a library pinned by many Packages cannot be applied by mutating the library in place (§29). REL's first CVE response needs one mechanism that reaches every dependent, records which Package could not take the fix, and never mutates a pinned object. This answers Q-021.

## Options

### Option A · Rebuild-and-republish of dependents
Summary: Every dependent Package is rebuilt against the fixed library and republished; the fix propagates as ordinary updates.
Consequences: Identities stay honest (a Package's identity is exactly what it links) and reproducible builds verify it. Response time is bounded by the slowest third-party publisher, and unmaintained Packages never receive the fix unless the store client can rebuild them itself.
Evidence: none

### Option B · Grafting a substitute object into a new generation
Summary: A graft rule in the SystemGeneration composer substitutes the fixed object for the vulnerable one for every dependent, producing a new generation without republishing dependents.
Consequences: One relock fixes every dependent in one generation switch, which meets the 14-day CVE SLA. The running identity of a dependent no longer equals its published identity, so the graft must be recorded in the generation and shown by `os inspect`, and an ABI-incompatible fix silently breaks dependents unless the graft rule checks compatibility.
Evidence: none

### Option C · Runtime relinking to a patched object
Summary: The dynamic linker substitutes the patched object at load time based on a system-wide substitution table.
Consequences: Immediate effect with no new generation. The substitution table is mutable global state consulted at launch, which §53 forbids, and it is indistinguishable from a global library directory; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
