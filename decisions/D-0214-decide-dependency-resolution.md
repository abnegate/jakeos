# D-0214 · Decide dependency resolution semantics and lockfile location
- Status: proposed
- Task: PKG-006
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §29, §53
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A Package manifest names Dependencies (§29); launch performs no resolution (§53), so whatever resolves must happen at build or install time and its result must be recorded. This decision fixes whether the manifest pins exact content identities, or names version ranges resolved at install into a lockfile, and where that lockfile lives; either way the resolved objects are stored by content identity and PKG-046 later decides how a security fix reaches a pinned library.

## Options

### Option A · Exact content-hash pins in the manifest
Summary: The manifest pins every Dependency by exact content identity; there is no lockfile because the manifest is the lock.
Consequences: Fully deterministic: the same manifest installs the same bytes everywhere, reproducible builds verify it, and launch has nothing to resolve. Every library update requires republishing every dependent Package, so security fixes propagate only through rebuilds (Q-021).
Evidence: none

### Option B · Version ranges resolved at install into a lockfile next to the manifest
Summary: The manifest names version ranges; install resolves them once and writes a lockfile beside the manifest in the Package's store object.
Consequences: Publishers express compatibility once and the lockfile makes the installed result deterministic. Two Packages can lock different versions of one library, so the store holds both; the lockfile is a second document to sign, verify and show in `os inspect`.
Evidence: none

### Option C · Version ranges resolved into a generation-level lock
Summary: The manifest names version ranges; resolution happens when a SystemGeneration is composed and one generation-level lock covers every Package in it.
Consequences: One consistent set of libraries per generation, so a security fix is one relock and one generation switch, and disk use is minimal. Composing a generation can fail on an unsatisfiable range across unrelated Packages, and an application cannot pin a library newer than the generation allows without a personality version pin (§29).
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
