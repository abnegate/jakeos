# D-0261 · Decide profiler export format and Task attribution
- Status: proposed
- Task: SDK-053
- Surfaces: none
- Layer: none
- Spikes: SDK-061
- Supersedes: none
- Superseded by: none
- Baseline: §24, §64
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Native profiles attribute time to Task, Component and TaskGroup rather than to threads (§24, §64), because a Task migrates between workers and a thread view says nothing about the program. SDK-061 spikes the profiler; this decision picks the export format and records the attribution keys, building on the trace schema (OBS-015).

## Options

### Option A · pprof
Summary: pprof protobuf profiles with Task, Component and TaskGroup as sample labels.
Consequences: Every pprof viewer and continuous-profiling backend opens them. pprof's model is thread and stack centric, so Task attribution is a label rather than the primary axis and viewers group by thread by default; asynchronous Task stacks need a convention.
Evidence: `reports/spikes/SDK-061.md`

### Option B · Firefox Profiler
Summary: Firefox Profiler processed-profile format with Tasks as tracks.
Consequences: A capable, maintained web UI with tracks, markers and flame graphs that already understands asynchronous work through markers. The format is defined by one consumer and changes with it, and server-side aggregation tooling is thinner than pprof's.
Evidence: `reports/spikes/SDK-061.md`

### Option C · Native format folded into traces
Summary: Samples are trace events on the S-035 schema; the profiler is a view over a trace.
Consequences: One data model for tracing and profiling, so a profile and a trace of the same run line up exactly and the Task attribution is native. Every viewer needs a converter (to pprof or Firefox Profiler), and sampling at profile rates stresses the trace ring.
Evidence: `reports/spikes/SDK-061.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
