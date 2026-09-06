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
Streaming services require a content decryption module. GAP-0302 sits at V2 so the native media player never grows one. Options are Widevine L3 through the personality browser's own CDM, no DRM playback at 1.0, or a native or Layer 1 secure-path CDM (§3, §48, §56.5, §57). Circumventing DRM is forbidden (I-071) and a Layer 1 secure path requires vendor trust chains the project cannot obtain. The accepted option states that native player and decoder Components ship no CDM and names the 1.0 compatibility statement MED-041 publishes; it sits on the browser decision (D-0022) and GOV-020's codec policy.

## Options

### Option A · Widevine L3 only through the Linux-personality browser CDM
Summary: DRM playback happens only inside the Linux-personality browser (Firefox per the owner's direction on D-0022) using its own Widevine L3 module; the native player and decoders have no CDM and the compatibility statement says so.
Consequences: Streaming services work in the browser at the quality L3 permits (typically up to 1080p), nothing DRM-related enters native code, and the project distributes no CDM. Native and Windows-personality players cannot play protected content, and the L3 ceiling is a visible gap against Windows.
Evidence: none

### Option B · No DRM playback at 1.0
Summary: No DRM playback anywhere at 1.0.
Consequences: No CDM dependency and no vendor terms to accept. Every major streaming service is unusable, which for a desktop OS is a daily-driving failure the L corpora would record.
Evidence: none

### Option C · Native CDM or Layer 1 secure-path
Summary: A native CDM Component or a Layer 1 secure video path for L1-class playback.
Consequences: Full-quality protected playback in native applications. Requires vendor trust chains, attestation and secure-path hardware agreements that are not available to an independent project at 1.0; recorded as rejected for 1.0 and left as a LATER question.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
