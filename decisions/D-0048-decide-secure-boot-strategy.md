# D-0048 · Decide Secure Boot distribution: Microsoft-signed shim, enrolled project keys, or both
- Status: proposed
- Task: BOOT-031
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Most retail hardware ships only Microsoft keys and dual-boot machines must keep Windows Secure Boot and BitLocker working, so the V3 Secure Boot distribution strategy is fixed at V2.

## Options

### Option A · Microsoft-signed shim plus MOK
Summary: A Microsoft-signed shim boots the project loader with MOK-enrolled keys.
Consequences: Works on retail hardware and keeps BitLocker; depends on Microsoft's signing process.
Evidence: none

### Option B · User-enrolled project PK/KEK/db
Summary: Users enrol project keys in firmware.
Consequences: No third-party dependency; BitLocker and Windows trust may break and enrolment is manual.
Evidence: none

### Option C · Both
Summary: Shim by default with enrolled keys as an option.
Consequences: Every machine has a path; two documented procedures to support.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
