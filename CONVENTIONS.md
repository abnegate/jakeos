# Conventions

This document is the normative rulebook for the JakeOS roadmap repository. Every rule here is either enforced mechanically by the `roadmap` tool or is a review duty stated as such.

## 1. Purpose and precedence

The repository is the single source of truth for what JakeOS will build, in what order, and how each piece is proven done. It is read by humans and by AI agents for years, so it is optimized for staying truthful under churn rather than for ease of casual editing.

Precedence, highest first:

1. `tools/schema/fields.json`: the machine contract (field names, order, enums, patterns, register schemas). The validator and formatter read it directly.
2. This document: the meaning of every rule and every procedure around it.
3. `AGENTS.md`, `README.md`, `GLOSSARY.md`, `reports/README.md`, `decisions/README.md`: derived guidance.

If `fields.json` and this document disagree, `fields.json` is what runs; fix the document. If the tool and this document disagree, one of them has a bug; open a GOV task.

Terms in Title Case (Component, Capability, MemoryObject, Milestone, Gate) are defined in `GLOSSARY.md`.

## 2. Repository map

| Path | Kind | Contents |
|---|---|---|
| `README.md` | hand-written | Entry point and reading guide |
| `AGENTS.md` | hand-written | Rules and commands for AI agents |
| `CONVENTIONS.md` | hand-written | This document |
| `GLOSSARY.md` | hand-written | Canonical terms, casing and § refs |
| `BASELINE.md` | hand-written, immutable § numbers | The architecture baseline; the only citable target for `Baseline:` |
| `ROADMAP.md` | generated whole file | Ladder, gate status, progress, ready head, critical path, decision leverage |
| `STATUS.md` | generated whole file | Per-workstream table, blocked-by aggregation, unanchored tasks, steering signals |
| `roadmap.toml` | hand-written | Size weights, policy flags, warning thresholds, generated-file list |
| `workstreams/<PREFIX>.md` | source of truth | One file per workstream; header, scope, tasks ascending by ID |
| `milestones/<TOKEN>.md` | source of truth | Purpose, gates, demos, hardware scope, surfaces to freeze, risks to retire; one generated block |
| `decisions/D-NNNN-<slug>.md` | source of truth | One decision record per file |
| `decisions/README.md` | hand-written + generated block | How decisions work; generated index |
| `decisions/TEMPLATE.md` | hand-written | Decision skeleton |
| `registers/*.md` | source of truth + generated blocks | Typed registers: risks, benchmarks, corpora, threats, invariants, questions, hardware, surfaces, repos |
| `reports/spikes/`, `reports/benchmarks/`, `reports/compat/` | hand-written evidence | Committed reports per fixed skeletons (see `reports/README.md`) |
| `generated/**` | generated | Per-milestone and per-workstream views, ready, blocked, critical path, benchmarks, graph, index |
| `tools/roadmap/` | code | Rust crate, binary `roadmap` |
| `tools/schema/fields.json` | machine contract | Allowed keys, order, enums, patterns |
| `tools/coverage/` | generation inputs | `inventory.jsonl`, `gaps.jsonl`, `extra.jsonl`, `slugs.tsv`, design notes |
| `.githooks/pre-commit` | hook | `roadmap fmt && roadmap gen && roadmap check` |
| `.gitattributes` | config | Generated files marked `linguist-generated -diff merge=ours` |

Structural rules:

- One file per workstream, ordered by task ID. There are no milestone sub-headings and no per-milestone directories: the `Milestone` field is the only copy of milestone membership.
- There is no ID ledger. The workstream file is the ledger; IDs are never deleted, renumbered or moved.
- `blocked`, `ready`, `in-review` and `stale` are derived, never stored.
- Done tasks are frozen (section 7).
- No calendar dates anywhere in the repository content the tool reads.
- Generated output depends only on repository contents. No timestamps, SHAs or environment leak into committed output.

## 3. ID families and allocation

All IDs are disjoint by regular expression (`fields.json` → `idFamilies`).

| Family | Pattern | Example | Lives in |
|---|---|---|---|
| Task | `PREFIX-NNN` (2 to 4 uppercase letters, 3 or more digits) | `EX-014` | `workstreams/<PREFIX>.md` |
| Draft task | `PREFIX-@slug` (`slug` is `[a-z0-9][a-z0-9-]*`) | `EX-@ring-fast-path` | branches only |
| Decision | `D-NNNN` | `D-0007` | `decisions/D-0007-<slug>.md` |
| Risk | `R-NNN` | `R-012` | `registers/risks.md` |
| Benchmark | `B-NNN` | `B-004` | `registers/benchmarks.md` |
| Corpus | `C-NNN` | `C-002` | `registers/corpora.md` |
| Threat | `T-NNN` | `T-003` | `registers/threats.md` |
| Invariant | `I-NNN` | `I-010` | `registers/invariants.md` |
| Question | `Q-NNN` | `Q-004` | `registers/questions.md` |
| Hardware | `H-NNN` | `H-002` | `registers/hardware.md` |
| ABI surface | `S-NNN` | `S-003` | `registers/surfaces.md` |
| Gate | `<TOKEN>-GNN` | `V0-G01`, `1.0-G03` | `milestones/<TOKEN>.md` |
| Demo | `<TOKEN>-DNN` | `V0.5-D01` | `milestones/<TOKEN>.md` |

