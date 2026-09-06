# D-0285 · Decide encryption layering across the verified system store and encrypted user data
- Status: proposed
- Task: STO-039
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §26, §27, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The system store contains publicly known bytes (every installed Package) while user and application data are private (§26, §27, §51). SEC-005 picks the encryption mechanism; this decision states whether the store is verified but unencrypted, whether everything is encrypted, or whether the store uses convergent encryption, and how deduplication behaves under each. It precedes the V3 FDE installer and sits on the store mapping (STO-017) and store objects (STO-009).

## Options

### Option A · Verified-unencrypted system store plus encrypted user data
Summary: The system store and generations are verified (verity or signed index) but not encrypted; user data, ApplicationData and settings volumes are encrypted.
Consequences: Deduplication and verification of the store work on plaintext, boot needs no key to start the system, and nothing secret is in the store by construction. The set of installed Packages is visible to anyone with the disk, which is a privacy signal (installed applications) the threat model must accept and document.
Evidence: none

### Option B · Encrypt-everything
Summary: Every volume including the store is encrypted under the device key.
Consequences: One rule and no installed-application disclosure. Deduplication across the store still works inside the volume, but early boot needs the device key before any Package is readable, so unlock precedes native init (D-0300) and the recovery environment must carry its own copy of what it needs.
Evidence: none

### Option C · Encrypt-store with convergent encryption
Summary: The store is encrypted with convergent (content-derived) keys so identical objects encrypt identically and deduplicate.
Consequences: Deduplication with encryption at rest. Convergent encryption confirms known plaintext (an attacker learns whether a known Package is installed), which is the same disclosure as A with more machinery; rejected unless a threat the register does not list appears.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
