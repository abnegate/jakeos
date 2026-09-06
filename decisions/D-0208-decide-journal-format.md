# D-0208 · Decide the persistent journal record format and retention model
- Status: proposed
- Task: OBS-030
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §24, §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Persistent structured logs from previous boots are distinct from the live tracing pipeline (§24) and are excluded from SystemGeneration rollback (D-0216, §30). This decision fixes the record format, retention, per-boot indexing and how Linux personality logs (syslog, journald clients) are ingested, building on the trace schema decided by OBS-015.

## Options

### Option A · systemd-journal-compatible export
Summary: Records are written in the systemd journal file format so journalctl and every journal reader work; native fields carry Component identity.
Consequences: Existing tooling, forwarding integrations and personality applications that read the journal work unchanged. The format encodes systemd's field vocabulary and pid-centric identity, native concepts are extension fields, and the project depends on a format it does not control.
Evidence: none

### Option B · Native typed records over the trace schema
Summary: Native typed records on the S-035 trace schema, persisted in per-boot indexed segments with a retention policy by size and age; the personality's syslog and journald clients are ingested by a shim that maps them to records.
Consequences: One schema for live traces and persistent logs, Component identity and Capability context in every record, and `os inspect` reads both with one decoder. Journal readers need a converter or a compatibility shim, and index and retention code is new.
Evidence: none

### Option C · Plain structured text
Summary: Plain structured text lines (JSON per line) in rotated files.
Consequences: Readable by anything, trivial to ship. No efficient per-boot or per-Component index, no typed fields for the crash window, and rotation is the whole retention model.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
