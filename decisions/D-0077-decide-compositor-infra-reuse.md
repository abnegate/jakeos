# D-0077 · Decide compositor infrastructure reuse versus build-anew
- Status: proposed
- Task: GFX-013
- Surfaces: none
- Layer: none
- Spikes: GFX-032
- Supersedes: none
- Superseded by: none
- Baseline: §39, §40, §2
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Which existing compositor infrastructure, if any, is vendored for DRM/KMS plumbing must be decided while the native object model stays independent of Wayland (§39, §40, §2, I-048).

## Options

### Option A · Smithay crates
Summary: Smithay's DRM, GBM and input crates are vendored.
Consequences: Rust and modular; Wayland-shaped assumptions in some crates.
Evidence: none

### Option B · wlroots via FFI
Summary: wlroots is used through FFI.
Consequences: Mature and widely deployed; C dependency and Wayland-centric design.
Evidence: none

### Option C · Mutter or KWin fork
Summary: A desktop compositor is forked.
Consequences: Complete feature set; enormous surface tied to GNOME or KDE.
Evidence: none

### Option D · Greenfield Rust
Summary: Everything is written fresh.
Consequences: Exact fit; longest path and violates §57 without a recorded benefit.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
