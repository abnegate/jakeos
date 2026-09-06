# D-0250 · Decide hierarchical versus flat ResourceDomains and budget delegation via Capability
- Status: proposed
- Task: SCH-002
- Surfaces: S-009
- Layer: L1
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §8, §9.1, §23
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether ResourceDomains nest with parent enforcement or form a flat set shapes the V0 kernel object and later launcher paths (§8, §9.1, §23).

## Options

### Option A · Hierarchical nested budgets with parent enforcement
Summary: Domains nest and parents enforce.
Consequences: Natural delegation; tree cost on accounting.
Evidence: none

### Option B · Flat set structured only by Capability attenuation
Summary: Domains are flat; structure comes from Capabilities.
Consequences: Simple kernel; delegation only through Capabilities.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
