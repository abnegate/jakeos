# Reports

Committed evidence that survives link rot. Reports are hand-written to fixed skeletons; `roadmap check` verifies that a referenced report exists and carries the required headings. Tasks reference reports from `#### Evidence` as `report:<path>`.

## What is committed

Only two kinds of run are committed here:

- **Gate runs**: the measurement or corpus run that a milestone gate is judged on, one report per benchmark or corpus per in-scope reference machine.
- **Milestone summaries**: the consolidated result set when a milestone closes.

Nightly and per-merge results are not committed; they go to the benchmark time-series export maintained by BLD and BEN. The generated `results` block in `registers/benchmarks.md` and `generated/benchmarks.md` show the latest committed report per B-ID per H-ID against the active target.

Registers hold definitions and targets only. A measured value never appears in a register.

## Layout

```
reports/spikes/<TASK-ID>.md
reports/benchmarks/<B-NNN>/<alias>@<sha>-<H-NNN>.md
reports/compat/<C-NNN>/<alias>@<sha>-<H-NNN>.md
```

`<alias>` is a repository alias from `registers/repos.md`; `<sha>` is the commit measured; `<H-NNN>` is the reference machine or QEMU profile from `registers/hardware.md`.

## Spike report skeleton

`reports/spikes/<TASK-ID>.md`. Required for a spike task to be done; its Evidence must reference it.

```markdown
# <TASK-ID> · <spike title>
- Surfaces: <S-IDs explored, or none>
- Hardware: <H-IDs used, or none>

## Question
The question the spike set out to answer, as stated in the task's Report: verification line.

## Built
What was prototyped, where it lives (alias@sha), and what was deliberately left out.

## Measured
What was measured, how, and the raw numbers or a reference to a benchmark report under reports/benchmarks/.

## Rules out
Options or designs the evidence eliminates, with the observation that eliminates each.

## Recommends
The recommended option and the conditions under which it holds. Names the decision (adr task) that consumes this report.
```

## Benchmark report skeleton

`reports/benchmarks/<B-NNN>/<alias>@<sha>-<H-NNN>.md`. One measured result per benchmark per commit per machine.

```markdown
# <B-NNN> · <alias>@<sha> on <H-NNN>
- Benchmark: <B-NNN>
- Machine: <H-NNN>
- Commit: <alias>@<sha>
- Milestone: <TOKEN whose target this run is judged against>

## Machine
Exact configuration as run: firmware, kernel build, display mode, power profile, isolation settings, anything that differs from the register entry.

## Method
Harness invocation, iteration count, warm-up, statistic reported (p50, p99, mean), and any deviation from the Method field of the register entry.

## Raw numbers
The measured values, in a table or as an attached CSV path, for every statistic the register defines.

## Comparison baseline
The same measurement on the comparison systems named in the register's Baselines field (Linux, Windows, macOS, containers, runtimes) on the same machine where possible, and the resulting ratio. No superiority claim beyond the numbers.
```

## Compatibility report skeleton

`reports/compat/<C-NNN>/<alias>@<sha>-<H-NNN>.md`. One corpus run per commit per machine.

```markdown
# <C-NNN> · <alias>@<sha> on <H-NNN>
- Corpus: <C-NNN>
- Machine: <H-NNN>
- Commit: <alias>@<sha>
- Milestone: <TOKEN whose threshold this run is judged against>

## Summary
Counts per rating on the corpus's scale and the resulting percentages against the milestone threshold. Regressions from the previous committed run, by entry.

## Per-entry ratings
| Entry | Rating | Integration | Notes |
|---|---|---|---|
One row per corpus entry: the scenario's rating on the register's scale (Linux: pass or fail with integration score; Windows: Platinum, Gold, Silver, Bronze, Broken), the integration checks passed (launcher, clipboard, file chooser, notifications, audio, scaling), and the failure signature if any.

## Method
Harness invocation, scenario script version, and any entries skipped with the reason.
```

## Rules

- Never edit a committed report; a new run is a new file.
- A report cites the register entry it measures and never redefines the metric or the corpus.
- Numbers live here and in the generated views, never in task prose, gate prose or register fields.
- No calendar dates. The commit identifies the point in time.
