# D-0326 · Decide input routing and focus arbitration model for focused surfaces
- Status: proposed
- Task: UIP-005
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9, §41, §60
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Pointer, keyboard, touch, pen and gamepad events must reach only the focused Surface, and focus must be arbitrated among compositor, shell and applications without X11-style grabs (§9, §41, §60). Synthetic input into trusted UI and focus stealing are threats (T-012). Device enablement stays in HW; this decision is the routing and focus contract the UI protocol and toolkit implement, and it forbids an unfocused Surface from observing events and global key grabs without a Capability.

## Options

### Option A · Compositor-owned focus with delivery only to the focused Surface
Summary: The compositor owns focus and delivers events only to the focused Surface; the shell requests focus changes through a privileged Interface.
Consequences: One arbiter, the shortest path from device to Surface, and the input-to-photon rig measures one hop. Focus policy (click-to-focus, focus-follows-pointer, modal handling) is compositor code, so shell experiments require compositor changes, and the compositor's threat surface includes every focus rule.
Evidence: none

### Option B · Shell-owned focus arbitration with compositor as delivery path
Summary: The shell decides focus and tells the compositor, which is only the delivery path.
Consequences: Focus policy lives with the UI that presents windows, so alternative shells change behaviour without touching the compositor. Every focus change crosses a Component boundary before delivery resumes, a crashed shell freezes focus until rebind, and the compositor must still enforce that delivery matches the shell's last decision.
Evidence: none

### Option C · Per-seat input-broker Component
Summary: A per-seat input-broker Component receives all device events, applies focus and grants, and forwards to the compositor for delivery.
Consequences: Input policy, accessibility rewriting (switch access, key remapping) and gaming grabs live in one supervised Component isolated from rendering. One more hop on the latency path for every event, and the broker is a Component whose compromise is total input capture.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
