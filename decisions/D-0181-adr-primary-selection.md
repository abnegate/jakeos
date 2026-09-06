# D-0181 · Decide X11 primary selection stays inside the bridge
- Status: proposed
- Task: LNX-020
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §47, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
X11 has a second, implicit selection (primary) that is set by highlighting and pasted by middle click. The native clipboard is a Capability (S-032, UIP-003) and clipboard reads are a threat surface (T-032). This decision fixes whether primary selection exists only inside the Wayland/X11 bridge, is merged into the native clipboard, or is dropped, and states that a native Component without a clipboard Capability can never read primary-selection contents (I-048, §41, §47, §57).

## Options

### Option A · Primary selection emulated only inside the bridge
Summary: Primary selection is emulated inside the bridge for X11 and Wayland clients only and never enters the native clipboard service.
Consequences: X11 and Wayland applications keep middle-click paste among themselves; native Components and the native clipboard never see primary contents, so T-032 does not gain a second channel and the clipboard Capability stays the only read path. Middle-click paste between a Linux application and a native application does not work, which is documented behaviour.
Evidence: none

### Option B · Merging primary into the native clipboard
Summary: Primary selection becomes a second slot of the native clipboard, readable with the same Capability.
Consequences: Middle-click paste works everywhere. Every highlight in any application writes to a system-wide slot that any clipboard-holder can read, which is exactly the leak T-032 describes; the clipboard consent model of UIP-003 would have to cover implicit writes.
Evidence: none

### Option C · Dropping primary selection
Summary: The bridge advertises no primary selection.
Consequences: Simplest bridge and no leak. Terminal emulators, editors and many X11 tools rely on primary selection; entries in the L corpora would rate as regressions for a behaviour users notice daily.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
