# D-0226 · Decide running-application behaviour when its Package is replaced by a new Generation
- Status: proposed
- Task: PKG-049
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §34
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Running-application behaviour when its Package is replaced by a new Generation must be decided so no Component observes a mixed-version tree (§30, §34, T-034).

## Options

### Option A · Old objects stay mapped until exit
Summary: Running apps keep old objects.
Consequences: Seamless; stale until restart.
Evidence: none

### Option B · Restart prompt
Summary: The user is prompted to restart.
Consequences: Explicit; interruption.
Evidence: none

### Option C · Deferred activation of N+1 for that Package
Summary: The new version activates later.
Consequences: No stale state; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
