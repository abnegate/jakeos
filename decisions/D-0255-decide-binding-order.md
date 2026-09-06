# D-0255 · Decide SDK language binding order and milestones
- Status: proposed
- Task: SDK-024
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§50 lists the SDK languages and the ladder they arrive on: Rust at V0, C at V1, C++ at V2 and the remaining languages at V3. Later binding tasks (SDK-06x, SDK-07x) must not each invent a different order, and the V1 gate cites the C binding. This decision records the ladder; it is not a binding implementation and it cites ABI-007's Layer 1 C-ABI shape as the constraint every non-Rust binding sits on.

## Options

### Option A · Rust V0, C V1, C++ V2, others V3
Summary: Rust at V0, C at V1, C++ at V2, C#, Swift, Python and the rest at V3, exactly as §50 states.
Consequences: Matches the baseline and the V1 C-binding gate, and the IDL compiler (IPC-047) grows one backend per rung. C++ developers, including most game and creative-tool ports, wait until V2, so the V1 corpus of third-party native applications is Rust and C only.
Evidence: none

### Option B · C++ pulled to V1
Summary: C++ arrives with C at V1.
Consequences: A wider V1 developer reach and the C++ binding is a thin layer over the C one. The IDL compiler needs a second backend before V1 and the SDK team is on the V1 critical path twice; the C++ binding freezes against an SDK v1 that is itself a freeze candidate.
Evidence: none

### Option C · C delayed to V2
Summary: C slips to V2; V1 ships Rust only.
Consequences: Focus on the Rust runtime and one binding while Layer 1 is still changing. The V1 gate that cites the C binding moves or is dropped, personality and driver work that consumes the C headers waits a rung, and the ABI header exception (D-0008) has no consumer to test it until V2.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
