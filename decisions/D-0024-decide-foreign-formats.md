# D-0024 · Decide which foreign Package formats map to Personality launches
- Status: proposed
- Task: APP-052
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §49
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Foreign Package formats (.exe, .msi, .AppImage, .deb, .rpm, Flatpak) must map to Personality launches so a double-click feels native with no compatibility wizard (§49).

## Options

### Option A · Open selected formats only
Summary: Only a chosen subset opens via personality launch; others are unsupported.
Consequences: Small, well-tested surface; users hit unsupported formats.
Evidence: none

### Option B · Open all listed formats via personality
Summary: Every listed format opens through its personality.
Consequences: Nothing feels broken; deb and rpm installs inside a personality need a defined install story.
Evidence: none

### Option C · Ask once per format
Summary: The first open of each format asks the user, then remembers.
Consequences: Consent without repeated prompts; a wizard-like step on first contact.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
