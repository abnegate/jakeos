# D-0169 · Decide lab site and the remote power, console and capture stack
- Status: proposed
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
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
