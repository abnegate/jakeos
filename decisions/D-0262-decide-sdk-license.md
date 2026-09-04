# D-0262 · Decide the license of the native SDK, runtime and language bindings
- Status: proposed
- Task: SDK-027
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50, §52
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Third-party native applications begin at V0.5, so the SDK, runtime and binding licence must permit proprietary applications (§50, §52).

## Options

### Option A · Permissive Apache-2.0 or MIT
Summary: The SDK is permissively licensed.
Consequences: Proprietary applications are clearly fine; no share-back of fixes.
Evidence: none

### Option B · Weak copyleft such as MPL
Summary: The SDK is MPL.
Consequences: Fixes to the SDK stay open; some friction for static linking.
Evidence: none

### Option C · GPL with an SDK exception
Summary: The SDK is GPL with a linking exception.
Consequences: Copyleft core; exception text to explain.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
