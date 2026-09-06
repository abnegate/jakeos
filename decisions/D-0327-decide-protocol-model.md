# D-0327 · Decide UI protocol model: retained scene tree, client Buffers, or hybrid
- Status: proposed
- Task: UIP-006
- Surfaces: S-015
- Layer: L2
- Spikes: UIP-017
- Supersedes: none
- Superseded by: none
- Baseline: §12, §41, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
S-015 is the native UI protocol between applications and the compositor (§12, §41). Whether an application ships a retained scene tree, renders into Buffers and ships Surfaces, or does both decides how accessibility metadata is emitted, how compositor crash-rebind restores windows (the V0.5 gate), who owns Buffers, and what the protocol IDL looks like. UIP-017 prototypes the three; the decision picks one of them and does not invent a fourth (§65).

## Options

### Option A · Retained scene tree
Summary: Applications send a retained tree of typed elements and properties; the compositor lays out, renders and composites.
Consequences: Accessibility metadata, rebind after compositor crash and remote or scaled rendering come for free because the compositor holds the whole scene; the toolkit is thin. Expressiveness is bounded by the element vocabulary, custom rendering (games, canvases, video) needs an escape hatch anyway, and the compositor does every application's layout work.
Evidence: `reports/spikes/UIP-017.md`

### Option B · Client Buffers
Summary: Applications render into Buffers (MemoryObjects) and attach them to Surfaces; the compositor composites Surfaces.
Consequences: Any renderer works and applications control every pixel; closest to Wayland so the personality bridge is simple. Accessibility needs a separate tree protocol, rebind requires the application to re-create and re-render every Surface, and the compositor cannot restyle or scale content it does not understand.
Evidence: `reports/spikes/UIP-017.md`

### Option C · Hybrid
Summary: A retained tree whose leaf elements may be Buffers: the toolkit ships elements for standard UI and attaches client-rendered Buffers for canvases, video and games.
Consequences: Standard UI gets accessibility and rebind from the tree while custom content keeps full control; this is the shape every mature toolkit converges on. Two rendering paths in the compositor, Buffer ownership rules inside a tree, and a protocol IDL that must version both halves together on S-015.
Evidence: `reports/spikes/UIP-017.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
