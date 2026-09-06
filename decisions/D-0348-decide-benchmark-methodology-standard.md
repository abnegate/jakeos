# D-0348 · Decide the benchmark methodology standard: hardware list, warm and cold runs, percentiles, iterations, pinning and mitigations
- Status: proposed
- Task: BEN-064
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §54, §59
- Revisit when: a Tier 1 machine class appears that the standard cannot describe, or a published report is challenged on method

## Context
D-0031 fixed target kinds and register-only numbers but left Q-001 open: which H-IDs each rung measures on, what warm and cold mean, which percentiles every report states, iteration and warm-up counts, CPU frequency pinning, SMT and mitigation settings, and how QEMU-profile results are labelled. BEN-005 needs these as runner-enforced fields and every V0 benchmark task depends on Q-001 (§54, §59; I-061).

## Options

### Option A · One standard for every B-ID
Summary: p50 and p99 with fixed iteration and warm-up counts, frequency pinned, mitigations and SMT at the shipped default, QEMU results labelled as functional coverage.
Consequences: Simple to enforce in the runner and to read across reports; a metric that needs a different statistic must argue for an exception.
Evidence: none

### Option B · Per-B-ID methodology on each register entry
Summary: Each benchmark entry states its own statistics, counts and settings.
Consequences: Fits unusual metrics such as energy; forty entries drift and cross-metric tables stop being comparable.
Evidence: none

### Option C · Adopt an existing published standard verbatim
Summary: Take a published benchmarking methodology (for example the SPEC or Phoronix run rules) as written.
Consequences: Nothing to author; the standard was not written for kernel-object microbenchmarks or input-to-photon rigs and needs exceptions immediately.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
