# D-0210 · Decide the trace event schema and export format
- Status: proposed
- Task: OBS-015
- Surfaces: S-035
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §24
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0 gates show trace flow live; V1-G10 requires offline export that existing analysis tools can open, so V0.5 is the last rung for this decision (§24). The event schema and the export format on S-035 (Layer 2) decide how an external decoder discovers event types, how native identities (Component, Task, Channel) are encoded, and whether the profiler and crash capture (§64) reuse the same records. No Layer 1 surface is frozen by it.

## Options

### Option A · Perfetto/CTF-compatible
Summary: Events are emitted in a Perfetto-compatible protobuf trace (or CTF) with native identities carried as typed tracks and args.
Consequences: Perfetto UI, trace_processor and the Android and Chromium tooling open traces immediately, and the SDK ships no viewer. Native concepts must be squeezed into Perfetto's track and slice model, schema evolution follows Perfetto's, and the dependency on its protobuf schema becomes part of S-035.
Evidence: none

### Option B · OpenTelemetry mapping
Summary: Events map onto OpenTelemetry spans, metrics and logs and export through OTLP.
Consequences: Interoperates with every observability backend and suits fleet telemetry (REL) as well as local traces. OTel is designed for distributed services, not kernel-rate events; the encoding overhead is high and the mapping from Operations and Tasks to spans is lossy without a native extension.
Evidence: none

### Option C · Native binary with schema registry
Summary: A native binary record format with a schema registry shipped in the trace header; converters to Perfetto and OTel are separate tools.
Consequences: Exact fit for native identities and kernel-rate volume, and the registry makes decoders self-describing. Every viewer needs the converter, the converter is on the critical path of V1-G10, and the format is one more thing to version on S-035.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
