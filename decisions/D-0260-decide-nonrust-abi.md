# D-0260 · Decide how non-Rust bindings map onto Layer 1 and IDL stubs
- Status: proposed
- Task: SDK-072
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Swift, Kotlin, C# and TypeScript bindings arrive at V3 (D-0255). Before they start, the mapping of each language onto Layer 1 (the C ABI of ABI-007) and onto Layer 2 interfaces (the IDL compiler of IPC-047) must be recorded so no language grows a second Native ABI (§50, §65). It sits on the binding order (SDK-024), the C binding (SDK-033) and the IDL compiler's backend API.

## Options

### Option A · C ABI only
Summary: Every language binds through the C headers and hand-written or generated FFI; interfaces are reached through the C stubs.
Consequences: One ABI, one set of headers to freeze, and every language toolchain already consumes C. Bindings are unidiomatic (raw handles, manual lifetimes), so each language community wraps them again, differently, and IDL types lose their structure at the C boundary.
Evidence: none

### Option B · Per-language IDL codegen
Summary: The IDL compiler gains a backend per language that emits idiomatic stubs directly over the Layer 1 C ABI.
Consequences: Idiomatic types, async shapes and ownership per language, generated from one source of truth so every binding stays in step with interface versions. Each backend is a maintained compiler component with its own tests, and the C ABI remains the only thing beneath them.
Evidence: none

### Option C · Both
Summary: The C headers are the Layer 1 binding for every language; IDL backends generate idiomatic Layer 2 stubs over them.
Consequences: Kernel-level calls have exactly one shape (the C ABI) while interfaces are idiomatic per language, which is where developers spend their time. Two layers of binding to document and version, and the backends must call the C layer identically.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
