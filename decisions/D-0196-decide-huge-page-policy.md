# D-0196 · Decide the per-Component page-table and huge-page policy for MemoryObjects
- Status: proposed
- Task: MEM-021
- Surfaces: none
- Layer: none
- Spikes: MEM-031
- Supersedes: none
- Superseded by: none
- Baseline: §16, §34, §38
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Warm startup depends on how cheaply verified immutable pages are mapped and shared (§16, §34, §38); the spike measures page-table cost and huge-page effects.

## Options

### Option A · Shared page-table fragments
Summary: Immutable mappings share page-table fragments across Components.
Consequences: Startup and memory wins; kernel complexity.
Evidence: none

### Option B · Transparent huge pages
Summary: THP is relied upon.
Consequences: TLB wins; Linux-shaped behaviour that is hard to control.
Evidence: none

### Option C · Explicit huge-page property on the MemoryObject
Summary: A property requests huge pages.
Consequences: Explicit control; an API surface to version.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
