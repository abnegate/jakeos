# D-0040 · Decide the anti-rollback policy for SystemGenerations older than a security watermark
- Status: proposed
- Task: BOOT-040
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§30 first-class rollback and downgrade-attack protection are in tension once V3 exposes rollback to strangers, so booting generations older than a security watermark needs a policy (T-022).

## Options

### Option A · Block below watermark
Summary: Generations older than the watermark cannot boot.
Consequences: Downgrade attacks are closed; a last-known-good older than the watermark is unreachable.
Evidence: none

### Option B · Allow with warning
Summary: Older generations boot after a visible warning.
Consequences: Recovery always possible; an attacker can accept the warning.
Evidence: none

### Option C · Allow only in developer mode
Summary: Older generations boot only when developer mode is enabled.
Consequences: Recovery for those who need it; developer mode becomes a downgrade path.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
