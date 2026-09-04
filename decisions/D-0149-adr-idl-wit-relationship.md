# D-0149 · Decide the relationship between the native IDL and WIT
- Status: proposed
- Task: IPC-022
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §13, §14
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The relationship between the native IDL and WIT must precede the V1 Wasm-component-on-native-Channel prototype (§13, §14); Wasm is not the Native ABI.

## Options

### Option A · Native IDL is WIT
Summary: One language serves both native and Wasm.
Consequences: No mapping layer to maintain; WIT's limits bound the native IDL.
Evidence: none

### Option B · Bidirectional mapping
Summary: Two languages with a defined mapping in each direction.
Consequences: Independence with interoperability; gaps where one side has no equivalent concept.
Evidence: none

### Option C · Independent languages with an explicit bridge
Summary: Separate languages and a hand-written bridge.
Consequences: Full freedom for the native IDL; duplicated definitions and bridge maintenance.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
