# D-0240 · Decide release, SystemGeneration and Channel versioning
- Status: accepted
- Task: REL-001
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28, §30, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Packages, SystemGenerations and public release names need one vocabulary before the first immutable install (§28, §30, §66, I-080).

## Options

### Option A · Generation counters plus channel names for OS artifacts and semver for Layer 2 and Layer 3
Summary: Mixed scheme.
Consequences: Fits each artifact; two schemes.
Evidence: none

### Option B · Semver for every artifact including generations
Summary: Semver everywhere.
Consequences: Uniform; generations are not semver.
Evidence: none

### Option C · Calendar-free public names plus opaque generation IDs
Summary: Names plus IDs.
Consequences: Marketing-friendly; opaque.
Evidence: none

## Decision
Option A. OS artifacts are identified by a monotonic SystemGeneration counter plus an update channel name (testing, stable); Layer 2 interfaces and Layer 3 SDK crates use semantic versioning. No identifier anywhere carries a calendar date. Public release names, when used, map to a generation range and are recorded in REL.

## Consequences
- os history and os restore address generations by counter; the updater subscribes to a channel.
- Interface evolution rules (IPC) define what constitutes a semver major for Layer 2.
- Release notes are keyed by generation range and channel.

## Rejected options and why
- Option B (semver for generations) rejected: major.minor.patch implies compatibility semantics a whole-OS image does not have; the counter and channel express ordering and stability directly.
- Option C (public names over opaque IDs) rejected: names are fine as labels but poor as primary identifiers for ordering and rollback.

## Follow-ups
none
