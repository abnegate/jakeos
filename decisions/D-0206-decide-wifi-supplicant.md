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
The Wi-Fi supplicant authenticates the laptop to every network it joins, so it is the security and maintenance surface of V1 daily driving (§5.1, §61, V1-G05). cfg80211 and mac80211 stay the retained mechanism (I-009, I-054); this decision picks what drives them and which Component hosts it under the C-library strategy of SDK-097. Each option must record WPA2 and WPA3 personal, hidden-network and enterprise EAP consequences for NET-021 and NET-028. APP owns the picker UI.

## Options

### Option A · iwd
Summary: iwd, hosted as a personality-hosted helper under a native Wi-Fi service Component that owns the `NetworkConnection` grants and the shell Interface.
Consequences: Small, actively maintained, uses kernel crypto so it carries little of its own, and its D-Bus API maps onto a typed Interface cleanly; WPA3 personal and hidden networks work. Enterprise EAP coverage is narrower than wpa_supplicant's (some TTLS and PEAP variants, certificate handling), which NET-028 must test per method, and iwd owns its own network configuration store that the settings model (D-0303) must wrap.
Evidence: none

### Option B · wpa_supplicant
Summary: wpa_supplicant, hosted the same way.
Consequences: Every EAP method, every driver quirk and twenty years of interoperability. A large C code base with a history of security fixes running as the network's authenticating agent, a control interface designed for scripts, and slower roaming than iwd on the reference laptop.
Evidence: none

### Option C · Native Rust supplicant
Summary: A native Rust supplicant over nl80211.
Consequences: Memory safety in the most exposed network component and a typed Interface with no helper hosting. A from-scratch reimplementation of EAP, SAE and the 802.11 state machines is security-critical work with a long tail of interoperability bugs, and it cannot be ready for V1-G05; viable only as a later replacement measured against option A.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
