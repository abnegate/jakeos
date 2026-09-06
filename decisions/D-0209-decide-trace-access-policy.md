# D-0209 · Decide who may trace and inspect which Components
- Status: proposed
- Task: OBS-014
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §24
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Tracing is security-aware by design (§24): a trace of a Channel exposes its payloads and an inspect graph exposes which Components hold which Capabilities. Unrestricted tracing would defeat the V0.5 file-chooser demo, where Image Viewer must not learn about files it was not granted (§9.1). This decision fixes who may trace and inspect which Components, whether Channel payloads are visible without rights on that Channel, and how debugger attach (T-027) and the no-ambient-authority rule (I-021) are honoured.

## Options

### Option A · Ambient debug right per session
Summary: A session-wide debug right lets any Component in the session trace and inspect any other Component of the same user.
Consequences: Developers get the Linux experience: everything is visible. Any application with the right sees every other application's Channel payloads and Capability graph, which is the confused-deputy path T-027 describes and contradicts I-021.
Evidence: none

### Option B · Capability<Trace> per ResourceDomain subtree
Summary: `Capability<Trace>` is scoped to a ResourceDomain subtree; holding it grants trace and inspect over Components in that subtree, and Channel payloads are visible only for Channels whose endpoints are both inside the subtree.
Consequences: Precise and composable: a launcher traces what it launched, a developer tool traces its `os env`, and the session shell traces the session. Grants must be managed like any other Capability, the debugger attach flow (T-027) becomes a grant, and payload visibility follows domain membership, which the SDK must explain.
Evidence: none

### Option C · Owner-only with elevation
Summary: A Component may trace only itself and its children; anything wider requires an elevation prompt that mints a temporary `Capability<Trace>`.
Consequences: Safe default with no standing grants. Every cross-Component investigation interrupts the user, so developers will script around it, and the elevation UI arrives with APP work later than OBS needs it.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
