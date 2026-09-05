# D-0034 · Decide the CI platform with self-hosted KVM runners
- Status: accepted
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
Option A. Lint, unit tests and the roadmap validator run on GitHub-hosted runners; kernel boots, QEMU/KVM integration, hardware boots and every benchmark run on self-hosted runners in the lab (LAB-002). Benchmark runners are a quiet, dedicated subset that never runs unrelated jobs.

## Consequences
- Every required check that needs KVM or real hardware is labelled for the self-hosted pool; the roadmap validate check stays hosted.
- Lab machines register as runners with the same labels recorded in registers/hardware.md.
- Runner availability is a LAB responsibility and a steering signal when it blocks gates.

## Rejected options and why
- Option B (hosted runners with nested virtualisation) rejected: too slow for boot matrices and useless for performance numbers or real hardware.
- Option C (all jobs self-hosted) rejected: owning uptime for lint and unit jobs buys nothing.

## Follow-ups
none
