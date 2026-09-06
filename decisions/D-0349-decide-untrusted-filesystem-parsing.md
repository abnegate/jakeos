# D-0349 · Decide how untrusted removable and foreign filesystem images are parsed
- Status: proposed
- Task: STO-085
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §26, §51, §57
- Revisit when: a fuzzing campaign finds a mountable-image crasher in an allowlisted kernel parser, or the user-space option's measured cost changes class

## Context
Inherited C filesystem drivers parse attacker-controlled bytes in kernel mode whenever a removable drive or foreign partition is mounted (T-044). §26 forbids writing a new filesystem and HW-002 decides residency by measured cost, so the question is where the existing parsers run for untrusted volumes and what auto-mount does (§51, §57; I-009).

## Options

### Option A · Isolated user-space filesystem Component
Summary: Removable and foreign volumes are served by a user-space Component that hosts the inherited filesystem code behind the storage Interface.
Consequences: A parser bug is confined to that Component and its Capabilities; throughput and latency cost must be measured and may exclude the system volume.
Evidence: none

### Option B · Kernel parsers behind a type allowlist
Summary: Only allowlisted types mount in the kernel and unknown volumes are never auto-mounted; the rest are refused or offered read-only through a personality tool.
Consequences: No new hosting work; the allowlisted parsers remain kernel attack surface and users hit refusals.
Evidence: none

### Option C · Unrestricted kernel mounting
Summary: Mount as Linux does today with auto-mount on attach.
Consequences: Maximum compatibility and zero cost; every removable drive is a kernel-mode fuzzing session.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
