# D-0156 · Decide eBPF's native role and the Linux Personality's bpf() exposure
- Status: proposed
- Task: KRN-024
- Surfaces: none
- Layer: none
- Spikes: KRN-017
- Supersedes: none
- Superseded by: none
- Baseline: §24, §46, §58
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
eBPF's native role and the personality's bpf() exposure must be decided since OBS and NET tasks at V1 and V2 depend on it (§24, §46, §58).

## Options

### Option A · Native tracing substrate
Summary: eBPF underlies native tracing.
Consequences: Reuse of mature tracing; eBPF semantics shape trace design.
Evidence: none

### Option B · sched_ext host
Summary: eBPF hosts scheduling policy via sched_ext.
Consequences: Flexible scheduling experiments; policy lives in BPF programs.
Evidence: none

### Option C · Network-policy engine
Summary: eBPF implements network policy natively.
Consequences: Fast filtering; another BPF surface to gate.
Evidence: none

### Option D · Personality-only
Summary: eBPF is exposed only inside the Linux personality.
Consequences: Native stays clean; no reuse for tracing or scheduling.
Evidence: none

### Option E · Combination of roles
Summary: A recorded mix of the above roles.
Consequences: Pragmatic; more Capability gates to define.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
