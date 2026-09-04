# D-0106 · Decide the 1.0 portability commitment as x86-64 only shipping
- Status: proposed
- Task: GOV-025
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §1, §38, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§38 says ship x86-64 only while the ABI stays architecture-neutral; this must be recorded before SDK v1 freeze candidates so ARM64 and RISC-V remain LATER (§1, §38, §66).

## Options

### Option A · x86-64-only shipping with architecture-neutral ABI
Summary: Only x86-64 ships at 1.0 while ARM64 and RISC-V keep compiling in the fork.
Consequences: Focus and one lab matrix; ARM laptops are not 1.0 hardware.
Evidence: none

### Option B · Promise ARM64 at 1.0
Summary: ARM64 is a 1.0 shipping platform.
Consequences: Broader hardware reach; a second lab, driver and benchmark matrix before 1.0.
Evidence: none

### Option C · Promise RISC-V at 1.0
Summary: RISC-V is a 1.0 shipping platform.
Consequences: Openness signal; immature desktop hardware and drivers.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
