# D-0232 · Decide the Layer 2 power service model over retained ACPI
- Status: proposed
- Task: PWR-001
- Surfaces: none
- Layer: none
- Spikes: PWR-005
- Supersedes: none
- Superseded by: none
- Baseline: §2, §22, §32, §61, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§61 requires power management at V1 (V1-G07: suspend and resume on H-002 and H-004) but names no Power Component, no inhibit model and no statement of whether ACPI is a kernel Object. ACPI is a mature retained mechanism (§2, I-009, I-054) and is not rewritten; the question is where suspend, shutdown, Battery, DisplayPower, InhibitIdle and InhibitSuspend live, which are Layer 2 Interfaces and which are Layer 1, with no Layer 1 power surface frozen before V4 (I-040, §66). Native software never sees logind, sysfs or POSIX power APIs (§57, I-006). PWR-005 studies the options.

## Options

### Option A · Userspace Power Component over retained Linux ACPI
Summary: A user-space Power Component owned by SVC drives retained ACPI and exposes Power, Battery, DisplayPower and the inhibit Interfaces at Layer 2.
Consequences: The kernel gains nothing new and the power model is ordinary typed Interfaces that evolve under S-014 rules; inhibits are Capabilities held by the Component that needs them and drop when it dies. The Component is on the path of every suspend, so it is supervised and its restart must not lose inhibits; wake-source and lid policy live in user space.
Evidence: `reports/spikes/PWR-005.md`

### Option B · Native kernel Object<Power> as Layer 1
Summary: A native kernel `Object<Power>` at Layer 1 with suspend, shutdown and inhibit as kernel Operations.
Consequences: First-class and available before any service is up, and inhibits are kernel-enforced against a dying holder. A Layer 1 surface that cannot freeze before V4 and cannot change after, for a domain whose policy (lid, dock, thermal) is exactly what changes; it also moves policy into the kernel against D-0157.
Evidence: `reports/spikes/PWR-005.md`

### Option C · logind-shaped native API
Summary: A logind-shaped native API mirroring the systemd-logind methods and signals.
Consequences: Familiar to Linux developers and easy for the personality to map. It is a POSIX-era API surface presented as native, which §57 and I-006 forbid; recorded so it is never re-proposed, with the personality's logind mapped onto option A instead.
Evidence: `reports/spikes/PWR-005.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
