# D-0211 · Decide the tracing substrate and its measured overhead ceiling
- Status: proposed
- Task: OBS-003
- Surfaces: S-010
- Layer: L1
- Spikes: OBS-010
- Supersedes: none
- Superseded by: none
- Baseline: §24, §58
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Observability is architecture (§24): every primitive is traceable, `os inspect` and `os trace` read one data model (V0-G10, V0-G17) and the overhead is bounded by B-012. OBS-010 studies eBPF, ftrace, Fuchsia tracing and Perfetto. This decision fixes the substrate V0 primitives emit into, the event schema on S-010 (prototyped, not frozen) and the rule that a disabled scope costs a predictable branch and nothing else on the hot path. Trace records are also the input to crash capture and the profiler (§64), so the schema carries Component and Task identity, not pids.

## Options

### Option A · Extend ftrace/tracepoints/eBPF
Summary: Native primitives add tracepoints to the retained ftrace machinery; eBPF programs attach and aggregate; `os trace` reads through the existing ring interfaces.
Consequences: No new kernel infrastructure and a large tooling ecosystem; disabled tracepoints are already patched-out nops. The event model is Linux-shaped (pid, tid, cpu) and must be translated to Components and Tasks in user space on every read. eBPF becomes a required dependency, which pre-empts KRN's eBPF-role decision, and the ring is global so per-Component isolation of trace data is weak.
Evidence: `reports/spikes/OBS-010.md`

### Option B · Native per-Component structured ring
Summary: Each Component owns a ring MemoryObject that the kernel writes typed events into; `os trace` attaches by Capability.
Consequences: Events carry native identity from the start, access to a trace follows the capability model, and the cost is one fixed-size record write. All tooling is new, retained driver tracepoints are invisible unless bridged, and the record format becomes Layer 1 ABI on S-010 that the V4 conformance suite must cover.
Evidence: `reports/spikes/OBS-010.md`

### Option C · Native semantic schema over eBPF
Summary: A native event schema and identity mapping defined on S-010, emitted through eBPF-attached tracepoints and materialised per Component by a user-space tracing service.
Consequences: Reuses the kernel plumbing while presenting native semantics, and the schema rather than a ring format is the ABI. Two layers must be kept in step, the tracing service sits on the path of every trace and is itself a Component to supervise, and the eBPF verifier limits what the schema can compute in the kernel.
Evidence: `reports/spikes/OBS-010.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
