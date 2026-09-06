# D-0222 · Decide that global dependency installation is replaced by per-Package dependency objects
- Status: proposed
- Task: PKG-013
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §2, §29
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§2 replaces global dependency installation with per-Package dependency objects referenced by content identity: installing any Package must never break another and there is no global conflict state (§29). This decision records that rule as I-036 and names what is rejected so no later task reintroduces a shared library directory for convenience; deduplication of identical objects is a store concern (PKG-014), not a reason to share by name.

## Options

### Option A · Per-Package content-identity dependency objects
Summary: Every Package references its Dependencies by content identity; identical objects are stored once by the store but there is no name-based lookup path at runtime.
Consequences: Installing, removing or updating one Package cannot change what another Package loads, and `os inspect` shows exactly which objects a Component runs against. Two versions of one library coexist as a matter of course, so security updates reach applications only through their own dependency update (Q-021), and the personality's expectation of a global `lib` directory is served by a per-Package view, not a shared tree.
Evidence: none

### Option B · Global shared library directory
Summary: Libraries install into one shared directory resolved by name and soname at load time.
Consequences: The Linux model the personality already understands, with one copy of each library on disk. Installing one Package can break another, conflicts must be resolved at install time, and the runtime resolves by mutable name lookup, which §2 and §53 forbid.
Evidence: none

### Option C · Generation-wide dependency set with conflict resolution
Summary: Each SystemGeneration composes one consistent dependency set; Packages reference by identity but the composer picks one version per library per generation.
Consequences: One copy per generation and one relock for a security fix. Two Packages that need incompatible versions cannot coexist in one generation, so compose fails or one Package is excluded, and the composer becomes a solver whose result is the real dependency graph.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
