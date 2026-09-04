# D-0125 · Decide NVIDIA support and Secure Boot handling of proprietary modules
- Status: proposed
- Task: HW-018
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §56.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The NVIDIA adr sits at V1 so V2 experimental and V3 Tier 1 bring-up are not research (§56.1), coordinated with KRN module signing and the GFX compositor stance.

## Options

### Option A · Open modules plus NVK
Summary: Only NVIDIA's open kernel modules with NVK and Mesa are supported.
Consequences: Signable under Secure Boot and supportable; feature and performance gaps on older generations.
Evidence: none

### Option B · Vendor-signed proprietary module
Summary: NVIDIA signs its module for the project's Secure Boot chain.
Consequences: Full features; the project depends on a vendor signing process it does not control.
Evidence: none

### Option C · Machine-owner MOK signing
Summary: Owners enrol a MOK and sign the proprietary module locally.
Consequences: Works on today's hardware; manual enrolment and a tainted kernel.
Evidence: none

### Option D · Defer past 1.0
Summary: NVIDIA is not a 1.0 platform.
Consequences: Focus; a large share of desktops excluded.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
