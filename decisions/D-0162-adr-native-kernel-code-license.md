# D-0162 · Decide the licence for new native kernel code
- Status: accepted
- Task: KRN-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §5.1, §50
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
New code linked into a GPLv2 kernel must be GPLv2-compatible and the choice decides whether kernel abstractions can be shared with permissive userspace (§5.1, §50), before the first native commit.

## Options

### Option A · GPLv2-only
Summary: New kernel code is GPLv2-only.
Consequences: Matches Linux exactly; nothing can be copied to permissive userspace crates.
Evidence: none

### Option B · GPLv2-or-later
Summary: New kernel code is GPLv2-or-later.
Consequences: Future license flexibility; still copyleft and not shareable with userspace.
Evidence: none

### Option C · Dual GPLv2/MIT for reusable abstractions
Summary: Selected reusable files are dual-licensed.
Consequences: Abstractions can be shared with userspace; per-file license tracking.
Evidence: none

## Decision
Option A. All new native kernel code is GPLv2-only, the same licence as Linux. No file in the kernel tree carries a second licence. Code intended for reuse in user space is authored as an MIT crate in the platform repository and vendored into the kernel tree under a recorded exception listing the crate and version.

## Consequences
- One licence inside the kernel tree means no boundary to police between native and inherited code.
- The vendoring exception list is a GOV register-like file in the kernel repository and is reviewed at every upstream merge.
- SPDX headers are enforced by lint (BLD).

## Rejected options and why
- Option B (dual GPLv2/MIT for reusable abstractions) rejected: per-file dual licensing invites accidental copyleft leaks and complicates upstream-first contribution of the same files.
- Option C (GPLv2-or-later) rejected: it differs from Linux without practical gain and would block copying code between native and inherited files.

## Follow-ups
none
