# D-0034 · Decide the CI platform with self-hosted KVM runners
- Status: proposed
- Task: BLD-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §55, §59
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Boot, graphics, performance and hardware jobs need nested virtualization or lab devices hosted CI cannot provide (§55, §59), so the CI platform and runner policy must precede CI tiers and the QEMU harness.

## Options

### Option A · Self-hosted KVM runners with hosted lint and unit only
Summary: Boot, graphics, performance and hardware jobs run on self-hosted KVM; lint and unit tests may use hosted runners.
Consequences: Hardware-class jobs are reliable; the project operates runner infrastructure.
Evidence: none

### Option B · All jobs self-hosted
Summary: Every job runs on project runners.
Consequences: One environment; more operational load and no hosted burst capacity.
Evidence: none

### Option C · Hosted runners with nested virtualization for boot jobs
Summary: Hosted runners with nested KVM run boot jobs.
Consequences: No infrastructure to run; nested virtualization is slow and unavailable for graphics and hardware.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
