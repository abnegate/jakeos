# D-0072 · Decide how environment endpoints are granted without ambient network
- Status: proposed
- Task: ENV-006
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9, §9.1, §35
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The environment NetworkNamespace must expose postgres, redis and later catalogue services to the developer's shell, IDE and browser without granting ambient network authority (§9, §9.1, §35).

## Options

### Option A · Per-service loopback Endpoint Capabilities
Summary: ENV mints one loopback Endpoint Capability per declared service.
Consequences: Exact least authority; each new service is a new grant.
Evidence: none

### Option B · Named Capability<NetworkConnection> filtered to declared ports
Summary: One connection Capability filtered to the declared ports.
Consequences: Fewer grants; the filter is policy the holder can probe.
Evidence: none

### Option C · Single shared environment network grant
Summary: One grant covers the whole environment network.
Consequences: Simplest for developers; a wildcard connect right returns ambient authority (T-002).
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
