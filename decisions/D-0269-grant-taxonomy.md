# D-0269 · Decide user-mediated grant taxonomy
- Status: proposed
- Task: SEC-007
- Surfaces: S-022
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every Capability an application receives from the user arrives through a chooser (the user picks the object), a prompt (the user answers a question) or a settings page (the user pre-authorises), and lasts one time, one session, persistently or until revoked (§9.1). The taxonomy must be accepted before the Package capability-request schema (D-0220) so requests match grant classes, and it decides whether the model is usable or ignored through prompt fatigue (T-001, T-012, I-060). It also answers Q-013: which scheduling intents (Realtime, LowLatency) are revocable grants and how an unprivileged Component is kept from starving others. S-022 stays prototyped.

## Options

### Option A · Chooser versus prompt versus settings-only with durations
Summary: Three classes with four durations: chooser for objects (files, devices, contacts), prompt for ambient-but-sensitive rights (camera, location, Realtime intent) with one-time or session duration by default, settings-only for rights that need no moment of use (autostart, background networking); persistent grants are revocable in Settings and listed by `os inspect`.
Consequences: Prompts appear only where a chooser cannot express the grant, which is the known cure for prompt fatigue, and every grant is visible and revocable. Each Capability kind must be classified and the classification defended, the chooser UI (APP) is on the critical path of every object grant, and the schema on S-022 carries class and duration.
Evidence: none

### Option B · Prompt-everything
Summary: Every grant is a prompt.
Consequences: One interaction pattern, trivially complete. Users learn to click through, which removes the security the model exists for (T-012), and file access through a prompt cannot express which file.
Evidence: none

### Option C · Settings-only for all classes
Summary: Every grant is pre-authorised in Settings; applications never ask at the moment of use.
Consequences: No interruptions and no prompt fatigue. Users cannot discover what to enable until something fails, first-run experience is a wall of toggles, and a chooser-shaped grant (this file, now) has no expression, so applications ask for whole folders.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
