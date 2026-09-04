# GOV · Governance, legal, process
- Prefix: GOV
- Lead: none
- Baseline: §1, §57, §58, §67, §68

<!-- roadmap:generated:begin summary -->
Tasks: 84 live, 1 done, 0 in-progress, 83 todo, 0 dropped. Ready: 4. Blocked: 79. Weighted: 1%.
<!-- roadmap:generated:end -->

## Scope

GOV owns project identity, legal policy, contribution and community process, and this repository's roadmap tooling. It records the license firewall that maps stability Layers to license classes, contribution provenance, trademark and naming, legal-entity and funding form, codec and firmware redistribution, RFC and ADR process including the §57 checklist and §68 litmus questions, research-programme citation rules, Milestone Gate governance, and the Rust `roadmap` tool that validates and generates the graph. Native software never sees POSIX, Linux syscalls or Win32; those remain Personality concerns (§3, §57).

Mechanised compliance is not this prefix: SPDX and cargo-deny CI, SBOM generation, corresponding-source publication and notices bundles belong to BLD and REL. GOV writes the policy those pipelines enforce.

## Out of scope

Kernel outbound license for new in-tree code (KRN). SPDX headers, cargo-deny, SBOM CI and licence scanning (BLD). Corresponding source beside channels, signing ceremony, Hardware Compatibility List publication and advisory feed (REL). Docs site, IDL-to-docs and research study papers (DOC). Threat model and technical audit fixes (SEC). EAA conformity evidence (ACC). Font family inventory (TXT). Per-blob firmware audit (HW). Wine hosting and clean-room policy (WIN). Personality packaging mechanics and LGPL relinking (PKG). Semantic registry implementation (SEM). Codec Components (MED). Layer 1 review-gate mechanics (ABI). Privacy consent UI (INS). SDK outbound license (SDK).

## Tasks

### GOV-001 · Decide code hosting forge and repository layout
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-004
- Baseline: §50, §65
- Decision: D-0089

Every V0 CI and review path needs a recorded home for git: which forge hosts the trees, whether the Markdown roadmap is a standalone repository, and how Evidence aliases will resolve. Kernel versus userspace topology remains BLD; this Decision is hosting and layout of the planning corpus and product trees.

<!-- covers: GAP-0384 -->

#### Out of scope
Monorepo versus pinned-manifest multi-repo for kernel and userspace (BLD-005). CI platform (BLD-003). Repository-alias URLs (GOV-014).

#### Acceptance criteria
- [ ] Options evaluated include a self-hosted forge, a public hosted forge, and a hybrid with the roadmap standalone.
- [ ] The accepted option names where the Markdown roadmap lives and whether product trees share that forge.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-002 · Decide contributor licensing, copyright holder and DCO or CLA
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-003
- Baseline: §50, §57
- Decision: D-0092
- Risks: R-082

Provenance cannot be reconstructed after the first external commit. This Decision records DCO Signed-off-by on kernel commits, DCO versus a non-assignment CLA for userspace, whether contributors retain copyright or assign it, and whether a relicensing clause exists, so BLD can enforce headers and sign-off from V0.

<!-- covers: GAP-0036, GAP-0039, GAP-0385 -->

#### Out of scope
Outbound Layer licenses (GOV-003). Kernel-side GPLv2-compatible choice (KRN-003). Signed-off-by CI (BLD-011). AI-assisted contribution rules (GOV-007).

#### Acceptance criteria
- [ ] Options evaluated include DCO-only on all trees, DCO on the kernel and a non-assignment CLA on userspace, and assignment CLA on all trees.
- [ ] The accepted option states who holds copyright and whether a relicensing clause exists.
- [ ] Kernel commits are required to carry Signed-off-by under the accepted option.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-003 · Decide the license firewall and outbound project licenses
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §1, §66, §67
- Decision: D-0102
- Invariants: I-067

Copyleft leak into platform Interfaces and missing userspace headers cannot be fixed later. This Decision maps Layer 1 kernel code to GPLv2, picks the permissive class for Layers 2 through 4 including native userspace services, licenses the roadmap and Decision corpus, and states outbound terms for native kernel code beside inherited GPLv2. It coordinates with KRN-003; neither depends on the other.

<!-- covers: GAP-0004, GAP-0005, GAP-0074, GAP-0385 -->

#### Out of scope
New in-tree kernel license choice (KRN-003). SDK crate license (SDK-027). IDL generated-code exception (IPC-005). Userspace allowlist text (GOV-016). SPDX and cargo-deny CI (BLD-023, BLD-011).

#### Acceptance criteria
- [ ] Options evaluated include Layer 1 GPLv2 with Layers 2 through 4 MIT, the same split with Apache-2.0, the same split with MPL-2.0, and dual MIT/Apache-2.0 userspace, each stating whether an explicit patent grant is required.
- [ ] The accepted option names the license of the roadmap and Decision files.
- [ ] The accepted option states outbound terms for native kernel code beside inherited GPLv2 without restating KRN-003.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer and kernel architecture lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-004 · Decide the Markdown-plus-Rust-CLI roadmap process
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: none
- Baseline: §54, §65, §66, §67
- Decision: D-0001
- Invariants: I-087
- Verified by: @jakebarnby

The roadmap must stay truthful under years of parallel human and agent edits: a claim of progress needs Evidence, a freeze needs a Spike then a Decision, and a performance claim needs a B-ID (§54, §65). This Decision records how the graph is stored and validated so a single task ID is enough to start work.

<!-- covers: EXTRA-062, EXTRA-064, GAP-0164 -->

#### Out of scope
Register schema completion and parse CI (GOV-013). Mutation commands (GOV-012). Diff-aware check (GOV-010).

#### Acceptance criteria
- [x] Options evaluated include Markdown only, Markdown plus a Python validator, Markdown plus a Rust tool, and an issue tracker as source of truth.
- [x] The accepted option forbids calendar dates in sources the tool reads (I-087) and requires generated roll-ups never to be hand-edited.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0001

### GOV-005 · Publish RFC, ADR, research and ABI-review process with principles
- Type: docs
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: GOV-004, GOV-006
- Baseline: §57, §58, §65, §67, §68
- Risks: R-007
- Invariants: I-009, I-040, I-065, I-066

V0 exit requires accepted Decisions with rejected options. This document encodes RFC stages, ADR lifecycle, the §57 checklist on the ADR template, PRINCIPLES.md, the §68 litmus questions, fossilization review, heightened ABI review versus SDK, and the research-programme process that graduates Spikes. Borrowing and replacement invariants become template gates rather than prose.

<!-- covers: GAP-0056, GAP-0057, GAP-0070, INV-0717, INV-1129, INV-1152, INV-1281, INV-1309, INV-1310, INV-1311, INV-1312 -->

#### Out of scope
Layer 1 review-gate mechanics (ABI-006). Research study papers (DOC-001). Research-programme index (GOV-015). External RFC venue (GOV-035).

#### Acceptance criteria
- [ ] `decisions/TEMPLATE.md` requires at least two options, the §57 checklist, which principles are upheld and traded off, and both §68 litmus questions.
- [ ] PRINCIPLES.md exists and lists the seventeen §67 principles with their Baseline citations.
- [ ] The RFC process document names stages, discussion venue for V0, final-comment period as a condition not a date, Decision authority and kill criteria for Spikes.
- [ ] ABI changes require a higher approval bar than SDK changes, recorded in the same process document.
- [ ] An ADR whose litmus answer is history, or that replaces a mature mechanism without a cited B-ID, fails the template review.
- [ ] A Review line names who accepts the process documents.

#### Verification
- Unit: `tools/roadmap` fixture rejecting an ADR template missing options or litmus questions.
- Review: ABI lead and GOV maintainer sign-off recorded on the pull request.

#### Evidence
- none

### GOV-006 · Publish charter, vision, architecture map and layer stability policy
- Type: docs
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-004
- Baseline: §1, §4, §66, §69, §70
- Invariants: I-003, I-004

V0 identity surface: MISSION.md from §70, a vision of what is not built (§1), a living §4 and §69 architecture map with Workstream owners, and the per-Layer stability policy (§66). ABI later fills concrete Layer 1 and Layer 2 deprecation windows.

<!-- covers: INV-0059, INV-0116, INV-1288, INV-1333, INV-1334 -->

#### Out of scope
Layer 1 versus Layer 2 enumeration (ABI-011). Per-Layer deprecation windows (ABI-039). PRINCIPLES.md and ADR template (GOV-005).

#### Acceptance criteria
- [ ] MISSION.md reproduces the §70 mission statement and names the nine pillars.
- [ ] The vision document states the project is not a Linux distribution and not a desktop environment on existing Linux userspace (I-003, I-004).
- [ ] The architecture map links every §69 box to its owning prefix.
- [ ] A per-Layer stability policy names allowed change types for Layer 1 through Layer 4 without freezing any Layer 1 surface (I-040).

#### Verification
- Review: architecture lead sign-off recorded on the pull request that adds MISSION.md, the vision page, the architecture map and the Layer policy.

#### Evidence
- none

### GOV-007 · Publish contributing, CODEOWNERS, AI policy and V0 Gate governance
- Type: docs
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-002, GOV-001, GOV-005
- Baseline: §57, §59, §65
- Risks: R-006
- Invariants: I-094, I-099

CONTRIBUTING, MAINTAINERS/CODEOWNERS, AI-assisted contribution rules, and who may close a Gate with what Evidence. The V0 Milestone file maps every §59 item and states that V0 is not a usable desktop.

<!-- covers: GAP-0038, GAP-0064, GAP-0066, GAP-0069, INV-1153, INV-1176 -->

#### Out of scope
V0.5 Gate mapping (GOV-017). Code of Conduct (GOV-030). Merge-queue CI (BLD-001).