Rules:

- **Prefix equals file.** A task `KRN-014` lives in `workstreams/KRN.md`. Prefixes are the 39 workstreams listed in `fields.json` → `workstreams`. Prefixes never contain digits.
- **Allocation is monotonic per family.** `next = max(numbers present, including dropped) + 1`. Numbers are zero-padded to at least three digits (four for decisions) and sorted numerically, never lexically.
- **Never reused, never deleted, never moved.** An ID present on `main` must remain in the same file with the same number forever. Tasks are retired only by `Status: dropped`. The diff-aware check (`check --base`) enforces this once it ships; until then it is a review duty.
- **Splitting** drops the original with `Superseded by:` naming the fresh IDs and repoints every dependent in the same change. The parent never keeps its ID with narrowed scope. No suffixes.
- **Draft IDs** exist only on branches. `PREFIX-@slug` is accepted wherever a task ID is accepted, including `Depends on` across new tasks. `roadmap assign-ids --index tools/coverage/slugs.tsv` converts every draft to a real number in one deterministic pass (sorted by milestone rank, then slug). The validator rejects any draft ID on `main`. Slugs are unique across the repository; `tools/coverage/slugs.tsv` is the shared namespace during generation.
- **Parallel authors** allocating the same number conflict at the file tail; the later change renumbers its own new IDs. Renumbering is permitted only for IDs that have never reached `main`.
- **Prefix is allocation origin, not current ownership.** If a workstream is split or renamed by a GOV decision, existing tasks keep their prefix and file; new tasks take the new prefix; a `See also:` header line records the relation.
- **Reserved prefix `EX`** is ignored by referential-integrity checks so this document and `AGENTS.md` can show worked examples. Never create `workstreams/EX.md`.

## 4. Task grammar

A task begins at a heading matching `^### ([A-Z]{2,4})-(\d{3,}|@[a-z0-9][a-z0-9-]*) · (.+)$` (the separator is the middle dot `·`, U+00B7) and ends at the next level-2 or level-3 heading or end of file. `roadmap fmt` rewrites every task into exactly this shape.

```markdown
### EX-014 · Implement attenuating derivation of capability rights
- Type: build            | adr | spike | benchmark | docs
- Milestone: V0          | V0.5 | V1 | V2 | V3 | V4 | 1.0 | LATER
- Status: todo           | in-progress | done | dropped
- Size: M                | S | L | XL
- Owner: none            | @handle | @agent/<name>
- Depends on: EX-003, EX-007, Q-004      | none
- Baseline: §7, §8, §9.1                 | none   (only in Baseline-gap workstreams)
- Benchmarks: B-004      (required when Type is benchmark; optional otherwise)
- Corpora: C-002         (optional)
- Decision: D-0007       (required when Type is adr; forbidden otherwise; exactly one)
- Explores: S-003        (spike only)
- Freezes: S-003         (any Type except spike)
- Risks: R-012           (optional)
- Threats: T-003         (optional)
- Invariants: I-010      (optional)
- Verified by: @handle   (done only; see section 12)
- Superseded by: EX-031, EX-032   (dropped only)
- Dropped because: superseded: split into EX-031 and EX-032   (dropped only)

Description: one or two paragraphs of plain prose stating what is built and why.
No headings, no lists of requirements (those are acceptance criteria).
<!-- covers: INV-0123, GAP-0045 -->

#### Out of scope
Optional. Names adjacent work and the task ID or prefix that owns it.

#### Acceptance criteria
- [ ] Observable, testable statement. At least one. Never contains "should".

#### Verification
- Unit: <test path, CI matrix entry>
- Integration: <harness, scenario>
- Fuzz: <harness, duration>
- Bench: <B-ID> on <H-IDs>; target per register        (required when Type is benchmark)
- Compat: <C-ID> scenario run on <H-IDs>
- Manual: <procedure>
- Review: <who reviews and how the sign-off is recorded>
- Demo: <what is shown, on which H-ID>
- Report: <the questions the spike report must answer>  (required when Type is spike)

#### Evidence
- none                                        (only while Status is not done)
```

### 4.1 Field rules

- Field lines are consecutive `- Key: value` lines immediately after the heading. Keys come from `fields.json` → `task.fieldOrder`; unknown keys are errors; the formatter enforces canonical order.
- `none` is the only null token. Required fields (`Type`, `Milestone`, `Status`, `Size`, `Owner`, `Depends on`, `Baseline`) always appear, with `none` where empty. Conditional fields are omitted when empty; the formatter drops a conditional field whose value is `none`.
- `Depends on` accepts task IDs and Q-IDs only (`fields.json` → `dependsOnFamilies`). To depend on a decision, name the **adr task**, never the D-ID. Self-dependency and cycles are errors.
- `Baseline` cites `§N` or `§N.M` headings that exist in `BASELINE.md`. `Baseline: none` is legal only in a workstream whose header carries `Baseline gap:` (section 10).
- `Owner` matches `^(none|@[a-z0-9][a-z0-9-]*|@agent/[a-z0-9][a-z0-9-]*)$`. One owner per task; co-work is expressed by splitting.
- Every referenced ID (task, D, R, B, C, T, I, Q, H, S) must exist, except the `EX` prefix.

