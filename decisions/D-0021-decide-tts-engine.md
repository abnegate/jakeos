# D-0021 · Decide the text-to-speech engine for the native screen reader
- Status: proposed
- Task: ACC-009
- Surfaces: none
- Layer: none
- Spikes: ACC-012
- Supersedes: none
- Superseded by: none
- Baseline: §41, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The native screen reader needs a speech engine decided at V1 so the V2 speech Component is not invented on the screen-reader path (§41, §67); the licence allowlist is an input and findings cite the spike report.

## Options

### Option A · espeak-ng
Summary: espeak-ng is the default engine.
Consequences: Small, broad language coverage and permissively licensed; voice quality is synthetic.
Evidence: none

### Option B · Piper
Summary: Piper neural voices are the default engine.
Consequences: Natural voices; larger models and heavier compute per utterance.
Evidence: none

### Option C · Vendor voices
Summary: Proprietary vendor voices are the default.
Consequences: Highest quality; licensing and redistribution conflict with the allowlist.
Evidence: none

### Option D · Pluggable Interface with a named default
Summary: Engines register behind one Interface and one open engine is the default Package.
Consequences: Users can swap engines; the Interface is one more Layer 2 surface to version.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
