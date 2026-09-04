# D-0054 · Decide explicit grant sources replacing ambient permissions
- Status: proposed
- Task: CAP-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9, §9.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A Component starts with exactly the Capabilities it was handed, so the sources of that initial set must be named to keep V0 Components free of ambient filesystem, network, device and process-enumeration authority (§9, §9.1).

## Options

### Option A · Creator-at-launch only
Summary: The creating Component hands over every Capability at launch and no other source exists.
Consequences: Simplest to audit; user choice and manifest requests must route through the creator.
Evidence: none

### Option B · Creator at launch plus user choice plus manifest request
Summary: Capabilities come from the creator, from user choosers and prompts, and from manifest-declared requests granted by policy.
Consequences: Matches how applications actually get files and devices; three sources to order by precedence.
Evidence: none

### Option C · Manifest-declared wildcard sets
Summary: A manifest may request broad sets such as all files or all network.
Consequences: Convenient for porting; reintroduces ambient authority (T-001, I-021).
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
