# D-0205 · Decide whether 1.0 ships an SMB server and where it is hosted
- Status: proposed
- Task: NET-029
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Sharing a folder with other machines is a desktop expectation. GAP-0434 moved this decision from V4 to V3 so an implementation can land before feature freeze. Options are no server in 1.0, Samba inside the Linux personality (personality-only, I-006) or a native sharing service over Collections; any listening path is an explicit network Capability (I-021, D-0202). STO owns SMB clients (STO-059); the accepted option names whether NET-036 is in scope or dropped.

## Options

### Option A · Do not ship a server in 1.0
Summary: 1.0 ships no file-sharing server; NET-036 is dropped as descoped.
Consequences: No listening service before the audit and no Samba in the base image. Sharing a folder to another machine is impossible without third-party software, which the 1.0 non-promises must list.
Evidence: none

### Option B · Samba in the Linux personality
Summary: Samba runs as a personality-hosted service with a listen Capability, exporting folders the user grants it; it is never visible to native software.
Consequences: Full SMB compatibility with Windows and macOS clients on day one. A large C daemon with a long CVE history listens on the network, its configuration lives in personality files the settings service must wrap, and every exported folder is a grant to Samba.
Evidence: none

### Option C · Native sharing service
Summary: A native sharing service exports Collections over SMB (and later a native protocol) using a Rust SMB implementation, with a Capability per exported Collection and an explicit listen right.
Consequences: Sharing follows the grant model exactly and the service is inspectable. A Rust SMB server that Windows and macOS clients accept is a substantial protocol effort with authentication and signing requirements; it is unlikely to reach parity before V4.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
