# D-0169 · Decide lab site and the remote power, console and capture stack
- Status: accepted
- Task: LAB-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
BASELINE.md has no lab section, so the first LAB Decision defines where machines live and how power, console and capture reach operators and CI before H-002 is procured.

## Options

### Option A · Colocation with PDU plus serial or USB-debug consoles and capture cards
Summary: Machines are colocated with switched PDUs, consoles and capture cards.
Consequences: Reliable remote control and capture; colocation cost and remote hands.
Evidence: none

### Option B · Office lab with Redfish or IPMI
Summary: Office machines controlled through BMCs.
Consequences: Cheap and hands-on; consumer laptops lack BMCs.
Evidence: none

### Option C · Hybrid of colocated desktops and office laptops
Summary: Desktops colocated, laptops in the office.
Consequences: Practical for both classes; two operating procedures.
Evidence: none

## Decision
Option B, adapted: the lab is an office or home lab, not a colocation. Machines are power-cycled through a network-controlled PDU, consoles are reached over USB debug or serial adapters (Redfish or IPMI where a board offers it), displays are captured with HDMI capture devices, and the input-to-photon rig sits on the same bench. Laptops stay physically reachable for lid, dock and battery tests.

## Consequences
- LAB procures the PDU, console adapters, capture cards and a scheduler that hands machines to CI runners (BLD-003).
- Power and network reliability are the project owner's responsibility; a failed soak run caused by the site is recorded as such, not as a hardware regression.
- Growth to ten Tier 1 machines at V4 may exceed a home bench; a colocation decision is revisited then.

## Rejected options and why
- Option A (colocation) rejected: laptops, lids, docks and photodiode rigs are impractical to operate remotely, and the V1 and V2 gates are laptop-heavy.
- Option C (hybrid) rejected: two sites from day one for a one-person team.

## Follow-ups
Revisit before V4 when the Tier 1 fleet reaches ten machines.
