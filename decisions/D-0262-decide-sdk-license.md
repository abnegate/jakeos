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
Third-party native applications begin at V0.5, so the licence of the SDK crates, the runtime a Component links and the language bindings must be fixed before any external developer builds against them (§50, §52). It sits beside GOV-003's layer firewall (Layer 1 GPLv2, Layers 2 to 4 permissive) and the ABI header exception (D-0008), and it must state plainly that proprietary native applications are permitted.

## Options

### Option A · Permissive Apache-2.0 or MIT
Summary: Apache-2.0 or MIT for SDK, runtime and bindings, matching D-0102's permissive rule for everything above the kernel.
Consequences: Proprietary applications are unambiguously fine, static linking carries no obligation, and the SDK can be vendored into any build. Fixes made by application vendors need not come back, and no patent grant exists under MIT, which Apache-2.0 would add.
Evidence: none

### Option B · Weak copyleft such as MPL
Summary: MPL-2.0 for the SDK and runtime.
Consequences: Modifications to SDK files stay open while applications remain proprietary. File-level copyleft complicates static linking and vendoring in Rust, where every crate is compiled into the binary, and it diverges from the D-0102 layer rule that everything above Layer 1 is permissive.
Evidence: none

### Option C · GPL with an SDK exception
Summary: GPLv2 with a linking exception like the classic runtime exceptions.
Consequences: Copyleft protection for the SDK itself. Exception text must be explained to every vendor's counsel, it contradicts D-0102, and the bindings for other languages inherit an exception that their ecosystems do not recognise.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
