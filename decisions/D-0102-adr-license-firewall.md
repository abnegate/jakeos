# D-0102 · Decide the license firewall and outbound project licenses
- Status: accepted
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
Option A. Layer 1 (everything inside the kernel tree, including the native ABI implementation) is GPLv2-only like Linux. Layers 2 through 4 (runtime, IDL compiler output, SDK, frameworks, compositor, applications, personalities) are MIT. The ABI boundary is the licence boundary: user space links against Layer 1 only through the syscall and shared-page interfaces, never by including kernel code.

## Consequences
- Third-party and proprietary native software can link the SDK without copyleft obligations.
- Kernel-side Rust abstractions cannot be copied into user space; shared code must be written once in an MIT crate and vendored into the kernel under the GPLv2 exception process (KRN-003 consequences).
- Every repository carries an SPDX header lint (BLD) and a LICENSE file naming its layer.

## Rejected options and why
- Option B (Apache-2.0 userspace) rejected: Apache-2.0 is incompatible with GPLv2, which would forbid mixing personality code that borrows from GPLv2 Linux user space.
- Option C (MPL-2.0) rejected: file-level copyleft on the platform deters the SDK adoption §56.5 depends on.
- Option D (dual MIT/Apache-2.0) rejected: the extra patent grant is not worth the same GPLv2 incompatibility as Option B for any code that must interoperate with the personalities.

## Follow-ups
KRN-003 (accepted).
