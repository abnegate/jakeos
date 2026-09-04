# D-0004 · Decide the Layer 1 and platform deprecation process: announcement, overlap, detection
- Status: proposed
- Task: ABI-045
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §65, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Layer 1 and platform interfaces must be retirable with a process that exists before V4 removes deprecated entry points, since third-party packages arrive at V2 (§65, §66).

## Options

### Option A · Announce plus overlap plus detection
Summary: A deprecation is announced, the old and new interface overlap for a minimum window, and tooling detects use of the deprecated entry.
Consequences: Retirement is predictable and mechanical; the project must ship and maintain detection tooling and honour the overlap window even under pressure.
Evidence: none

### Option B · Shim forever
Summary: Nothing is ever removed from Layer 1; deprecated entries are kept as shims indefinitely.
Consequences: Zero breakage for old binaries; the Layer 1 entry-point count only grows and every shim is attack surface and test burden forever.
Evidence: none

### Option C · Major-version-only removal
Summary: Deprecated entries are removed only with a new major OS version, even before freeze.
Consequences: Simple rule aligned with the 1.x stability declaration; pre-freeze cleanup becomes impossible and V4 cannot remove anything without a major version.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
