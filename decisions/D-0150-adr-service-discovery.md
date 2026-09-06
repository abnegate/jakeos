# D-0150 · Decide service naming and discovery: kernel-held directory or user-space broker
- Status: proposed
- Task: IPC-023
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §32, §14
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A client that holds a Channel to the compositor loses it when the compositor crashes; the V0.5 crash-recovery gate requires the client to rebind by Interface identity under SVC supervision (§32) without ambient authority (§14). CAP-022 decides how a Component obtains Capabilities; this decision fixes how a client finds a named Interface, what survives peer death, whether a resolved name can be attenuated on the way, and what `os inspect` shows about who is bound to whom.

## Options

### Option A · Kernel-held directory of Interface identities
Summary: The kernel holds a directory that maps Interface identity to the current endpoint; a client resolves through a kernel Operation and receives a fresh Channel Capability.
Consequences: The directory survives every user-space crash including the supervisor's, and rebind after peer death is a kernel-mediated retry with no broker to restart. Naming and its policy grow the kernel (against D-0157's boundary), attenuation must be expressed as directory rights in the kernel, and the directory shape becomes Layer 1 ABI.
Evidence: none

### Option B · User-space broker Component
Summary: A user-space broker Component owned by SVC holds the directory; clients hold a Channel to the broker and resolve names over it, receiving attenuated Channel Capabilities.
Consequences: The kernel stays minimal and the broker applies grant policy and attenuation in ordinary code that `os inspect` can walk. The broker itself must be supervised and rebound, so its own identity is the one name the runtime resolves at startup; a broker restart is a visible event every client must tolerate.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
