# D-0001 · Roadmap repository process
- Status: proposed
- Task: GOV-004
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §54, §65, §66, §67
- Revisit when: the validator cannot express a rule the project needs, or the task count or contributor count makes single-file workstreams unworkable despite generated views

## Context

JakeOS will take years and thousands of tasks to reach 1.0, with humans and AI agents working in parallel. The roadmap must stay truthful under that churn: a claim of progress must be backed by evidence, a decision that fixes an ABI surface must be preceded by a spike (§65), a performance claim must point at a benchmark definition (§54), and the stability layers (§66) must be visible in what is frozen when. The principles in §67 apply to the project's own process: preserve mature mechanisms (git, Markdown, pull requests), replace inherited semantics (free-text status, hand-maintained percentages, dated plans that rot).

The question is how to store and validate the roadmap so that a single task ID is enough to start work, mechanical checks keep the graph consistent, and the format survives tool and vendor changes.

## Options

### Option A · Markdown only
Summary: hand-written Markdown with conventions enforced by review.
Consequences: zero tooling to maintain and nothing to install. Consistency depends entirely on reviewers; with two thousand tasks, dangling references, cycles, duplicated IDs and stale roll-ups accumulate within weeks. Progress views must be hand-maintained and rot immediately.
Evidence: none.

### Option B · Markdown plus a Python validator
Summary: the same Markdown source with a Python script for validation and generation.
Consequences: fast to write and familiar. Requires a Python toolchain in every contributor environment and in CI, conflicts with the project's Rust-first language strategy (§50), and the interpreter dependency drifts independently of the repository. Zero-install is not achievable.
Evidence: none.

### Option C · Markdown plus a Rust tool
Summary: Markdown remains the source of truth; a small Rust crate (`tools/roadmap`) validates grammar, IDs, references, cycles, milestone monotonicity, done invariants, freeze discipline and gate rules, and generates every roll-up deterministically.
Consequences: one toolchain for the whole project; a single static binary usable in pre-commit and CI; the grammar is enforced rather than reviewed; generated views are always current. The tool is load-bearing and needs fixture tests, idempotence and determinism property tests, and a GOV owner.
Evidence: none.

### Option D · Issue tracker
Summary: tasks as issues in a hosted tracker with labels for milestones and workstreams.
Consequences: familiar UI and built-in state transitions. The dependency graph, freeze discipline, definition-of-done rules and evidence grammar cannot be enforced in a tracker; history is not reviewable as a diff; content is hostage to a vendor; AI agents need API access instead of files; the roadmap cannot be read offline or cited by stable path.
Evidence: none.

## Decision

Option C. The roadmap is a git repository of Markdown files with a strict grammar defined in `tools/schema/fields.json` and `CONVENTIONS.md`, validated and rendered by the Rust binary `roadmap`. Workstream files are the source of truth for tasks; milestone files are the source of truth for gates; decisions, registers and reports are separate typed files; every roll-up is generated and never hand-edited. IDs are permanent. Status is stored in four values and everything else is derived. Done requires evidence and, for surface freezes and decisions from the start and for everything from V1, an independent verifier.

## Consequences

- `tools/roadmap` exists, is tested with a fixture per violation class, and is owned by the GOV workstream.
- Every contributor runs `roadmap fmt && roadmap gen && roadmap check` before committing; the pre-commit hook does this.
- Pull requests are the only path to `main`; metadata-only changes merge on green checks without review.
- Rules that cannot yet be enforced mechanically (diff-aware transition checks, mutation commands) are review duties until the corresponding GOV tasks ship.
- Changes to the grammar, the workstream list or `BASELINE.md` are themselves GOV decisions.

## Rejected options and why

Option A was rejected because review cannot keep two thousand cross-referenced blocks consistent, and hand-maintained progress is exactly the inherited semantics §67 tells the project to replace.

Option B was rejected because it introduces a second toolchain that the project's language strategy does not otherwise need, and because two judges of the design pass found the Rust-first rule decisive once zero-install is recovered with a prebuilt binary.

Option D was rejected because the rules that make this roadmap trustworthy (spike before freeze, evidence before done, benchmark before number, dependency monotonicity) are not expressible in a tracker, and because the roadmap must remain a citable, diffable, offline artifact.

## Follow-ups

none