Conditional field summary (`fields.json` → `task.conditional`):

| Field | Required when | Forbidden when | ID family |
|---|---|---|---|
| Benchmarks | Type benchmark | | B |
| Corpora | | | C |
| Decision | Type adr | Type not adr | D (single) |
| Explores | | Type not spike | S |
| Freezes | | Type spike | S |
| Risks | | | R |
| Threats | | | T |
| Invariants | | | I |
| Verified by | Status done and policy (section 12) | Status not done | handle |
| Superseded by | | Status not dropped | task |
| Dropped because | Status dropped | Status not dropped | enum prefix (section 7) |

### 4.2 Description and covers comment

The description is free Markdown prose without headings. It states what is built and why in one or two paragraphs. It never records a decision (that belongs in a decision file), never states a performance number (cite a B-ID) and never says "blocked" (add a dependency or a Q).

The covers comment `<!-- covers: INV-0123, GAP-0045 -->` sits on its own line directly after the description. It matches `fields.json` → `task.coversComment` and names the coverage input items (`INV-`, `GAP-`, `EXTRA-`) this task implements. `roadmap fmt` preserves it; `roadmap coverage` reports every input item covered by no non-dropped task and every task that covers nothing and is cited by no gate. A task added after generation either carries a covers comment or is cited by a gate.

### 4.3 Sections

Exactly these `####` headings, in this order: `Out of scope` (optional), `Acceptance criteria`, `Verification`, `Evidence`. The last three are mandatory with at least one item each.

- **Acceptance criteria** use exactly `- [ ]` or `- [x]`. Each criterion is observable and testable by someone other than the author. The word "should" is banned (`fields.json` → `task.bannedCriteriaWords`).
- **Verification** lines begin with one of the nine kinds: `Unit`, `Integration`, `Fuzz`, `Bench`, `Compat`, `Manual`, `Review`, `Demo`, `Report`. Type `benchmark` requires a `Bench:` line; Type `spike` requires a `Report:` line (`fields.json` → `task.verificationRequiredKind`). Verification is the plan, written before work starts.
- **Evidence** is the result, appended as work lands. Grammar:

| Line | Meaning |
|---|---|
| `- none` | No evidence yet; only legal while Status is not done |
| `- <alias>@<sha>` | Commit in a repository registered in `registers/repos.md` |
| `- <alias>#<number>` | Pull request in a registered repository |
| `- https://<url>` | Review record or external artifact |
| `- report:reports/spikes/<TASK-ID>.md` | Spike report |
| `- report:reports/benchmarks/<B-NNN>/<alias>@<sha>-<H-NNN>.md` | Benchmark report |
| `- report:reports/compat/<C-NNN>/<alias>@<sha>-<H-NNN>.md` | Compatibility report |
| `- decision:<D-NNNN>` | Accepted or rejected decision |

Prose assertions are not evidence. Report paths must exist.

### 4.4 Length

Target 20 to 60 lines per task. The tool warns above `task_lines_warning` (100). A task that needs more is two tasks.

## 5. Size semantics

| Size | Meaning | Weight |
|---|---|---|
| S | One pull request in one subsystem | 1 |
| M | A few pull requests in one subsystem | 3 |
| L | Many pull requests, or work that crosses subsystems | 8 |
| XL | Placeholder that must be split before it may leave `todo` | 20 |

Weights (`roadmap.toml` → `[weights]`) feed size-weighted progress and the critical path. They are estimates and every generated view labels them as such. An XL task whose Description carries no split plan is a warning; an XL task in the active or next rung is a steering signal.

## 6. Type semantics

| Type | Use for | Requires |
|---|---|---|
| `build` | Code, infrastructure, hardware bring-up, anything shipped | Default. May carry `Freezes:` after the surface's spike and decision are done |
| `adr` | Making a decision | `Decision: D-NNNN`; acceptance criteria name the options to evaluate and the review required; a `Review:` verification line; when done, Evidence contains `decision:D-NNNN` and the file is `accepted` or `rejected`. `Verified by` is always required when done |
| `spike` | Time-boxed investigation that produces a report | Optional `Explores: S-IDs`; never `Freezes:`; a `Report:` verification line; when done, `reports/spikes/<TASK-ID>.md` exists and Evidence references it |
| `benchmark` | Defining, harnessing or running a benchmark | `Benchmarks: B-IDs`; a `Bench:` verification line; when done, a report file per in-scope H-ID meeting the register target kind for the task's Milestone |
| `docs` | Documentation, policy text, published statements | When done, at least one `Review:` line or `https://` evidence |

Choosing a Type: an item that says "decide" or "choose" is `adr`; "prototype", "measure X against Y" or "find out whether" is `spike`; a B-ID target is `benchmark`; "document", "publish" or "write the guide" is `docs`; everything else is `build`.

