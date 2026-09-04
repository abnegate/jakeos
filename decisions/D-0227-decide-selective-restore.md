# D-0227 · Decide selective restore semantics and how partial restore avoids inconsistency
- Status: proposed
- Task: PKG-071
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Selective restore semantics and how partial restore avoids inconsistency must be decided (§31), answering Q-026.

## Options

### Option A · Restore-OS-only
Summary: Only the OS is restored.
Consequences: Useful; consistency checks.
Evidence: none

### Option B · Restore-Packages-only
Summary: Only Packages are restored.
Consequences: Useful; checks.
Evidence: none

### Option C · Restore-configuration-only
Summary: Only configuration is restored.
Consequences: Useful; checks.
Evidence: none

### Option D · Forbid inconsistent combinations
Summary: Listed combinations are refused.
Consequences: Safety; fewer options.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
