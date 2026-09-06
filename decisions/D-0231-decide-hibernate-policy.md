# D-0231 · Decide hibernation policy for 1.0
- Status: proposed
- Task: PWR-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §61, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V2 lists hibernation as optional, but a hibernation image is a copy of memory on disk: it must not contain unlocked disk keys (T-009) and must not be a leftover attack surface (T-010). This decision chooses the 1.0 power product between no hibernation, suspend-then-hibernate after idle, and full hibernation with an encrypted authenticated image, states whether an image may exist on H-004 and H-005, cites SEC-031 for key handling under lockdown, and names the inspectable unsupported status when hibernation is not delivered (§61, §62).

## Options

### Option A · No hibernate in 1.0
Summary: 1.0 ships without hibernation; `os inspect power` reports `hibernate: unsupported`.
Consequences: No image on disk, so T-009 and T-010 do not apply and lockdown is unaffected. A laptop left suspended for days drains its battery, which Windows and macOS users compare against, and the swap partition exists for swap only.
Evidence: none

### Option B · Suspend-then-hibernate after idle
Summary: Suspend to idle first; after a configurable idle period the system writes an encrypted, authenticated hibernation image and powers off.
Consequences: Battery safety without a visible mode switch, matching what the comparison platforms do. The image threats apply: the key that encrypts the image must be sealed to the TPM or derived at resume from re-authentication, kernel lockdown must accept only images it can authenticate, and resume from image is a second boot path to test in LAB-014.
Evidence: none

### Option C · Full hibernate with an encrypted authenticated image
Summary: Full user-initiated hibernation with an encrypted authenticated image, plus option B behaviour.
Consequences: Complete power product. The same threats and work as B plus a user-visible action and the UI for it; the only extra value over B is manual invocation.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
