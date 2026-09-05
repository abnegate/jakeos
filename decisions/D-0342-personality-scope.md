# D-0342 · Accept the Windows Personality scoping Decision
- Status: accepted
- Task: WIN-001
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §48, §49, §56.2, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The only V0 Windows deliverable: the personality starts from Wine and Proton, is not a clone, and native software never sees Win32 (§3, §48, §49, §56.2, §57).

## Options

### Option A · Wine/Proton starting point with no obvious VM, separate desktop or wizard
Summary: Start from Wine and Proton.
Consequences: Practical; Wine limits.
Evidence: none

### Option B · Defer the Windows personality past 1.0
Summary: Defer.
Consequences: Focus; no Windows.
Evidence: none

### Option C · Clean-room Win32 implementation without Wine
Summary: Clean-room.
Consequences: Control; infeasible.
Evidence: none

## Decision
Option A. The Windows personality starts from Wine and Proton running on the Linux personality, with native bindings replacing Linux dependencies over time. A user double-clicks an .exe and gets a normal window: no visible VM, no separate desktop, no compatibility wizard (§49). Gaming is a major objective (§48). Kernel-level anti-cheat and vendor DRM playback are excluded from 1.0 and stated publicly as unsupported.

## Consequences
- V1 carries Wine bring-up and the Wine test suite as non-gated work; V2 is the first gated Windows milestone (W1 corpus).
- The VIRT workstream provides the fallback for software that cannot run under the personality.
- WIN never exposes Win32 or NT concepts to native software (§3).

## Rejected options and why
- Option B (defer past 1.0) rejected: Windows software, and games in particular, are existential for adoption (§56.5).
- Option C (clean-room Win32) rejected: it discards decades of Wine work for no architectural gain (§48).

## Follow-ups
none
