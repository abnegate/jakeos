# JakeOS Roadmap Repository — Final Structure

**Basis.** The three judges split (maintainer → P0, contributor → P1, steering → P2). P0 has the highest aggregate (86 vs 82 vs 75) and won the lens that governs a document meant to stay truthful for years, so P0 is the base. Grafts adopted below are those two or more judges endorsed, or one judge endorsed with no judge objecting. Where judges disagree, the resolution names who prevailed and why. Every judge-rejected idea is excluded; the notable ones are listed in section 10 so they are not re-proposed.

---

## 1. Directory layout

```
roadmap/
├── README.md                    Entry point: what this repo is, 60-second reading guide, links to CONVENTIONS, AGENTS, ROADMAP. Hand-written, short.
├── AGENTS.md                    Instructions for AI agents: load order, hard rules, exact commands. (Graft P1; J1.)
├── ROADMAP.md                   GENERATED whole file. Milestone ladder, gate status, progress (count + weighted + gate), ready head, critical-path head, decision leverage.
├── STATUS.md                    GENERATED whole file. Per-workstream table, blocked-by aggregation, steering signals, anomalies, unanchored tasks.
├── BASELINE.md                  The baseline architecture document, §-numbered. Only citable target for `Baseline:`. Section numbers immutable; changes via GOV adr task.
├── CONVENTIONS.md               Normative rules: grammar (EBNF), enums, ID allocation, transitions, evidence grammar, commit grammar. The validator is its implementation.
├── GLOSSARY.md                  Canonical terms and casing with §refs. Validator warns on non-canonical casing in titles.
├── roadmap.toml                 Tool config: size weights, policy flags (require_independent_verification), generated-file list, repo aliases live in registers/repos.md not here.
├── workstreams/                 SOURCE OF TRUTH for tasks. One file per prefix, file name = prefix, tasks in ascending ID order, no milestone sub-headings.
│   ├── KRN.md … GOV.md          36 files. Header block (Prefix, Lead, Baseline, optional Baseline gap), ## Scope, ## Out of scope, ## Tasks.
├── milestones/                  SOURCE OF TRUTH for gates, demos, hardware scope, surfaces to freeze, risks to retire. Task lists are generated marker blocks.
│   ├── V0.md  V0.5.md  V1.md  V2.md  V3.md  V4.md  V5.md   Sequence 1–7. V5 display title "1.0 Public Stable".
│   └── LATER.md                 Sequence 99. Deferred-beyond-1.0 parking rung. No gates. Excluded from 1.0 progress, counted in totals.
├── decisions/                   One ADR per file, immutable once accepted (superseded by a new file).
│   ├── README.md                Hand-written intro + GENERATED index block (D-ID, title, status, task, surfaces, supersession chain).
│   ├── TEMPLATE.md              Required header fields and sections.
│   └── D-0001-<slug>.md …       D-0001 is this repository's own process.
├── registers/                   Small typed registers; entries have IDs and are referenced from tasks by ID.
│   ├── risks.md                 R-NNN: statement, Likelihood, Impact, Status, Mitigated by (task IDs), Retire by (milestone token).
│   ├── benchmarks.md            B-NNN: metric definition, method, harness alias, comparison baselines, per-milestone targets per H-ID. Definitions and targets only; no measured values.
│   ├── questions.md             Q-NNN: open questions usable as dependency targets; Status open|answered|withdrawn; Answered by (task ID or D-ID).
│   ├── hardware.md              H-NNN: reference machines and QEMU profiles.
│   ├── surfaces.md              S-NNN: ABI/interface surfaces with Layer (L1–L4 per §66), Name, State open|prototyped|frozen|superseded, Explored by, Decided by, Frozen by. (Graft P2; all three judges.)
│   └── repos.md                 Alias → URL for code repositories. Evidence lines use aliases.
├── reports/                     Committed evidence that survives link rot. Hand-written per fixed skeletons; validated for existence and header fields.
│   ├── spikes/<TASK-ID>.md      Spike write-up: Question, Built, Measured, Rules out, Recommends. Required for a spike task to be done.
│   └── benchmarks/<B-NNN>/<alias>@<sha>-<H-NNN>.md   One measured result per benchmark per commit per machine: method, raw numbers, comparison baseline. Source of the generated latest-results table.
├── generated/                   GENERATED whole directory. Marked linguist-generated and -diff in .gitattributes.
│   ├── by-milestone/V0.md …     Full task tables per milestone grouped by workstream, with derived state.
│   ├── by-workstream/KRN.md …   Compact tables per workstream, sub-grouped by milestone (the "what does V0 need from IPC" view).
│   ├── ready.md                 Todo tasks with all dependencies satisfied, ranked by downstream weight.
│   ├── blocked.md               Derived blocked tasks with exact blocker IDs; aggregated by blocker.
│   ├── critical-path.md         Longest weighted chains to each gate, per-task slack, bottleneck ranking.
│   ├── benchmarks.md            Latest result per B-ID per H-ID vs active-milestone target, met/unmet.
│   ├── graph.dot                Graphviz source.
│   └── index.json               Complete machine-readable export of every entity with derived fields.
├── tools/
│   ├── roadmap/                 Rust crate, std-only, single binary `roadmap`. (Sided with J2/J3 over J1's Python; see section 8.)
│   │   ├── Cargo.toml
│   │   └── src/                 parser, model, validate, transitions (diff-aware), generate, graph, ids, fmt, cli
│   ├── schema/grammar.md        Normative EBNF for every block type.
│   ├── schema/fields.json       Allowed keys, canonical order, enums, per-Type and per-Status requirements. Single source for validator and formatter.
│   ├── templates/               task, workstream, milestone, gate, decision, register entries, spike report, benchmark report.
│   └── tests/                   Fixture repos per violation class; golden generated outputs; fmt idempotence and gen determinism property tests.
├── .github/workflows/roadmap.yml   `fmt --check && check --base origin/main && gen --check`; required for merge; also runs on main; applies `metadata-only` label.
├── .githooks/pre-commit         `roadmap fmt && roadmap gen && roadmap check`.
└── .gitattributes               generated/** ROADMAP.md STATUS.md: linguist-generated, -diff; documented resolution "take either side, regenerate".
```

