# D-0079 · Decide the compositor frame scheduling model
- Status: proposed
- Task: GFX-015
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §40, §22
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0.5 exit requires an accepted frame scheduling model so client frame callbacks align to display deadlines (§40, §22); this is the compositor's callback model, not SCH intent.

## Options

### Option A · Fixed vblank-aligned callbacks
Summary: Clients are called back at each vblank.
Consequences: Simple and predictable; no per-client budget.
Evidence: none

### Option B · Deadline-scheduled callbacks with per-client budgets
Summary: Callbacks are scheduled against a deadline with budgets per client.
Consequences: Late clients are observed and bounded; more scheduling logic in the compositor.
Evidence: none

### Option C · Client-driven presentation timing
Summary: Clients request presentation times.
Consequences: Maximum control for media apps; the compositor must reconcile competing requests.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
