# D-0340 · Decide NT Object-manager, async I/O, descriptor and section fidelity
- Status: proposed
- Task: WIN-035
- Surfaces: none
- Layer: none
- Spikes: ABI-031
- Supersedes: none
- Superseded by: none
- Baseline: §7, §48
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Windows programs rely on NT semantics that Wine emulates: the object manager and its namespace, asynchronous I/O (IOCP, APCs), security descriptors and section objects (§7, §48). ABI-031's spike compares Wine's NT layer with the native Object model. This decision fixes which semantics are emulated and to what fidelity (emulated, stubbed, out of scope), and feeds the V2 mapping onto native Objects (D-0341); it sits on the userspace-only rule (D-0343).

## Options

### Option A · Wine's current NT layer
Summary: Wine's current NT layer (wineserver plus ntdll) as shipped, with its existing fidelity.
Consequences: Proven against the whole Wine test suite and every Proton title. Its object manager is a Linux-process emulation with wineserver round trips, security descriptors are largely stubs, and none of it maps onto native Objects, so the native security model does not see Windows objects.
Evidence: `reports/spikes/ABI-031.md`

### Option B · Documented subset
Summary: A documented subset: object manager and sections emulated fully, asynchronous I/O emulated over native Operations, security descriptors stubbed to a fixed policy, with the list recorded in the compatibility statement.
Consequences: Effort goes where titles need it and the gaps are honest. Titles that depend on descriptor semantics (enterprise software, some launchers) fail in documented ways, and the subset must be re-argued as the corpora grow.
Evidence: `reports/spikes/ABI-031.md`

### Option C · Native Object mapping per NT type
Summary: Each NT type maps onto a native Object: sections are MemoryObjects, events and mutants are waitable Objects, IOCP completions are Operation completions, descriptors are Capability rights.
Consequences: The deepest integration: Windows programs become first-class in `os inspect` and the grant model, and the performance of native primitives. A large redesign of wineserver's core that must be carried against Wine upstream forever, and a mismatch in semantics (descriptors versus rights) that will surface as compatibility bugs.
Evidence: `reports/spikes/ABI-031.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
