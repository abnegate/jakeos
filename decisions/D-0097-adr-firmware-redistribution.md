# D-0097 · Decide firmware blob redistribution for official versus non-free
- Status: accepted
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
Option A. The official image ships firmware blobs whose licences permit redistribution (the redistributable subset of linux-firmware), so reference hardware works out of the box. Firmware that may not be redistributed is never shipped; hardware that requires it is excluded from Tier 1 by HW-003 criteria.

## Consequences
- The notices bundle (REL) lists every shipped firmware file and its licence.
- The hardware register records the firmware each Tier 1 machine needs; a non-redistributable requirement disqualifies a candidate.
- The installer never downloads firmware from third parties.

## Rejected options and why
- Option B (separate non-free repository) rejected: reference laptops would lack Wi-Fi after install until the user finds the second repository.
- Option C (download on demand) rejected: it needs a network on machines whose network firmware is the thing missing.

## Follow-ups
none