## 7. Status model

### 7.1 Stored states

`todo`, `in-progress`, `done`, `dropped`. Nothing else is ever written in `Status`.

### 7.2 Derived states

Computed by the tool, shown in generated views, never typed:

| Derived | Definition |
|---|---|
| `ready` | `todo` and every `Depends on` entry is resolved: task done, or dropped with a done superseder, or Q answered |
| `blocked` | `todo` or `in-progress` with at least one unresolved dependency; views list the exact blocker IDs and aggregate by blocker |
| `in-review` | `in-progress` with at least one Evidence line other than `none` |
| `stale` | `in-progress` with no change to its block across a configured number of commits; local-only via git, never committed |

Depending on a dropped task that has no superseder is an error, not a state.

### 7.3 Transitions

| From | To | Preconditions |
|---|---|---|
| todo | in-progress | Owner is not `none`; Size is not XL |
| in-progress | todo | Always allowed (unclaim); Owner may reset to `none` |
| todo or in-progress | done | Definition of done (7.4). `todo → done` in one change is allowed only when every done rule holds, including `Verified by` |
| done | in-progress | Reopen: at least one acceptance box is unticked in the same change; Evidence lines stay |
| todo or in-progress | dropped | `Dropped because:` present; if any non-dropped task depends on it, `Superseded by:` names existing tasks and dependents are repointed in the same change |
| done | dropped | Forbidden. Done work is history; add a new task |
| dropped | todo | Revive: remove `Dropped because` and `Superseded by` |

### 7.4 Definition of done

All of the following are errors if unmet when `Status: done`:

1. Every acceptance checkbox is `[x]`.
2. Verification has at least one line.
3. Evidence has at least one line of valid grammar, and every dependency is resolved.
4. `Verified by: @handle` is present when required by section 12; it differs from Owner and is never an `@agent/` identity. The verifier reruns the Verification section rather than reading the pull request.
5. Type `adr`: the linked decision file is `accepted` or `rejected`, and Evidence contains `decision:D-NNNN`.
6. Type `spike`: `reports/spikes/<TASK-ID>.md` exists with the skeleton headings from `reports/README.md`, and Evidence references it.
7. Type `benchmark`: for each listed B-ID, a `reports/benchmarks/<B-NNN>/` report exists for every H-ID in the task's Milestone hardware scope and meets the register target kind for that Milestone, or the Description states why the target is deferred (warning). A task listing `Corpora:` with a `Compat:` line likewise needs a `reports/compat/<C-NNN>/` report per in-scope H-ID.
8. Type `docs`: at least one `Review:` line or `https://` evidence line.
9. `Freezes:` present: each S-ID's register state is `frozen` and names this task; the dependency closure contains a done spike whose `Explores` names the surface and a done adr task whose decision lists the surface; for L1 surfaces the decision cites a benchmark report.

### 7.5 Frozen once done

A done task's acceptance criteria, Milestone, Type and Size never change. Description clarifications and additional Evidence lines remain allowed. To change scope, add a new task.

### 7.6 Dropped tasks

`Dropped because:` begins with one of `duplicate`, `descoped`, `superseded`, `infeasible`, `merged` (`fields.json` → `task.droppedReasons`), followed by a colon and one line of explanation:

```
- Dropped because: duplicate: EX-021 covers the same deliverable
- Dropped because: superseded: split into EX-031 and EX-032
```

Dropped tasks stay in place, keep their ID, are hidden by default in views, and are excluded from progress denominators. A dropped task is edited only to revive it.

### 7.7 Other invariants

- All boxes ticked while Status is not done: warning.
- Any box unticked while Status is done: error.
- `in-progress` requires Owner not `none` and Size not XL.

## 8. Milestones

### 8.1 Tokens and ranks

| Token | Rank | Title |
|---|---|---|
| `V0` | 1 | Execution model |
| `V0.5` | 2 | Application model |
| `V1` | 3 | Developer preview |
| `V2` | 4 | Desktop preview |
| `V3` | 5 | Public alpha |
| `V4` | 6 | Beta |
| `1.0` | 7 | Public stable |
| `LATER` | 99 | Parking rung for work deferred beyond 1.0 |

`1.0` is the token, not `V5`. `LATER` has no gates, is excluded from 1.0 progress and counted in totals.

### 8.2 Monotonicity

A dependency's Milestone rank must be less than or equal to the dependent's rank. There is no exception for adr or spike dependencies: a decision or investigation that gates work must be scheduled in the same or an earlier rung than that work. If a V0.5 task needs a decision currently planned for V1, move the adr task to V0.5; do not misstate the dependent's Milestone. Nothing outside `LATER` may depend on a `LATER` task; promoting a `LATER` task means changing its Milestone first.

### 8.3 Milestone file grammar

