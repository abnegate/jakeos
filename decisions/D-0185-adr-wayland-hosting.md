# D-0185 · Decide Wayland hosting and X11 via Xwayland
- Status: proposed
- Task: LNX-004
- Surfaces: none
- Layer: none
- Spikes: LNX-010
- Supersedes: none
- Superseded by: none
- Baseline: §41, §47, §57, §60
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A Wayland application in the Linux personality must appear as an ordinary native window, and X11 arrives only through Xwayland (§47, §60). GFX-020 decides whether the native compositor speaks Wayland itself or a bridge translates; LNX-010 prototypes nested and in-compositor hosting. This decision picks the hosting arrangement for the personality and records that Wayland and X11 are never the native UI API (§41, §57): native software sees Surfaces on the native UI protocol only.

## Options

### Option A · Nested compatibility compositor
Summary: A nested compatibility compositor (a personality Component) serves Wayland and Xwayland clients and presents each toplevel to the native compositor as a native Surface.
Consequences: Wayland stays entirely inside the personality boundary and the native compositor stays Wayland-free; crash of the nested compositor takes down only Linux windows. Every frame and input event crosses one more Component hop, which the input-to-photon rig (LAB-001) will measure, and window management features (workspaces, tiling, decorations) must be forwarded through the nesting.
Evidence: `reports/spikes/LNX-010.md`

### Option B · In-compositor Wayland serving
Summary: The native compositor implements the Wayland server protocols directly beside the native UI protocol.
Consequences: No extra hop, and Linux windows get native window management for free. Wayland protocol code lives in the core compositor, a crash in the Wayland path takes down native windows too, and the compositor's threat surface includes every Wayland extension a client can speak.
Evidence: `reports/spikes/LNX-010.md`

### Option C · Translating bridge Component
Summary: A bridge Component translates Wayland objects and buffers into native UI protocol messages one-to-one and holds no compositor of its own.
Consequences: The native compositor stays pure and the bridge is stateless enough to restart without losing windows. Translation is only complete for the subset of Wayland the native protocol can express; extensions with no native counterpart (for example xdg-foreign, layer-shell) either get native equivalents in UIP or are unsupported.
Evidence: `reports/spikes/LNX-010.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
