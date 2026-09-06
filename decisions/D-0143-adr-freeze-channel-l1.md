# D-0143 · Freeze the Channel Layer 1 ABI Surface
- Status: proposed
- Task: IPC-064
- Surfaces: none
- Layer: none
- Spikes: IPC-017
- Supersedes: none
- Superseded by: none
- Baseline: §65, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V4 exit freezes Layer 1 with the freeze decision accepted (V4-G01, §65, §66). IPC's amendment covers the Channel syscalls, the handle-transfer layout and the version header, and removes deprecated entry points, starting from the candidate list of D-0140 and the evidence of IPC-017, the B-004 and B-005 reports and the conformance suite (ABI-049). S-012 becomes frozen by IPC-065 on acceptance.

## Options

### Option A · Freeze the V1 candidate set as S-012
Summary: Freeze the whole V1 candidate set on S-012: create, send, receive, close, handle transfer, inspect, the version header and the completion layout.
Consequences: SDK v1 code has a complete, permanent IPC contract and the conformance suite covers all of it. Every entry must have its evidence complete at V4 and any mistake is permanent until a versioned successor.
Evidence: `reports/spikes/IPC-017.md`

### Option B · Freeze a reduced send/receive/close core
Summary: Freeze send, receive, close and the version header; handle transfer and inspect remain Layer 2 until 1.0.
Consequences: Lower risk for the parts that changed most recently. Handle transfer is used by every service, so a Layer 2 status means SDK code still depends on a surface that may move after V4.
Evidence: `reports/spikes/IPC-017.md`

### Option C · Defer freeze to 1.0
Summary: Defer the Channel freeze to 1.0.
Consequences: More evidence time. It contradicts the V4 exit gate and the stability statement 1.0 publishes; listed to be rejected.
Evidence: `reports/spikes/IPC-017.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
