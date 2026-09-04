# D-0301 · Decide how a service Component reports readiness and liveness to the supervisor
- Status: proposed
- Task: SVC-004
- Surfaces: none
- Layer: none
- Spikes: SVC-014
- Supersedes: none
- Superseded by: none
- Baseline: §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Dependency-ordered startup needs one readiness signal and one liveness signal for every supervised Component (§32); the supervision spike is the evidence.

## Options

### Option A · Explicit ready notification with heartbeat over the supervisor Channel
Summary: The service sends ready and periodic heartbeats over its supervisor Channel.
Consequences: Clear semantics for ordering and rebind; a protocol every service implements.
Evidence: none

### Option B · Interface-advertised-means-ready with kernel death notification only
Summary: Advertising an Interface means ready; kernel death notification is the only liveness signal.
Consequences: Nothing extra for services to do; hangs go undetected until a client times out.
Evidence: none

### Option C · Probe-based readiness with heartbeat liveness
Summary: The supervisor probes readiness and watches heartbeats.
Consequences: Robust to misreporting; a probe to design per service.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
