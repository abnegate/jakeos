# D-0336 · Decide anti-cheat vendor engagement and required legal agreements
- Status: proposed
- Task: WIN-058
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §48, §56.2
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
D-0335 fixed the policy (owner direction: no kernel anti-cheat, VM fallback). This later decision fixes whether the project engages Easy Anti-Cheat and BattlEye at all: what agreements they require, whether the project's legal entity (GOV-024, Q-049) can sign them, and what user-space-only anti-cheat modes could be supported without loading a driver (§48, §56.2). WIN-062's spike reports on vendor terms. Nothing here may relax WIN-002 or permit bypass (I-071).

## Options

### Option A · No vendor deals in 1.0
Summary: No vendor agreements before 1.0; titles requiring anti-cheat follow D-0335's VM route and the compatibility database says so.
Consequences: No legal exposure and nothing to negotiate without an entity. Coverage stays at whatever the VM route achieves, which vendors may block.
Evidence: `reports/spikes/WIN-062.md`

### Option B · Userspace-only titles
Summary: Engage vendors only for their user-space-only modes (as Proton does on Linux), so titles whose anti-cheat supports that mode run in the personality without any driver.
Consequences: A real coverage gain for titles already enabled for Linux, with no kernel change and no bypass. Requires the vendors' Linux-mode terms to accept a JakeOS personality, which the spike must confirm, and each title's developer must opt in on the vendor side.
Evidence: `reports/spikes/WIN-062.md`

### Option C · Pursue contracts
Summary: Pursue full vendor contracts including driver loading.
Consequences: Maximum coverage if vendors agree. Driver loading is exactly what D-0335 forbids, so this option can only be recorded as rejected; listed so no later task re-proposes it.
Evidence: `reports/spikes/WIN-062.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
