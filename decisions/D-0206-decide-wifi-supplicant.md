# D-0206 · Decide the Wi-Fi supplicant: iwd, wpa_supplicant or native Rust
- Status: proposed
- Task: NET-009
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §5.1, §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The Wi-Fi supplicant is the security and maintenance surface for V1 laptop daily-driving (§5.1, §61), over retained cfg80211 and mac80211.

## Options

### Option A · iwd
Summary: iwd is the supplicant.
Consequences: Small and modern; fewer enterprise EAP features.
Evidence: none

### Option B · wpa_supplicant
Summary: wpa_supplicant is the supplicant.
Consequences: Complete; a large C surface.
Evidence: none

### Option C · Native Rust supplicant
Summary: A Rust supplicant is written.
Consequences: Memory safety; a rewrite of a security-critical component.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
