# D-0300 · Decide native init versus retained initramfs/systemd for early boot
- Status: proposed
- Task: SVC-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The native init Component must take over from Linux early userspace without fossilising systemd into the native boot path (§30, §32).

## Options

### Option A · Native init from the first instruction after kernel handoff
Summary: Immediate native init.
Consequences: Pure; unlock and verity in native.
Evidence: none

### Option B · Native init after root-store unlock and verity setup
Summary: Native init after unlock.
Consequences: Pragmatic; a small Linux stage.
Evidence: none

### Option C · Native init after a systemd handoff
Summary: Native init after systemd.
Consequences: Reuse; systemd fossilisation.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
