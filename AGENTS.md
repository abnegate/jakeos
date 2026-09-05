# Agents

Rules for AI agents working in this repository. `CONVENTIONS.md` is normative; this document tells you how to apply it with the least context.

## 1. Load order

Before touching a task, load exactly this, in this order:

1. `roadmap show <ID>`: the task block, resolved dependencies and dependents, derived state, the Decision and Consequences of every adr task in the closure, the gate criteria the task serves, the B-IDs it cites with the active target, glossary entries for its capitalized terms and the repository aliases it may use.
2. The task's own block in `workstreams/<PREFIX>.md`, if you intend to edit it.
3. The workstream header (`Prefix`, `Lead`, `Baseline`, `## Scope`, `## Out of scope`).
4. Each `decisions/D-NNNN-*.md` cited by the task or by adr tasks in its closure.
5. The cited `BASELINE.md` sections, by `§` number.

Do not load whole workstream files or `generated/` views unless the task is about them. `generated/index.json` is the machine-readable export when you need graph-wide facts.

## 2. Hard rules

- **Never edit generated output.** `ROADMAP.md`, `STATUS.md`, `generated/**` and every `<!-- roadmap:generated:begin … -->` block are rewritten by `roadmap gen`. Edit sources and regenerate.
- **Never allocate an ID by hand.** Use `roadmap new task`. On a branch during generation, use only draft IDs (`PREFIX-@slug`) listed in `tools/coverage/slugs.tsv`.
- **Never mark a task done and never set `Verified by`.** Tick acceptance boxes and append Evidence lines as work lands; a human sets `Status: done` and `Verified by`. `Verified by` is never an `@agent/` identity.
- **Never invent register IDs.** Every B, R, Q, S, C, T, I and H you cite must already exist in `registers/`. To add one, add the register entry in the same change with a proper title and fields.
- **Never write "blocked" in prose.** If Verification cannot run in your environment (no hardware, no credentials, no network), record it: `roadmap block <ID> "reason"` mints a Q entry and adds it to `Depends on`.
- **Leave `Owner: none` when authoring roadmap content.** Writing or editing tasks is not doing the work they describe. Set `Owner: @agent/<name>` only when you claim a task to execute it, and unclaim when you stop.
- **Edit only what you own.** When executing a task, edit that task's block plus generated output. When authoring a workstream file during generation, edit only that file.
- **Never edit a done task's** acceptance criteria, Milestone, Type or Size.
- **Never encode a decision in a Description.** Decisions live in `decisions/` and are made by adr tasks.
- **Never cite a performance number in prose.** Cite the B-ID.
- **Never write a calendar date** anywhere the tool reads.
- **Never delete or renumber an ID**, and never move a task between files.
- **Never depend on a D-ID.** Depend on the adr task.
- **Never break milestone monotonicity.** A dependency's rank must be less than or equal to the dependent's rank, with no exception for adr or spike dependencies. Move the decision earlier; do not misstate the dependent.

## 3. Commands

Run from the repository root. `roadmap` means `cargo run -q --manifest-path tools/roadmap/Cargo.toml --` or a built binary on `PATH`.

