# D-0140 · Decide which Channel syscalls become Layer 1 freeze candidates for SDK v1
- Status: proposed
- Task: IPC-041
- Surfaces: none
- Layer: none
- Spikes: IPC-017
- Supersedes: none
- Superseded by: none
- Baseline: §65, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Layer 1 surfaces are prototyped through V0, become freeze candidates at V1 with SDK v1 and are frozen at V4 (§65, §66). IPC names which Channel entry points are candidates on S-012 and what stays behind user-space Interfaces, using the V0 spike (IPC-017) and the B-004 and B-005 reports. Nothing is frozen here (I-040); the V4 freeze in V4-G01 starts from this list, so every candidate must already have evidence.

## Options

### Option A · Create, send, receive, close, handle-transfer and inspect as candidates
Summary: Create, send, receive, close, handle-transfer and inspect are all Layer 1 candidates.
Consequences: SDK v1 gets a complete kernel IPC contract and the C binding can target it directly. Every entry needs spike and benchmark evidence now, and inspect and handle-transfer semantics are the parts most likely to change with CAP's revocation work, so freezing them early is the riskiest.
Evidence: `reports/spikes/IPC-017.md`

### Option B · Reduced send/receive/close core with handle-transfer at Layer 2
Summary: Send, receive and close are candidates; handle transfer and inspection stay Layer 2 Interfaces over them until V2.
Consequences: The smallest surface that makes a Channel usable is committed and the parts still moving keep moving. Handle transfer across Channels is the mechanism every service uses, so leaving it at Layer 2 means SDK v1 code depends on a surface that may change in V2.
Evidence: `reports/spikes/IPC-017.md`

### Option C · Defer candidacy to V2
Summary: No Channel entry point is a candidate at V1; candidacy is decided at V2.
Consequences: Another rung of evidence and freedom to change. SDK v1 ships with no stable IPC contract at all, so the V1 developer preview cannot promise that its IPC code survives to V2, and V4-G01 has one rung less to converge.
Evidence: `reports/spikes/IPC-017.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
