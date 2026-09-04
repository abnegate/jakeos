# D-0217 · Decide how a SystemGeneration is materialised on disk
- Status: proposed
- Task: PKG-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §26, §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How a SystemGeneration is materialised on disk is required by V0.5 exit and sits on the STO filesystem Decision (§26, §30).

## Options

### Option A · Profile tree over the content store
Summary: A tree of links into the store.
Consequences: Nix-like; symlink farm.
Evidence: none

### Option B · Filesystem snapshot per generation
Summary: A filesystem snapshot per generation.
Consequences: Native to btrfs; filesystem coupling.
Evidence: none

### Option C · Verified image per generation
Summary: A verity image per generation.
Consequences: Verity-friendly; space.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
