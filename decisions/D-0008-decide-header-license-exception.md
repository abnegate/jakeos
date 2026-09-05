# D-0008 · Decide whether ABI headers carry a syscall-note-style exception for native programs
- Status: accepted
- Task: ABI-029
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether Native ABI headers carry a Linux-syscall-note-style exception decides if native userspace programs can ever be derivative works of the kernel (§65); GOV-003 is the input.

## Options

### Option A · Syscall-note-style exception on Layer 1 headers
Summary: Layer 1 headers carry an explicit exception stating that programs using them are not derivative works of the kernel.
Consequences: Proprietary native applications are clearly permitted; the exception text must be maintained alongside the kernel license.
Evidence: none

### Option B · SDK-license-only headers
Summary: Headers are published only under the SDK license with no kernel-side exception.
Consequences: Simple licensing story for developers; leaves the derivative-work question about the syscall boundary unanswered on the kernel side.
Evidence: none

### Option C · Dual-licensed headers
Summary: Headers are dual-licensed under the kernel license and the SDK license.
Consequences: Maximum flexibility for consumers; two licenses on one artifact invite confusion about which applies to the boundary.
Evidence: none

## Decision
Option A. Layer 1 ABI headers remain GPLv2 with a syscall-note-style exception: including the headers to make system calls, or to define the ABI structures needed to do so, does not make the including program a derivative work. This is the Linux UAPI model; the SDK regenerates typed bindings from the same definition under MIT.

## Consequences
- The exception text is recorded in the kernel repository and cited by every Layer 1 header.
- The IDL compiler consumes the same machine-readable ABI definition, so the MIT bindings never copy header text.
- GOV lists the exception in the licence firewall documentation.

## Rejected options and why
- Option B (SDK-licence-only headers) rejected: the kernel tree must still ship its own ABI definition for testing and conformance, so two copies would exist.
- Option C (dual-licensed headers) rejected: two licences on the most permanent files in the project for no gain over the exception.

## Follow-ups
none
