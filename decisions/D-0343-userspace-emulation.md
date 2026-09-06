# D-0343 · Decide that Win32 emulation stays in userspace
- Status: proposed
- Task: WIN-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §48, §5.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Wine is LGPL and the kernel is GPLv2 (D-0162). Putting NT object emulation or a PE loader into the kernel would either duplicate Wine's work under an incompatible licence or pull LGPL code across the boundary (Q-050, §48, §5.1). This decision fixes that all Win32, NT and PE emulation stays in user space, or names the exception, and keeps native software free of Win32 types (I-007, §3). It constrains the architecture before bring-up.

## Options

### Option A · All Win32, NT and PE emulation in userspace
Summary: Every part of the Windows personality (PE loading, NT object model, Win32 API, WoW64) is user-space code hosted per D-0345; the kernel gains nothing Windows-specific.
Consequences: Licensing is clean, Wine and Proton are used as upstream ships them, and the kernel's threat surface does not grow. Some NT semantics (object waits, asynchronous procedure calls) cost more in user space than a kernel implementation would, which the corpora measure.
Evidence: none

### Option B · PE loader in-kernel
Summary: The kernel loads PE binaries directly with a binfmt-style loader.
Consequences: Faster process start and no user-space loader indirection. A PE loader in GPLv2 code either reimplements Wine's LGPL loader or links it, and kernel-side PE parsing is a new parser of untrusted input in the kernel (T-044 class); rejected.
Evidence: none

### Option C · NT objects in-kernel
Summary: NT kernel objects (events, mutants, sections) are implemented as kernel Objects for fidelity.
Consequences: Closest behaviour for tricky synchronisation-heavy titles. It grows the native kernel ABI with Windows shapes that native software could see (I-007), and duplicates wineserver under GPLv2; rejected, with performance work directed at wineserver and the native Object mapping (WIN-036) instead.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