#### Acceptance criteria
- [ ] CONTRIBUTING names commit conventions, review expectations, response-time targets as conditions, and how to propose or modify roadmap tasks.
- [ ] A MAINTAINERS or CODEOWNERS mapping exists per Workstream and the forge requires that review on non-metadata pull requests.
- [ ] The AI-assisted contribution policy covers disclosure, provenance, license compatibility of generated code and reviewer responsibility.
- [ ] The V0 Milestone file maps every §59 required item and states that V0 is not a usable desktop and includes no UI or desktop deliverable (I-094).
- [ ] Gate-exit rules name who approves, what Evidence is required, and how a Milestone is re-scoped or a task dropped.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands CONTRIBUTING, CODEOWNERS, the AI policy and the V0 Milestone file.
- Manual: a metadata-only Status change merges on green checks without human review; an acceptance-criteria change requires the owning Lead.

#### Evidence
- none

### GOV-008 · Record the V0 exit review
- Type: docs
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-008, ABI-010, CAP-010, IPC-006, IPC-003, CMP-010, WIN-001, BOOT-004
- Baseline: §57, §59, §65

Record the V0 exit review: V0 execution-model Decisions (handle representation, capability encoding, IDL, fast-path, wrapper-versus-native, Windows scoping, boot strategy) plus the threat model and ABI specification tasks named by V0-G20. Layer 1 surfaces remain unfrozen.

#### Out of scope
Per-Gate verification (the Gate `Verified by` tasks). Generated roll-up (GOV roadmap tooling).

#### Acceptance criteria
- [ ] Every Decision named in Depends on is accepted.
- [ ] The V0 Milestone file records this exit review and the generated roll-up shows every V0 task `done` or `dropped` with reason.
- [ ] Layer 1 surfaces remain unfrozen.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands the exit review.

#### Evidence
- none

### GOV-009 · Decide font shipping versus metric-compatible substitutes
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-003
- Baseline: §41, §49
- Decision: D-0098
- Invariants: I-068, I-070

V0.5 ships the first immutable image with fonts. This Decision allows only redistributable fonts in that image. Proprietary fonts and metric substitutes for migrated documents stay with the V1 codec Decision. TXT selects families and inventories terms after this policy exists.

<!-- covers: GAP-0030 -->

#### Out of scope
Default family set and script coverage (TXT-002). Per-font license inventory (TXT-005). Proprietary fonts and metric substitutes for migrated documents (GOV-020). Userspace allowlist (GOV-016).

#### Acceptance criteria
- [ ] Options evaluated include shipping only OSI-approved font licenses in the default image, shipping those plus documented metric-compatible substitutes, and shipping no fonts (applications bundle their own).
- [ ] The accepted option forbids Microsoft core fonts in the default image (I-070).
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer and TXT lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-010 · Implement roadmap check --base, impact summary and covers audit
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-013
- Baseline: §65
- Risks: R-067

Parallel Workstreams need a diff-aware check against a base ref, `impact --summary`, and covers-orphan reporting before V1 contributor traffic. IDs must never be deleted, renumbered or moved once on the base; the check enforces that.

#### Out of scope
Mutation commands (GOV-012). Fan-in warning (GOV-011). Coverage input authoring (`tools/coverage/`).

#### Acceptance criteria
- [ ] `roadmap check --base <ref>` fails a fixture that deletes, renumbers or moves an ID present on that ref, with a file:line diagnostic.
- [ ] `roadmap impact <ID> --summary` prints the count of transitively unblocked tasks grouped by prefix.
- [ ] `roadmap coverage` reports every `tools/coverage/*.jsonl` item covered by no non-dropped task and every task that covers nothing and is cited by no Gate.

#### Verification
- Unit: `tools/roadmap` fixtures for deleted, renumbered and moved IDs against a base ref.
- Review: GOV maintainer confirms `--base`, `impact --summary` and `coverage` are documented in AGENTS.md.

#### Evidence
- none

### GOV-011 · Warn when a hub Task's fan-in exceeds the configured threshold
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-013
- Baseline: §65
- Risks: R-067

ABI-shape, IDL, threat-model and hardware ADRs will be depended on by hundreds of tasks. The validator must warn at `fan_in_warning` so hubs can be split rather than becoming semantic events on every edit.

#### Out of scope
Workstream-split procedure (GOV-043). Diff-aware check (GOV-010).

#### Acceptance criteria
- [ ] `roadmap check` emits a warning when dependents of a task exceed `roadmap.toml` `fan_in_warning`.
- [ ] The warning names the hub task ID and the dependent count.
- [ ] A fixture below the threshold emits no warning.

#### Verification
- Unit: `tools/roadmap` fixtures above and below `fan_in_warning`.
- Review: GOV maintainer confirms the threshold is the configured policy value, not a hand-typed number in prose.

#### Evidence
- none

### GOV-012 · Implement roadmap mutation commands from claim through renumber
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-013
- Baseline: §65

AGENTS.md and CONVENTIONS list `claim`, `unclaim`, `block`, `done`, `drop`, `split`, `move` and `renumber` as still done by hand. Ship them before V1 external contributors edit tasks, so Status transitions stay inside section 7 of CONVENTIONS.

#### Out of scope
`stale`, `slipped` and `history` (GOV-028). Diff-aware check (GOV-010). Independent verification flag (GOV-027).

#### Acceptance criteria
- [ ] `roadmap claim`, `unclaim`, `block`, `done`, `drop`, `split`, `move` and `renumber` exist and reject transitions forbidden by CONVENTIONS section 7.
- [ ] `roadmap block <ID> "reason"` mints a Q entry in `registers/questions.md` and adds it to `Depends on`.
- [ ] `roadmap split` drops the parent with `Dropped because: superseded` and `Superseded by:` naming the new IDs, and repoints dependents in the same change.
- [ ] Agents cannot set `Verified by`; the command rejects `@agent/` identities on that field.

#### Verification
- Unit: `tools/roadmap` tests per mutation command, including forbidden transitions.
- Review: GOV maintainer confirms AGENTS.md command table matches the shipped binary.

#### Evidence
- none

### GOV-013 · Complete roadmap registers, ADR CI and Task parseability
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-004
- Baseline: §54, §65, §66
- Invariants: I-087, I-088

Workstreams.md gives this repository's tooling to GOV. Finish corpus-register schema and validation (entries stay LNX and WIN), wire the invariants register with Enforced-by links, and fail CI on broken ADR or task ID links or unparseable templates. IDL-to-docs generation remains DOC and SDK.

<!-- covers: EXTRA-062, EXTRA-064, GAP-0164 -->

#### Out of scope
Corpus scenario content (LNX, WIN). IDL-to-docs generator (DOC-010). Mutation commands (GOV-012). Standing-invariant lint text (GOV-019).

#### Acceptance criteria
- [ ] `roadmap check --allow-drafts --index tools/coverage/slugs.tsv` exits 0 on this repository and fails a fixture whose ADR or task ID link is dangling.
- [ ] A task that cites a C-ID or I-ID absent from `registers/` fails check with a file:line diagnostic.
- [ ] CI runs `roadmap fmt --check`, `roadmap gen --check` and `roadmap check` on every pull request to this repository.

#### Verification
- Unit: `tools/roadmap` tests covering register reference and unparseable-template fixtures.
- Review: GOV maintainer confirms the CI workflow file exists and the three commands are required checks.

#### Evidence
- none

### GOV-014 · Populate the repository-alias Register from the hosting Decision
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-001
- Baseline: §65

`registers/repos.md` aliases are owned by GOV. After the hosting Decision, replace placeholder URLs so Evidence lines resolve to real trees.

#### Out of scope
Forge and layout Decision (GOV-001). Evidence grammar (CONVENTIONS section 4.3). Product-tree contents (KRN, SDK, REL).

#### Acceptance criteria
- [ ] Every alias in `registers/repos.md` has a URL that matches the accepted hosting Decision.
- [ ] `roadmap check` fails a fixture Evidence line whose alias is absent from the register.
- [ ] Placeholder example hostnames do not remain on any alias that a V0.5 task cites.

#### Verification
- Unit: `tools/roadmap` fixture for an unknown Evidence alias.
- Review: GOV maintainer sign-off recorded on the pull request that replaces placeholder URLs.

#### Evidence
- none

### GOV-015 · Publish the research-programme index that every ADR must cite
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-005
- Baseline: §58, §68

GOV tracks the §58 research programme. DOC writes the studies; this index is what ADR templates require. Non-gating at V0.5 per the research ordering.

#### Out of scope
Study papers and catalog template (DOC-001). Spike graduation process (GOV-005).

#### Acceptance criteria
- [ ] An index lists every §58 inspiration with its study path or an explicit deferred marker.
- [ ] The ADR template requires a citation into this index.
- [ ] The acceptance test stated in the index is "does it produce the strongest coherent system", never pedigree or novelty (INV-1152).

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands the index and the template citation rule.

#### Evidence
- none

### GOV-016 · Publish the userspace dependency license allowlist
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-003
- Baseline: §51, §66
- Invariants: I-068

The first immutable Packages land at V0.5. The allowlist excludes AGPL, SSPL, BUSL and similar source-available licenses from the default image and SDK tree. BLD enforces the list in CI.

<!-- covers: GAP-0012 -->

#### Out of scope
CI enforcement (BLD-023). Kernel GPLv2-compatible allowlist (I-067, BLD-011). Font shipping Decision (GOV-009). SDK crate license (SDK-027).

#### Acceptance criteria
- [ ] The published allowlist names permitted userspace licenses and explicitly excludes AGPL, SSPL and BUSL (I-068).
- [ ] Default-image and SDK dependency trees are in scope; optional non-free repositories are named as out of this list.
- [ ] BLD-023 is cited as the enforcer, not reimplemented here.

#### Verification
- Review: GOV licensing reviewer sign-off recorded on the pull request that publishes the allowlist.

#### Evidence
- none

