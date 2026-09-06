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
When a new SystemGeneration replaces a Package that has running Components, those Components must never observe a mixed-version tree (T-034, §30, §34): every object they map must come from one version. This decision fixes what happens to the running application, what APP prompts and what PKG tests; it sits on generation switching (PKG-016) and activation (PKG-037).

## Options

### Option A · Old objects stay mapped until exit
Summary: Running Components keep their mapped objects from generation N until they exit; new launches use N+1; the store keeps N's objects until the last reference drops.
Consequences: Nothing running is interrupted and the rule is mechanical: an object is versioned by the generation that mapped it. Long-running applications stay stale until restart (security fixes included), the shell must show which applications are pending restart, and garbage collection (Q-019) must count running references.
Evidence: none

### Option B · Restart prompt
Summary: The shell prompts the user to restart affected applications when N+1 activates; until then option A behaviour applies.
Consequences: Users learn that an update is pending and choose the moment; combined with A it is safe. Prompts interrupt, and applications without session restore lose state on restart, so the prompt is only acceptable once APP-056's session restore exists.
Evidence: none

### Option C · Deferred activation of N+1 for that Package
Summary: N+1 is composed but the affected Package is not activated until its running Components exit; the generation switch completes lazily.
Consequences: No stale application and no prompt. A partially activated generation is a mixed system state that `os inspect`, rollback and the integrity story must all understand, and a never-exiting application blocks the update indefinitely.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
