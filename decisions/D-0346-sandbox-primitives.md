# D-0346 · Retention and exposure of Linux sandbox primitives
- Status: proposed
- Task: LNX-021
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §36, §46
- Revisit when: a Chromium, Flatpak or container engine on the Personality cannot meet V1 Gates without changing the retained set

## Context
V1 daily-driving of a GPU-accelerated Linux browser and IDE requires seccomp-bpf, user namespaces, overlayfs and FUSE to keep working from the Linux Personality (§46). Native software must not see those primitives as the native isolation API (§3). This Decision records which of those mechanisms stay in the kernel, which are exposed only through the Personality, and which are declined.

## Options

### Option A · Retain all four, Personality-only
Summary: Keep seccomp-bpf, user namespaces, overlayfs and FUSE in the kernel; expose them only to Linux-personality processes.
Consequences: Chromium, Flatpak/bubblewrap and podman keep their upstream sandbox; native Components never receive these as native objects; the retained surface is larger.
Evidence: none

### Option B · Retain a subset and emulate the rest
Summary: Keep overlayfs and FUSE; drop or stub user namespaces and seccomp-bpf and emulate their effects in the Personality.
Consequences: Smaller retained kernel surface; browser and container sandboxes that require the dropped primitives fail V1 Gates unless the Personality reimplements them.
Evidence: none

### Option C · Expose the four as native objects
Summary: Mint native Capability types for seccomp filters, user namespaces, overlayfs mounts and FUSE.
Consequences: Native software would see POSIX-shaped isolation APIs, violating §3 and §57; Personality and native models collapse into one.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
LNX-061.
