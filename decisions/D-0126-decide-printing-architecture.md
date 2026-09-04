# D-0126 · Decide driverless-first native printing with PDF spool and CUPS in LNX
- Status: proposed
- Task: HW-041
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §33
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V3 print service needs an architecture: driverless native with PDF spool, CUPS native, or CUPS in the personality (§33).

## Options

### Option A · IPP Everywhere/driverless native service with PDF spool
Summary: A native driverless print service with a PDF spool.
Consequences: Capability-shaped and small; legacy driver-only printers are unsupported.
Evidence: none

### Option B · CUPS as the native service
Summary: CUPS is the native print service.
Consequences: Broad printer support; a POSIX-shaped daemon at the centre of native printing.
Evidence: none

### Option C · CUPS only inside the Linux personality for legacy drivers
Summary: Native driverless plus CUPS in the personality.
Consequences: Legacy coverage without polluting native; two print paths for users.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
