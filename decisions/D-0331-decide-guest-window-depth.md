# D-0331 · Decide guest-window integration depth and agent protocol
- Status: proposed
- Task: VIRT-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §40, §49
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How deeply a VM guest integrates with the desktop fixes the guest-agent protocol that V2 guest tools ship against, so this decision lands with the tools rather than after them (§40, §49). Options run from one Surface per VM to per-application guest windows with clipboard, drag and drop, chooser and notification bridging. GFX-092 owns the compositor's remote-Surface presentation; native software still never sees Wayland or X11 (I-048).

## Options

### Option A · One virtio-gpu Surface per VM
Summary: Each VM is one virtio-gpu Surface: a desktop in a window.
Consequences: Simplest guest tools (display, clipboard, time) and no per-window protocol. The VM is obviously a VM, which is acceptable for the triage fallback but is the experience WIN-001 forbids for ordinary software.
Evidence: none

### Option B · Per-application guest windows as native Surfaces with bridging
Summary: The guest agent exports each guest application window as a native Surface with clipboard, drag and drop, chooser and notification bridging over the agent protocol.
Consequences: Guest applications look native, which makes the fallback usable for daily work. A full agent protocol for Linux and Windows guests, window-management semantics mapped across the boundary, and clipboard and file bridging that must respect the grant model (a guest is one Component).
Evidence: none

### Option C · Single-display default with opt-in seamless
Summary: Single-display default with per-VM opt-in to seamless mode when the guest tools are installed.
Consequences: Works without guest tools and improves with them. Two modes to test and a mode switch the user must find; the protocol of B must exist anyway.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
