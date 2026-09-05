# D-0068 · Decide the native Component spawn primitive that replaces fork and exec
- Status: accepted
- Task: CMP-009
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §2, §10, §53
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
fork has no native equivalent, so the native Component spawn primitive must be chosen and Unix process startup recorded as never the native creation mechanism (§2, §10, §53); S-007 is prototyped.

## Options

### Option A · Spawn from immutable code object
Summary: A Component is created from an immutable, verified code object plus initial Capabilities.
Consequences: Clean and auditable; no cheap copy-of-self pattern.
Evidence: none

### Option B · Template clone
Summary: A prewarmed template Component is cloned.
Consequences: Fast creation; template state must be provably neutral.
Evidence: none

### Option C · Builder object then start
Summary: A builder object accumulates configuration and is started atomically.
Consequences: Rich, typed configuration; two-phase creation to get right.
Evidence: none

## Decision
Option C. Components are created through a builder Object: user space obtains a ComponentBuilder, attaches the immutable code object, the initial Capability set, the ResourceDomain, Channel endpoints for Inputs and Outputs and the supervising endpoint, then issues one start Operation. Nothing is inherited implicitly; there is no fork window and no exec-time environment.

## Consequences
- The builder is a Layer 1 surface (ABI) and its fields are the Component manifest in kernel form (PKG manifest maps onto it).
- Warm start (§34) is achieved by pre-built builders and pre-mapped code objects, not by cloning running state.
- fork and exec exist only inside the Linux personality (LNX).

## Rejected options and why
- Option A (one-shot spawn from a code object) rejected: incremental attachment of channels and capabilities before start is the common case and a single call makes it unwieldy.
- Option B (template clone) rejected: cloning running state inherits authority and memory the capability model would then have to scrub; it is fork by another name.

## Follow-ups
none
