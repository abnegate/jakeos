# D-0295 · Decide three-view mapping of user data across native and personalities
- Status: proposed
- Task: STO-042
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §46, §48
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A user's Documents must be one set of objects whether seen as a native Collection, the Linux personality's home directory or the Windows personality's profile (§25, §46, §48). This decision fixes the mapping that STO-036's personality view API implements and INS migration imports consume, and states that native software never receives path strings as authority. It sits on the storage model of D-0282.

## Options

### Option A · One object graph with two path facades
Summary: One object graph; the Linux personality and the Windows personality each present a path facade over the same objects, so a file saved in a Linux editor appears in the native Collection and the Windows profile at once.
Consequences: One truth and no synchronisation. Two facades with different naming rules (case sensitivity, reserved names, path separators) must map onto one graph, and rename or move semantics differ per facade.
Evidence: none

### Option B · Copy-on-first-use
Summary: Each personality gets its own tree; objects are copied in on first use and copied back on close.
Consequences: Facades are real filesystems and personality software behaves natively. Three copies diverge, conflicts appear on copy-back, and large files copy in full; the sync problem this option creates is the one A avoids.
Evidence: none

### Option C · Adopt-in-place per personality
Summary: Each personality adopts its own storage in place (a real home directory, a real profile) and the native Collection indexes them.
Consequences: Fast to build and personality-native behaviour. Three models of the same data, native Collections become an index over foreign trees rather than the source of truth, and Capability scope is enforced only at the index.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