| Command | Purpose |
|---|---|
| `roadmap check [--strict] [--json]` | Validate everything. Non-zero exit on error. `--json` gives `file:line` diagnostics with fix hints |
| `roadmap check --allow-drafts --index tools/coverage/slugs.tsv` | Validate a branch that still uses draft IDs |
| `roadmap fmt [--check]` | Canonical field order, `none` handling, checkbox syntax, `####` headings, glossary casing. Idempotent |
| `roadmap gen [--check]` | Regenerate whole files and marker blocks. `--check` fails if committed output is stale |
| `roadmap coverage` | Coverage orphans in both directions between tasks and `tools/coverage/*.jsonl` |
| `roadmap assign-ids --index tools/coverage/slugs.tsv` | Convert every draft ID to a real ID in one pass (orchestrator only) |
| `roadmap show <ID>` | Everything about one task (section 1) |
| `roadmap ready [--workstream P] [--milestone T] [--size S]` | Todo tasks whose dependencies are all resolved |
| `roadmap blocked [--by <ID>]` | Derived blocked tasks with exact blocker IDs |
| `roadmap critical-path [<TOKEN>]` | Longest weighted chains to the milestone's gates |
| `roadmap gate <TOKEN>` | Every gate of a milestone with every failing reason |
| `roadmap impact <ID> [--summary]` | What a task transitively unblocks |
| `roadmap progress` | Counts per milestone and workstream |
| `roadmap export --json` | Full export |
| `roadmap new task <PREFIX> "<Title>" --milestone <TOKEN> --size <S> [--type <T>] [--depends <IDs>]` | Allocate an ID and insert a stub |
| `roadmap check --base <ref>` | Everything above plus the diff-aware rules: no ID deleted, renumbered or moved since `<ref>`; done tasks frozen; no done → dropped; reopening unticks a box. CI runs it against the base branch of every pull request |
| `roadmap claim <ID> @handle` | todo → in-progress with an owner. Rejects XL tasks and anything not todo |
| `roadmap unclaim <ID>` | in-progress → todo, Owner `none`. `--all [--owner PATTERN]` resets in bulk |
| `roadmap block <ID> "reason"` | Mints the next Q entry in `registers/questions.md` and adds it to `Depends on` |
| `roadmap done <ID> --evidence <entry>… [--verified-by @handle] [--tick]` | Marks done. Requires evidence in the 4.3 grammar and every box ticked (`--tick` ticks them when every criterion holds). Enforces the verifier policy and rejects `@agent/` verifiers. Reverts the edit if the validator finds any error |
| `roadmap drop <ID> --because "<reason>" [--superseded-by <IDs>]` | Retires a live task; the reason starts with `duplicate`, `descoped`, `superseded`, `infeasible` or `merged`; dependents are repointed to the superseders. Never drops a done task |
| `roadmap split <ID> --into "<Title>" --into "<Title>" [--size S]` | Allocates children inheriting milestone, type, dependencies and baseline; drops the parent as superseded; repoints dependents |
| `roadmap move <ID> --milestone <TOKEN>` | Changes the rung; monotonicity is validated and the edit reverted on error |
| `roadmap renumber <OLD> <NEW> [--base origin/main]` | Renames an ID that has never reached the base branch, across every file |

Every mutation runs the validator afterwards and reverts itself if any error appears, so a mutation either leaves the repository green or leaves it untouched. Still planned: `stale`, `slipped`, `history` (git-derived views).

Before returning any change: `roadmap fmt && roadmap gen && roadmap check` must exit 0 (with `--allow-drafts --index` on generation branches). Fix every error yourself; do not report a red check as finished work.

## 4. Size heuristic

Estimate from the task itself, not from optimism:

| Size | Rule of thumb | Signals |
|---|---|---|
| S | One pull request, one subsystem | 1 to 2 acceptance criteria; one crate, service or document; no new interface |
| M | A few pull requests, one subsystem | 3 to 5 criteria; one subsystem; may add an interface used only inside it |
| L | Many pull requests, or crosses subsystems | 6 or more criteria, or criteria that name two or more prefixes, or a new cross-component interface, or hardware bring-up |
| XL | Must be split before leaving todo | You cannot list the criteria without "and" joining unrelated deliverables; or the work spans three or more subsystems; or it is a whole feature ("Windows personality") |

Count acceptance criteria and count the subsystems (prefixes) the criteria touch. If you write XL, put the split plan in the Description, naming the pieces so a later `split` is mechanical.

## 5. The "do not" checklist (§57)

Apply to every task you write or edit. Reject the task if any line matches:

