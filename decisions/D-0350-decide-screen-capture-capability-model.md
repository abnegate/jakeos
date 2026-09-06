# D-0350 · Decide the screen-capture Capability model: per-Surface, per-Display and per-Session grants with indicator semantics
- Status: proposed
- Task: GFX-099
- Surfaces: S-034
- Layer: L2
- Spikes: GFX-033
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §40
- Revisit when: a GPU sharing or scanout bug shows client-mapped frames leak pixels, or a personality screen-share path cannot be expressed with the chosen grants

## Context
No application receives unrestricted screen capture (§40). S-034 registers the explicit screen-share and screen-record Capability with a persistent indicator, and SEC-011 requires it at V0.5, but its shape is undecided: grant granularity, how frames reach the client, how trusted-UI Surfaces are excluded, how the indicator is made non-suppressible and what a denied client receives (T-013, T-031; I-085).

## Options

### Option A · Typed grants per Surface, per Display and per Session
Summary: Each grant kind mints a read-only MemoryObject frame stream scoped to exactly those pixels; the compositor owns the indicator.
Consequences: Least authority per use case and a clean permissions-UI story; three grant kinds to prompt for and audit.
Evidence: none

### Option B · One per-Session grant with compositor-side redaction
Summary: A single capture grant over the whole Session; the compositor blanks trusted-UI and excluded Surfaces before frames leave it.
Consequences: Simplest prompt; every capture sees everything not redacted, so exclusion correctness carries the whole model.
Evidence: none

### Option C · Compositor-mediated stream the client never maps
Summary: Frames are encoded or forwarded by the compositor to a destination the client names; the client never holds frame memory.
Consequences: Strongest against pixel leakage and GPU sharing bugs (T-031); rules out client-side processing and costs a copy or an encode per frame.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
