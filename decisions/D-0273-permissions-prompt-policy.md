# D-0273 · Decide permission prompt policy against fatigue
- Status: proposed
- Task: SEC-043
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
D-0269 fixed the grant classes; this decision fixes the runtime rules for prompts (§9.1): the default persistence of a granted prompt, how repeated prompts are coalesced without granting silently, and the escape into Settings. Prompt-everything reproduces the deny-list experience users click through (T-012, R-041, I-060). It sits on the taxonomy (SEC-007) and the threat model (SEC-002).

## Options

### Option A · Prompt every time
Summary: Every use of a prompt-class Capability prompts.
Consequences: Maximum explicitness. Users habituate and click through within a day, which removes the protection (T-012); rejected as the default and kept as an optional strict mode.
Evidence: none

### Option B · Session default with a settings escape
Summary: A prompt-class grant persists for the session by default; Settings shows it and can make it persistent or revoke it; repeated requests within a session are coalesced into the first prompt.
Consequences: One prompt per application per right per session, which is the level users tolerate, and nothing is granted without a prompt at least once. Session persistence means a compromised application keeps the right until logout, and the coalescing rule must be defined per right.
Evidence: none

### Option C · Chooser silent, prompt once-per-session, settings-only never prompt
Summary: Chooser-class grants never prompt (the choice is the consent), prompt-class grants prompt once per session, settings-only grants never prompt and are enabled in Settings.
Consequences: Prompts appear only where a moment of use exists and cannot be expressed by a chooser, which is the minimum. Requires every Capability to be classified (D-0269 already does) and Settings to be discoverable for the settings-only class.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
