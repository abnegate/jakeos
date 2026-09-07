# Review 05 · Every task executable from its own block

**Scope.** All 191 open V0 tasks, read one at a time with the question an agent asks on claim day: can I carry this to done with nothing but this block and the current code? The standard set by the maintainer is exact: no further context than the task description and the repository as it stands.

**Method.** Each block was measured before rewriting (description length, whether any repository path was named, whether the description named a single identifier such as a type, file, Operation or decision), then rewritten in full and re-measured. The rewrite touched the Description, `Out of scope`, the acceptance criteria and the Verification of every open V0 task, and added a section to the grammar.

## 1. Findings

| # | Finding | Severity | Resolution |
|---|---|---|---|
| 1 | Descriptions averaged 33 to 48 words, restated the baseline and named no artifact. Roughly half the open V0 tasks named no repository path anywhere in the block, and about 95 percent named no identifier in the description. An agent had to invent file names, crate names, type names and test paths, so two agents on adjacent tasks would invent them differently. | high | Every open V0 block rewritten. Descriptions now average 148 words and name the files, types, Operations, error variants, decisions and matrix entries the task touches. |
| 2 | The grammar had no place to say where the work lands. Acceptance criteria say how a reader knows the work is right; nothing said what the agent creates. | high | New optional `#### Deliverables` section (`- <alias>:<path> · <what it is>`), parsed, formatted and checked: unknown alias is E-020, a malformed line is E-017, a missing section on a build, docs or benchmark task in a `deliverables_required` milestone is W-020 while todo and E-117 once in progress. Done tasks are frozen and exempt. `policy.deliverables_required = ["V0"]`. |
| 3 | Test paths, crate names and header locations were named differently across workstreams (some `kernel:tests/...`, some `tools/testing/selftests/...`, some none). | medium | Every V0 block follows KRN-013 for the kernel tree and BLD-081 for the platform monorepo: crates `jakeos-<area>-<name>`, tests `<alias>:tests/<area>/<name>_*`, kernel selftests under `kernel:tools/testing/selftests/jakeos/<area>/`, generated uapi headers under `kernel:include/uapi/linux/jakeos/`. |
| 4 | Verification lines named environments loosely ("on QEMU", "on the reference machine"). | medium | Verification names the matrix entry (`qemu-x86_64`, `hw-h002`) and the test path; W-019 rejects an undeclared entry. |
| 5 | Error handling was described in prose ("fails", "is refused"). | low | Criteria name the variant: `Error::Rights`, `Exhausted`, `Cancelled`, `DeadlineExceeded`, `Revoked`, `Disconnected`, `Integrity`. |

Dependency changes made while rewriting, each a real ordering the old block hid: BLD-082 (platform monorepo) added to every task that lands a platform artifact; KRN-013 to every kernel-tree task; OBS-006 to tasks whose criteria inspect state through `os inspect`; SCH-008 to tasks that create a ResourceDomain. CAP-006 lost IPC-014 (a cycle).

## 2. Measured before and after

| Measure | Before | After |
|---|---|---|
| Open V0 tasks | 191 | 191 |
| Mean description words | 33 to 48 by workstream | 148 |
| Blocks naming a repository path | about 50 percent | 191 of 191 |
| Descriptions naming an identifier | about 5 percent | 190 of 191 |
| Build, docs and benchmark tasks with Deliverables | 0 of 124 | 124 of 124 |
| Mean acceptance criteria per task | 3.4 | 3.8 |

`roadmap check --strict` reports no errors and no warnings with the V0 policy on; `roadmap coverage` is unchanged.

## 3. What was left alone

- Done V0 tasks (KRN-010, SEC-002 and the GOV tooling tasks) keep their original blocks. Their Evidence names what they produced and CONVENTIONS forbids editing a done task.
- adr and spike tasks carry no Deliverables section. Their artifact is fixed by Type: the decision file named by `Decision:` and the report at `reports/spikes/<ID>.md`.
- Milestones after V0 are unchanged and stay off the `deliverables_required` list until each is rewritten to the same standard. The rewrite order is V0.5, V1, V2, V3, V4, 1.0, LATER; each joins the policy list when its last open build, docs or benchmark task has a Deliverables section.

## 4. The standard, for later rewrites

A block meets the standard when an agent reading only that block and the code can answer, without asking: which files it will create or change (Deliverables, alias-qualified); which types, Operations, error variants and decisions it must use (Description); what a reviewer will run and where (Verification with test paths and matrix entries); what it must not do (Out of scope naming the owning task). Words that hide a choice ("appropriate", "suitable", "as needed", "fails") are replaced by the choice.