```markdown
# V0 · Execution model
- Sequence: 1
- Title: Execution model
- Baseline: §59
- Hardware scope: H-001, H-002
- Surfaces to freeze: none
- Risks to retire: R-004, R-012

## Purpose
What this milestone proves and what it explicitly is not.

## Not in this milestone
Explicit non-goals.

## Gates
### V0-G01 · Fork boots on all hardware in scope
- Kind: functional
- Verified by: EX-003, EX-004
One or two lines stating the criterion in human terms. Never a number: cite the B-ID or C-ID.

### V0-G05 · Component creation latency published
- Kind: benchmark
- Verified by: EX-040
- Benchmark: B-001
- Or: EX-041
Satisfied when the B-001 target for V0 is met on every in-scope H-ID, or when adr task EX-041 is done with an accepted decision.

### V0-G06 · L0 corpus has zero regressions
- Kind: compatibility
- Verified by: EX-050
- Corpus: C-001

### V0-G09 · Exit review recorded
- Kind: process
- Verified by: EX-090

## Demos
### V0-D01 · Component to Channel to MemoryObject transfer round trip
- Verified by: EX-020, EX-021
Prose describing what is shown and on which H-ID.

## Notes
Free prose; may cite R-IDs and Q-IDs; may not introduce task IDs as requirements.

<!-- roadmap:generated:begin milestone -->
<!-- roadmap:generated:end -->
```

Header fields in `fields.json` → `milestoneFile.fieldOrder`; sections in `milestoneFile.sections`. Gate fields: `Kind`, `Verified by`, `Benchmark`, `Corpus`, `Or`. Gate kinds: `functional`, `benchmark`, `compatibility`, `demo`, `process`. `Benchmark:` is required for Kind benchmark; `Corpus:` for Kind compatibility; `Or:` names an adr task that satisfies the gate when its decision is accepted, for gates whose target may be renegotiated.

A milestone file never lists tasks by hand and never carries a hand-set status. Task membership comes solely from each task's `Milestone` field; the file names the gates and the task IDs that verify them.

### 8.4 Derived milestone status

- A gate is **satisfied** when all its `Verified by` tasks are done (a dropped task counts through its done superseder), its benchmark target for this milestone is met on every in-scope H-ID or its `Or:` task is done, and its corpus threshold for this milestone is met.
- A milestone is **complete** when every gate is satisfied, every listed surface is `frozen` or deferred by an accepted decision naming it, every listed risk is `mitigated`, `accepted` or `closed`, and no task with this Milestone is `todo` or `in-progress`.
- **active** is the lowest Sequence not complete; **next** is active plus one; everything else is **planned**.

`roadmap gate <TOKEN>` enumerates every failing reason.

### 8.5 Validator rules

- Every milestone except `LATER` has at least one gate.
- Every `Verified by` task exists and has Milestone rank less than or equal to the gate's rank.
- Kind benchmark cites a B-ID that has a target clause for this milestone; Kind compatibility cites a C-ID with a threshold for this milestone.
- Hardware, surface and risk IDs exist.
- A task whose Milestone is this token but that is reachable from no gate or demo of any milestone (directly as `Verified by`, or transitively as a dependency of a verifying task) is **unanchored**, reported in `STATUS.md`. Unanchored tasks are a warning; the target is zero.
- Changing `Verified by` or `Sequence` on a complete milestone requires the commit trailer `Roadmap-Decision: D-NNNN`.

## 9. Workstream files

```markdown
# KRN · Kernel fork and upstream tracking
- Prefix: KRN
- Lead: none
- Baseline: §2, §5, §6, §55, §56.4
- Baseline gap: <one line stating why the baseline has no section for this scope>   (only for prefixes listed below)
- See also: <PREFIX>   (optional; records a split or rename)

<!-- roadmap:generated:begin summary -->
<!-- roadmap:generated:end -->

## Scope
Prose naming every deliverable class this workstream owns.

## Out of scope
Prose naming adjacent work and the prefix that owns it.

## Tasks
### KRN-001 · …
```

- Tasks appear in ascending numeric ID order with no sub-headings between them.
- `Baseline gap:` is permitted only for `BOOT`, `NET`, `AUD`, `TXT`, `PWR`, `SVC`, `DOC`, `LAB`, `MED`, `VIRT` (`fields.json` → `baselineGapAllowed`). Only tasks in those files may write `Baseline: none`, and each such file's first adr task defines the scope the baseline left open.
- The tool warns when a file exceeds `workstream_lines_warning` (6000) lines; the remedy is a GOV adr task proposing a split.
- Scope disputes are resolved by editing scope prose, never by duplicating tasks.

## 10. Decisions

A decision is made by a task and recorded in a file. The two are coupled mechanically.

**The task** is Type `adr`, lives in the workstream that owns the question, carries `Decision: D-NNNN`, names the options and the required review in its acceptance criteria, has a `Review:` verification line, and when done carries `decision:D-NNNN` in Evidence.

**The file** `decisions/D-NNNN-<slug>.md` follows `decisions/TEMPLATE.md`:

```markdown
# D-0007 · Rights encoding for Capability<T>
- Status: proposed | accepted | rejected | superseded
- Task: EX-007
- Surfaces: S-003 | none
- Layer: L1 | L2 | L3 | L4 | none
- Spikes: EX-009 | none
- Supersedes: none
- Superseded by: none
- Baseline: §7, §8, §66
- Revisit when: <a condition, never a date>

## Context
## Options
### Option A · <name>
### Option B · <name>
## Decision
## Consequences
## Rejected options and why
## Follow-ups
```

