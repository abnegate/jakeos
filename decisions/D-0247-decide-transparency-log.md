# D-0247 · Decide whether releases use a transparency log
- Status: proposed
- Task: REL-030
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §51, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Binary transparency is the defence against a compromised pipeline silently shipping a targeted build to one machine (§51, §63): if every published artifact is logged in an append-only log and clients verify inclusion, a build not in the log is refused. This decision fixes whether 1.0 clients verify inclusion proofs on the stable path, which artifacts are logged (generations, Packages, repository metadata), and leaves operating the log to V4. It sits on the signing scheme (D-0245).

## Options

### Option A · Sigstore or Rekor-style log with client inclusion proofs on stable
Summary: Every generation, Package and metadata snapshot is logged in a Sigstore or Rekor-style transparency log; stable-channel clients verify an inclusion proof before activation.
Consequences: A targeted or unlogged build is refused by the client, which is the strongest available defence against a compromised signer. The log must be operated (or a public instance relied on) with high availability, the client gains a verifier and a dependency, and offline installs need bundled proofs.
Evidence: none

### Option B · Log without client proofs
Summary: Artifacts are logged but clients do not verify proofs; auditors monitor the log.
Consequences: Public auditability with no client change. A targeted build is detectable after the fact but not prevented, so the guarantee is forensic rather than protective.
Evidence: none

### Option C · No transparency log for 1.0
Summary: No transparency log for 1.0.
Consequences: Nothing to operate. A compromised pipeline can ship a targeted build with no external signal, which the external audit (V4) will flag as the weakest link in the supply chain; rejected unless the log service cannot be funded.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
