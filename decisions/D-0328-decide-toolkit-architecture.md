# D-0328 · Decide new Rust toolkit versus adapting an existing toolkit and renderer
- Status: proposed
- Task: UIP-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §50, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The toolkit is a multi-year cost (R-015) and must be scoped before toolkit-core and the four V0.5 demo applications start (§41). UIP-018 scores a new Rust toolkit and renderer against adapting Slint and adapting Xilem on accessibility-tree output, declarative-model fit (§41), GPU renderer fit, licence (must sit inside the D-0102 layer firewall) and fit with the protocol model of D-0327. The toolkit is Layer 4 in Rust with bindings, the SDK stays Layer 3 (§50, §66), and Wayland is never the toolkit API.

## Options

### Option A · New Rust toolkit and renderer
Summary: Build a new Rust toolkit and GPU renderer designed around the S-015 protocol and the semantic and accessibility trees.
Consequences: Exact fit: the element tree, accessibility metadata and semantic actions are one model and nothing is adapted away. The largest single user-space effort in the roadmap, with no ecosystem of widgets or examples until the project writes them, and the four demo applications wait on it.
Evidence: `reports/spikes/UIP-018.md`

### Option B · Adopt and adapt Slint
Summary: Adopt Slint and adapt its renderer and platform layer to S-015.
Consequences: A mature declarative toolkit with a designer-friendly markup, existing widgets and a GPU renderer. Slint's licence model (GPL or royalty-free with conditions or commercial) must be reconciled with D-0102 and third-party proprietary applications, its element model is its own rather than the native protocol's, and accessibility output must be mapped.
Evidence: `reports/spikes/UIP-018.md`

### Option C · Adopt and adapt Xilem
Summary: Adopt Xilem (with Vello and Masonry) and adapt it to S-015.
Consequences: A declarative Rust architecture and a permissively licensed compute-centric renderer that match the native model's intent, with an active upstream. Less mature than Slint with fewer widgets and an evolving API, so the project tracks a moving target and may end up maintaining a fork.
Evidence: `reports/spikes/UIP-018.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
