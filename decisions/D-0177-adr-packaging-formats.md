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
Linux applications arrive in several formats; the personality must say which it installs for end users and which is first-class, how each is confined inside the personality, and how each satisfies PKG immutability (§28, §36, §47). The L2 corpus includes Flatpak. LNX-008 surveys the formats; PKG-047 decides how compatibility applications are packaged immutably.

## Options

### Option A · Flatpak first-class
Summary: Flatpak is first-class: the store client installs Flatpaks into personality-scoped immutable objects and portals carry their grants.
Consequences: The format most Linux desktop applications already publish in, with a sandbox model that maps onto portals and Capabilities. Flatpak runtimes are large and duplicate libraries the personality already has, and ostree-based repositories must be mirrored into the content store.
Evidence: `reports/spikes/LNX-008.md`

### Option B · AppImage first-class
Summary: AppImage is first-class.
Consequences: Single-file applications with no runtime dependency. No sandbox and no update mechanism, so confinement and immutability are entirely the personality's work, and fewer applications publish AppImages than Flatpaks.
Evidence: `reports/spikes/LNX-008.md`

### Option C · deb/rpm via a distro container
Summary: deb and rpm packages install into a per-user distro container inside the personality.
Consequences: The widest catalogue, including server and CLI software. A full distribution image per container, package managers that mutate in place inside it (immutability holds only at the container boundary), and GUI integration through the container boundary.
Evidence: `reports/spikes/LNX-008.md`

### Option D · Nix
Summary: Nix packages are first-class.
Consequences: Reproducible and content-addressed like the native store, so the fit with PKG is natural. A small desktop catalogue for end users and a model few users know; better as a developer-environment path (ENV) than the first-class end-user format.
Evidence: `reports/spikes/LNX-008.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
