# D-0341 · Decide how Wine and Proton map onto native Objects
- Status: proposed
- Task: WIN-036
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §4, §48, §69
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V2 architecture for how Wine and Proton map onto native Objects is required by V2 exit (§4, §48, §69).

## Options

### Option A · Wine stays on Linux-personality syscalls with native UX chrome
Summary: Linux syscalls plus chrome.
Consequences: Low risk; not native.
Evidence: none

### Option B · unixlib replaced by Native ABI bindings
Summary: Native ABI unixlib.
Consequences: Native; a large port.
Evidence: none

### Option C · Hybrid with graphics and input native
Summary: Hybrid.
Consequences: Balance; two paths.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
