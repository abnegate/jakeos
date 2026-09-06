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
On a machine shared with Windows, the real-time clock is either UTC (the Linux default) or local time (the Windows default); mismatch makes the clock jump by the timezone offset on every reboot into the other system (§63). The V3 installer applies the choice; SVC's time service (D-0304, SVC-032) keeps it after first boot. The accepted option states what the installer writes to the RTC and shows when a Windows Boot Manager entry is present.

## Options

### Option A · UTC with a documented Windows registry fix
Summary: The RTC is UTC; when Windows is detected the installer shows the documented registry setting (`RealTimeIsUniversal`) that makes Windows agree and offers to write it to the Windows volume.
Consequences: Correct time semantics on the JakeOS side and a permanent fix once applied. The fix modifies the Windows registry, which the installer may not be permitted to touch (BitLocker), and it is undone by some Windows updates.
Evidence: none

### Option B · Localtime to match Windows default
Summary: The RTC is local time whenever Windows is detected, matching Windows.
Consequences: Windows needs no change. Local-time RTCs are ambiguous across DST transitions and the time service must adjust the RTC on every DST change; Windows still fights over the hour if both systems adjust.
Evidence: none

### Option C · UTC-only with a warning when Windows is detected
Summary: The RTC is always UTC; when Windows is detected the installer warns that Windows will show a wrong time until the user applies the documented setting.
Consequences: Correct and honest with no writes to foreign volumes. Windows drifts by the offset until the user acts, which most will experience as a JakeOS bug.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