### GOV-017 · Author the V0.5 Milestone gates mapping the application model
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-007
- Baseline: §60
- Invariants: I-088, I-094

Gate-verifying docs: every §60 item in the V0.5 Milestone file, including four demo native applications from immutable Packages on the native compositor.

<!-- covers: INV-1177, INV-1193 -->

#### Out of scope
Demo application implementation (APP). Compositor (GFX). Package format (PKG). V1 Gates (GOV-037).

#### Acceptance criteria
- [ ] `milestones/V0.5.md` maps every §60 required item to a Gate or Demo with `Verified by` task IDs.
- [ ] A Gate requires four demo native applications running on the native compositor from immutable Packages.
- [ ] Gate prose cites B-IDs and C-IDs only; it does not restate numbers (I-088).

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that updates `milestones/V0.5.md`.
- Unit: `roadmap gate V0.5` enumerates each Gate and its unsatisfied `Verified by` tasks.

#### Evidence
- none

### GOV-018 · Record the V0.5 exit review
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-016, PKG-011, PKG-005, STO-013, UIP-006, GFX-015, PKG-008, SEC-007, ACC-002
- Baseline: §57, §59, §65

Record the V0.5 exit review: V0.5 application-model Decisions (filesystem, package manifest, content-addressing, UI protocol, compositor scheduling, generation materialisation, grant taxonomy, a11y tree). Layer 1 surfaces remain unfrozen.

#### Out of scope
Per-Gate verification (the Gate `Verified by` tasks). Generated roll-up (GOV roadmap tooling).

#### Acceptance criteria
- [ ] Every Decision named in Depends on is accepted.
- [ ] The V0.5 Milestone file records this exit review and the generated roll-up shows every V0.5 task `done` or `dropped` with reason.
- [ ] Layer 1 surfaces remain unfrozen.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands the exit review.

#### Evidence
- none

### GOV-019 · Encode standing invariants and non-goals as linted Register rules
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-013, GOV-005
- Baseline: §1, §2, §3, §53, §54, §56.5, §57, §66, §67, §69
- Risks: R-006
- Invariants: I-003, I-004, I-009, I-025, I-042, I-050, I-061, I-065, I-066, I-087, I-088, I-094, I-096

Collapse nongoals and invariants into lint and register rules, not one task each. Populate I-IDs for the §1, §2, §3, §53, §54, §56.5, §57, §66, §67 and §69 rules (not a distro or DE, mechanism-versus-semantics, Personality firewall, no fake speedups, ecosystem Gates from V1, ideas stay in Layer 3 and Layer 4) and enforce them on ADRs and public material.

<!-- covers: INV-0008, INV-0009, INV-0060, INV-0080, INV-0637, INV-0988, INV-1017, INV-1039, INV-1094, INV-1102, INV-1114, INV-1124, INV-1291, INV-1292, INV-1293, INV-1306, INV-1307, INV-1308, INV-1332 -->

#### Out of scope
Kernel-fork staffing document (GOV-033). ABI POSIX-name lint (ABI-018). Native-crate firewall (ABI-003).

#### Acceptance criteria
- [ ] Each listed I-ID exists in `registers/invariants.md` with Status `stated` and a Baseline citation.
- [ ] `roadmap check` fails a fixture ADR that claims a POSIX or Linux syscall is available natively, or that freezes a Layer 1 surface before V4.
- [ ] `roadmap check` fails a fixture whose Description contains a performance number without a B-ID, or that promises a uniform speedup versus Linux.
- [ ] Public-material lint rejects "not a Linux distribution" and "not a desktop environment" violations as well as warm-startup figures stated as guarantees (I-042, I-003, I-004).

#### Verification
- Unit: `tools/roadmap` fixtures for I-003, I-004, I-025, I-040, I-042, I-050, I-061 and I-087 violations.
- Review: GOV maintainer confirms every listed I-ID is cited by this task's `Invariants:` field.

#### Evidence
- none

### GOV-020 · Decide codec and proprietary-font shipping and patent policy
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-003, GOV-009
- Baseline: §56.5, §57
- Decision: D-0090
- Risks: R-068

MED packages codecs at V1, so this Decision cannot wait. One Decision covers default-image royalty-free codecs, hardware-passthrough H.264, HEVC and AAC, optional Packages, jurisdiction splits, and proprietary fonts versus metric substitutes for migrated documents.

<!-- covers: GAP-0029, GAP-0301, GAP-0473 -->

#### Out of scope
Codec Package schema and decoder Components (MED). Font family inventory (TXT-005). V0.5 redistributable font shipping (GOV-009). Protected-content CDM (MED, LNX).

#### Acceptance criteria
- [ ] Options evaluated include royalty-free-only in the default image with hardware passthrough of encumbered codecs, shipping software H.264/HEVC/AAC in optional Packages, and jurisdiction-split images.
- [ ] The accepted option names which codecs may appear in the default image, which are hardware-passthrough only, and which are optional user-installed Packages.
- [ ] The accepted option records proprietary-font versus metric-substitute policy for migrated documents.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer and MED lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-021 · Decide the documentation license and translation terms
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-003
- Baseline: §56.5, §66
- Decision: D-0095

CC-BY-SA versus CC-BY versus CC0, and translation-contribution terms, before the V1 docs site and V3 translations. DOC consumes the Decision. Dataset license is a later Decision.

<!-- covers: GAP-0073, GAP-0081 -->

#### Out of scope
Docs site and toolchain (DOC). Translation pipeline (DOC-025). Benchmark and HCL dataset license (GOV-040). Roadmap file license (GOV-003).

#### Acceptance criteria
- [ ] Options evaluated include CC-BY-SA, CC-BY and CC0 for documentation, each with translation-contribution terms.
- [ ] The accepted option states whether translations are independent works or must use the same license.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer and DOC lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-022 · Decide firmware blob redistribution for official versus non-free
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-003
- Baseline: §33, §55, §62
- Decision: D-0097

V1 Intel-laptop Wi-Fi and GPU images cannot ship blobs without redistribution rights. This Decision chooses official image versus non-free repository versus download-on-demand. HW audits per blob; REL hosts any non-free repository.

<!-- covers: GAP-0373 -->

#### Out of scope
Per-blob audit (HW-023). Firmware update service (HW-046). Non-free repository mechanics (REL). Userspace allowlist (GOV-016).

#### Acceptance criteria
- [ ] Options evaluated include shipping redistributable blobs in the official image with the rest excluded, a separate non-free repository, and download-on-demand at first boot.
- [ ] The accepted option states what may appear in official images versus a non-free channel versus excluded.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer and HW lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-023 · Adopt the governance charter for decisions and maintainers
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-007, GOV-024
- Baseline: §67, §70
- Decision: D-0100
- Risks: R-082

External contributors will not invest without knowing how Decisions are made. This charter records BDFL versus council versus foundation TSC, appointment, removal, voting and conflict resolution, before the repository opens at developer preview.

<!-- covers: GAP-0063 -->

#### Out of scope
Legal-entity form (GOV-024). Code of Conduct (GOV-030). Maintainer succession plan (GOV-060). RFC process text (GOV-005).

#### Acceptance criteria
- [ ] Options evaluated include BDFL, a maintainer council, and a foundation technical steering committee.
- [ ] The accepted option names appointment, removal, voting and conflict-resolution rules.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-024 · Decide legal-entity form able to hold marks and signing keys
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-039
- Baseline: §56.4, §63
- Decision: D-0101
- Risks: R-082

Foundation, fiscal sponsor or company as counterparty for trademarks, signing keys, shim, OIN and vendor agreements. Needed before V1 trademark filing and V3 public distribution. Jurisdiction evidence comes from the Spike.

<!-- covers: GAP-0040 -->

#### Out of scope
Jurisdiction comparison (GOV-039). Trademark filing (GOV-036). Domain registration (GOV-034). OIN membership (GOV-052). Signing ceremony (REL-032).

#### Acceptance criteria
- [ ] Options evaluated include a foundation, a fiscal sponsor and a company, each against the Spike report.
- [ ] The accepted option names the entity able to hold trademarks and root signing keys.
- [ ] Q-049 is marked answered by GOV-039 in the Decision file.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file, citing `reports/spikes/GOV-039.md`.

#### Evidence
- none

### GOV-025 · Decide the 1.0 portability commitment as x86-64 only shipping
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-006
- Baseline: §1, §38, §66
- Decision: D-0106
- Invariants: I-001, I-011, I-012, I-100

§38: ship x86-64 only while the ABI stays architecture-neutral. Recorded before SDK v1 freeze candidates so ARM64 and RISC-V remain LATER without fossilizing x86 into Layer 1.

<!-- covers: INV-0722 -->

#### Out of scope
ARM64 cross-build CI (BLD-030). RISC-V build-only CI (KRN). Fossilization review (ABI-054). Layer 1 freeze (ABI-049).

#### Acceptance criteria
- [ ] Options evaluated include x86-64-only shipping with architecture-neutral ABI, promising ARM64 at 1.0, and promising RISC-V at 1.0.
- [ ] The accepted option states that ARM64 and RISC-V remain compiling in the fork (I-011, I-012) and are not 1.0 platforms (I-001).
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: ABI lead and GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-026 · Decide vendor NDA policy for hardware documentation
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-007
- Baseline: §55, §57
- Decision: D-0112

What may be read versus reproduced. Hardware enablement may use NDA docs; the tree and public docs may not. The published policy text is a later docs task.

<!-- covers: GAP-0079 -->

#### Out of scope
Published policy document (GOV-046). Vendor influence and conflict-of-interest (GOV-068). Driver implementation (HW).

#### Acceptance criteria
- [ ] Options evaluated include forbidding NDA docs entirely, allowing NDA docs to be read but never reproduced in the tree, and allowing quoted excerpts under recorded conditions.
- [ ] The accepted option states that the public tree and public docs contain no NDA material.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer and HW lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-027 · Enable independent Verification for every done V1 and later Task
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-013, GOV-007
- Baseline: §65

