# D-0179 · Decide xdg-desktop-portal as the native grant bridge
- Status: proposed
- Task: LNX-018
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §25, §47
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Sandboxed Linux applications already speak xdg-desktop-portal for file choosing, clipboard, screen sharing and notifications (T-002). This decision fixes whether portals are the single bridge through which personality applications reach the native Capability grants (§9.1, §25, §47), which portal interfaces map onto UserSelected, Notify and screen-share Capabilities, and rejects ambient home access for Linux GUI applications (I-035).

## Options

### Option A · Portals as the sole grant bridge
Summary: The personality implements the portal D-Bus interfaces as the sole grant path: FileChooser mints UserSelected objects, Notification maps to Notify, ScreenCast to the screen-share Capability, and applications without portal support get nothing beyond their Package grants.
Consequences: Applications that already use portals work unchanged with native security, and one implementation serves Flatpak and non-Flatpak alike. Applications that never adopted portals (older toolkits, many games' launchers) see no home directory and fail until a compatibility grant is configured.
Evidence: none

### Option B · Second Personality-specific permission model
Summary: A personality-specific permission model with its own prompts beside the portals.
Consequences: Coverage for applications that do not speak portals. Two permission systems for one personality, two prompt vocabularies for the user, and every grant class implemented twice.
Evidence: none

### Option C · Ambient home access for Linux GUI apps
Summary: Linux GUI applications receive the user's home directory ambiently, as on a conventional desktop.
Consequences: Everything works immediately. The personality becomes the universal exfiltration path around the native model, which I-035 forbids; recorded as rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
