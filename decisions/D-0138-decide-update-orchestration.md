# D-0138 · Decide client update orchestration, metered links and deferral
- Status: proposed
- Task: INS-009
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Background download, apply-on-reboot versus live activation, metered-network behaviour and deferral must precede the V3 updater (§30, §63, I-086).

## Options

### Option A · Reboot-only apply
Summary: Updates apply only at reboot.
Consequences: Simple and consistent; a reboot for every update.
Evidence: none

### Option B · Live userspace activation with kernel reboot for kernel changes
Summary: Userspace switches live and the kernel needs a reboot.
Consequences: Fewer reboots; mixed-version risk (T-034).
Evidence: none

### Option C · Defer-until-idle with a deadline
Summary: Updates apply when idle before a deadline.
Consequences: Unobtrusive; deadlines to enforce.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