CONVENTIONS section 13: switch `require_independent_verification` on when V1 is active so agents cannot self-close. V0 already requires it for adr and Freezes.

#### Out of scope
Mutation command rejection of `@agent/` on `Verified by` (GOV-012). Verifier identity policy text (GOV-007).

#### Acceptance criteria
- [ ] `roadmap.toml` sets `require_independent_verification` to true.
- [ ] `roadmap check` fails a done V1 task that lacks `Verified by` naming a human `@handle` different from Owner.
- [ ] `@agent/` identities remain rejected on `Verified by`.

#### Verification
- Unit: `tools/roadmap` fixtures for missing verifier, owner-as-verifier and `@agent/` verifier on a V1 done task.
- Review: GOV maintainer confirms the policy flag is true in the committed `roadmap.toml`.

#### Evidence
- none

### GOV-028 · Implement roadmap stale, slipped, history and dropped reasons
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-012
- Baseline: §65

AGENTS.md planned commands `stale`, `slipped` and `history`, plus the dropped-reason taxonomy already in `fields.json`. Needed once V1 has a living task graph. `stale` stays local-only via git and is never committed.

#### Out of scope
Claim through renumber (GOV-012). Diff-aware check (GOV-010).

#### Acceptance criteria
- [ ] `roadmap stale` lists in-progress tasks with no block change across the configured commit window and writes nothing to committed files.
- [ ] `roadmap slipped` lists tasks whose Milestone moved to a later rung, using git history rather than stored dates.
- [ ] `roadmap history <ID>` prints commits whose subject or trailer cites that ID.
- [ ] `Dropped because:` values outside the `fields.json` enum fail `roadmap check`.

#### Verification
- Unit: `tools/roadmap` tests for stale, slipped, history and invalid dropped reasons.
- Review: GOV maintainer confirms AGENTS.md documents the three commands.

#### Evidence
- none

### GOV-029 · Add See also citations so gates can name LATER tasks
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-013
- Baseline: §65, §66

The 1.0 2.0-planning RFC must cite LATER items without depending on them. Ship a `See also` field on Gates and tasks before V3 Gates grow those lists. Nothing outside LATER may depend on a LATER task. Required by 1.0-G17 (Governance, 1.x branch and 2.0 planning RFC).

#### Out of scope
2.0 planning RFC (GOV-082). Workstream-header `See also` for prefix splits (GOV-043).

#### Acceptance criteria
- [ ] Gates and tasks may carry `See also:` listing task IDs, including LATER IDs, without those IDs appearing in `Depends on`.
- [ ] `roadmap check` still fails a non-LATER task whose `Depends on` lists a LATER task.
- [ ] `roadmap show` inlines See-also IDs without treating them as blockers.

#### Verification
- Unit: `tools/roadmap` fixtures for See-also to LATER (allowed) versus Depends-on LATER (forbidden).
- Review: GOV maintainer confirms `fields.json` documents the field.

#### Evidence
- none

### GOV-030 · Adopt a Code of Conduct with named enforcement body
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-023
- Baseline: §70
- Risks: R-082

Required before the repository opens to external contributors at developer preview. V3 community infrastructure adds moderation roles on top.

<!-- covers: GAP-0065 -->

#### Out of scope
Community channels (GOV-057). Issue triage (GOV-032). Governance charter (GOV-023).

#### Acceptance criteria
- [ ] A Code of Conduct file exists in the public repository.
- [ ] The document names an enforcement body and an escalation path.
- [ ] The charter's conflict-resolution rules point at this document.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that adds the Code of Conduct.

#### Evidence
- none

### GOV-031 · Assemble a legally reviewed redistributable compatibility Corpus
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-003, GOV-016
- Baseline: §3, §46, §48, §49
- Corpora: C-001, C-002, C-003, C-007

Legal review of Linux and Windows applications, games, fonts, images and documents CI may redistribute. Distinct from corpus-register schema. V1 L2 corpus cannot run on runners without it.

<!-- covers: GAP-0156 -->

#### Out of scope
Corpus scenario scripts (LNX, WIN). Register schema (GOV-013). CI plumbing (BLD-017). Wine license review (GOV-047).

#### Acceptance criteria
- [ ] A review record lists each C-ID in V1 hardware scope and states whether its inputs may be redistributed to CI runners.
- [ ] Inputs that cannot be redistributed are named with the substitute or skip used on runners.
- [ ] LNX and WIN scenario owners are cited; this task does not rewrite scenarios.

#### Verification
- Review: GOV licensing reviewer sign-off recorded on the review record, with LNX and WIN leads confirming C-ID coverage.
- Compat: each listed C-ID has a redistribution verdict in the record.

#### Evidence
- none

### GOV-032 · Publish issue triage labels, severity, templates and stale policy
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-007, GOV-001
- Baseline: §61, §63

Define at V1 (generation ID, hardware and Personality tags, response targets) so V3 community infrastructure only staffs it. A public tracker without triage rules becomes noise.

<!-- covers: GAP-0072, GAP-0386 -->

#### Out of scope
Community channels (GOV-057). Forge hosting (GOV-001). Bug-reporter client (INS-019).

#### Acceptance criteria
- [ ] Labels cover severity, priority, hardware and Personality, and templates capture generation ID and diagnostics.
- [ ] Response-time targets and stale handling are stated as conditions, not dates.
- [ ] The policy is published in the same forge the hosting Decision selected.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that publishes the triage policy.
- Manual: opening a new issue shows the template fields for generation ID and diagnostics.

#### Evidence
- none

### GOV-033 · Document kernel-fork maintenance staffing and process
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-019
- Baseline: §6, §56.4
- Risks: R-029

§56.4 standing cost, encoded as an invariant at V0; the staffing and process document is due before Phase D and at the first V1 upstream rebase.

<!-- covers: INV-1094 -->

#### Out of scope
CVE response steps (KRN-031). Advisory publication (REL). Phase D entry Decision (KRN-042). Divergence ledger (KRN-008).

#### Acceptance criteria
- [ ] The document names roles for rebase, CVE applicability and driver adaptation as a permanent cost.
- [ ] The document is cited from the V1 Milestone notes so ecosystem Gates are not kernel-only (INV-1102).
- [ ] Phase D is named as a condition that this document must already exist.

#### Verification
- Review: KRN lead and GOV maintainer sign-off recorded on the pull request that lands the staffing document.

#### Evidence
- none

### GOV-034 · Register domains, namespaces and handles under the entity
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-024
- Baseline: §63
- Risks: R-082

Domains, package-repository namespace, forge organisation and social handles held by the legal entity, not an individual. Blocks V3 public-alpha identity.

<!-- covers: GAP-0035 -->

#### Out of scope
Legal-entity form (GOV-024). Trademark filing (GOV-036). Repository developer agreement (GOV-054).

#### Acceptance criteria
- [ ] A register lists domains, forge organisation, package-repository namespace and social handles.
- [ ] Each entry names the legal entity as holder, not an individual.
- [ ] Transfer or registrar evidence is attached as Evidence URLs.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands the register.
- Manual: WHOIS or registrar records match the entity named in GOV-024.

#### Evidence
- none

### GOV-035 · Publish RFC templates and the discussion venue for contributors
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-005, GOV-001
- Baseline: §65, §66

The process was defined at V0; V1 opens to external developers with SDK v1. Templates and venue must exist before the V3 Gate that counts processed external RFCs.

<!-- covers: GAP-0056 -->

#### Out of scope
RFC process definition (GOV-005). Exercising five external RFCs (GOV-064). Docs site (DOC).

#### Acceptance criteria
- [ ] RFC templates exist in the public repository and match `decisions/TEMPLATE.md` required sections.
- [ ] A discussion venue is named and reachable without project-private credentials.
- [ ] The CONTRIBUTING guide links to the templates and venue.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that publishes templates and venue.
- Manual: an external contributor can open an RFC from the published template without extra access.

#### Evidence
- none

### GOV-036 · Run trademark clearance and Register word and logo marks
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-024
- Baseline: §63
- Risks: R-082

Clearance and filing before any public release. V1 is the last rung before V2 desktop preview names the product in public.

<!-- covers: GAP-0031 -->

#### Out of scope
Usage policy for derivatives (GOV-056). Linux mark in product naming (GOV-051). Third-party mark rules (GOV-044). Entity form (GOV-024).

#### Acceptance criteria
- [ ] A clearance search record exists for the word mark and logo in the jurisdictions the entity Decision named.
- [ ] Filing receipts name the legal entity as applicant.
- [ ] The product name used in V2 public materials matches the filed word mark.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that attaches clearance and filing Evidence.
- Manual: filing receipts are stored as `https://` Evidence lines on this task.

#### Evidence
- none

### GOV-037 · Author the V1 Milestone gates including self-hosting and dogfooding
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-017
- Baseline: §56.5, §61
- Invariants: I-088

Gate-verifying docs: every §61 item, self-hosting, internal daily-driving metrics, and ecosystem criteria on every V1 and later Gate.

<!-- covers: INV-1102, INV-1194, INV-1210, INV-1212 -->

#### Out of scope
Self-host build graph (BLD-049). V2 Gates (GOV-045). Kernel-fork staffing (GOV-033).

#### Acceptance criteria
- [ ] `milestones/V1.md` maps every §61 required item to a Gate or Demo with `Verified by` task IDs.
- [ ] A Gate requires developers to build the OS while running the OS.
- [ ] Every V1 and later Gate includes an ecosystem criterion, not only kernel criteria.
- [ ] Gate prose cites B-IDs and C-IDs only (I-088).

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that updates `milestones/V1.md`.
- Unit: `roadmap gate V1` enumerates each Gate and its unsatisfied `Verified by` tasks.

#### Evidence
- none

