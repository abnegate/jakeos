# D-0102 · Decide the license firewall and outbound project licenses
- Status: proposed
- Task: GOV-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §1, §66, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Copyleft leak into platform Interfaces and missing userspace headers cannot be fixed later, so the license firewall between Layer 1 and Layers 2 through 4 and the corpus license must be fixed (§1, §66, §67).

## Options

### Option A · Layer 1 GPLv2 with Layers 2 through 4 MIT
Summary: Kernel GPLv2, everything else MIT.
Consequences: Simplest permissive terms; no explicit patent grant.
Evidence: none

### Option B · Layer 1 GPLv2 with Apache-2.0 userspace
Summary: Kernel GPLv2, userspace Apache-2.0.
Consequences: Explicit patent grant; GPLv2-incompatible for code shared with the kernel.
Evidence: none

### Option C · Layer 1 GPLv2 with MPL-2.0 userspace
Summary: Kernel GPLv2, userspace MPL-2.0.
Consequences: File-level copyleft keeps fixes open; weaker for proprietary linking stories.
Evidence: none

### Option D · Dual MIT/Apache-2.0 userspace
Summary: Kernel GPLv2, userspace dual MIT/Apache-2.0.
Consequences: Rust-ecosystem norm with patent grant available; two licenses to explain.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
