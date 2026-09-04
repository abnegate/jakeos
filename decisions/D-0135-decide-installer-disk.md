# D-0135 · Decide installer disk layout, wipe and dual-boot policy
- Status: proposed
- Task: INS-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §30, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Wipe-versus-alongside, ESP reuse and recovery placement must be fixed so the V3 installer is not inventing disk policy (§25, §30, §63).

## Options

### Option A · Wipe-only
Summary: The installer only wipes the disk.
Consequences: Simple and safe; no dual-boot.
Evidence: none

### Option B · Shrink-and-install-alongside as a first-class offer
Summary: The installer resizes and installs beside an existing OS.
Consequences: Dual-boot for migrating users; resize risk and Windows detection logic.
Evidence: none

### Option C · Refuse-to-install when space cannot hold recovery plus retained kernels
Summary: The installer refuses with a typed error when the layout cannot fit.
Consequences: Honest failure; some machines are refused.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