Each option has a one-line summary, its consequences, and its evidence (spike or benchmark report paths). At least two options (`fields.json` → `decision.minOptions`).

Coupling rules:

- Exactly one adr task per decision file and vice versa.
- Task `todo` or `in-progress` ⇔ file `proposed`. Task `done` ⇔ file `accepted` or `rejected` (a rejection is a completed decision). Task `dropped` ⇔ file absent or `superseded` before acceptance, with the reason in `Dropped because`.
- A decision cannot be `accepted` while its task has unticked boxes.
- Follow-up IDs exist. `Supersedes` chains are acyclic.
- A decision listing L1 or L2 surfaces cites at least one done spike whose `Explores` names each surface; L1 additionally cites a benchmark report.
- Layer `none` is for decisions that fix no interface shape (process, licensing, scoping).

Immutability: an accepted decision changes only in `Superseded by`. Revising means a new D-ID with `Supersedes:`, a new adr task, and the old file set to `superseded` while the old task stays done. When a superseding decision touches a frozen surface, the surface returns to `prototyped` and every task with `Freezes:` on it is flagged.

Tasks that need a decision list the **adr task ID** in `Depends on`, never the D-ID; the graph has one edge type. `roadmap show` inlines the Decision and Consequences sections of every adr task in the closure.

Roadmap-process decisions (grammar changes, workstream splits, baseline amendments) are ordinary GOV adr tasks. A diff to `BASELINE.md` requires the commit trailer `Roadmap-Decision: D-NNNN`.

## 11. Registers

Each register is one file holding entries of one ID family. Entries are referenced from tasks, gates and decisions by ID; the tool validates every reference and every field against `fields.json` → `registers`.

Entry grammar, shared by all registers:

```markdown
### R-012 · Rights encoding cannot be made hardware-checkable later
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: none
- Retire by: V1
One paragraph stating the entry in plain terms.
```

Field lines follow the order in `fields.json`; `none` is the null token. Reverse-link fields (`Mitigated by`, `Addressed by`, `Enforced by`, `Explored by`, `Decided by`, `Frozen by`) are derived from the task graph by `roadmap gen` and overwritten on every run; authors write `none` and let the tool fill them. `Answered by` on questions is set by hand when the question closes (a task ID or a D-ID).

| Register | File | Fields | Enums |
|---|---|---|---|
| R risks | `registers/risks.md` | Likelihood, Impact, Status, Mitigated by, Retire by | Likelihood low/medium/high; Impact low/medium/high/critical; Status open/mitigated/accepted/closed |
| B benchmarks | `registers/benchmarks.md` | Metric, Method, Harness, Baselines, Targets, Status | Status defined/harnessed/measured |
| C corpora | `registers/corpora.md` | Personality, Size, Scenario, Scale, Thresholds, Status | Personality linux/windows; Status defined/scripted/measured |
| T threats | `registers/threats.md` | Actor, Asset, Vector, Status, Addressed by | Status open/mitigated/accepted |
| I invariants | `registers/invariants.md` | Baseline, Enforced by, Status | Status stated/enforced |
| Q questions | `registers/questions.md` | Workstream, Status, Answered by | Status open/answered/withdrawn |
| H hardware | `registers/hardware.md` | Kind, Tier, CPU, GPU, Network, First milestone, Status | Kind qemu/desktop/laptop; Status planned/procured/in-lab/retired |
| S surfaces | `registers/surfaces.md` | Layer, Owner, State, Explored by, Decided by, Frozen by | Layer L1/L2/L3/L4; State open/prototyped/frozen/superseded |
| repos | `registers/repos.md` | Alias, URL, Purpose (table) | |

Register-specific rules:

- **Benchmarks** hold definitions and targets only. No measured value ever appears in the register; results live in `reports/benchmarks/` and the generated `results` block. `Targets` is a semicolon-separated list of clauses, one per milestone that gates on the metric, each of the form `<TOKEN> publish`, `<TOKEN> absolute <expression>` or `<TOKEN> regression <pct>% vs <TOKEN>`:

  ```
  - Targets: V0 publish; V1 absolute p50 ≤ 2 µs; V4 regression 5% vs V3
  ```

  `publish` means measure and publish with no threshold. `absolute` states the threshold. `regression` compares against the latest committed report for the named earlier milestone on the same H-ID. `Baselines` names what the metric is compared against (Linux, Windows, macOS, containers, language runtimes). `Harness` is a repository alias plus path. Every §54 metric has a B-ID.
