# D-0279 · Decide where assistant models execute
- Status: proposed
- Task: SEM-017
- Surfaces: none
- Layer: none
- Spikes: SEM-032
- Supersedes: none
- Superseded by: none
- Baseline: §37, §44, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The assistant demo needs somewhere to run a model: a local ComputeDevice or NPU through HET (§37), a remote service, or either at the user's choice (§44). The OS provides Capability plumbing and never bundles a model runtime or ships weights as a privileged service (§57); HET owns device enumeration (HET-001). SEM-032's spike measures local inference on the reference hardware; this decision cites it.

## Options

### Option A · Local ComputeDevice or NPU only
Summary: Models run only locally on a ComputeDevice or NPU the user grants; the assistant is a Package that brings its own runtime and weights.
Consequences: Nothing leaves the machine, so the privacy story is simple and grants are the only authority. Model quality is bounded by local hardware, the reference desktop has no NPU until H-007, and battery cost on laptops is real.
Evidence: `reports/spikes/SEM-032.md`

### Option B · Remote service only
Summary: Models run only on a remote service the assistant Package holds a network grant for.
Consequences: Any model size and no local hardware dependence. Every prompt and the objects it touches leave the machine, which the grant taxonomy must make explicit per invocation, and the assistant is unusable offline.
Evidence: `reports/spikes/SEM-032.md`

### Option C · User-selectable both
Summary: The assistant Package may use either; the user selects per assistant and the grant shows which is active.
Consequences: Choice with an explicit indicator. Two execution paths to test, and the indicator must be trustworthy UI (T-012) so a Package cannot claim local while sending remote.
Evidence: `reports/spikes/SEM-032.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
