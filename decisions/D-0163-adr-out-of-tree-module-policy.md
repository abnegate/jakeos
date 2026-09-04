# D-0163 · Decide out-of-tree module policy: GPL-only native exports and taint semantics
- Status: proposed
- Task: KRN-028
- Surfaces: none
- Layer: none
- Spikes: KRN-017
- Supersedes: none
- Superseded by: none
- Baseline: §5.1, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether native kernel symbols are exported GPL-only and how taint behaves must settle before the native ABI stabilises at V1 (§5.1, §51).

## Options

### Option A · GPL-only native exports with inherited MODULE_LICENSE unchanged
Summary: Native symbols are GPL-only and inherited modules keep their semantics.
Consequences: Consistent with Linux; proprietary modules cannot use native objects.
Evidence: none

### Option B · Permissive native exports
Summary: Native symbols are exported without GPL-only marking.
Consequences: Proprietary modules can use native objects; legal ambiguity with the GPLv2 kernel.
Evidence: none

### Option C · No loadable modules for native objects
Summary: Native objects are built in only.
Consequences: No export question; no modularity for native objects.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
