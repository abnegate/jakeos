# D-0286 · Decide how the storage model degrades on foreign filesystems lacking its metadata
- Status: proposed
- Task: STO-055
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §26
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
NTFS and exFAT support need a degradation contract so native Collections do not lose metadata silently (§25, §26).

## Options

### Option A · xattr fallback
Summary: Metadata in xattrs.
Consequences: Native-ish; not on exFAT.
Evidence: none

### Option B · Sidecar metadata
Summary: Sidecar files.
Consequences: Universal; clutter.
Evidence: none

### Option C · Refuse-unsupported
Summary: Unsupported volumes are refused.
Consequences: Safe; limits.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
