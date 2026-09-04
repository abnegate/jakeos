# D-0177 · Decide first-class Linux packaging formats
- Status: proposed
- Task: LNX-017
- Surfaces: none
- Layer: none
- Spikes: LNX-008
- Supersedes: none
- Superseded by: none
- Baseline: §28, §36, §47, §56.3
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Which Linux packaging formats the Personality supports for end-user installation and which is first-class must be decided (§28, §36, §47, §56.3).

## Options

### Option A · Flatpak first-class
Summary: Flatpak is the primary format.
Consequences: Sandboxed and popular; runtime size.
Evidence: none

### Option B · AppImage first-class
Summary: AppImage is primary.
Consequences: Simple; unsandboxed.
Evidence: none

### Option C · deb/rpm via a distro container
Summary: Distro packages run in a container.
Consequences: Broad coverage; heavy.
Evidence: none

### Option D · Nix
Summary: Nix packages are primary.
Consequences: Reproducible; niche.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