**Resolutions.**
- *Single file per workstream, ID-ordered* (P0). J1 and J3 both rejected P1's directory-per-milestone; J1 and J2 both rejected P2's `##` milestone sections. J2 alone argued for small per-milestone files. Sided with J1: the file path or section heading would be a second copy of the `Milestone` field, and every re-milestone becomes a block move that breaks history. J2's real need (a small loaded unit, a per-milestone view) is met by `generated/by-workstream/*.md` sub-grouped by milestone, `roadmap show`, `index.json`, and a size warning that triggers a split adr task.
- *No ID ledger file* (P0). J1 and J3 rejected ids.tsv/IDS.lock; the never-delete rule enforced by the diff-aware validator makes the workstream file the ledger.
- *Rust tool* (P1/P2). J2 and J3 both rejected Python; J1 called it defensible but not essential. Two judges plus the project's Rust-first rule win; zero-install is recovered with a prebuilt CI binary and `cargo run -p roadmap --`.
- *reports/ directory* (P2). Grafted by J1 and J2, praised by J3.

---

## 2. Final workstream table

Base is P0's 36. Changes: `SVC` added (J1: §32/§33 needed a single owner); `OPS` merged into `TSK` to stay at 36 (P1's argument that cancellation and completion are one design; no judge objected). BOOT, NET, AUD, TXT carry `Baseline gap:` headers and their first adr task defines scope (graft P1; J1).

