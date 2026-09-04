# D-0097 · Decide firmware blob redistribution for official versus non-free
- Status: proposed
- Task: GOV-022
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §33, §55, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 Intel-laptop Wi-Fi and GPU images cannot ship firmware blobs without redistribution rights (§33, §55, §62).

## Options

### Option A · Redistributable blobs in the official image
Summary: Blobs with redistribution rights ship; others are excluded.
Consequences: Hardware works out of the box; some devices never work.
Evidence: none

### Option B · Separate non-free repository
Summary: Blobs live in a non-free channel.
Consequences: Clean official image; extra step for users.
Evidence: none

### Option C · Download-on-demand at first boot
Summary: Blobs are fetched at first boot.
Consequences: No redistribution; first boot needs network, which Wi-Fi blobs defeat.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