### GOV-038 · Record the V1 exit review
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-054, SDK-055, NET-006, AUD-002, REL-003, LNX-018, WASM-007, SDK-028, LNX-015, KRN-024, KRN-026, IPC-042, LNX-021
- Baseline: §57, §59, §65

Record the V1 exit review: V1 developer-preview Decisions (SDK stability and crate surface, network capability, audio object model, signing, portals, Wasm runtime, `std` at Layer 3, ia32, eBPF, live-patching non-goal, L2 evolution rules, sandbox primitives). S-014 and S-031 may be freeze candidates; Layer 1 surfaces remain unfrozen.

#### Out of scope
Per-Gate verification (the Gate `Verified by` tasks). Generated roll-up (GOV roadmap tooling).

#### Acceptance criteria
- [ ] Every Decision named in Depends on is accepted.
- [ ] The V1 Milestone file records this exit review and the generated roll-up shows every V1 task `done` or `dropped` with reason.
- [ ] S-014 and S-031 may be freeze candidates; Layer 1 surfaces remain unfrozen.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands the exit review.

#### Evidence
- none

### GOV-039 · Compare legal-entity jurisdictions for export, GDPR and tax
- Type: spike
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: Q-049
- Baseline: §63

Question answered before the entity Decision: export controls, GDPR, trademark cost and donation tax treatment. Jurisdiction is expensive to change.

<!-- covers: GAP-0041 -->

#### Out of scope
Entity form Decision (GOV-024). Export-control classification Decision (GOV-050). EAR filing (GOV-058).

#### Acceptance criteria
- [ ] `reports/spikes/GOV-039.md` compares at least two jurisdictions on export control, GDPR, trademark cost and donation tax treatment.
- [ ] The report names what is expensive to change after formation.
- [ ] Q-049 is listed as the question this report answers.

#### Verification
- Report: which jurisdictions were compared; how each scores on export control, GDPR, trademark cost and donation tax; which option is recommended for GOV-024 and why; what becomes expensive to change after formation.
- Review: GOV maintainer confirms the report uses the spike skeleton in `reports/README.md`.

#### Evidence
- none

### GOV-040 · Decide the license for published benchmark and HCL datasets
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-021
- Baseline: §54, §62
- Decision: D-0094

CC0 versus ODbL for community datasets. REL consumes the Decision for Hardware Compatibility List and benchmark publication.

<!-- covers: GAP-0081 -->

#### Out of scope
Documentation license (GOV-021). HCL publication mechanics (REL-048). Benchmark methodology (BEN). Privacy of probe submissions (GOV-061).

#### Acceptance criteria
- [ ] Options evaluated include CC0 and ODbL for published benchmark and HCL datasets.
- [ ] The accepted option states whether community submissions are under the same license.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer and REL lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-041 · Decide the funding model and publish infrastructure-cost finances
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-024, Q-053
- Baseline: §56.4, §62
- Decision: D-0099
- Risks: R-053

Donations, sponsorship tiers or fiscal sponsor, with transparent finances. Hardware, CI, CDN and signing cost money before V2 lab and desktop Gates. Answers Q-053.

<!-- covers: GAP-0068, GAP-0395 -->

#### Out of scope
Legal-entity form (GOV-024). Vendor conflict-of-interest (GOV-068). CDN and download-site mechanics (REL). Lab procurement (LAB).

#### Acceptance criteria
- [ ] Options evaluated include donations, sponsorship tiers and a fiscal sponsor, each with a transparency rule for infrastructure cost.
- [ ] The accepted option names how CDN, build farm, hardware lab and signing hardware are funded through 1.0.
- [ ] Q-053 is marked answered by this task.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file and the first published finance summary.

#### Evidence
- none

### GOV-042 · Decide governance of the standard Semantic Interface catalogue
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-035, SEM-007
- Baseline: §42, §44, §45, §57, §67
- Decision: D-0109
- Risks: R-043
- Invariants: I-023, I-051

Who may extend Document, Mail, Calendar and Project and how RFCs land, before the V2 automation-then-AI demo. SEM owns registry implementation; GOV owns catalogue governance. The AI broker stays after a done registry task.

<!-- covers: INV-0800 -->

#### Out of scope
Registry service (SEM-029). Automation rules (SEM-013). AI broker (SEM-010). IDL language (IPC).

#### Acceptance criteria
- [ ] Options evaluated include a GOV-held standard catalogue extended only by RFC, an open catalogue any publisher may extend, and a hybrid with a frozen core plus optional extensions.
- [ ] The accepted option names who may add catalogue Interfaces and that the AI broker is not in scope of this Decision.
- [ ] Depends on includes a Semantic interface registry task (SEM-007).
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer and SEM lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-043 · Decide the Workstream-split procedure at the size warning
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-004, GOV-011
- Baseline: §65
- Decision: D-0114

Files over `workstream_lines_warning` are remedied by a GOV adr. Critique expects an APP shell/apps split around V2. Records prefix permanence, `See also`, and that old IDs never move.

#### Out of scope
Performing any specific split (owning prefix). Header `See also` implementation already in the schema. Fan-in warning (GOV-011).

#### Acceptance criteria
- [ ] Options evaluated include splitting a file when it exceeds the line warning, splitting by sub-scope on a named condition, and never splitting (new tasks stay in the original file).
- [ ] The accepted option states that existing IDs keep their prefix and file, new tasks take the new prefix, and a `See also:` header records the relation.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-044 · Define rules for third-party marks in compatibility claims
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-036
- Baseline: §3, §48, §49

Pulled from V3 to V2: Windows, Steam, Proton and related marks first appear in public V2 W1 and gaming Gates. Trademark usage policy for derivatives stays V3.

<!-- covers: GAP-0034 -->

#### Out of scope
Project trademark usage policy (GOV-056). Linux mark in product naming (GOV-051). Compatibility guide book (DOC-028).

#### Acceptance criteria
- [ ] Rules name how Windows, Steam, Proton and related marks may appear in compatibility documentation and marketing.
- [ ] Compatible-with claims are required to cite a C-ID rather than an unmeasured superiority statement.
- [ ] The rules are linked from V2 compatibility notes.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that publishes the third-party mark rules.

#### Evidence
- none

### GOV-045 · Author the V2 Milestone gates mapping the desktop preview
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-037
- Baseline: §62
- Invariants: I-088, I-095

Gate-verifying docs: every §62 item in the V2 Milestone file.

<!-- covers: INV-1234 -->

#### Out of scope
Desktop shell implementation (APP). Hardware bring-up (HW). V3 Gates (GOV-067).

#### Acceptance criteria
- [ ] `milestones/V2.md` maps every §62 required item to a Gate or Demo with `Verified by` task IDs.
- [ ] Hardware scope is the named Reference machines; unlisted hardware is unsupported (I-095).
- [ ] Gate prose cites B-IDs and C-IDs only (I-088).

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that updates `milestones/V2.md`.
- Unit: `roadmap gate V2` enumerates each Gate and its unsatisfied `Verified by` tasks.

#### Evidence
- none

### GOV-046 · Publish vendor NDA policy for hardware documentation
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-026
- Baseline: §55, §57

Vendor NDA policy (what may be read versus reproduced) is legal and governance. HW consumes the policy when writing drivers. Distinct from influence and conflict-of-interest.

<!-- covers: GAP-0079 -->

#### Out of scope
NDA Decision (GOV-026). Conflict-of-interest policy (GOV-068). Driver code (HW).

#### Acceptance criteria
- [ ] A published policy restates the accepted NDA Decision in contributor-facing language.
- [ ] The policy states that NDA material is absent from the public tree and public docs.
- [ ] HW CONTRIBUTING or equivalent links to this policy.

#### Verification
- Review: GOV maintainer and HW lead sign-off recorded on the pull request that publishes the policy.

#### Evidence
- none

### GOV-047 · Complete the Wine, DXVK and VKD3D-Proton license review
- Type: docs
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-003, WIN-005, PKG-010
- Baseline: §3, §48, §56.2
- Invariants: I-069, I-070

LGPL Wine and related Windows-personality Components before W1 Gates. WIN owns clean-room and upstream-first policy; GOV records the license review. PKG owns LGPL relinking.

<!-- covers: INV-0915 -->

#### Out of scope
Clean-room policy (WIN-005). Upstream-first Wine policy (WIN-007). LGPL relinking mechanics (PKG-010). Wine hosting (WIN-013). Wrapper redistribution (GOV-049).

#### Acceptance criteria
- [ ] A review record lists Wine, DXVK and VKD3D-Proton licenses and corresponding-source obligations for modified builds.
- [ ] The record cites PKG-010 for substitution rights (I-069).
- [ ] Microsoft fonts and unsigned redistributables are recorded as excluded (I-070).

#### Verification
- Review: GOV licensing reviewer and WIN lead sign-off recorded on the review record.

#### Evidence
- none

### GOV-048 · Record the V2 exit review
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-003, WIN-036, GFX-063, HW-040, SEC-043, SEM-018, WIN-002, HW-042
- Baseline: §57, §59, §65

Record the V2 exit review: V2 desktop-preview Decisions (target hardware, Windows object mapping, HDR pipeline, Bluetooth stack, permission prompts, automation rules, anti-cheat policy, sensor scope). Layer 1 surfaces remain unfrozen.

#### Out of scope
Per-Gate verification (the Gate `Verified by` tasks). Generated roll-up (GOV roadmap tooling).

#### Acceptance criteria
- [ ] Every Decision named in Depends on is accepted.
- [ ] The V2 Milestone file records this exit review and the generated roll-up shows every V2 task `done` or `dropped` with reason.
- [ ] Layer 1 surfaces remain unfrozen.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands the exit review.

#### Evidence
- none

### GOV-049 · Decide redistribution licensing for Personality software wrappers
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-003, PKG-047
- Baseline: §3, §28, §49
- Decision: D-0091

