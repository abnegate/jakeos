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
V0 boots Linux init from a retained initramfs with native Components beside it (D-0051). V0.5 introduces native init (SVC) and must say where it takes over from Linux early userspace, how disk unlock, verity setup and service supervision sit on that path, and which Component owns reboot and halt (§30, §32). Reusing systemd past the handoff would fossilise Linux semantics into the native boot path (GAP-0170); the retained initramfs is a V0 deliverable only (R-010), and native supervision must never be confused with systemd inside the Linux personality (R-072).

## Options

### Option A · Native init from the first instruction after kernel handoff
Summary: The kernel hands off directly to the native init Component, which performs disk unlock, verity setup, mounts and supervision itself.
Consequences: One boot path with no Linux early userspace at all, so measured boot, crash capture and `os inspect` see the whole sequence natively. Unlock, verity, LVM or dm setup and firmware loading are re-implemented natively in Rust before V0.5 can boot, which is the largest SVC and BOOT effort in the rung.
Evidence: none

### Option B · Native init after root-store unlock and verity setup
Summary: A minimal retained Linux stage (a static initramfs or UKI stub) unlocks the root store and sets up verity, then execs native init, which supervises everything after.
Consequences: The hard early-boot mechanisms stay retained (§2) and native init owns everything a user can observe; the Linux stage is a few hundred lines with no service manager. Two stages appear in measured boot and boot timing, and the boundary between them (what the stage hands to native init) is a documented contract.
Evidence: none

### Option C · Native init after a systemd handoff
Summary: systemd boots as it does on Linux and native init is started as a systemd service that takes over supervision of native Components.
Consequences: Maximum reuse and the fastest V0.5 boot to demonstrate. systemd remains the real PID 1 and owns reboot, mounts and cgroups, so native supervision is a tenant of Linux semantics, R-072 is realised and the native boot path can never drop systemd without a rewrite; recorded as rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
