# D-0335 · Decide the kernel-level anti-cheat policy
- Status: proposed
- Task: WIN-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §48, §56.2, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Kernel-level anti-cheat drivers cannot run under the Capability model (Q-043, I-071); the policy must be fixed (§48, §56.2, §57).

## Options

### Option A · Refuse kernel-level anti-cheat in 1.0
Summary: Refuse.
Consequences: Integrity; titles excluded.
Evidence: none

### Option B · Refuse in the personality and offer the VIRT fallback
Summary: Refuse plus VM.
Consequences: A path for titles; VM experience.
Evidence: none

### Option C · Vendor engagement loading anti-cheat kernel drivers
Summary: Engage vendors.
Consequences: Titles; rejected if it breaks the model.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
