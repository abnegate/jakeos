# D-0032 · Decide the visible-UI measurement boundary
- Status: accepted
- Task: BEN-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §34, §54
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0.5 startup gates are incomparable unless the visible-UI boundary is fixed; B-016 cites Q-029 and this Decision answers it (§34, §54).

## Options

### Option A · First compositor presentation of a non-blank frame
Summary: Startup ends when the compositor presents the first non-blank frame.
Consequences: Measurable in software and comparable across apps; ignores display latency.
Evidence: none

### Option B · First client commit
Summary: Startup ends when the client commits its first frame.
Consequences: Cheapest to measure; a committed frame may not yet be visible.
Evidence: none

### Option C · First photodiode edge
Summary: Startup ends when a photodiode sees the display change.
Consequences: True end-to-end; needs lab hardware and is not comparable across systems.
Evidence: none

## Decision
Option A. The application-startup clock (B-016) stops at the compositor's first scanout of a frame from the new application that is not the placeholder colour, timestamped by the compositor and attributed to the Component. This is measurable on QEMU in CI; input-to-photon (B-019) remains a separate metric measured with the lab rig.

## Consequences
- The compositor emits a first-real-frame trace event per Component (GFX, OBS).
- Startup reports in CI cite this event; hardware reports may add the photodiode figure alongside.
- The placeholder colour and the non-blank test are specified in the harness definition.

## Rejected options and why
- Option B (first photodiode edge) rejected: it ties every startup measurement to the lab rig.
- Option C (first client commit) rejected: it stops before anything is visible and flatters the number.

## Follow-ups
none
