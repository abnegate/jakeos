# D-0189 · Decide Widevine L3 Personality path and native CDM non-goals
- Status: proposed
- Task: MED-023
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §48, §56.5, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Widevine L3 through the personality, no DRM at 1.0, or a native or Layer 1 CDM must be decided so the native player never grows a CDM (§3, §48, §56.5, §57, I-071).

## Options

### Option A · Widevine L3 only through the Linux-personality browser CDM
Summary: DRM playback happens only in the personality browser.
Consequences: Streaming services work in the browser; the native player has no DRM and says so.
Evidence: none

### Option B · No DRM playback at 1.0
Summary: No DRM anywhere.
Consequences: Simplicity and no CDM dependency; no streaming services at all.
Evidence: none

### Option C · Native CDM or Layer 1 secure-path
Summary: A native or kernel-level CDM.
Consequences: Full support; vendor trust chains are unavailable, so rejected for 1.0.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