- **Corpora** define the Linux (L0 to L5) and Windows (W1 to W3) compatibility corpora. `Size` is the entry count; `Scenario` is the harness alias that runs the scripted scenarios; `Scale` is the rating scale (Windows: Platinum, Gold, Silver, Bronze, Broken; Linux: pass, fail, with integration scoring); `Thresholds` follows the same per-milestone clause grammar as benchmark targets, for example `V2 Gold ≥ 50%, Silver ≥ 70%; V3 Gold ≥ 60%, Silver ≥ 80%`. Results live in `reports/compat/`.
- **Hardware** entries are QEMU profiles and physical machines. `Tier` is `1`, `2` or `none` (QEMU profiles). `First milestone` is the earliest rung whose gates run on it; a procurement or bring-up task must exist in an earlier or equal rung.
- **Surfaces** are ABI and interface shapes with a stability Layer (§66). See section 12.
- **Invariants** are standing rules (constraints, non-goals) that are not work items. Each becomes `enforced` when at least one done task lists it under `Invariants:` and that task installs a lint, gate or test.
- **Questions** are the only way to record an external impediment as a dependency. A Q entry has an owning Workstream and a Status; open questions with no bound task are a warning.
- **Repos** map evidence aliases to URLs. An evidence line `kernel@3f9c1ab` requires an alias `kernel`.

Generated blocks inside registers: `risks.md` (`status`), `benchmarks.md` (`results`), `questions.md` (`status`), `surfaces.md` (`status`).

Reports directory structure:

```
reports/spikes/<TASK-ID>.md
reports/benchmarks/<B-NNN>/<alias>@<sha>-<H-NNN>.md
reports/compat/<C-NNN>/<alias>@<sha>-<H-NNN>.md
```

Only gate runs and milestone summaries are committed; nightly results go to the benchmark time-series export outside this repository. Skeletons are in `reports/README.md`.

## 12. Surfaces and freeze discipline

An ABI surface (S-NNN) moves through `open → prototyped → frozen`, or to `superseded`. The sequence is always spike, then decision, then freeze:

1. A `spike` task lists the surface in `Explores:` and is done with a report.
2. An `adr` task's decision lists the surface in `Surfaces:` and is `accepted`.
3. A `build` (or other non-spike) task lists the surface in `Freezes:` and is done; the register state becomes `frozen` and `Frozen by` names the task.

L1 surfaces additionally require the decision to cite a benchmark report. The tool rejects any `Freezes:` whose closure lacks the spike and the decision.

Schedule: L1 surfaces are `prototyped` through V0 (V0 lists no surfaces to freeze), become freeze candidates at V1 (SDK v1) and are `frozen` at V4 with a conformance suite. L2 interface-evolution rules are frozen at V1; L2 interface versions are locked at V4. L3 and L4 never enter the register as frozen surfaces.

## 13. Verification policy

- `Verified by` is required on every done task from V1 onward (`roadmap.toml` → `require_independent_verification` is switched on when V1 becomes active).
- `Verified by` is required from V0 on any done task that carries `Freezes:` or is Type `adr` (`verify_freezes_and_adr_always = true`). These are the most permanent decisions (§65).
- The verifier differs from the Owner, is a human `@handle`, and is never `@agent/<name>`. Agents never verify their own or any other task.
- The verifier reruns the Verification section. Reading the pull request is not verification.

## 14. Generated output

- Whole files: `ROADMAP.md`, `STATUS.md`, everything under `generated/`. Each starts with a static header comment naming the generator and source directories.
- Marker blocks, replaced verbatim between `<!-- roadmap:generated:begin <name> -->` and `<!-- roadmap:generated:end -->`: `milestone` in each milestone file, `index` in `decisions/README.md`, `status` or `results` in registers, `summary` in each workstream header.
- Generation is deterministic: two clones at the same content produce byte-identical output. No timestamps, SHAs, hostnames or environment.
- Never hand-edit generated output. `roadmap gen --check` fails when committed output is stale; the pre-commit hook regenerates.
- `.gitattributes` marks generated paths `linguist-generated -diff merge=ours`. On a merge conflict in generated output, take either side and run `roadmap gen`.

## 15. Writing style

- **Titles** are imperative and name the deliverable: "Implement attenuating derivation of capability rights", not "Capability derivation" or "Rights should be derivable". Non-imperative titles are warnings.
- **No "should"** in acceptance criteria (error). Avoid it everywhere else; write what is.
- **No calendar dates** anywhere the tool reads (`fields.json` → `datePattern`). Sequence is expressed by Milestone and `Depends on`; "revisit when" is a condition.
- **No performance numbers in prose.** Descriptions, criteria, gate text and milestone prose cite B-IDs and C-IDs; the numbers live in the registers. "Component creation latency meets the B-001 target for V1" is correct; a number in the sentence is not.
- **Glossary casing.** Use the spellings in `GLOSSARY.md` (MemoryObject, ResourceDomain, TaskGroup, UserSelected). `roadmap fmt` rewrites exact case-insensitive matches in task titles to the canonical spelling.
- **Baseline citations** are `§N` or `§N.M` against `BASELINE.md` only. Never cite by page, quote or paraphrase.
- **Out of scope** names the owning task or prefix, not just "elsewhere".
- No em-dashes. American spelling.

## 16. Commit grammar

