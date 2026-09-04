# D-0060 · Decide how a Component obtains its initial and later Capabilities
- Status: proposed
- Task: CAP-022
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §7, §9.1, §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A Component must obtain Capabilities after creation and rebind to a restarted peer without ambient authority (§7, §9.1, §32); the V0 fault demo depends on the initial handoff named here.

## Options

### Option A · Capability namespace object
Summary: A per-Component namespace object resolves names to Capabilities.
Consequences: Familiar and inspectable; the namespace is authority and must be attenuated per Component.
Evidence: none

### Option B · Broker Component
Summary: A broker service hands out Capabilities on request under policy.
Consequences: Central policy point; the broker is a confused-deputy risk (T-002).
Evidence: none

### Option C · Manifest-declared static wiring
Summary: Connections are declared in the manifest and wired at instantiation.
Consequences: No runtime lookup and fully auditable; dynamic rebind after restart needs an extra mechanism.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
