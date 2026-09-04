# D-0090 · Decide codec and proprietary-font shipping and patent policy
- Status: proposed
- Task: GOV-020
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §56.5, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
MED packages codecs at V1, so default-image royalty-free codecs, hardware passthrough of encumbered codecs, optional Packages, jurisdiction splits and proprietary-font policy must be settled in one Decision (§56.5, §57).

## Options

### Option A · Royalty-free-only default image with hardware passthrough
Summary: The default image ships only royalty-free codecs and passes H.264, HEVC and AAC through to hardware.
Consequences: Minimal patent exposure; software fallback for encumbered formats is absent.
Evidence: none

### Option B · Software H.264/HEVC/AAC in optional Packages
Summary: Encumbered software codecs ship as user-installed optional Packages.
Consequences: Complete playback on demand; the optional channel carries patent risk.
Evidence: none

### Option C · Jurisdiction-split images
Summary: Separate images per jurisdiction.
Consequences: Exact legal fit; build, test and distribution matrix doubles.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
