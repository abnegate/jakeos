# D-0107 · Decide redistribution policy for third-party Linux and Windows software
- Status: proposed
- Task: GOV-053
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §28, §49
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Flatpak, AppImage and Wine wrappers raise whose license applies to the wrapped payload; GOV records the legal policy PKG implements (§3, §28, §49).

## Options

### Option A · Redistribute only redistributable payloads
Summary: The repository hosts a payload only when its license permits redistribution.
Consequences: Legally clean hosting; many proprietary titles become pointer-only entries.
Evidence: none

### Option B · Redistribute with publisher permission
Summary: Payloads are hosted when the publisher grants explicit permission.
Consequences: More titles hosted; permission records to track and renew.
Evidence: none

### Option C · Never redistribute third-party payloads
Summary: The repository hosts wrappers that download the payload from the publisher at install time.
Consequences: No redistribution liability; installs depend on publisher URLs staying alive.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
