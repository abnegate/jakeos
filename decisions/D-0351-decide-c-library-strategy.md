# D-0351 · Decide the Layer 3 C-library strategy for inherited C stacks inside native Components
- Status: proposed
- Task: SDK-097
- Surfaces: none
- Layer: none
- Spikes: GFX-036
- Supersedes: none
- Superseded by: none
- Baseline: §3, §50, §57, §66
- Revisit when: a second hosting model appears in any workstream, or the Mesa measurement shows the chosen model cannot meet the compositor frame budget

## Context
Native crates are `no_std` plus `alloc` and may not link libc (SDK-003, ABI-003). The compositor, GPU userspace, codecs, Bluetooth, Wi-Fi supplicants and printing are inherited C stacks that assume libc, file descriptors and `dlopen`. GFX-036 measures Mesa at V0.5; GFX-056, MED-007, AUD-002, NET-009 and HW-029 need the same answer at V1. One rule is fixed here so the roadmap does not grow three hosting models (§3, §50, §57, §66; I-013, I-026; T-002, T-011; R-016).

## Options

### Option A · Layer 3 libc-compatible library over native Objects
Summary: A C library implemented over Capabilities, Operations, MemoryObjects and Tasks, in the manner of Fuchsia's fdio plus musl, that inherited C stacks link unmodified inside native Components.
Consequences: One hosting model and the C stack stays inside the Component's Capability set; the library is a large Layer 3 deliverable and POSIX shapes it cannot express become typed errors.
Evidence: none

### Option B · Personality-hosted helper Components
Summary: Each inherited C stack runs as a Linux-personality process wrapped as a Component and bridged to native clients over Channels.
Consequences: No libc port and unmodified upstream stacks; every helper is a confused deputy holding device Capabilities on behalf of native clients, and the compositor's GPU path crosses a bridge on every frame.
Evidence: none

### Option C · No inherited C inside native Components
Summary: Rust ports or rewrites per stack; C stacks exist only inside the Linux personality.
Consequences: Purest firewall; a Mesa rewrite is forbidden by §57 and I-045, so this option cannot cover GPU userspace before 1.0.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