Redistribution licensing and wrapper ownership for third-party Linux and Windows software (Flatpak, Wine wrappers, installers). PKG decide-personality-packaging is mechanics only. Options: project-owned wrappers with redistributable licenses versus publisher-owned wrappers versus no official repackaging.

<!-- covers: GAP-0463 -->

#### Out of scope
Packaging mechanics (PKG-047). Payload redistribution policy (GOV-053). Store publisher identity (REL-025).

#### Acceptance criteria
- [ ] Options evaluated include project-owned wrappers with redistributable licenses, publisher-owned wrappers, and no official repackaging.
- [ ] The accepted option names who holds copyright on wrapper metadata and who is responsible for the wrapped payload license.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer and PKG lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-050 · Assess export-control and cryptography distribution obligations
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-024
- Baseline: §51, §63
- Decision: D-0096

Public alpha distributes disk encryption and TLS worldwide. This Decision records classification and mirror obligations; filing is a later docs task.

<!-- covers: GAP-0394 -->

#### Out of scope
EAR notice filing (GOV-058). FDE implementation (SEC). Download-site mechanics (REL). Jurisdiction Spike (GOV-039).

#### Acceptance criteria
- [ ] Options evaluated include using the open-source encryption exception with a filed notice, restricting distribution by jurisdiction, and treating mirrors as independently obligated.
- [ ] The accepted option names classification for FDE and TLS releases and who files.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-051 · Decide whether Linux appears in product naming
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-036
- Baseline: §1, §3
- Decision: D-0103
- Invariants: I-003

Linux is a registered trademark. Obtain a Linux Mark Institute sublicense or forbid Linux in product naming before public-alpha marketing.

<!-- covers: GAP-0033 -->

#### Out of scope
Project word-mark filing (GOV-036). Third-party mark rules (GOV-044). Derivative branding (GOV-056).

#### Acceptance criteria
- [ ] Options evaluated include obtaining an LMI sublicense and using Linux in naming, and forbidding Linux in product naming and marketing.
- [ ] The accepted option is reflected in V3 public materials.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-052 · Decide Open Invention Network membership and a patent pledge
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-024
- Baseline: §51, §56.4
- Decision: D-0105

Patent exposure grows with public distribution. OIN membership and a defensive pledge require the V1 legal entity.

<!-- covers: GAP-0080 -->

#### Out of scope
Entity form (GOV-024). Codec patents (GOV-020). Vulnerability reward (GOV-079).

#### Acceptance criteria
- [ ] Options evaluated include OIN membership plus a defensive patent pledge, a pledge without OIN, and neither.
- [ ] The accepted option names the entity as the member or pledgor.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-053 · Decide redistribution policy for third-party Linux and Windows software
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-049
- Baseline: §3, §28, §49
- Decision: D-0107

Flatpak, AppImage, Wine wrappers and whose license applies to the wrapped payload. PKG implements; GOV records the legal policy. Wrapper ownership is the sibling Decision.

<!-- covers: GAP-0463 -->

#### Out of scope
Wrapper ownership (GOV-049). Packaging mechanics (PKG-047). Store DMCA terms (GOV-054).

#### Acceptance criteria
- [ ] Options evaluated include redistributing only redistributable payloads, redistributing with publisher permission, and never redistributing third-party payloads (pointers only).
- [ ] The accepted option names whose license applies to Flatpak, AppImage and Wine-wrapped payloads.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer and PKG lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-054 · Decide Package-repository developer agreement, content and DMCA terms
- Type: adr
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-024, GOV-023
- Baseline: §28, §63
- Decision: D-0108
- Threats: T-006, T-007

Public-alpha repository is an intermediary: developer agreement, content policy, malware removal and DMCA notice. Legal, not REL mechanics.

<!-- covers: GAP-0044 -->

#### Out of scope
Published terms text (GOV-063). Publisher pipeline (REL-021). Publisher identity scheme (REL-025). Developer-program onboarding (GOV-071).

#### Acceptance criteria
- [ ] Options evaluated include a curated store with a signed developer agreement, an open repository with notice-and-takedown, and third-party repos only.
- [ ] The accepted option names content policy, malware-removal authority and DMCA notice and counter-notice.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-055 · Decide opt-in usage telemetry policy apart from crash reporting
- Type: adr
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-030
- Baseline: §24, §63
- Decision: D-0110
- Risks: R-048
- Threats: T-023

V3 opt-in alpha telemetry Gate. What is collected, anonymisation, retention and the user-visible toggle. Implementation is OBS, INS and REL; INS owns the privacy-policy consumption and consent UI.

<!-- covers: GAP-0320, GAP-0043, GAP-0360 -->

#### Out of scope
Privacy policy publication (GOV-061). Consent and redaction UI (INS). On-device counters (OBS-051). Intake (REL-042). Crash-report client (INS-020).

#### Acceptance criteria
- [ ] Options evaluated include no usage telemetry, opt-in counters with a user-visible toggle, and opt-out telemetry (rejected unless the Decision records why).
- [ ] The accepted option names what is collected, anonymisation, retention and that crash reporting is a separate path.
- [ ] Collection without the toggle on is forbidden by the accepted option.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer and OBS lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-056 · Decide trademark usage, derivative branding and compatible-with claims
- Type: adr
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-036, GOV-051, GOV-044
- Baseline: §63
- Decision: D-0111
- Risks: R-082

V3 exit requires a published trademark policy. Derivatives, spins, merchandise, compatible-with claims, remixes and unofficial builds. Third-party marks already handled at V2.

<!-- covers: GAP-0032, GAP-0393 -->

#### Out of scope
Word and logo filing (GOV-036). Third-party mark rules (GOV-044). Linux mark (GOV-051). Downstream distribution policy (GOV-072).

#### Acceptance criteria
- [ ] Options evaluated include a Debian-style mark policy, a Fedora-style policy and a Rust-style policy, each covering derivatives, spins, merchandise and compatible-with claims.
- [ ] The accepted option states how unofficial builds must be branded so users can tell them from official SystemGenerations.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-057 · Stand up community channels with moderation and archives
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-030, GOV-032, GOV-034
- Baseline: §63

V3 scope: forum, chat, announcement feed, mailing lists, moderation and archive guarantees. Code of Conduct already adopted at V1.

<!-- covers: GAP-0071, GAP-0389 -->

#### Out of scope
Code of Conduct (GOV-030). Issue triage policy (GOV-032). RFC venue (GOV-035).

#### Acceptance criteria
- [ ] Forum, chat, announcement feed and mailing list exist and are linked from the public site.
- [ ] A moderation policy names roles and archive guarantees.
- [ ] The Code of Conduct is posted in each channel.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that documents the channels.
- Manual: each named channel is reachable and shows the Code of Conduct.

#### Evidence
- none

### GOV-058 · File the EAR encryption-source notice and classification
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-050
- Baseline: §51, §63

Public binary distribution of cryptography from or through the US. File EAR 742.15(b) and document classification for FDE and TLS releases.

<!-- covers: GAP-0042 -->

#### Out of scope
Export-control Decision (GOV-050). FDE implementation (SEC). Mirror operations (REL).

#### Acceptance criteria
- [ ] Filing Evidence for EAR 742.15(b) is attached as an `https://` Evidence line, or the accepted export Decision records that the notice is not required and why.
- [ ] Classification for FDE and TLS releases is documented next to the filing.
- [ ] REL download-site docs cite this classification.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that attaches filing Evidence or the not-required record.
- Manual: the classification document is reachable from the public download page.

#### Evidence
- none

### GOV-059 · Publish the GPL corresponding-source written-offer policy
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-003
- Baseline: §5.1, §56.4

Workstreams.md GPL boundary. REL publishes corresponding source; GOV states the written-offer policy the installer About view must surface.

<!-- covers: GAP-0374, GAP-0014 -->

#### Out of scope
Corresponding-source publication (REL-046). About and licenses view (INS-037, APP-057). Notices bundle (REL).

#### Acceptance criteria
- [ ] A published policy states how a user obtains corresponding source for every GPL Component of a released SystemGeneration.
- [ ] The policy names the installer About view as the surface that must link the offer.
- [ ] REL-046 is cited as the publisher, not reimplemented here.

#### Verification
- Review: GOV licensing reviewer sign-off recorded on the pull request that publishes the written-offer policy.

#### Evidence
- none

### GOV-060 · Adopt maintainer succession, key escrow and bus-factor plan
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-023, GOV-066
- Baseline: §56.4, §70
- Risks: R-082

Public alpha must survive loss of any maintainer: admin access, key escrow and Workstream continuity.

<!-- covers: GAP-0082 -->

#### Out of scope
Governance charter (GOV-023). Signing-key role list (GOV-066). Ceremony and HSM (REL-032).

#### Acceptance criteria
- [ ] A succession plan names a backup holder for each critical Workstream and for forge admin.
- [ ] Key escrow for root signing shares cites GOV-066.
- [ ] The charter's removal rules point at this plan.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands the succession plan.

#### Evidence
- none

### GOV-061 · Publish the privacy policy for crash reports, HCL submissions and telemetry
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-055
- Baseline: §24, §63
- Risks: R-048, R-085
- Threats: T-023, T-042
- Invariants: I-078

GDPR data-handling design for crash reporting, Hardware Compatibility List submissions and telemetry. GOV owns the policy; INS and REL implement.

<!-- covers: GAP-0043 -->

#### Out of scope
Telemetry Decision (GOV-055). Consent and redaction UI (INS). HCL intake (REL). Crash capture format (OBS).

#### Acceptance criteria
- [ ] A published privacy policy covers crash reports, HCL submissions and usage telemetry as separate processing.
- [ ] The policy names the user-visible telemetry toggle from GOV-055.
- [ ] Community HCL submissions are described so stable hardware identifiers are not ambient (T-042, I-078).

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that publishes the privacy policy.
- Manual: the public site links the policy from crash-report, HCL and telemetry consent surfaces.

