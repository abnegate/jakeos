# D-0028 · Decide native AudioStream service versus PipeWire-as-native
- Status: proposed
- Task: AUD-002
- Surfaces: S-025
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
BASELINE.md has no audio section, so this Decision records whether PipeWire is the native server wrapping Object<AudioStream> or a native AudioStream service runs with PipeWire as a personality client; I-009 forbids replacing a mature server without a measured benefit.

## Options

### Option A · PipeWire as the native server wrapping AudioStream
Summary: PipeWire is the audio server and Object<AudioStream> is minted around it.
Consequences: Mature routing and Bluetooth paths reused; PipeWire semantics shape the native object.
Evidence: none

### Option B · Native AudioStream service with PipeWire as a Linux-personality client
Summary: A native service owns audio and PipeWire runs only inside the personality.
Consequences: Clean native object model; a second audio server must reach parity with PipeWire.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