| Subject | Use |
|---|---|
| `roadmap(<ID or PREFIX>): <verb>` | Task changes; verbs `add`, `claim`, `unclaim`, `edit`, `block`, `done`, `drop`, `split`, `move`, `reopen` |
| `decision(D-NNNN): accept` / `reject` / `supersede` / `propose` | Decision file changes |
| `milestone(<TOKEN>): edit` | Milestone file changes |
| `register(<ID>): add` / `edit` / `retire` | Register changes |
| `gen: regenerate` | Generated output only, when not folded into another commit |
| `tool: <subject>` | Changes under `tools/` |
| `docs: <subject>` | Hand-written documents at the repository root |

Trailers:

- `Roadmap-Decision: D-NNNN` on any commit that changes `BASELINE.md`, or changes `Verified by` or `Sequence` on a complete milestone.
- In code repositories, `Roadmap: <TASK-ID>` on every commit that serves a task (section 18).

`git log --grep '<ID>'` is the task history.

## 17. Pull request policy

- Everything reaches `main` through a pull request. Nothing is pushed directly.
- Required checks: `roadmap fmt --check`, `roadmap check`, `roadmap gen --check` (a `check --base origin/main` diff-aware pass once it ships).
- **Metadata-only fast lane**: a pull request that touches only `Status`, `Owner`, `Verified by`, Evidence lines, checkbox states and generated output merges on green checks without human review.
- Pull requests touching acceptance criteria, `Depends on`, `Milestone`, Description, scope prose, gates or registers need approval from the owning workstream's Lead (or a GOV maintainer while `Lead: none`).
- The pre-commit hook (`.githooks/pre-commit`, enabled with `git config core.hooksPath .githooks`) runs `fmt`, `gen`, `check` so pull requests arrive formatted and regenerated.

## 18. Contributor procedures

Commands marked "manual until shipped" are planned GOV tasks; until they exist, perform the equivalent edit by hand and let `roadmap check` validate the result.

| Procedure | How |
|---|---|
| Pick work | `roadmap ready` (or `generated/ready.md`), then `roadmap show <ID>`. If the block is unclear, do not start; open a pull request that improves it |
| Claim | Set `Owner: @handle` (agents `@agent/<name>`) and `Status: in-progress`. `roadmap claim <ID> @handle` is manual until shipped |
| Unclaim | Set `Status: todo`, optionally `Owner: none`. `roadmap unclaim --all` exists for bulk resets |
| Update status | Only through the transitions in section 7. Never edit generated output |
| Add a task | `roadmap new task <PREFIX> "<Title>" --milestone <TOKEN> --size <S>` allocates the ID and inserts a stub. Fill Description, at least one criterion and one Verification line before pushing. Cite `Baseline:` or write `none` deliberately. Add the task to the workstream that owns the deliverable |
| Split | Drop the original with `Dropped because: superseded: split into …` and `Superseded by:` naming the new tasks; repoint dependents in the same change. Never suffixes, never reuse |
| Retire | Set `Status: dropped` with `Dropped because:`. Never delete a block |
| Move between milestones | Edit the `Milestone` field only; the commit message states why; check monotonicity of dependents and dependencies |
| Re-scope | Free for `todo` and `in-progress`. For `done`, add a new task |
| Record an impediment | Never write "blocked" in prose. Add the blocking task to `Depends on`, or add a Q entry to `registers/questions.md` and depend on it. `roadmap block <ID> "reason"` does both in one step once shipped |
| Attach evidence | Only the grammar in 4.3: registered repository commits and pull requests, report files, decisions, review URLs |
| Make a decision | Open the adr task first, then create `decisions/D-NNNN-<slug>.md` from the template with `Status: proposed`. Discuss in the pull request. Accept by ticking criteria, setting the file status and marking the task done in one change |
| Freeze a surface | Confirm a done spike `Explores` it and an accepted decision lists it (L1: with a benchmark report), then list it in `Freezes:` on the freezing task |
| Link code | Branch `ex-014-derive-rights`; commit trailer `Roadmap: EX-014`; pull request title begins with the ID. The roadmap side is authoritative for done-ness |
| Change the baseline | GOV adr task, then a commit to `BASELINE.md` carrying `Roadmap-Decision: D-NNNN`. Section numbers never change |

## 19. Rejected structural ideas

These were evaluated during the design pass and rejected. Do not re-propose them without new evidence:

- Per-milestone directories or `##` milestone sections inside workstream files.
- ID ledger files (`ids.tsv`, `IDS.lock`).
- Stored `blocked` or `review` states; free-text `Blocked by:`.
- Per-task `Risk:` enum (R-IDs carry Likelihood and Impact instead).
- `Moved from`, `Split from`, `Reopened`, `Tags` fields.
- Hand-set milestone status or hand-ticked gate checkboxes.
- `V5` or `post-1.0` tokens (the token is `1.0`; the parking rung is `LATER`).
- Bold field syntax; em-dash as a null token (the null token is `none`).
- Git SHA provenance or timestamps inside generated output.
- CI-committed snapshot files for burndown.
- Parent keeps its ID with narrowed scope after a split.
- `todo → done` shortcuts for small tasks.
- Direct pushes to `main`.
- Python tooling.
- Dual dependency targets (task IDs and D-IDs in the same field).
- A mandatory `Why` section.
- A cross-rank exception to milestone monotonicity for adr or spike dependencies.