- An acceptance criterion says a POSIX or Linux syscall, Win32 API, path or signal is "available natively". Native software never sees these (§3, §57). Compatibility work belongs to LNX or WIN.
- A `build` task creates a native filesystem or object store, a native GPU driver stack, or a native browser or IDE before 1.0 (§26, §56.1, §57).
- The task makes Wayland, X11 or POSIX the native UI or kernel API (§41, §57, §65).
- An AI-broker or assistant task has no `Depends on` reaching a done semantic-registry task (§44, §57: no AI before the semantic object model).
- A task makes distributed or remote interfaces a kernel concern (§43, §57).
- A task forces every application into Wasm, or makes everything user-space for purity (§57).
- A task rewrites a mature Linux subsystem (filesystem, network stack, DRM/KMS, driver) without an accepted decision listing what mature mechanism is replaced and why (§2, §15 of the principles in §67).
- A task preserves upstream mergeability at the expense of the architecture, or breaks hardware support without need (§57).
- Prose promises a speedup or states a number. Cite a B-ID; every claim gets a benchmark (§54, §57).
- A criterion contains "should", "works correctly", "is fast", "is robust" or any statement two reviewers could disagree about.
- Anything contains a calendar date.
- An L1 surface is `frozen` before V4, or a `Freezes:` has no spike and decision in its closure (§65, §66).
- An adr task's decision has fewer than two options.
- A task with `Type: spike` has no `Report:` line, or with `Type: benchmark` no `Bench:` line.

## 6. Writing a good task

- **Title**: imperative verb plus the deliverable, in glossary casing. "Implement Channel backpressure with bounded queue depth", not "Channel backpressure".
- **Description**: what is built and why it exists in the architecture, two paragraphs at most, citing `§` numbers. Say what mechanism is preserved and what semantics change (§2). No requirement lists.
- **Acceptance criteria**: each one is something a verifier can observe: a test that passes on a named CI matrix entry, a value visible in `os inspect`, a file that exists, a register state. Prefer "returns `Error::Rights` and allocates no handle" over "rejects invalid input". Three to six is typical.
- **Verification**: choose kinds that match the criteria. `Unit` and `Integration` name test paths and matrix entries; `Fuzz` names harness and duration; `Bench` names the B-ID and H-IDs; `Compat` names the C-ID; `Manual` is a procedure someone else can follow; `Review` names who signs off and where; `Demo` names what is shown on which H-ID; `Report` lists the questions a spike must answer.
- **Out of scope**: name the adjacent work and the owning prefix or task ("Revocation walk: CAP. Transfer over channels: IPC."). This prevents duplicate tasks in two files.
- **Dependencies**: declare generously and prune later. Include the adr task for any decision the work assumes, the spike for any surface it fixes, and the harness task for any benchmark it cites.
- **Type**: "decide" → adr; "measure", "prototype", "find out" → spike; a B-ID target → benchmark; "document", "publish" → docs; else build.
- **Milestone**: the earliest rung whose gates need it, never earlier than its dependencies.
- **Covers**: `<!-- covers: INV-0123, GAP-0045 -->` after the description when authoring from the coverage inputs.
- **Evidence**: `- none` until the work lands.

A complete example, using the reserved `EX` prefix:

```markdown
### EX-014 · Implement attenuating derivation of capability rights
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: EX-003, EX-007, EX-009
- Baseline: §7, §8, §9.1
- Benchmarks: B-004
- Risks: R-012

Implement `derive(cap, mask)` so a holder of `Capability<T, Rights>` obtains a new
capability whose rights are a strict subset of its own. The rights word is encoded so
a future hardware-tag path can check the subset relation without kernel metadata (§8);
the encoding is surface S-003, explored by spike EX-009 and decided by EX-007.
Derivation allocates nothing on the fast path.
<!-- covers: INV-0123, GAP-0045 -->

#### Out of scope
Revocation walk (EX-011). Capability transfer over channels (IPC).

#### Acceptance criteria
- [ ] Deriving with a mask that is not a subset returns `Error::Rights` and allocates no handle.
- [ ] A derived capability records its parent for the revocation walk defined by EX-011.
- [ ] No `unsafe` outside `cap/rights.rs`.

#### Verification
- Unit: `kernel:tests/cap/derive_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Fuzz: `kernel:fuzz/cap_derive` one hour nightly without panic.
- Bench: B-004 on H-002; target per register.
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none
```
