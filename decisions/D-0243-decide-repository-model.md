# D-0243 · Decide the repository model and source trust display
- Status: proposed
- Task: REL-012
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28, §62, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Before third parties publish at V3, the repository model must be fixed (§28, §62, §63): a curated store with explicit sideloading, an open repository with third-party remotes, or first-party Packages only until 1.0. The V2 store client needs the trust-level display contract so a user consents to Capability requests from a named source, and a sideloaded source is never ambient (I-021, T-006). It sits on the signing scheme (D-0245) and channels (D-0248).

## Options

### Option A · Curated store plus explicit sideloading with trust-level display
Summary: A curated first-party store; sideloading of any signed Package from a named source is allowed through an explicit flow that shows the source's trust level before install.
Consequences: Users see who is vouching for a Package and the store can hold quality and licence bars. Curation is ongoing human work (GOV-054's developer agreement, review), and sideloading must be easy enough that it does not become the normal path.
Evidence: none

### Option B · Open repository with third-party remotes
Summary: An open repository where anyone publishes, plus third-party remotes users add.
Consequences: Maximum ecosystem freedom and no curation cost. Trust is ambiguous at install time, malicious or abandoned Packages appear under the same UI as first-party ones, and the permissions prompt must carry the whole burden.
Evidence: none

### Option C · First-party Packages only until 1.0
Summary: Only first-party Packages until 1.0.
Consequences: Total control and a small trust root. No third-party ecosystem exists at 1.0, which makes the alpha and beta audiences testers rather than users and defeats the V3 gate for third-party Packages.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
