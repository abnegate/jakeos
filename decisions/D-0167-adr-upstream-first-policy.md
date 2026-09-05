# D-0167 · Decide the upstream-first policy for the hardware layer and Rust abstractions
- Status: accepted
- Task: KRN-006
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §5.1, §6, §55
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Fixes and reusable Rust abstractions either go upstream first or accumulate as fork-only patches (§5.1, §6, §55).

## Options

### Option A · Upstream-first
Summary: Patches land upstream before or with the fork.
Consequences: Minimal divergence and shared maintenance; slower landing in the fork.
Evidence: none

### Option B · Alongside
Summary: Patches are posted upstream in the same week they land in the fork.
Consequences: Balance of speed and alignment; two trees to track per patch.
Evidence: none

### Option C · Fork-only with later contribution
Summary: Patches land in the fork first and are contributed later.
Consequences: Fastest local progress; growing divergence and rebase pain.
Evidence: none

## Decision
Option A. Upstream-first for driver fixes, hardware enablement and generic Rust-for-Linux abstractions: patches go to the Linux mailing lists before or together with landing in the fork, and the fork carries them as a clearly tagged upstream-candidate series until they merge. Native-model code (the native ABI, Components, Capabilities, Channels, Operations, MemoryObjects, ResourceDomains) is fork-only by design.

## Consequences
- The divergence ledger classifies every patch as upstream-candidate, fork-only or temporary; unclassified patches fail the KRN CI gate.
- Upstream review latency is accepted as the cost of keeping the driver layer mergeable.
- Generic Rust abstractions the native model needs are shaped so they can plausibly be accepted upstream even when the native model itself cannot.

## Rejected options and why
- Option B (alongside) rejected: without a rule, upstream submission slips indefinitely and every driver fix becomes permanent merge debt.
- Option C (fork-only, contribute later) rejected: it maximises the divergence surface exactly where §55 says hardware support must not be destabilised.

## Follow-ups
none
