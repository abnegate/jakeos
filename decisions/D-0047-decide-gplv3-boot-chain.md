# D-0047 · Decide whether GPLv3 components may appear in the boot chain and how Installation Information is met
- Status: proposed
- Task: BOOT-010
- Surfaces: none
- Layer: none
- Spikes: BOOT-015
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Secure Boot plus GPLv3 components such as GRUB and gnupg triggers anti-tivoization duties, so whether GPLv3 may appear in the boot chain and how Installation Information is met must be decided; base-image GPLv3 remains GOV policy.

## Options

### Option A · No GPLv3 in the boot chain
Summary: Boot-chain binaries are GPLv2 or permissive only.
Consequences: No Installation Information duty; GRUB is excluded.
Evidence: none

### Option B · GPLv3 permitted with user-enrollable Secure Boot keys
Summary: GPLv3 binaries are allowed and key enrolment is the Installation Information.
Consequences: GRUB is available; the project must document and support key enrolment.
Evidence: none

### Option C · GPLv3 only in developer-mode images
Summary: GPLv3 boot binaries ship only in images that do not use project-signed Secure Boot.
Consequences: Retail images stay clean; two boot chains to maintain.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
