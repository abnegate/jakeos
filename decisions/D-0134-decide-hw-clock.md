# D-0134 · Decide hardware clock UTC versus localtime for dual-boot
- Status: proposed
- Task: INS-022
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
RTC UTC versus local time on Windows dual-boot machines must be decided so clocks do not drift by the timezone offset (§63).

## Options

### Option A · UTC with a documented Windows registry fix
Summary: RTC in UTC; document the Windows change.
Consequences: Correct; user action on Windows.
Evidence: none

### Option B · Localtime to match Windows default
Summary: RTC in local time.
Consequences: No Windows change; DST bugs.
Evidence: none

### Option C · UTC-only with a warning when Windows is detected
Summary: UTC always; warn on dual-boot.
Consequences: Correct with disclosure; Windows drifts.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
