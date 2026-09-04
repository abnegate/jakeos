# D-0045 · Decide ESP policy: reuse the existing OEM ESP or create a dedicated ESP for Generation entries
- Status: proposed
- Task: BOOT-029
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V2 laptops ship undersized Windows-created OEM ESPs and V3 dual-boot depends on the layout, so ESP policy must be fixed with Windows Boot Manager as a constraint.

## Options

### Option A · Reuse the existing ESP with compact entries
Summary: Generation entries are kept small enough to fit alongside Windows.
Consequences: No repartitioning; at least three generations may not fit and install must refuse with a typed error.
Evidence: none

### Option B · Dedicated ESP
Summary: The installer creates a second ESP for JakeOS.
Consequences: Plenty of room; firmware and Windows may behave inconsistently with two ESPs.
Evidence: none

### Option C · Reuse plus an XBOOTLDR-style extended partition
Summary: The OEM ESP holds the loader and an extended partition holds generation images.
Consequences: Room without a second ESP; requires bootloader support for the extended partition.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
