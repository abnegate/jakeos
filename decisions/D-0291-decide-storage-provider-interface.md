# D-0291 · Decide the native storage-provider Interface for network and cloud Collections
- Status: proposed
- Task: STO-073
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §27
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Network shares arrive at V3 and cloud storage at V4; both must present as Collections with on-demand hydration and Capability scope (§25, §27). Deciding the provider contract once means every provider (SMB, WebDAV, a cloud vendor's API) is one implementation of one Interface rather than a bespoke integration. This decision also names the 1.0 cloud provider set, or an explicit empty set; it sits on the view API (STO-036) and the SMB client (STO-059).

## Options

### Option A · Single hydration-capable provider Interface
Summary: One provider Interface: enumerate, hydrate on demand, write back, change notification and conflict reporting; every network or cloud provider is a Component implementing it, and Collections over a provider carry the same Capabilities as local ones.
Consequences: One contract for the File Browser, the chooser, history and the personalities' views, and adding a provider is a Package. The abstraction must be rich enough for every protocol's quirks (partial writes, versioning, share links), so the Interface is versioned on S-014 rules and early providers will surface gaps.
Evidence: none

### Option B · Per-protocol Interfaces
Summary: Each protocol gets its own Interface (SMB, WebDAV, per-vendor cloud).
Consequences: Exact fit per protocol and no abstraction tax. Every consumer (File Browser, chooser, history) handles N Interfaces, hydration and conflict logic is duplicated per provider, and third-party providers have no single contract to implement.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