| Prefix | Name | Scope | Baseline sections |
|---|---|---|---|
| KRN | Kernel fork and upstream tracking | Fork point, divergence policy phases A–E, upstream rebase cadence, CVE intake handoff to REL, retention of KVM and mature subsystems, removal of Linux semantics from native paths, kernel-side C/Rust boundary policy. Out: drivers (HW/SVC), boot (BOOT), object model (ABI/CAP). | §2, §5, §6, §55, §56.4, §57 |
| BOOT | Boot and firmware | UEFI path, bootloader, generation selection and last-known-good fallback, secure/measured boot, TPM handoff to SEC, early userspace replacing initramfs, boot-time integrity of the selected generation. Baseline gap declared. | §5, §30, §51, §63 |
| ABI | Native kernel ABI | Handle table and Object<T> registry, minimal syscall surface, error model, L1/L2 versioning and negotiation, hardware-escape-hatch review, ABI freeze process, owns L1 entries in surfaces.md. | §7, §8, §12, §65, §66 |
| CAP | Capabilities | Rights encoding, derivation, attenuation, revocation, transfer over channels, namespaces, grant audit, inspectability, CHERI/tagged-memory readiness without ABI change. Out: permission policy (SEC), consent UI (APP). | §7, §8, §9, §51 |
| CMP | Components | Component object, address-space construction from immutable packages, creation/teardown fast paths, lifecycle, Inputs/Outputs binding, component graphs, warm-start latency, task_struct/mm_struct mapping then native. Out: supervision/restart protocol (SVC). | §10, §11, §34 |
| TSK | Tasks, operations and structured concurrency | Task and TaskGroup, multiplexing 10^5 tasks over execution contexts, ownership hierarchy, cancellation propagation, background-execution capability, Operation<Result>, submission/completion queues, deadlines, priority, per-operation accounting and tracing hooks, io_uring lineage evaluation. | §18, §19, §20, §21 |
| IPC | Channels and typed interfaces | Channel<T>, IDL and codegen (wire format, stubs, ownership, tracing metadata), interface versioning and negotiation, small-message fast paths, large-payload handoff via MEM, transport abstraction for future distribution. Owns L2 interface-evolution rules. | §12, §14, §15, §43 |
| MEM | Memory objects and zero-copy dataflow | MemoryObject property set, ownership transfer/borrow/share, CoW, DMA/GPU suitability, NUMA and device locality, NIC→decoder→GPU pipelines, CXL/persistent/disaggregated abstraction review. | §16, §17, §38 |
| SCH | Scheduling intent and resource domains | Intent classes, ResourceDomain budgets and accounting for CPU/memory/GPU/IO/network/storage/energy/latency, cgroups mapping then native, frequency and core selection inputs with PWR. | §22, §23, §53 |
| OBS | Observability and tracing | Structured low-overhead tracing, inspection interfaces for every primitive, dynamic enablement, security-aware trace access, event schema shared with IPC codegen, crash capture format (intake is REL). SDK owns the `os inspect/trace` command surface; OBS owns the data. | §24, §64 |
| SVC | Service lifecycle and user-space driver hosting | Service manifests, supervision tree, restart and rebind protocol, client reconnect and state-restore semantics, degraded-recovery signalling, hosting framework and device-access capabilities for user-space drivers, kernel-residency criteria. Out: specific device classes (HW), compositor internals (GFX). | §32, §33 |
| STO | Storage and user-selected authority | Filesystem selection over a Linux filesystem, CoW/snapshots/checksums/atomic replacement, content-addressed store, File/Directory/Collection/Blob/ApplicationData, UserSelected<T> chooser authority minting capabilities (UI in APP), criteria for a future native object store. | §25, §26, §27 |
| PKG | Packages, dependencies and system generations | Immutable package format and manifest, coexisting dependency versions with dedup, generation creation and boot integration with BOOT, history events and restore semantics, verified-once mapped startup metadata. Out: repository/signing (REL), updater UX (INS). | §28, §29, §30, §31, §34 |
| GFX | Graphics and compositor | Surface/Buffer/RenderQueue/Display/Frame over DRM/KMS without exposing DRM, compositor as restartable privileged service, frame scheduling, HDR, VRR, scaling, multi-GPU, explicit screen-share capability, GPU driver strategy. | §32, §39, §40, §56.1 |
| UIP | Native UI protocol and toolkit | Window objects and UI protocol, declarative UI and retained-mode rendering, layout, animation, input routing and focus, drag/drop and clipboard as capabilities, high-DPI and adaptive form factors, automation and accessibility metadata emission. | §41 |
| TXT | Text, fonts and internationalisation | Font management and fallback, shaping, rasterisation, text editing primitives, input methods, locale data, localisation pipeline, bidi and complex scripts. Baseline gap declared. | §41 |
| ACC | Accessibility | Accessibility tree from UIP semantics, semantic actions for assistive technology, screen reader, magnification, keyboard-only and switch access, contrast and motion preferences, AT bridging for LNX (AT-SPI) and WIN, conformance testing as a gate. | §41, §42, §49 |
| SEM | Semantic interfaces, automation and AI | Typed semantic interface registry, discovery, automation engine (triggers, action graphs), AI assistant capability broker (typed, permissioned, logged, revocable, scoped), permission UX integration with SEC. Sequenced after the object model. | §42, §43, §44, §45 |
| LNX | Linux personality | Syscall ABI retention then translation onto native primitives, POSIX process model, fds, signals, procfs/sysfs, glibc and systemd expectations, D-Bus, Wayland/X11 bridging, PipeWire bridge, portals, OCI containers, desktop integration so Linux apps feel native, LTP and real-application conformance. A product, not a shim. | §3, §36, §46, §47, §49, §56.3 |
| WIN | Windows personality | Wine/Proton atop LNX then native bindings, PE loading, Win32 and selected NT semantics, registry emulation, Windows filesystem semantics, DXVK/VKD3D, gaming (input, audio, HDR, VRR, anti-cheat posture), .exe UX and desktop integration. | §3, §48, §49, §56.2 |
| ENV | Native development environments | environment.yaml schema, `os env enter` composing ResourceDomain + StorageSnapshot + CapabilityNamespace + NetworkNamespace + service components, sub-50 ms warm start, service catalogue, editor/terminal integration. | §35, §36 |
| HET | Heterogeneous compute | ComputeDevice and ComputeQueue, dispatch preferences (latency, throughput, energy, precision, locality), CPU/GPU/NPU enumeration, memory locality with MEM, fossilisation review of x86-64/coherent-memory/discrete-GPU assumptions. | §37, §38 |
| WASM | WebAssembly components | Component Model and WASI mapped onto native interfaces and capabilities, runtime as component host, plugin and sandboxed-extension model, package integration, cross-architecture workloads. Not the native machine ABI. | §13 |
| SEC | Security model and hardening | Threat model, no-ambient-authority enforcement, identity/login/sessions, disk encryption, secrets storage, permission policy and grant lifecycle (prompt UI in APP), kernel hardening, layered memory-safety strategy, security review of ABI and personalities, engineering side of vulnerability response. | §9, §51, §63 |
| NET | Networking | Linux network stack retention, NetworkConnection objects as capabilities, per-component network policy via ResourceDomain, user-space Wi-Fi/DHCP/DNS/VPN/firewall as SVC-hosted services, zero-copy receive with MEM, network namespaces for ENV and LNX. Baseline gap declared. | §5, §7, §9, §17, §23, §43 |
| AUD | Audio | AudioStream objects, low-latency path with LowLatency intent, restartable audio service, routing including Bluetooth audio with HW, PipeWire/PulseAudio compatibility for LNX and WIN. Baseline gap declared. | §22, §32, §33, §47 |
| HW | Hardware enablement | Reference machine selection (registers/hardware.md), input devices, Bluetooth, USB classes, cameras, sensors, printing, firmware loading, hardware compatibility data feeding REL. Out: driver hosting framework (SVC), power (PWR). | §33, §55, §62 |
| PWR | Power management | Suspend/resume, hibernation policy, battery and thermal management, frequency scaling driven by SCH intent, lid/dock/external-display power behaviour, energy measurement methodology with BEN. Laptop viability is a V2 gate. | §22, §54, §61, §62 |
| SDK | Native SDK and developer tools | Native runtime, `#[component]` and async model, language bindings (Rust first, then C, C++, Swift, Kotlin, C#, TypeScript/Wasm), API docs generation from IDL, the `os` CLI framework and inspect/trace/history/restore/env subcommand surfaces (data owned by OBS/PKG/ENV), debugger and profiler integration. | §50, §52, §64, §66 |
| APP | Native applications and shell | Launcher, panel, notifications, settings, session/lock UI, permission and consent UI (policy in SEC), the file chooser UI (authority in STO), terminal, file browser, editor, image viewer, store client, browser/IDE strategy, dogfooding readiness. | §9, §25, §49, §60, §61, §62, §63 |
| INS | Installer, updater and recovery | Install media, partitioning and encryption setup with SEC, updater creating generations via PKG, rollback/restore UX, recovery environment, first-boot experience, crash reporting client and consent. | §30, §31, §63 |
| BLD | Build, toolchain and CI | Toolchain pinning, Rust-in-kernel build integration, reproducible builds, CI pipelines, QEMU and physical test matrix over registers/hardware.md, fuzzing, static analysis, conformance-suite plumbing, artifact caching. | §50, §51, §54, §55 |
| BEN | Benchmarks and performance tracking | Ownership of registers/benchmarks.md and reports/benchmarks/, harnesses, comparison baselines (Linux, Windows, macOS, containers, language runtimes), regression tracking, gate measurement runs, energy methodology with PWR, anti-fake-claim policy. | §10, §34, §53, §54, §59 |
| REL | Release engineering and security response | Signing infrastructure, package repository, update channels, release checklists, CVE tracking and advisory operations, crash/telemetry intake (capture is OBS), hardware compatibility database publication, generation naming/versioning. | §27, §56.4, §63 |
| DOC | Documentation | Baseline and architecture doc maintenance, SDK/API docs, personality compatibility guides, user documentation, docs site and search, glossary stewardship. A V3 gate with its own large task set. | §56.5, §63 |
| GOV | Governance, legal and process | Licensing and legal (GPL boundary, Wine, fonts, codecs, trademarks), community and contribution policy, RFC/ADR process, research programme tracking §58 inspirations, this repository's tooling and conventions. | §1, §57, §58, §67, §68 |

---

## 3. Exact task template

Grammar (normative; `roadmap fmt` rewrites any task into exactly this shape):

```markdown
### <PREFIX>-<NNN> · <Title>
- Type: build | adr | spike | benchmark | docs
- Milestone: V0 | V0.5 | V1 | V2 | V3 | V4 | V5 | LATER
- Status: todo | in-progress | done | dropped
- Size: S | M | L | XL
- Owner: @<handle> | @agent/<name> | none
- Depends on: <task IDs and/or Q-IDs, comma-separated> | none
- Baseline: <§N or §N.M refs, comma-separated> | none
- Benchmarks: <B-IDs>                 (required when Type is benchmark; optional otherwise; omitted when none)
- Decision: <D-ID>                    (required when Type is adr; forbidden otherwise)
- Explores: <S-IDs>                   (spike only; optional)
- Freezes: <S-IDs>                    (optional; any Type except spike)
- Risks: <R-IDs>                      (optional; register entries this task mitigates)
- Verified by: @<handle>              (required when Status is done and policy is on; forbidden otherwise)
- Superseded by: <task IDs>           (dropped only; omitted when none)
- Dropped because: <one line>         (required when dropped; forbidden otherwise)

<Description: free Markdown prose. States what is built and why in one or two paragraphs. No headings.>

#### Out of scope
<Optional section. Names adjacent work and the task or prefix that owns it.>

#### Acceptance criteria
- [ ] <observable, testable statement; at least one; no "should">

#### Verification
- Unit: <test path / harness / CI matrix entry>
- Integration: <…>
- Fuzz: <harness, duration>
- Bench: <B-ID> <target restated, H-IDs>          (required when Type is benchmark)
- Manual: <procedure>
- Review: <who reviews>                            (typical for adr/docs)
- Demo: <what is shown, on which H-ID>
- Report: <what the spike report must answer>      (required when Type is spike)

#### Evidence
- none                                             (only while Status is not done)
- <alias>@<sha>
- <alias>#<pr-number>
- https://<url>
- report:reports/benchmarks/<B-NNN>/<alias>@<sha>-<H-NNN>.md
- report:reports/spikes/<TASK-ID>.md
- decision:<D-ID>
```

Parsing rules: a task starts at `^### ([A-Z]{2,4})-(\d{3,}|@[a-z0-9-]+) · (.+)$` and ends at the next level-2/level-3 heading or EOF. Field lines are consecutive `- Key: value` lines immediately after the heading; keys come from `fields.json`; unknown keys are errors; the formatter enforces canonical order and drops conditional fields whose value is `none`. `none` is the only null token. `#### Acceptance criteria`, `#### Verification` and `#### Evidence` are mandatory and exact; `#### Out of scope` is optional. Verification lines must begin with one of the eight kinds. Checkbox syntax is exactly `- [ ]` / `- [x]`. No calendar dates anywhere in the block. Titles are imperative and use GLOSSARY casing (warnings). Target length 20–60 lines; warning above 100.

Size semantics: S = one PR, one subsystem; M = a few PRs, one subsystem; L = multi-PR or crosses subsystems; XL = placeholder that must be split before it may leave `todo`.

**Filled example:**

```markdown
### CAP-014 · Implement attenuating derivation of capability rights
- Type: build
- Milestone: V0
- Status: in-progress
- Size: M
- Owner: @jakebarnby
- Depends on: CAP-003, CAP-011, ABI-007, CAP-009
- Baseline: §7, §8, §9.1
- Benchmarks: B-004
- Freezes: S-003
- Risks: R-012

Implement `derive(cap, mask)` so a holder of `Capability<T, Rights>` obtains a new
capability whose rights are a strict subset of its own. The rights word is encoded so a
future hardware-tag enforcement path can check the subset relation without kernel metadata
(§8); the encoding is surface S-003, explored by spike CAP-009 and decided by ABI-007.
Derivation is O(1) and allocates nothing on the fast path.

#### Out of scope
Revocation walk semantics (CAP-011). Capability transfer over channels (IPC-021).

#### Acceptance criteria
- [x] Deriving with a mask that is not a subset returns `Error::Rights` and allocates no handle.
- [x] A derived capability records its parent for the revocation walk defined by CAP-011.
- [ ] S-003 state is `frozen` in registers/surfaces.md and names this task.
- [ ] No `unsafe` outside `cap/rights.rs`.

#### Verification
- Unit: `kernel:tests/cap/derive_*` in CI matrix `qemu-x86_64` and `hw-h002`.
- Fuzz: `kernel:fuzz/cap_derive` one hour nightly without panic.
- Bench: B-004 derive latency p99 ≤ 200 ns on H-002.
- Review: ABI lead sign-off recorded in the PR.

#### Evidence
- kernel@3f9c1ab
- kernel#212
```

**Resolutions.** Separate `#### Evidence` section (J2 rejected P0's folding; J3 praised P2's split; J1 silent). Normative Verification kinds (J2 graft). No mandatory `Why` (J1 and J3 rejected it); `#### Out of scope` kept optional because J2 valued it and it costs one heading. No per-task `Risk:` enum (J1 rejected; J2 grafted) — R-IDs link to register entries whose Likelihood/Impact drive risk weighting instead. No `Moved from`/`Split from`/`Reopened`/`Tags`/`Blocked by` fields (J1 and J3 rejected; see sections 5 and 8). No task-side `Gates:` field: J2 wanted it, J3 praised gate-ness derived from the milestone side, J1 accepted P0's `Verified by` in the gate block; two to one for the milestone side, and `roadmap show` prints the gate criteria a task serves.

---

## 4. ID scheme and allocation rules for parallel authors

- **Task IDs**: `PREFIX-NNN`. PREFIX is a workstream prefix from section 2 (2–4 uppercase letters, equals the file name). NNN is a decimal integer, zero-padded to at least three digits, growing to four naturally; sorted numerically, never lexically.
- **Allocation**: monotonic per prefix; `next = max(numbers present in the file, including dropped) + 1`. The workstream file is the ledger; there is no counter or ledger file (J1 and J3 rejected ids.tsv/IDS.lock).
- **Never reused, never deleted, never moved**: the diff-aware validator (`check --base origin/main`) errors if an ID present on the base branch is absent, renumbered, or in a different file. Tasks are retired only by `Status: dropped`.
- **Splitting**: the original is dropped with `Superseded by:` pointing at fresh IDs; dependents are repointed in the same PR (`roadmap split ID --into N` does this). Parents never keep their ID with narrowed scope (all judges rejected P2's variant). No suffixes.
- **Parallel authors**: each branch allocates `max+1` locally via `roadmap new`. Two branches allocating the same number conflict at the file tail and, independently, `check` on main reports the duplicate. The later PR runs `roadmap renumber OLD NEW`, which rewrites the ID and every reference within the branch. Renumbering is permitted only for IDs that have never reached main.
- **Draft IDs** (graft P1; J1): on branches, `PREFIX-@slug` (for example `IPC-@ring-fast-path`) is accepted wherever a task ID is accepted, including `Depends on` across new tasks in the same PR. `roadmap assign-ids` converts drafts to real numbers at merge time; the validator rejects any draft ID on main. J3 rejected two ID grammars in one field; sided with J1 because the draft shape is disjoint by regex (`@` never appears in a real ID), it only lives on branches, and J2 rejected J3's alternative (`reserve` blocks that inflate dropped counts).
- **Prefix is allocation origin, not current ownership**: if a workstream is split or renamed via GOV adr task, existing tasks keep prefix and file; only new tasks get the new prefix; a `See also:` header line records the relation.
- **Other families** (all monotonic, never reused, disjoint by regex): decisions `D-NNNN` (file `D-0007-<slug>.md`, slug may change, number may not); risks `R-NNN`; benchmarks `B-NNN`; questions `Q-NNN`; hardware `H-NNN`; surfaces `S-NNN`; gates `<MILESTONE>-G<NN>`; demos `<MILESTONE>-D<NN>`. Workstream prefixes never contain digits; milestone tokens are `V<digit>[.<digit>]` or `LATER`. Spike and benchmark reports are files named by the task ID or B-ID and need no separate ID family.
- **Reserved prefix `EX`** is ignored by referential-integrity checks so CONVENTIONS.md and AGENTS.md can show worked examples.

---

## 5. Status model and definition of done

**Stored states** (the only values of `Status`): `todo`, `in-progress`, `done`, `dropped`.

**Derived display states** (computed, never typed):
- `ready`: todo and every `Depends on` entry is resolved (task done; dropped task with a done superseder; Q answered).
- `blocked`: todo or in-progress with at least one unresolved dependency; generated views list the exact blocker IDs and aggregate by blocker ("7 tasks blocked on ABI-003").
- `in-review`: in-progress with at least one Evidence line other than `none` and Status not yet done.
- `orphaned` (error, not a state): depends on a dropped task with no superseder.
- `stale`: in-progress with no change to its block across N commits; local-only via `roadmap stale` (git-derived, never committed).

**Resolutions.** Stored `blocked` rejected (J1 rejected; J3 asked for derived; J2 wanted a narrowed stored flag) — two to one for derived. J2's concern (ceremony to record "hardware not available") is met by `roadmap block ID "reason"`, which creates the Q entry and adds it to `Depends on` in one command; Q entries have owners and a status, unlike free text. Stored `review` rejected (J1 rejected; J2 and J3 grafted) — sided with J1 because the state is fully derivable from evidence presence and a stored flag rots when the PR merges; the derived `in-review` gives J2 and J3 the same visibility.

**Transitions** (mechanical preconditions enforced by the diff-aware validator):
- `todo → in-progress`: Owner ≠ none; Size ≠ XL.
- `in-progress → todo` (unclaim): always allowed; Owner may reset to none.
- `todo | in-progress → done`: definition of done below. `todo → done` in one change is permitted only if it satisfies every done rule including `Verified by` (J2 and J3 rejected an S-task shortcut).
- `done → in-progress` (reopen): only by unticking at least one acceptance box in the same change; Evidence lines stay.
- `todo | in-progress → dropped`: `Dropped because:` required; if any non-dropped task depends on it, `Superseded by:` with existing IDs and dependents repointed in the same PR.
- `done → dropped`: forbidden. Done work is history; add a new task.
- `dropped → todo` (revive): remove the drop fields.

**Definition of done** (all are errors if unmet):
1. Every acceptance checkbox is `[x]`.
2. Verification has at least one line.
3. Evidence has at least one line of valid grammar (section 3); every dependency is resolved.
4. `Verified by: @handle` present when `require_independent_verification = true` (default off for V0, on from V1); must differ from Owner; may never be an `@agent/` identity. The verifier reruns the Verification section. (Graft P1; all three judges.)
5. Type `adr`: the linked decision file is `accepted` or `rejected`.
6. Type `spike`: `reports/spikes/<TASK-ID>.md` exists with the required skeleton headings and Evidence references it.
7. Type `benchmark`: for each listed B-ID, a `reports/benchmarks/<B>/…` file exists for every H-ID in the task's milestone hardware scope and meets the register target for that milestone, or the Description states why the target is deferred (warning).
8. Type `docs`: at least one Review or URL evidence line.
9. `Freezes:` present: each S-ID's register state is `frozen` and names this task; the dependency closure contains a done spike whose `Explores` names the surface and a done adr task whose decision lists the surface; for L1 surfaces the decision cites a benchmark report.

**Frozen once done**: the validator rejects changes to a done task's acceptance criteria, Milestone, Type or Size; Description clarifications and additional Evidence lines remain allowed (all judges grafted or accepted this).

**Other invariants**: all boxes ticked with Status ≠ done is a warning; any box unticked with Status = done is an error. Dropped tasks stay in place, are hidden by default in views under a collapsed table, and are excluded from progress denominators. Dependency milestone monotonicity is an error (dependency rank ≤ dependent rank; LATER = ∞).

---

## 6. Milestone files

Each `milestones/<TOKEN>.md` is the single source for what a milestone means and how it is judged. It never lists tasks by hand; it never carries a hand-set status.

```markdown
# V0 — Execution model proof
- Sequence: 1
- Title: Execution model proof
- Baseline: §59
- Hardware scope: H-001, H-002
- Surfaces to freeze: S-001, S-003
- Risks to retire: R-004, R-012

## Purpose
Prose: what this milestone proves and explicitly what it is not.

## Not in this milestone
Explicit non-goals.

## Gates
### V0-G01 · Fork boots on all hardware in scope
- Kind: functional | benchmark | demo | process
- Verified by: KRN-003, BOOT-004, BLD-010
- Benchmark: B-003                     (Kind benchmark only; target lives in the register)
One or two lines stating the criterion in human terms.

### V0-G07 · Exit review recorded
- Kind: process
- Verified by: GOV-021
Human attestation modelled as a docs task with Review evidence and an independent Verified by.

## Demos
### V0-D01 · Component A → Channel<Request> → Component B → MemoryObject transfer → result
- Verified by: CMP-020, IPC-014, MEM-011, OBS-004
Prose describing what is shown.

## Notes
Free prose; may cite R-IDs and Q-IDs; may not introduce task IDs as requirements.

<!-- roadmap:generated:begin milestone -->
Generated: derived status; gate table (satisfied / n of m); demo status; surface states;
risk states; benchmark gate table (latest result per H-ID vs target); progress by
workstream (count and weighted); ready and blocked counts; unanchored tasks; critical
path to the last unsatisfied gate; link to generated/by-milestone/V0.md.
<!-- roadmap:generated:end -->
```

**Referencing without duplication.** A task belongs to a milestone solely through its own `Milestone` field. The milestone file names gates and the task IDs that verify them; the generator renders live titles and statuses. Adding a verifying task is a one-line edit to the gate's `Verified by`; moving a task between milestones is a one-line field edit.

**Derived milestone status** (J1 rejected hand-set `Status:` and hand-ticked gate checkboxes; J3's requirements are met mechanically): `complete` when every gate is satisfied, every listed surface is `frozen` or deferred by an accepted decision naming it, every listed risk is `mitigated|accepted|closed`, every benchmark gate has a report at target on every in-scope H-ID, and no task with this Milestone is todo or in-progress. `active` = lowest Sequence not complete; `next` = active + 1; `planned` otherwise. A gate is satisfied when all `Verified by` tasks are done (dropped tasks require a done superseder). `roadmap gate V0` enumerates every failing reason.

**Validator rules**: every milestone except LATER has ≥ 1 gate; every `Verified by` ID exists with Milestone rank ≤ the gate's rank; benchmark gates cite a B-ID with a target for this milestone; hardware, surface and risk IDs exist; a task whose Milestone is this token but not reachable from any gate (directly or transitively) is reported as *unanchored* in STATUS.md; changing `Verified by` or `Sequence` on a complete milestone is rejected unless the commit carries `Roadmap-Decision: D-NNNN`.

---

## 7. Decision records

A decision is made by a task and recorded in a file; the two are coupled mechanically.

**The task**: Type `adr`, lives in the workstream that owns the question, carries `Decision: D-NNNN`; acceptance criteria name the options to evaluate and the review required; Verification has Review lines; when done, Evidence contains `decision:D-NNNN`.

**The file** `decisions/D-NNNN-<slug>.md` (from TEMPLATE.md):

```markdown
# D-0007 · Rights encoding for Capability<T>
- Status: proposed | accepted | rejected | superseded
- Task: ABI-007
- Surfaces: S-003
- Layer: L1
- Spikes: CAP-009
- Supersedes: none
- Superseded by: none
- Baseline: §7, §8, §66
- Revisit when: <condition, never a date>

## Context
## Options            (at least two; each with a one-line summary, Consequences, and Evidence: spike/benchmark report paths)
## Decision
## Consequences
## Rejected options and why
## Follow-ups         (task IDs or Q-IDs created or changed because of this decision)
```

**Coupling rules** (validator): exactly one adr task per decision file and vice versa; task todo/in-progress ⇔ decision `proposed`; task done ⇔ decision `accepted` or `rejected` (a rejection is a completed decision); task dropped ⇔ file absent or `superseded` before acceptance with reason; a decision cannot be accepted while its task has unticked boxes; Follow-ups IDs exist; ≥ 2 options (graft P2; J1, J2); Supersedes chains acyclic; a decision listing L1/L2 surfaces cites ≥ 1 done spike whose `Explores` names each surface; L1 additionally cites a benchmark report.

**Immutability**: accepted decisions change only in `Superseded by`. Revising means a new D-ID with `Supersedes:` and a new adr task; the old file becomes `superseded`; the old task stays done. When a superseding decision touches a frozen surface, the tool flags every task with `Freezes:` on it and the surface returns to `prototyped` (graft P2 `impact`).

**Unblocking**: tasks that need a decision list the **adr task ID** in `Depends on`, never the D-ID, so the graph has one edge type (J1 and J3 rejected P1's dual targets; J2 dissented). J2's concern — landing on a task instead of rationale — is met by `roadmap show`, which inlines the Decision and Consequences sections of every adr task in the closure. Dependents become ready on the next regeneration with no further edits. Milestone monotonicity forces decisions to be scheduled no later than the work they gate. A generated *decision leverage* table ranks proposed adr tasks by the number and weight of tasks they transitively unblock (graft P1; J1, J3).

**Roadmap-process decisions** (grammar changes, workstream splits, baseline amendments) are ordinary GOV adr tasks; a BASELINE.md diff requires `Roadmap-Decision: D-NNNN` in the commit trailer.

---

## 8. Rollup and tooling

**Tool**: single binary `roadmap` (Rust, std-only, `tools/roadmap`, `cargo run -p roadmap --` or a prebuilt CI artifact). No network; no database; reads only this repository. Deterministic: output depends solely on repository contents — no git metadata, timestamps or environment inside any committed output (J1 rejected P1's `<!-- source: sha -->` because pre-commit and CI generate at different SHAs; all judges rejected P2's committed snapshots.csv).

**Inputs**: `workstreams/*.md`, `milestones/*.md`, `decisions/*.md`, `registers/*.md`, `reports/**`, `BASELINE.md`, `GLOSSARY.md`, `roadmap.toml`, `tools/schema/fields.json`; for `--base`, the same files at the base ref.

**Outputs**:
- Whole generated files, each starting with a static header comment naming the generator and source directories: `ROADMAP.md`, `STATUS.md`, `generated/**`.
- Marker blocks, replaced verbatim between `<!-- roadmap:generated:begin <name> -->` and `<!-- roadmap:generated:end -->`: inside `milestones/*.md` (`milestone`), `decisions/README.md` (`index`), `registers/risks.md` (`status`), `registers/benchmarks.md` (`results`), `registers/questions.md` (`status`), `registers/surfaces.md` (`status`), and each workstream file header (`summary`).
- Diagnostics: human text by default, `--json` for agents, each with `file:line` and a fix hint.

**ROADMAP.md shows**: ladder with derived status and gate n/m; per-milestone progress as three numbers side by side — count %, size-weighted % (S=1, M=3, L=8, XL=20), gate % — plus ready, blocked, in-review, dropped counts; workstream × milestone grid; top ten critical chains to the active milestone with slack; ready head ranked by downstream weight; decision leverage; blocked-by aggregation; steering signals. Percent is never shown alone; the header states the formulas and labels weights as estimates.

**STATUS.md shows**: per-workstream table; unowned in-progress tasks; all-boxes-ticked-not-done; unanchored tasks; risk mitigation progress; open questions blocking tasks; surfaces frozen without prototype (should be impossible, reported as error); steering signals.

**Commands**: `check [--base REF] [--strict] [--json]`, `fmt [--check]`, `gen [--check]`, `new task PREFIX "Title" --milestone --size [--type] [--depends]`, `new decision|risk|question|benchmark|hardware|surface`, `assign-ids`, `renumber OLD NEW`, `claim ID @owner`, `unclaim ID`, `block ID "reason"` (creates Q and dependency), `done ID --evidence … [--verified-by @h]`, `drop ID --because … [--superseded-by IDs]`, `split ID --into N`, `move ID --milestone TOKEN`, `show ID` (block, resolved deps and dependents, derived state, Decision/Consequences of adr tasks in closure, gate criteria served, B definitions and active target, GLOSSARY entries for capitalised terms, repo aliases), `ready [--workstream] [--milestone] [--size]`, `blocked [--by ID]`, `critical-path [MILESTONE] [--risk-weighted]`, `gate MILESTONE`, `impact ID`, `graph [--dot|--json]`, `export --json`; local git-based, never committed: `stale`, `slipped` (Milestone-field changes per rung), `history ID`, `progress [--rebuild]` (burndown series to stdout or a local file).

**Validation errors**: unknown/duplicate keys; enum values; ID syntax; unique IDs; prefix matches file; ascending numeric order; referential integrity of every ID token (tasks, D, R, B, Q, H, S, gates, demos) except `EX`; § refs resolve to BASELINE.md headings; `Baseline: none` only in workstreams with a `Baseline gap:` header; acyclic dependencies (Tarjan SCC, cycle printed); no self-dependency; milestone monotonicity; dependency on dropped task without superseder; done invariants (section 5); in-progress requires Owner and Size ≠ XL; Type conditionals; freeze discipline; dropped requires `Dropped because`; gate `Verified by` rank rule; ≥ 1 gate per non-LATER milestone; benchmark gate has register target; register entry field validity; decision coupling and ≥ 2 options; report files have required headings; no calendar dates in workstreams/, milestones/, decisions/; draft IDs on main; generated outputs stale.

**Diff-aware transition errors** (`--base`): ID removed, renumbered or moved between files; done task's acceptance criteria/Milestone/Type/Size changed; done → dropped; done → in-progress without an unticked box; illegal transitions; gate `Verified by` or `Sequence` changed on a complete milestone without `Roadmap-Decision:` trailer; BASELINE.md changed without the trailer.

**Warnings** (errors under `--strict`): all boxes ticked but not done; unanchored tasks; hand-typed percentages or status tables outside generated blocks; XL without a split plan in Description; B-IDs referenced by no task; non-canonical glossary casing; "should" in acceptance criteria; non-imperative titles; task over 100 lines; workstream file over the configured size (suggests a split adr task).

**Steering signals** (reported in STATUS.md and `--json`; never promoted to errors, per J2's objection to P2's transition-time WIP rule): XL task with Milestone in active or next rung; critical-path task with Owner none while its milestone is active; in-progress task more than one rung ahead of active; gate task linked to a critical-impact R-ID with no spike in its closure.

**CI** (`roadmap.yml`): `fmt --check`, `check --base origin/main`, `gen --check`; required status check; also runs on main to catch duplicates created by two clean merges; applies the `metadata-only` label when the diff touches only Status/Owner/Verified by/Evidence lines, checkbox states and generated output.

**Tests**: fixture repository per violation class; golden generated outputs; property tests that `fmt` is idempotent and `gen` is deterministic across two clones.

---

## 9. Contributor conventions

- **Picking work**: `roadmap ready --json` (or `generated/ready.md`), then `roadmap show ID`. If the task is unclear, do not start; open a PR that improves the block.
- **Claiming**: `roadmap claim ID @handle`; agents use `@agent/<name>`. One owner per task; co-work is expressed by splitting.
- **Commit grammar**: `roadmap(<ID or PREFIX>): <verb>` with verbs `add`, `claim`, `unclaim`, `edit`, `block`, `done`, `drop`, `split`, `move`, `reopen`; `decision(D-NNNN): accept|reject|supersede`; `milestone(V0): edit`; `register(R-012): retire`. This makes `git log --grep 'CAP-014'` the task history and feeds `roadmap history` and `roadmap slipped`.
- **PR policy**: everything goes through a PR; nothing is pushed to main (J1 and J3 rejected P2's direct pushes). `metadata-only` PRs merge on green CI without human review. PRs touching acceptance criteria, `Depends on`, Milestone, Description or gates need approval from the owning workstream's Lead.
- **Updating status**: only via the transitions in section 5, preferably via tool commands; never edit generated output; the pre-commit hook runs fmt, gen, check.
- **Adding tasks**: `roadmap new task …` allocates the ID and inserts a stub; fill Description, ≥ 1 acceptance criterion and ≥ 1 Verification line before pushing; cite Baseline §refs or write `none` deliberately; declare `Depends on` generously and prune later; tasks are added in the workstream that owns the deliverable; cross-cutting themes are expressed via gates, registers and `impact`, not tags.
- **Splitting**: `roadmap split ID --into N`; the original is dropped with `Superseded by`; dependents are repointed in the same PR; never suffixes, never reuse.
- **Retiring**: `roadmap drop ID --because "…"`; never delete a block; dropped tasks are edited only to revive.
- **Moving between milestones**: edit the Milestone field only; commit message states why; `roadmap slipped` surfaces it at steering reviews.
- **Re-scoping**: free for todo/in-progress; for done tasks, add a new task.
- **Blockers**: never write "blocked" in prose; add the blocking task or `roadmap block ID "reason"` to mint a Q entry.
- **Evidence**: only links into registered repos, report files, decisions, or review URLs; prose assertions are not evidence; Verification is the plan written before work starts, Evidence is the result appended as work lands. Benchmark results are committed as report files; the register holds definitions and targets only.
- **Verification**: from V1 onward `Verified by` is required and must differ from Owner; agents never verify their own or any task; the verifier reruns the Verification section rather than reading the PR.
- **Decisions**: open the adr task first, then `roadmap new decision --task ID`; discuss in the PR; accept by ticking criteria, setting the file status and marking the task done in one PR. Never encode a decision in a Description; put it in the decision file.
- **Surfaces**: any task that fixes an ABI or interface shape lists it in `Freezes`; a spike must precede it via `Explores`; L1 needs a benchmark report cited in the decision.
- **Code repositories**: branch `cap-014-derive-rights`; commit trailer `Roadmap: CAP-014`; PR title starts with the ID; the roadmap side is authoritative for done-ness.
- **Workstream boundaries**: `## Out of scope` names the owning prefix for adjacent work; disputes are resolved by editing scope prose, not duplicating tasks.
- **AGENTS.md rules**: load `roadmap show ID` first; edit only the claimed task's block plus generated output; never allocate IDs by hand; never mark done or set `Verified by`; when Verification cannot be run in the available environment, `roadmap block` with a Q rather than guessing.
- **Writing style**: no calendar dates in any field or block; no "should" in acceptance criteria; imperative titles; baseline citations by § only; GLOSSARY casing.

---

## 10. Risks of this structure and mitigations

| Risk | Mitigation |
|---|---|
| Tooling is load-bearing; if the Rust tool rots, CI blocks everyone. | Std-only crate, small strict grammar, fixture-per-rule tests, idempotence/determinism property tests, prebuilt binary in CI, GOV task owns tool maintenance. |
| Rigid grammar and evidence rules add friction for casual edits. | `metadata-only` fast lane, one-command mutations, `roadmap fmt` repairs formatting, `EX` examples in docs, policy flags relaxed in V0. |
| Large single files per workstream (LNX, WIN, HW) with tail-conflicts on concurrent additions. | Generated per-workstream×milestone views, `roadmap show`, `index.json` for agents; draft IDs and `assign-ids` make conflicts mechanical; size warning triggers a split adr task. |
| Derived blocked state forces a Q entry for external impediments. | `roadmap block` mints the Q and the dependency in one step; Q entries have owners and statuses so they cannot be forgotten as free text would. |
| Generated files committed in every PR add diff noise and merge conflicts. | `.gitattributes` marks them linguist-generated/-diff; documented resolution is "take either side, regenerate"; generation is deterministic so the result is identical. |
| No dates means committed views cannot show staleness, velocity or forecasts. | Local git-derived `stale`, `slipped`, `history`, `progress --rebuild`; burndown may be published outside the repo. Deliberate: dates in files rot. |
| Size weights and critical path are heuristics printed with precision and may be over-trusted. | Always shown beside count and gate progress; header labels them estimates; risk weighting comes from register entries with owners, not per-task guesses. |
| Prefix permanence makes workstream reorganisation awkward. | `See also:` header, GOV adr task, new tasks take the new prefix; history stays greppable under the old one. |
| Freeze discipline (spike + decision + L1 benchmark) lengthens the V0 critical path. | Accepted knowingly per §65; `impact` and decision-leverage views make the serial steps visible so they are scheduled early. |
| Register sprawl (risks, benchmarks, questions, hardware, surfaces, repos). | Every entry is referentially validated; unreferenced B-IDs and open Q with no bound task are warnings; content staleness remains a review duty. |
| Mechanical checks guarantee consistency, not correctness — a vacuous criterion or wrong dependency passes. | Independent `Verified by`, Lead approval for semantic edits, lint-prose warnings, unanchored-task detection. |
| Two representations of a decision (task + file) can diverge. | Bidirectional coupling checks on every PR; single dependency edge type keeps the graph simple. |
| Judge-rejected ideas will be re-proposed. | Recorded as rejected: per-milestone directories or `##` sections; ID ledger files; stored `blocked`/`review`; `Blocked by` free text; per-task `Risk`; `Moved from`/`Split from`/`Reopened`; `Tags`; hand-set milestone status and gate checkboxes; `1.0`/`post-1.0` tokens; bold field syntax and em-dash null; git-SHA provenance in generated output; CI-committed snapshots; parent-keeps-ID split; `todo → done` shortcuts; direct pushes to main; Python tooling; dual dependency targets; mandatory `Why`. |