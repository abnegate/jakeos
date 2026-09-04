# D-0017 · Decide assistive-technology access as Capability<AccessibilityTree> with redaction
- Status: proposed
- Task: ACC-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9, §9.1, §41, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Assistive-technology clients must reach the accessibility tree without an ambient bus (§9, §9.1, §41, §51), with password and secure fields redacted unless an elevated grant is held, before the V2 broker and screen reader exist.

## Options

### Option A · Per-app grants
Summary: An AT client holds a separate Capability<AccessibilityTree> per application.
Consequences: Fine-grained consent and revocation; screen readers need many grants and the user experience of granting each one is heavy.
Evidence: none

### Option B · Session-wide grant with secure-field redaction
Summary: One consent-scoped grant covers the session and secure nodes are redacted unless elevated.
Consequences: Practical for full-time AT users; a single grant is a large authority that must be clearly revocable.
Evidence: none

### Option C · Ambient bus
Summary: Any client on the session bus may read every tree.
Consequences: Matches AT-SPI2 today; rejected by §9 as ambient authority and a keylogging vector (T-001, T-013, T-039, I-021).
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
