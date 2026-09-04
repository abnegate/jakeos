# D-0233 · Decide the suspend state for V1 reference machines
- Status: proposed
- Task: PWR-002
- Surfaces: none
- Layer: none
- Spikes: PWR-004
- Supersedes: none
- Superseded by: none
- Baseline: §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1-G07 requires automated suspend and resume on H-004 and H-002; mixing s2idle and S3 without a Decision makes the harness untestable (§61).

## Options

### Option A · s2idle only
Summary: s2idle everywhere.
Consequences: Modern; firmware quality varies.
Evidence: none

### Option B · S3 only
Summary: S3 everywhere.
Consequences: Deep sleep; unsupported on newer laptops.
Evidence: none

### Option C · s2idle default with per-machine S3 fallback
Summary: s2idle with fallback.
Consequences: Coverage; two paths.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
