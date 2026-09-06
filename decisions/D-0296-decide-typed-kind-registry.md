# D-0296 · Decide the platform type registry behind choose<T>, UserSelected<T> and file.type
- Status: proposed
- Task: STO-018
- Surfaces: S-033
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §45, §52
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
`choose<T>` and `UserSelected<T>` are typed by object kind (Image, Document, Audio) and automation rules test `file.type` (§25, §45, §52); SEM and the SDK must share one type system or the chooser and the automation engine will disagree about what a file is. This decision names the single source of object kinds and how a kind maps to content types on S-033 (Layer 2), answering Q-038 and Q-042. The registry is a platform artifact with governance, not a per-application table.

## Options

### Option A · Platform type registry
Summary: A platform type registry: each kind has an identity, a parent kind, the content types that map to it and the Interfaces it implies; the registry is a Package and third parties register kinds through it.
Consequences: One answer for the chooser, the SDK generics, `file.type` and semantic discovery, and kinds can carry behaviour (which Interfaces an Image supports). Registration is governance work with a review path, an unregistered format is `Unknown` until someone registers it, and the registry is on the critical path of every chooser call.
Evidence: none

### Option B · MIME sniffing
Summary: Kinds are derived by content sniffing (magic numbers and MIME) at the moment of use.
Consequences: Compatible with every existing file and no registry to maintain. Sniffing is unreliable for text-based and container formats, gives different answers for the same object over time, and cannot express kinds that are not file formats (a Contact, a Session).
Evidence: none

### Option C · Per-chooser filter tables
Summary: Each chooser and each automation rule ships its own filter table of extensions and MIME types.
Consequences: Fastest to build for the V0.5 apps. Every table drifts from the others, `file.type` in automation means something different from the chooser's `T`, and third-party formats must be added in every table.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
