# D-0027 · Decide status-tray policy: StatusNotifierItem compatibility versus none
- Status: proposed
- Task: APP-054
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §49, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V2 shell must either host StatusNotifierItem and Windows tray clients natively or refuse them with a documented denial (§49, §62), and native software never speaks those protocols.

## Options

### Option A · Native host for StatusNotifierItem plus Windows tray
Summary: The shell hosts both personality tray protocols.
Consequences: Compat apps look complete; two legacy protocols to implement and maintain.
Evidence: none

### Option B · Native host for one personality only
Summary: Only one personality's tray protocol is hosted.
Consequences: Half the maintenance; the other personality's apps lose tray items.
Evidence: none

### Option C · No tray with explicit deny plus Settings copy
Summary: Tray protocols are refused with a typed error and explained in Settings.
Consequences: No legacy protocol in the shell; users of tray-dependent apps lose functionality.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
