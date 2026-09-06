# D-0325 · Decide global shortcut model: named actions bound in Settings, no key grabs
- Status: proposed
- Task: UIP-030
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9, §41
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Push-to-talk, screenshots and media keys need keys that work while the application is unfocused, and the X11 answer, a global key grab, is a keylogger primitive (T-012, §9, §41). This decision fixes the model the V1 shortcut and focus work depends on and UIP-045 implements at V2: applications request named actions that the user binds to keys in Settings and never receive raw global key events; a Component receives key events while unfocused only for actions the user bound to it.

## Options

### Option A · Named actions bound in Settings, no raw grabs
Summary: An application declares named actions ("Toggle mute", "Capture region") in its manifest; the user binds keys in Settings; the compositor delivers the action, not the key, to the unfocused Component.
Consequences: No Component can observe keystrokes it was not given, and the user sees every global binding in one place. Applications cannot ship default global bindings without user confirmation, so first-run experience for push-to-talk asks the user to bind a key, and the compositor holds the binding table.
Evidence: none

### Option B · Compositor-reserved media and screenshot keys only
Summary: Only compositor-reserved keys exist (media, brightness, screenshot, lock); applications cannot register global actions at all.
Consequences: Minimal surface and nothing to bind. Push-to-talk, streaming and accessibility switches have no path, so those applications are unsupported or rely on the personality's X11 grabs, which is worse.
Evidence: none

### Option C · X11-style global key grabs
Summary: Applications grab keys globally as under X11.
Consequences: Every existing application's shortcut code works. It is exactly the keylogger primitive T-012 describes; rejected, and the Wayland bridge must not expose it either.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
