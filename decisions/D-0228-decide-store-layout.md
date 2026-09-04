# D-0228 · Decide the Content-addressed store layout for Packages and SystemGenerations
- Status: proposed
- Task: PKG-014
- Surfaces: none
- Layer: none
- Spikes: PKG-040
- Supersedes: none
- Superseded by: none
- Baseline: §27, §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The persistent on-disk layout shared by Packages, SystemGenerations and environments is required by V0.5 exit (§27, §30); the dedup spike is the evidence.

## Options

### Option A · Nix-style store paths
Summary: Hash-prefixed paths.
Consequences: Proven; path hashes.
Evidence: none

### Option B · OSTree-style object repository
Summary: A git-like object repo.
Consequences: Hardlink dedup; checkout.
Evidence: none

### Option C · casync-like chunk store
Summary: A chunk store.
Consequences: Chunk dedup; reassembly.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
