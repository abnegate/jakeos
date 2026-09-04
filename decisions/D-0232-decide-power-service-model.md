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
§61 names power management as a V1 requirement without a Power Component or inhibit model, so the Layer 2 power service model over retained ACPI is the PWR baseline-gap scope (§2, §22, §32, §61, §66).

## Options

### Option A · Userspace Power Component over retained Linux ACPI
Summary: A userspace Component over ACPI.
Consequences: Minimal kernel; ACPI retained.
Evidence: none

### Option B · Native kernel Object<Power> as Layer 1
Summary: A kernel Power object.
Consequences: First-class; an L1 surface that cannot freeze before V4.
Evidence: none

### Option C · logind-shaped native API
Summary: A logind-like API.
Consequences: Familiar; rejected under §57 and I-006.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
