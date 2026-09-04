# D-0121 · Decide criteria classifying each driver as inherited, native or rewritten
- Status: proposed
- Task: HW-016
- Surfaces: none
- Layer: none
- Spikes: HW-014
- Supersedes: none
- Superseded by: none
- Baseline: §33, §55
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Each driver class must be classified as inherited, native or rewritten by written criteria feeding the driver status registry (§33, §55).

## Options

### Option A · Keep in-kernel
Summary: The class stays as inherited in-kernel driver.
Consequences: Hardware works; no isolation gain.
Evidence: none

### Option B · Move to user-space
Summary: The class moves to a user-space driver.
Consequences: Isolation; latency and DMA safety to prove.
Evidence: none

### Option C · Rewrite in-kernel
Summary: The class is rewritten in Rust in-kernel.
Consequences: Safety without user-space cost; rewrite effort.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
