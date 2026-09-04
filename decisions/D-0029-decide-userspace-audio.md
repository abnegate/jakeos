# D-0029 · Decide which audio device classes run in user space
- Status: proposed
- Task: AUD-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §33
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
INV-0616 asks which audio device classes stay on retained ALSA versus a user-space driver hosted by SVC (§33), with kernel residency acceptable where latency, DMA safety or inherited compatibility require it (I-008).

## Options

### Option A · Keep every in-scope audio class in-kernel through 1.0
Summary: USB, HDMI, Bluetooth offload and onboard all stay on retained ALSA drivers.
Consequences: No new driver work and known latency; no isolation benefit for audio.
Evidence: none

### Option B · User-space USB audio, other classes in-kernel
Summary: USB audio moves to a user-space driver and the rest remain in-kernel.
Consequences: Isolation for the most hot-pluggable class; a second driver path to maintain and a B-028 budget to meet.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
