# D-0259 · Decide the native linking model and reject path-based loaders
- Status: proposed
- Task: SDK-026
- Surfaces: none
- Layer: none
- Spikes: SDK-030
- Supersedes: none
- Superseded by: none
- Baseline: §34, §53
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§34 and §53 forbid path-based loader lookup and global constructors on the launch critical path; static, content-addressed shared or both must be chosen (I-039).

## Options

### Option A · Static only
Summary: Everything is statically linked.
Consequences: Simplest launch path; no code sharing between Packages.
Evidence: none

### Option B · Content-addressed shared objects
Summary: Shared objects are referenced by content hash.
Consequences: Sharing and dedup; a loader to build and audit.
Evidence: none

### Option C · Both with recorded defaults
Summary: Both exist with a recorded default.
Consequences: Flexibility; two paths to test.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
