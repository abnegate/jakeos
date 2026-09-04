# D-0188 · Decide native media pipeline versus GStreamer or FFmpeg
- Status: proposed
- Task: MED-007
- Surfaces: S-036
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §11, §17, §51, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Sandboxed decoder Components fed by a native pipeline versus retaining GStreamer or FFmpeg must be decided with codecs as separately updatable Packages (§11, §17, §51, §57); I-009 requires a measured benefit to replace a mature pipeline.

## Options

### Option A · Native pipeline of sandboxed decoder Components
Summary: A native pipeline of decoder Components exchanges MemoryObjects with codecs as Packages.
Consequences: Flagship isolation and independent codec updates; significant effort.
Evidence: none

### Option B · Retain GStreamer with elements wrapped as Components
Summary: GStreamer is the pipeline and elements run as Components.
Consequences: Mature pipeline; GStreamer semantics shape the native model.
Evidence: none

### Option C · Retain FFmpeg as the in-Component library
Summary: FFmpeg runs inside decoder Components behind the same Package schema.
Consequences: Broad codec support; a monolithic library per Component.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