#### Evidence
- none

### GOV-062 · Publish the public governance, license and trademark bundle
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-056, GOV-003, GOV-023, GOV-002
- Baseline: §63, §70
- Risks: R-066

V3 exit: governance document, license and trademark policy published. Packages already-accepted V0 through V3 Decisions into the public site.

#### Out of scope
Docs site generator (DOC). Individual Decision files (`decisions/`). 1.0 governance checklist (GOV-080). Platform security guide authorship consumed by DOC-033.

#### Acceptance criteria
- [ ] A public page links the charter, outbound licenses, contribution terms and trademark policy.
- [ ] Each link targets an accepted Decision or a docs task on this prefix, not a draft.
- [ ] The V3 Milestone Gate that requires published governance cites this task.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that publishes the bundle.
- Manual: the public site renders the four linked documents.

#### Evidence
- none

### GOV-063 · Publish Package repository developer agreement and intermediary terms
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-054
- Baseline: §28, §63
- Threats: T-006, T-007

Developer agreement, content policy, malware-removal and DMCA notice and counter-notice are legal intermediary duties. Developer-program onboarding is V4; REL implements the repository those terms govern.

<!-- covers: GAP-0044 -->

#### Out of scope
Repository-terms Decision (GOV-054). Publisher pipeline (REL-021). Developer program (GOV-071).

#### Acceptance criteria
- [ ] Published terms include the developer agreement, content policy, malware-removal authority and DMCA notice and counter-notice.
- [ ] The terms name the legal entity as the intermediary.
- [ ] REL publisher docs link these terms as the legal source.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that publishes the terms.
- Manual: a publisher can retrieve the agreement from the public repository site.

#### Evidence
- none

### GOV-064 · Exercise the RFC process through five external RFCs to Decision
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-035
- Baseline: §65, §66

V3 exit: public RFC process has processed external RFCs to Decision. Gate-verifying, not a new process. The count is the V3 Gate criterion, not restated here as a performance number.

#### Out of scope
Process definition (GOV-005). Templates and venue (GOV-035). 2.0 planning RFC (GOV-082).

#### Acceptance criteria
- [ ] The V3 Gate that requires processed external RFCs lists this task under `Verified by`.
- [ ] Each counted RFC has an accepted or rejected Decision file and a public discussion record.
- [ ] Authors of the counted RFCs are not the GOV Lead.

#### Verification
- Review: GOV maintainer records the counted RFC IDs on the pull request that closes this task.
- Manual: each counted RFC is reachable from the public RFC venue.

#### Evidence
- none

### GOV-065 · Record GOV roles in security-response and advisory sign-off
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-023, GOV-024
- Baseline: §56.4, §63

V3 scope names GOV beside REL and KRN for security response. REL owns the process; GOV names who may sign advisories and speak as the entity.

<!-- covers: GAP-0355, GAP-0239 -->

#### Out of scope
Disclosure policy and advisory format (REL-047). Kernel CVE steps (KRN-031). Advisory feed (REL-044).

#### Acceptance criteria
- [ ] A role list names who may sign advisories and who may speak as the entity during an incident.
- [ ] REL disclosure policy cites this role list.
- [ ] The list uses charter roles, not individuals only.

#### Verification
- Review: GOV maintainer and REL lead sign-off recorded on the pull request that lands the role list.

#### Evidence
- none

### GOV-066 · Define governance roles for root signing-key custody
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-024, GOV-023
- Baseline: §30, §63
- Risks: R-030
- Threats: T-029

REL owns ceremony and HSM; GOV names which governance roles hold quorum shares for Secure Boot and repository signing at public alpha.

<!-- covers: GAP-0045 -->

#### Out of scope
Ceremony, HSM and rotation (REL-032, REL-041). Trust-store layout (REL-010). Succession plan (GOV-060).

#### Acceptance criteria
- [ ] A role list names quorum shares for the offline root and who may authorise a signing or rotation.
- [ ] The legal entity is the holder of the root; individuals hold shares only as named roles.
- [ ] REL-032 is cited as the ceremony owner.

#### Verification
- Review: GOV maintainer and REL lead sign-off recorded on the pull request that lands the role list.

#### Evidence
- none

### GOV-067 · Author the V3 Milestone gates and audience-specific messaging
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-045
- Baseline: §63
- Invariants: I-088

Gate-verifying docs: every §63 item, written for enthusiasts, developers, OS researchers and experimenting gamers.

<!-- covers: INV-1235, INV-1250 -->

#### Out of scope
Install guide (DOC-030). Compatibility guide (DOC-028). V4 criteria the baseline omitted (GOV-076).

#### Acceptance criteria
- [ ] `milestones/V3.md` maps every §63 required item to a Gate or Demo with `Verified by` task IDs.
- [ ] Purpose and messaging name enthusiasts, developers, OS researchers and experimenting gamers as the audience.
- [ ] Gate prose cites B-IDs and C-IDs only (I-088).

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that updates `milestones/V3.md`.
- Unit: `roadmap gate V3` enumerates each Gate and its unsatisfied `Verified by` tasks.

#### Evidence
- none

### GOV-068 · Publish vendor influence and conflict-of-interest policy
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-023, GOV-041
- Baseline: §55, §62, §67

Hardware vendors, game platforms and sponsors at public alpha. Target-hardware selection and gaming partnerships are capture risks.

<!-- covers: GAP-0067 -->

#### Out of scope
NDA read-versus-reproduce policy (GOV-046). Funding model (GOV-041). Hardware Compatibility List (REL).

#### Acceptance criteria
- [ ] A published policy covers hardware vendors, game platforms and sponsors.
- [ ] Target-hardware selection and gaming partnerships require a recorded conflict disclosure.
- [ ] The charter's conflict-resolution rules point at this policy.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that publishes the policy.

#### Evidence
- none

### GOV-069 · Record the V3 exit review
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: INS-008, INS-007, REL-004, OBS-029, GOV-055, BOOT-031, REL-028, SEC-042
- Baseline: §57, §59, §65

Record the V3 exit review: V3 public-alpha Decisions (installer disk, encryption default, update channels, crash capture, telemetry, Secure Boot, repository curation, multi-user scope). Layer 1 surfaces remain unfrozen.

#### Out of scope
Per-Gate verification (the Gate `Verified by` tasks). Generated roll-up (GOV roadmap tooling).

#### Acceptance criteria
- [ ] Every Decision named in Depends on is accepted.
- [ ] The V3 Milestone file records this exit review and the generated roll-up shows every V3 task `done` or `dropped` with reason.
- [ ] Layer 1 surfaces remain unfrozen.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands the exit review.

#### Evidence
- none

### GOV-070 · Decide open-source-steward versus manufacturer status under CRA
- Type: adr
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-024
- Baseline: §51, §63
- Decision: D-0093

LIC fold sends CRA and EAA policy to GOV. This Decision picks steward versus manufacturer before 1.0. EAA conformity stays ACC.

<!-- covers: GAP-0078 -->

#### Out of scope
EAA checklist (ACC-028). Entity form (GOV-024). Technical audit (SEC-070).

#### Acceptance criteria
- [ ] Options evaluated include open-source-steward status and manufacturer status under the CRA.
- [ ] The accepted option names the entity as the responsible party and what evidence 1.0 must publish.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer and ACC lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-071 · Publish the developer program and third-party onboarding
- Type: docs
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-063, GOV-023
- Baseline: §56.5, §66

V4 scope: developer program and external native Packages. SDK owns bindings; GOV owns the programme terms and publisher identity. Thresholds live in the V4 Milestone file.

#### Out of scope
SDK bindings (SDK). Intermediary legal terms (GOV-063). Ecosystem metric publication (GOV-073). Publisher pipeline (REL-021).

#### Acceptance criteria
- [ ] Programme terms name how a third party publishes a native Package and which identity scheme REL uses.
- [ ] Onboarding docs link the repository developer agreement.
- [ ] The V4 ecosystem Gate cites this task or GOV-073 under `Verified by`.

#### Verification
- Review: GOV maintainer and SDK lead sign-off recorded on the pull request that publishes the programme terms.

#### Evidence
- none

### GOV-072 · Publish downstream and derivative-distribution policy
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-056
- Baseline: §30, §63

Beta-stage spins: rebranding, unofficial builds and use of the official package repository. Follows the V3 trademark Decision.

<!-- covers: GAP-0075 -->

#### Out of scope
Trademark Decision (GOV-056). Official repository mechanics (REL). SystemGeneration composition (PKG).

#### Acceptance criteria
- [ ] A published policy covers rebranding requirements, unofficial builds and use of the official package repository.
- [ ] The policy cites GOV-056 for mark use.
- [ ] Users can tell official SystemGenerations from forks by the rules in this policy.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that publishes the policy.

#### Evidence
- none

### GOV-073 · Track external-contributor and native-Package ecosystem metrics
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-071
- Baseline: §56.5
- Risks: R-059

V4 exit thresholds for external Packages and contributors need a published GOV metric, not only REL store counts. Thresholds live in the V4 Milestone file and are not restated here.

#### Out of scope
Store counts (REL). Developer-program terms (GOV-071). SDK bindings (SDK).

#### Acceptance criteria
- [ ] A published metric defines how external native Packages and external contributors are counted.
- [ ] The V4 ecosystem Gate cites this metric rather than restating counts in Gate prose.
- [ ] REL store counts are named as an input, not the definition.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that publishes the metric definition.
- Manual: the V4 Gate `Verified by` list includes this task.

#### Evidence
- none

### GOV-074 · Publish feature-freeze and RC soak governance for the beta
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-007, GOV-076
- Baseline: §65, §66

V4 exit: feature freeze at RC1, RC soak cycles, P0 restarts the cycle. Extends V0 Gate governance to freeze and soak rules.

