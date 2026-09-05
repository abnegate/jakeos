# D-0126 · Decide driverless-first native printing with PDF spool and CUPS in LNX
- Status: accepted
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
Option A. Native printing is driverless-first: an OS-owned print service speaks IPP Everywhere and AirPrint-class protocols, the print dialog is OS-owned and mints Capability<PrintJob>, and the spool format is PDF. Printers that need proprietary drivers are served by CUPS inside the Linux personality, which appears in the same dialog as a discovered printer.

## Consequences
- HW owns discovery (mDNS, IPP), APP owns the dialog, LNX owns the CUPS bridge, WIN routes the Wine spooler to the same service.
- No CUPS filter chain runs natively.
- Scanning follows the same shape with eSCL as the driverless protocol.

## Rejected options and why
- Option B (CUPS as the native service) rejected: it imports a large POSIX daemon and its filter chain into the native platform.
- Option C (no native printing) rejected: native applications could not print without the personality running.

## Follow-ups
none
