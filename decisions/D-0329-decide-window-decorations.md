# D-0329 · Decide server-side versus client-side decorations for native and compat windows
- Status: proposed
- Task: UIP-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §47, §49
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Who draws the title bar, shadow and close button fixes the window Surface roles in UI protocol v0 (§41) and must land before the Wayland bridge shows compatibility windows at V0.5 (§47, §49). Native and compatibility windows may take different answers; this decision records both and names the protocol roles and which side draws shadow, title and controls for each. Consistency of the desktop, theming, and what a misbehaving or hung application can do to its own frame are the stakes.

## Options

### Option A · Server-side for native and compatibility windows
Summary: The compositor draws decorations for every window, native and compatibility; applications supply only their content Surface.
Consequences: One consistent look, theming and accessibility of window controls live in one place, and a hung application still has a working close button. Applications that want custom title bars (browsers with tabs in the title area, media players) cannot have them without a protocol extension, and Wayland clients that insist on client-side decorations draw a second frame unless the bridge negotiates.
Evidence: none

### Option B · Client-side for native and server-side for compatibility
Summary: Native windows draw their own decorations through the toolkit; the compositor decorates compatibility windows.
Consequences: Native applications integrate controls into their content with a toolkit that guarantees the standard controls exist, and Linux windows look consistent without bridge negotiation. Two decoration paths, a native application that hangs shows a dead frame, and the toolkit becomes the enforcement point for close and move semantics.
Evidence: none

### Option C · Client-side for all
Summary: Every window, native and compatibility, draws its own decorations.
Consequences: Maximum application control and the least compositor work. Inconsistent frames across toolkits and personalities, no reliable close for a hung application, and X11 through Xwayland has no decoration concept at all, so the bridge must synthesise one anyway.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
