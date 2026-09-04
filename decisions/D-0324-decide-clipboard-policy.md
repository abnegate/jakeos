# D-0324 · Decide clipboard authority policy: paste gesture or Capability, no ambient read
- Status: proposed
- Task: UIP-004
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9, §9.1, §41
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Silent clipboard reading is a data-exfiltration vector (§9, §9.1, §41, T-001); the policy S-032 enforces must be named.

## Options

### Option A · Reads only on a paste gesture plus Capability<ClipboardRead> for managers
Summary: Gesture-gated reads.
Consequences: Safe and usable; manager grants.
Evidence: none

### Option B · Every read requires an explicit Capability
Summary: A Capability for every read.
Consequences: Strict; friction.
Evidence: none

### Option C · Ambient clipboard read
Summary: Anyone reads.
Consequences: Convenient; rejected per §9.1.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
