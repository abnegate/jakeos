# D-0230 · Decide verified-once launch trust for cached Package objects
- Status: proposed
- Task: PKG-045
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §34
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Warm launch (§34) maps Package objects from the store without re-hashing them, so something must vouch for cached objects. This decision fixes what a V0.5 launch may skip after a cache hit and what V1 signed activation (PKG-050) must still prove; it sits on the store layout (PKG-014) and the hash choice (PKG-005), and it determines whether verification is a kernel property or a runtime one.

## Options

### Option A · dm-verity-style block verification
Summary: Store objects live on dm-verity-protected volumes; the kernel verifies every block on read against a root hash, so launch trusts any mapping.
Consequences: Verification is continuous and kernel-enforced: a corrupted or tampered block faults rather than runs, and launch does no hashing at all. Verity needs an image or a verity-capable filesystem per store volume, adding an object means rebuilding or appending a verity tree, and the root hash must itself be signed and measured (BOOT).
Evidence: none

### Option B · Signed content-store index
Summary: The store keeps a signed index of object identities and their on-disk locations; launch trusts objects listed in a valid index.
Consequences: Flexible with any filesystem and cheap to update per install; a launch costs one signature check per index. Trust rests on the index and on the filesystem not being modified beneath it, so a local attacker with write access to the store defeats it unless the store volume is separately protected.
Evidence: none

### Option C · Per-launch hash of a small manifest
Summary: Every launch hashes a small manifest that lists object identities and sizes, and trusts the objects it names.
Consequences: Cheap and simple. Only the manifest is verified per launch; the objects themselves are trusted because they were verified when installed, so the window between install and launch is unprotected and V1 signed activation has to add the missing proof anyway.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
