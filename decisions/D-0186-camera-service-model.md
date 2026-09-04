# D-0186 · Decide Camera service model over V4L2 and libcamera
- Status: proposed
- Task: MED-001
- Surfaces: none
- Layer: none
- Spikes: MED-006
- Supersedes: none
- Superseded by: none
- Baseline: §7, §9.1, §33
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The baseline left the Camera service model unspecified; Object<Camera> is a §7 kernel object but whether the service talks V4L2, libcamera or both must be decided against the libcamera spike (§7, §9.1, §33, I-009).

## Options

### Option A · V4L2 as the mechanism, libcamera only on ISP pipelines V4L2 cannot drive
Summary: The Camera service uses V4L2 directly and adds libcamera only where an ISP pipeline requires it.
Consequences: Minimal new code on simple webcams; two code paths to test.
Evidence: none

### Option B · libcamera on every sensor
Summary: libcamera is the Camera service's mechanism for all sensors.
Consequences: One path with full ISP support; a heavier dependency on every machine.
Evidence: none

### Option C · V4L2 only, no libcamera
Summary: The service uses V4L2 only.
Consequences: Simplest stack; laptops with ISP-based cameras are unsupported.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
