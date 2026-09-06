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
History can restore the OS, Packages or configuration separately (§31), but some combinations are unbootable or mismatched (an old OS with new Package objects that depend on newer services). This decision answers Q-026 by fixing which selective restores exist, the consistency checks that reject an inconsistent combination, and that a rejected partial restore leaves the current generation selected; it sits on the class ladder (D-0225) and the restore command (PKG-060).

## Options

### Option A · Restore-OS-only
Summary: Restore the OS generation only, keeping the current Package set.
Consequences: Undo a bad OS update without touching applications. Packages that depend on a service Interface version the older OS lacks must be detected, so the check is an Interface-version compatibility walk.
Evidence: none

### Option B · Restore-Packages-only
Summary: Restore the Package set only, keeping the current OS.
Consequences: Undo an application update without an OS rollback. Older Packages may need older Interface versions the current OS still serves under S-014 rules, so this is usually safe and the check is the same walk in the other direction.
Evidence: none

### Option C · Restore-configuration-only
Summary: Restore configuration only.
Consequences: Undo a settings change. Configuration schemas are versioned (D-0303), so restoring older settings onto newer applications runs their migration in reverse or is refused per schema.
Evidence: none

### Option D · Forbid inconsistent combinations
Summary: Any combination whose consistency check fails is refused with a typed reason listing the conflicting objects; the current generation stays selected.
Consequences: Never produces an unbootable or mismatched system. The rule accompanies A to C rather than replacing them, and the checks must be complete enough that a refusal is trusted.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