#### Out of scope
RC soak matrix jobs (BLD-076). Layer 1 freeze Decision (ABI-049). Support-policy drafts (GOV-075).

#### Acceptance criteria
- [ ] A published rule states that after RC1 no new features enter the 1.0 set without a Gate re-scope.
- [ ] A P0 during soak restarts the soak cycle; the rule cites the V4 Milestone, not a calendar date.
- [ ] REL qualification docs link this rule.

#### Verification
- Review: GOV maintainer and REL lead sign-off recorded on the pull request that publishes the freeze and soak rules.

#### Evidence
- none

### GOV-075 · Draft support window, CVE SLA, HCL and ABI-stability policy
- Type: docs
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-076, ABI-050
- Baseline: §56.4, §63, §66
- Risks: R-061, R-062

V4 exit: support window, CVE SLA, HCL tiers and ABI stability statement drafted, RFC-reviewed and accepted. REL owns lifecycle mechanics; ABI drafts the stability statement; GOV owns the published contract drafts. KRN-055 consumes this draft.

#### Out of scope
1.0 stability contract (GOV-083). ABI statement authorship (ABI-050). Kernel LTS base (KRN-055). CVE SLA register entry (REL-060).

#### Acceptance criteria
- [ ] Drafts exist for support window, CVE SLA, HCL tiers and ABI stability, each RFC-reviewed.
- [ ] Drafts cite B-IDs, C-IDs and register targets rather than restating numbers.
- [ ] ABI-050 is the Layer 1 statement source.

#### Verification
- Review: GOV maintainer, ABI lead and REL lead sign-off recorded on the pull request that lands the drafts.

#### Evidence
- none

### GOV-076 · Define V4 beta and 1.0 stable criteria the baseline omitted
- Type: docs
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-067, Q-046
- Baseline: §63, §66, §70
- Invariants: I-088

§63 left V4 and 1.0 unspecified. Write SLOs, audit, ABI freeze, support commitment, hardware list and compatibility pass rates into the V4 and 1.0 Milestone files, citing B-IDs and C-IDs only. Answers Q-046 and Q-003.

<!-- covers: INV-1251, INV-0155 -->

#### Out of scope
Support-policy drafts (GOV-075). Layer 1 freeze Decision (ABI-049). External audit contract (SEC-070). Phase D entry (KRN-042).

#### Acceptance criteria
- [ ] `milestones/V4.md` and `milestones/1.0.md` each have Gates covering stability, audit, ABI freeze, support commitment, hardware list and compatibility, all citing B-IDs or C-IDs (I-088).
- [ ] Q-046 is marked answered by this task.
- [ ] Q-003 is answered: the 1.0 Gate states which of Phase C, D or E is required, or records that none is a hard requirement.
- [ ] No Gate prose contains a calendar date.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that updates the V4 and 1.0 Milestone files.
- Unit: `roadmap gate V4` and `roadmap gate 1.0` enumerate Gates and unsatisfied `Verified by` tasks.

#### Evidence
- none

### GOV-077 · Record the V4 exit review
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-049, GOV-074, GOV-075
- Baseline: §57, §59, §65

Record the V4 exit review: V4 beta Decisions: Layer 1 freeze ADR, feature-freeze governance and support-policy drafts named by the V4 Gates. Layer 1 freeze is in scope for this review; the freeze ADR is accepted before the review records.

#### Out of scope
Per-Gate verification (the Gate `Verified by` tasks). Generated roll-up (GOV roadmap tooling).

#### Acceptance criteria
- [ ] Every Decision named in Depends on is accepted.
- [ ] The V4 Milestone file records this exit review and the generated roll-up shows every V4 task `done` or `dropped` with reason.
- [ ] Layer 1 freeze is in scope for this review; the freeze ADR is accepted before the review records.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands the exit review.

#### Evidence
- none

### GOV-078 · Decide OEM partnerships or hardware certification versus later
- Type: adr
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-024, Q-055
- Baseline: §62, §70
- Decision: D-0104

A certified machine is the strongest adoption vector and a scarce-engineering commitment. This Decision answers whether OEM partnerships or a hardware certification programme precede 1.0 or park to post-1.0.

<!-- covers: GAP-0472 -->

#### Out of scope
Hardware Compatibility List publication (REL-048). Hardware bring-up (HW). Entity form (GOV-024).

#### Acceptance criteria
- [ ] Options evaluated include OEM partnerships before 1.0, a project-run certification programme before 1.0, and parking both to post-1.0.
- [ ] Q-055 is marked answered by this task.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer and HW lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-079 · Decide whether to run a vulnerability reward program
- Type: adr
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-041, GOV-024
- Baseline: §51, §63
- Decision: D-0113

1.0 funding and triage-capacity Decision. SEC designs bounty-or-not; this confirms funding at the stable declaration.

<!-- covers: GAP-0359, GAP-0047 -->

#### Out of scope
Security-response process (REL-047). Technical bounty design (SEC). Funding model (GOV-041).

#### Acceptance criteria
- [ ] Options evaluated include running a funded reward program at 1.0, deferring it past 1.0, and never running one.
- [ ] The accepted option names funding source and triage capacity, or records that no program runs.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV maintainer and SEC lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### GOV-080 · Verify published governance, license, trademark, RFC and security
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-062, GOV-064, GOV-061, GOV-083
- Baseline: §63, §66, §70
- Risks: R-066

1.0 exit: license, trademark, contribution, RFC and security policies published. Gate-verifying checklist over earlier GOV deliverables.

#### Out of scope
Authoring those policies (earlier GOV tasks). Docs site snapshot (DOC-040). Security-posture claims (SEC-076).

#### Acceptance criteria
- [ ] A checklist records that license, trademark, contribution, RFC and security policies are published and linked from the 1.0 site.
- [ ] Each checklist row cites a done GOV task ID, not prose.
- [ ] The 1.0 Gate that requires published governance lists this task under `Verified by`.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that attaches the checklist.
- Manual: each checklist URL returns the published policy.

#### Evidence
- none

### GOV-081 · Publish 1.x maintenance-branch and backport governance
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-075, GOV-083, KRN-058
- Baseline: §56.4, §66
- Risks: R-061

1.0 scope: 1.x maintenance branch and backport policy (GOV, KRN). KRN creates the kernel branch; GOV states who may backport and how ABI freeze is preserved.

#### Out of scope
Kernel branch creation (KRN-058). Support-window contract (GOV-083). CVE pipeline (KRN, REL).

#### Acceptance criteria
- [ ] A published policy names who may backport to the 1.x branch and that Layer 1 remains frozen.
- [ ] The policy cites KRN-058 as the branch owner.
- [ ] ABI-breaking backports are forbidden unless a new major version Decision exists.

#### Verification
- Review: GOV maintainer and KRN lead sign-off recorded on the pull request that publishes the backport policy.

#### Evidence
- none

### GOV-082 · Open the 2.0 planning RFC listing deferred LATER items
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-029, GOV-076, GOV-035
- Baseline: §38, §57, §66, §70

1.0 exit: 2.0 planning RFC opened listing explicitly deferred items (ARM64, native filesystem and GPU, distributed interfaces, casting). Cites LATER via See also without depending.

#### Out of scope
See-also field implementation (GOV-029). Performing any LATER task. Fossilization review (ABI-054).

#### Acceptance criteria
- [ ] An RFC exists in the public venue listing deferred LATER items including ARM64, native filesystem, native GPU stack, distributed interfaces and casting.
- [ ] The RFC cites those tasks with See also and does not list them in `Depends on`.
- [ ] Native software is still described as never seeing POSIX, Linux syscalls or Win32.

#### Verification
- Review: GOV maintainer and ABI lead sign-off recorded on the pull request that opens the RFC.
- Unit: `roadmap check` on the RFC's tracking task does not report a LATER dependency.

#### Evidence
- none

### GOV-083 · Publish the 1.0 stability contract, support lifecycle and UX bar
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-075, GOV-076
- Baseline: §49, §66, §70
- Risks: R-061
- Invariants: I-040, I-096

The 1.0 promise: frozen ABIs, breaking-change definition, support lifecycle, CVE SLA, each §70 pillar mapped to a Workstream with exit criteria, and compatibility UX proving Linux and Windows software feel native (§49). Thresholds stay in registers and Milestone Gates.

<!-- covers: GAP-0060, INV-1335, INV-1336 -->

#### Out of scope
Support-policy drafts (GOV-075). Layer 1 freeze Decision (ABI-049). Compatibility implementation (LNX, WIN). 1.x backport rules (GOV-081).

#### Acceptance criteria
- [ ] A published contract names which ABIs and Interfaces are frozen, what constitutes a breaking change, and that Layer 1 changes require a new major version.
- [ ] Each §70 pillar maps to a Workstream with a 1.0 Gate citation.
- [ ] Compatibility UX criteria cite C-IDs for Linux and Windows software feeling like native citizens (§49, I-096).
- [ ] Support window and CVE SLA cite register targets, not restated numbers.

#### Verification
- Review: GOV maintainer, ABI lead and REL lead sign-off recorded on the pull request that publishes the contract.
- Manual: the 1.0 site links the contract from the download and SDK landing pages.

#### Evidence
- none

### GOV-084 · Record the 1.0 exit review
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-053, GOV-080, GOV-083
- Baseline: §57, §59, §65

Record the 1.0 exit review: 1.0 public-stable Decisions: the 1.x stability declaration, governance gate and published stability contract. The stability contract matches the accepted 1.x declaration.

#### Out of scope
Per-Gate verification (the Gate `Verified by` tasks). Generated roll-up (GOV roadmap tooling).

#### Acceptance criteria
- [ ] Every Decision named in Depends on is accepted.
- [ ] The 1.0 Milestone file records this exit review and the generated roll-up shows every 1.0 task `done` or `dropped` with reason.
- [ ] The stability contract matches the accepted 1.x declaration.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request that lands the exit review.

#### Evidence
- none
