# DOC · Documentation
- Prefix: DOC
- Lead: none
- Baseline: §3, §12, §52, §54, §56.5, §58, §61, §63, §64, §66
- Baseline gap: §56.5 names documentation as an ecosystem need and §63 requires it at public alpha, but specifies no taxonomy, IDL-to-docs generator, versioned site, offline os help or research-study catalog.

<!-- roadmap:generated:begin summary -->
Tasks: 42 live, 0 done, 0 in-progress, 42 todo, 0 dropped. Ready: 0. Blocked: 42. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

DOC owns the product documentation pipeline: the IDL-to-docs generator, the versioned public site with full-text search and per-release snapshots, offline `os help` content, man-page equivalent pages for the Linux personality, and the research-study catalog under §58. It publishes references and guides (user, administrator, developer, ABI, architecture and Decision record, compatibility, security, install, migration, contribution) so strangers can install and developers can target SDK v1 without a private briefing.

Domain workstreams author normative prose for their objects and Interfaces. DOC generates pages from IDL and SDK sources, applies one style and glossary, packages the corpus into the SystemGeneration, gates coverage in CI, and runs the documentation translation pipeline. Glossary stewardship for the product site lives here. Native software never treats POSIX man pages, Win32 help or a web round-trip as the native help Interface.

## Out of scope

IDL compiler and doc-comment IR (IPC). Layer 1 and Layer 2 semantics authorship (ABI, CAP, CMP, MEM, TSK). `os` CLI binary and `os help` command (SDK). Content-addressed store and Package format (PKG). Hosting, CDN, channel notes publication and HCL (REL). CI platform and merge queue (BLD). BASELINE.md, PRINCIPLES.md, RFC and ADR process, repo CONTRIBUTING, documentation license, trademark wording and research-programme citation index (GOV). UI and OS string catalogs (TXT). Installer, recovery environment and unaided-install study execution (INS). Linux personality viewer, corpus ratings and Linux compatibility-guide chapters (LNX). Windows personality chapters and unsupported-title matrix (WIN). Threat model document (SEC). Storage chapter authorship (STO). Claim-to-benchmark lint (BEN). In-app chrome (APP). VM fallback product (VIRT).

## Tasks

### DOC-001 · Publish the research study catalog and template
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-005
- Baseline: §58, §68

§58 requires comparative studies of named prior systems. This catalog is the template and index other workstreams' studies link from: citation fields, what was borrowed, what was rejected, which prefix consumes the findings, and the §68 litmus questions. GOV keeps process, kill criteria and the ADR citation index.

<!-- covers: INV-1151 -->

#### Out of scope
Research-programme process and ADR citation index (GOV-005, GOV-015). Individual studies (DOC-003, DOC-004, DOC-005).

#### Acceptance criteria
- [ ] `research/README.md` lists every §58 inspiration with status empty, in-progress or published, and a path to the study file or a named owning prefix.
- [ ] `research/TEMPLATE.md` requires prior-art citations, borrowed mechanisms, rejected semantics, consuming workstream and the §68 litmus answers.
- [ ] Three published studies in this workstream use the template headings without extra top-level sections.
- [ ] The catalog states that GOV ADR templates cite studies by path, not the reverse.

#### Verification
- Review: GOV process lead confirms the template matches GOV-005 on the pull request.
- Manual: `research/README.md` and `research/TEMPLATE.md` exist and the three DOC studies link from the index.

#### Evidence
- none

### DOC-002 · Prototype IDL-to-docs Generation from the V0 IDL
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-012, ABI-017
- Baseline: §12, §52, §56.5
- Risks: R-052

V1 must ship IDL-to-docs with SDK v1 or V3 Layer 1 reference pages are handwritten (R-052). This spike takes the V0 IDL compiler output and a page template, and records IR shape, missing-comment behavior and which toolchain candidates are ruled out so the V1 generator is not invented on the freeze-candidate path. Required by V1-G12 (Semantic interfaces and a Wasm channel prototype): the IDL-to-docs generator that gate names is built on this spike's IR findings.

#### Out of scope
V1 generator (DOC-010). Toolchain Decision (DOC-009). Doc-comment IR in the compiler (IPC-049). Normative ABI prose (ABI-017).

#### Acceptance criteria
- [ ] A prototype consumes V0 IDL and emits one HTML or Markdown page per Interface, including methods with and without doc comments.
- [ ] The report records missing-comment behavior as fail, warn-and-stub or skip, with one option ruled out.
- [ ] The report names at least two toolchain candidates and rules one out for V1.
- [ ] `reports/spikes/DOC-002.md` exists with the spike skeleton headings.

#### Verification
- Report: What IR carries doc comments, deprecation and semantic metadata; what happens when a method has no comment; which page templates cover Interface, method, type and error; what is generated versus authored by domain workstreams; which toolchain candidates are ruled out for V1.
- Unit: prototype fixture on `qemu-x86_64` emitting pages for one V0 Interface with a missing-comment method.
- Review: IPC lead confirms the prototype reads compiler IR rather than parsing IDL text ad hoc.

#### Evidence
- none

### DOC-003 · Write the Android permissions, Binder and A/B updates study
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-001
- Baseline: §9, §14, §30, §58

Android is a §58 inspiration whose permissions, Binder IPC, app sandbox and A/B updates inform SEC, IPC and PKG, not a V0 kernel surface. The study uses the catalog template and states what is borrowed and what is rejected so later ADRs can cite a path.

<!-- covers: INV-1139 -->

#### Out of scope
Threat register (SEC-002). Package and SystemGeneration immutability (PKG). Binder-shaped native IPC (IPC). Research-programme index (GOV-015).

#### Acceptance criteria
- [ ] `research/android-permissions-binder-ab.md` uses `research/TEMPLATE.md` headings.
- [ ] The study records Binder versus Channel, install-time permissions versus Capability grants, and A/B slots versus SystemGeneration, each as borrow or reject.
- [ ] The consuming prefixes named are SEC, IPC and PKG, with no V0 kernel surface claimed.
- [ ] The catalog row for Android points at this file.

#### Verification
- Review: SEC and PKG leads sign off on the pull request that the borrow and reject list does not invent a native Binder.
- Manual: the catalog index links the study and the template headings are present.

#### Evidence
- none

### DOC-004 · Write the Redox Rust-in-kernel and schemes study
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-001
- Baseline: §5, §14, §50, §51, §58

Redox is a §58 inspiration for Rust-in-kernel practice and scheme-based resources. The study does not inform a V0 kernel surface; it feeds KRN language policy and IPC resource naming so later ADRs can cite prior art without rediscovering it.

<!-- covers: INV-1135 -->

#### Out of scope
Kernel fork and retained-mechanism inventory (KRN). Channel and IDL (IPC). Research-programme process (GOV).

#### Acceptance criteria
- [ ] `research/redox-rust-schemes.md` uses `research/TEMPLATE.md` headings.
- [ ] The study records Rust-in-kernel practices as borrow or reject against §50 and §51, and schemes versus Channel against §14.
- [ ] The consuming prefixes named are KRN and IPC, with no V0 syscall claimed.
- [ ] The catalog row for Redox points at this file.

#### Verification
- Review: KRN lead sign-off recorded on the pull request.
- Manual: the catalog index links the study and the template headings are present.

#### Evidence
- none

### DOC-005 · Write the XNU Mach ports, launchd and entitlements study
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-001
- Baseline: §9, §14, §32, §58

XNU Mach ports, launchd activation and the app-sandbox entitlements model are §58 inspirations for SVC supervision and SEC sandbox, not the kernel fork. The study lands at V0.5 beside native init so supervision ADRs can cite it.

<!-- covers: INV-1138 -->

#### Out of scope
Native init and supervisor (SVC). Capability rights and sandbox (SEC, CAP). Mach-shaped native IPC (IPC). Kernel fork (KRN).

#### Acceptance criteria
- [ ] `research/xnu-mach-launchd-entitlements.md` uses `research/TEMPLATE.md` headings.
- [ ] The study records Mach ports versus Channel, launchd versus native init supervision, and entitlements versus Capability grants, each as borrow or reject.
- [ ] The consuming prefixes named are SVC, SEC and IPC, with no V0 kernel surface claimed.
- [ ] The catalog row for macOS/XNU points at this file.

#### Verification
- Review: SVC and SEC leads sign off on the pull request.
- Manual: the catalog index links the study and the template headings are present.

#### Evidence
- none

### DOC-006 · Add full-text search to the documentation site
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-009, DOC-007, DOC-010
- Baseline: §56.5, §61

GAP-0380 requires full-text search on the versioned site so SDK v1 consumers can find generated Interface pages without reading the tree. Search is a sibling of the site build and uses the toolchain Decision.

<!-- covers: GAP-0380 -->

#### Out of scope
Site and snapshot trees (DOC-007). Hosting and CDN (REL). Toolchain Decision (DOC-009).

#### Acceptance criteria
- [ ] A query for a generated Interface name from DOC-010 returns that page as a hit on the current snapshot.
- [ ] Search is available on every per-release snapshot the site serves, not only on `latest`.
- [ ] A query that matches no page returns an empty-result page, not an error.
- [ ] The implementation matches the search option accepted in DOC-009.

#### Verification
- Integration: `docs:tests/search_*` against a fixture site containing two snapshots and one known Interface name.
- Review: DOC lead confirms the search backend is the accepted toolchain option on the pull request.

#### Evidence
- none

### DOC-007 · Build the versioned documentation site with per-release snapshots
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-008, DOC-009, GOV-021
- Baseline: §56.5, §61, §66

Developer preview means external developers. The site serves the taxonomy's classes as versioned trees so SDK v1 pages stay pinned when the next snapshot is published. Search is a sibling task. Hosting remains REL.

<!-- covers: GAP-0380 -->

#### Out of scope
Full-text search (DOC-006). IDL page generator (DOC-010). Hosting, CDN and mirrors (REL-024, REL-020). Documentation license (GOV-021).

#### Acceptance criteria
- [ ] Publishing a snapshot writes a versioned URL tree that remains readable after a later snapshot is published.
- [ ] The site root lists snapshots and the current preview, and each snapshot shows the SystemGeneration or SDK version it was built from.
- [ ] Taxonomy classes from DOC-008 appear as top-level navigation on every snapshot.
- [ ] Every page shows the license chosen by GOV-021.
- [ ] Native crates do not fetch the site at runtime; the site is a publication, not a help Interface.

#### Verification
- Integration: `docs:tests/site_snapshots_*` builds two consecutive snapshots and fetches a page from the first after the second exists.
- Review: GOV licensing lead confirms the license notice matches GOV-021.
- Manual: navigation lists every taxonomy class named in the Decision.

#### Evidence
- none

### DOC-008 · Decide the documentation taxonomy and ownership
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-006
- Baseline: none
- Decision: D-0070

BASELINE.md has no documentation section. This first adr records the documentation classes and owners §56.5 and §63 left unnamed, so the V1 site and later guides are not a single undifferentiated pile. GOV keeps BASELINE.md, PRINCIPLES.md and the living architecture map; this adr assigns the product-site classes.

<!-- covers: GAP-0381 -->

#### Out of scope
Toolchain, search and snapshots (DOC-009). License (GOV-021). Charter and architecture map (GOV-006). Repo CONTRIBUTING (GOV-007).

#### Acceptance criteria
- [ ] Option A (class-based taxonomy with a named owner per class: user, administrator, developer, ABI specification, architecture and Decision record, compatibility) and Option B (audience-first trees for end user, developer and researcher with mixed class ownership) are evaluated with consequences.
- [ ] The accepted option names every class, its owner prefix or task, and what DOC generates versus what the owner authors.
- [ ] The accepted option states that POSIX, Win32, man pages and Wayland help are personality or foreign materials, never native API docs.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with GOV and SDK leads named.
- Manual: the decision file lists at least two options, rejected options and an owner per accepted class.

#### Evidence
- none

### DOC-009 · Decide the documentation toolchain, search and snapshots
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-002
- Baseline: §12, §52, §56.5
- Decision: D-0071
- Risks: R-052

The V1 IDL-to-docs site needs a recorded generator, site builder, full-text search and per-release snapshot scheme before those builds start. Information architecture is the taxonomy adr. The spike report is an input so the generator is not chosen by fashion (R-052). Required by V1-G12 (Semantic interfaces and a Wasm channel prototype): the IDL-to-docs generator that gate names runs on the toolchain this Decision picks.

#### Out of scope
Taxonomy and class owners (DOC-008). Site implementation (DOC-007). Generator implementation (DOC-010). Hosting (REL).

#### Acceptance criteria
- [ ] Option A (static site generator plus a custom IDL-to-pages compiler, client-side search, versioned snapshot trees) and Option B (unified rustdoc or Sphinx pipeline, server-side search, tagged snapshot aliases) are evaluated against `reports/spikes/DOC-002.md`.
- [ ] The accepted option names the generator crate or tool, the site builder, the search implementation and the snapshot URL scheme.
- [ ] The accepted option states missing-comment behavior for generated pages, matching or superseding the spike.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with IPC and SDK leads named.
- Manual: the decision file lists at least two options, cites the spike report and names search and snapshot scheme.

#### Evidence
- none

### DOC-010 · Generate API reference pages from the IDL and SDK
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-008, DOC-009, DOC-002, IPC-049, ABI-023, ABI-038
- Baseline: §12, §52, §56.5, §66
- Risks: R-052
- Invariants: I-040

V1-G12 requires IDL-to-docs so later rungs are not writing reference pages by hand (R-052). Pages are generated from IDL (via IPC doc-comment IR) and SDK sources so they cannot drift from Interfaces. Domain workstreams author normative prose; this task owns the generator. It does not freeze Layer 1 (I-040).

<!-- covers: GAP-0380 -->

#### Out of scope
Doc-comment IR (IPC-049). Layer 1 semantics (ABI-046, ABI-038). Complete V3 coverage gate (DOC-023, DOC-024). Package format reference (PKG-061).

#### Acceptance criteria
- [ ] The generator emits one page per IDL Interface and per public SDK crate item the toolchain Decision places in scope, including deprecation metadata from IR.
- [ ] Missing doc comments are handled exactly as DOC-009 records (fail, warn-and-stub or skip), verified by a fixture Interface with an uncommented method.
- [ ] Regenerating after an IDL comment change updates the page text without a hand edit.
- [ ] Generated pages are listed under the ABI specification or developer class named by DOC-008.
- [ ] No Layer 1 surface is marked frozen by this generator.

#### Verification
- Unit: `docs:tests/idl_generate_*` on `qemu-x86_64` with a fixture IDL file and a missing-comment method.
- Integration: CI job rebuilds pages from the V1 ABI specification and SDK crate and diffs against the committed snapshot.
- Review: IPC lead confirms the generator reads IR, not IDL source text.

#### Evidence
- none

### DOC-011 · Publish architecture, Decision and glossary records
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-008, DOC-007, GOV-006
- Baseline: §56.5, §65, §66, §69

The taxonomy includes an architecture and Decision record. This task publishes Decision and architecture indexes and the product glossary on the V1 site so SDK v1 readers can find accepted Decisions without cloning the roadmap repo. GOV keeps BASELINE.md, PRINCIPLES.md and the living §69 diagram.

#### Out of scope
BASELINE.md, PRINCIPLES.md and the §69 diagram (GOV-006). Decision file format (GOV-005). Site builder (DOC-007).

#### Acceptance criteria
- [ ] The site has an architecture index that links BASELINE.md sections and the GOV architecture map, and a Decision index that lists every accepted Decision by ID and title.
- [ ] The product glossary on the site uses GLOSSARY.md spellings for Component, Capability, Channel, Interface, MemoryObject, Package, SystemGeneration and Personality.
- [ ] A rejected or proposed Decision is labeled as such and is not presented as accepted.
- [ ] The taxonomy class for this record matches DOC-008.

#### Verification
- Review: GOV lead confirms the site does not fork BASELINE.md prose on the pull request.
- Manual: Decision index and glossary pages exist on the current site snapshot.
- Integration: a fixture proposed Decision is rendered with status proposed, not accepted.

#### Evidence
- none

### DOC-012 · Wire documentation Generation into CI
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-010, DOC-007, BLD-001
- Baseline: §56.5, §61
- Risks: R-052

Generated pages must not drift: CI builds them and fails on generator errors. GOV GAP-0164 remains roadmap and ADR parse CI at V0; this is the product-docs pipeline that keeps R-052 from silently accumulating. Required by V3-G12 (Layer 1 ABI reference pages exist for every entry point): the documentation build in CI that gate requires is this job, which DOC-024 extends with the coverage gate.

#### Out of scope
Roadmap and ADR parse CI (GOV-013). Coverage gate for every Layer 1 symbol (DOC-024). Broken-link and example compilation (DOC-034). Merge queue (BLD-001).

#### Acceptance criteria
- [ ] A post-merge CI job builds generated pages and the site; a generator panic or non-zero exit fails the job.
- [ ] A fixture that breaks IDL generation fails the job with the Interface name in the log.
- [ ] The job is distinct from GOV roadmap parse CI and does not parse workstream Markdown.
- [ ] Passing the job is required before a docs snapshot is published.

#### Verification
- Integration: `docs:tests/ci_generate_*` on the CI matrix entry that runs documentation jobs, including a failing fixture.
- Review: BLD lead confirms the job sits on the merge queue without replacing GOV parse CI.

#### Evidence
- none

### DOC-013 · Write the os CLI reference from command metadata
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-015, SDK-006, SDK-035, SDK-007, SDK-008, SDK-019, SDK-044, SDK-041
- Baseline: §64, §61

§64 first-class tooling (`os inspect`, `os trace`, `os history`, `os restore`, `os env`) is the daily-driving surface at V1. SDK owns the CLI and command metadata; DOC publishes the reference that V2 `os help` snapshots.

#### Out of scope
`os` binary and subcommands (SDK-006 and siblings). Offline `os help` (DOC-020, SDK-067). Inspect data plane (OBS).

#### Acceptance criteria
- [ ] Every `os` subcommand shipped at V1 (`inspect`, `trace`, `history`, `restore`, `env`) has a reference page generated or copied from command metadata, including flags and the JSON output shape where SDK-035 provides it.
- [ ] The pages use glossary casing and cite no performance number.
- [ ] A new flag in command metadata appears on the page after regeneration without a hand edit of the prose file.
- [ ] Native software is documented as calling typed Interfaces, not as shelling out to `os` for authority.

#### Verification
- Review: SDK CLI lead sign-off recorded on the pull request.
- Integration: metadata fixture with a new flag rebuilds the page containing that flag.
- Manual: the five V1 subcommands are listed in site navigation.

#### Evidence
- none

### DOC-014 · Write the developer guide for the native SDK
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-015, DOC-017, CMP-037
- Baseline: §10, §11, §52, §61, §66
- Invariants: I-006, I-007

V1 ships SDK v1 to external developers. This is the first-cut Rust developer guide on the docs site: Component graphs, Capabilities, Operations, Packages and the chooser, linking the getting-started tutorial and the POSIX/Win32 map. Domain chapters (UIP, CMP, SCH, SVC, PKG) are authored there and published through this pipeline. V3 completes and reviews it.

<!-- covers: INV-0978, GAP-0455, INV-1110 -->

#### Out of scope
Crate-level SDK guide (SDK-056). Reference tutorial sample (SDK-062). Component design guidelines authorship (CMP-037). UI protocol guide authorship (UIP-031). V3 completion (DOC-032). Accessibility developer guidelines authorship (ACC-010).

#### Acceptance criteria
- [ ] The guide has chapters covering Component, Capability, Operation, Package, UserSelected chooser and the getting-started path, each linking generated IDL or SDK pages.
- [ ] The POSIX/Win32 map is linked from the porting chapter and the chapter states those APIs exist only inside Personalities (I-006, I-007).
- [ ] Every code sample builds against SDK v1 as published by SDK-059.
- [ ] No chapter claims a performance number; any comparison cites a B-ID.
- [ ] Style-guide glossary casing holds for every glossary term in headings.

#### Verification
- Review: SDK and CMP leads sign off on the pull request.
- Integration: listed samples compile in the SDK v1 CI job.
- Manual: porting chapter links DOC-017.

#### Evidence
- none

### DOC-015 · Write the documentation style guide
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-008
- Baseline: §54, §57
- Invariants: I-050, I-061

The V1 site and later translation need one voice, glossary casing and the no-unmeasured-claim rule (I-061, I-050) before third-party contributions. Later docs CI enforces the mechanical subset. Required by V3-G12 (Layer 1 ABI reference pages exist for every entry point): the guide review that gate requires is conducted against this style guide.

#### Out of scope
Claim-to-benchmark lint implementation (BEN-004, GOV-019). Translation pipeline (DOC-025). Taxonomy Decision (DOC-008).

#### Acceptance criteria
- [ ] The style guide requires GLOSSARY.md spellings, American spelling, no em-dashes, and no performance number unless a B-ID and harness report are cited (I-061, I-050).
- [ ] The style guide maps each taxonomy class to audience, tense and whether generated pages may be hand-edited.
- [ ] A worked bad example (superiority claim without a B-ID) is shown as rejected.
- [ ] The guide is itself published on the V1 site under the developer or architecture class.

#### Verification
- Review: DOC and BEN leads sign off that I-061 wording matches the invariant text.
- Manual: the style guide is reachable from site navigation.

#### Evidence
- none

### DOC-016 · Write the getting-started tutorial for a first Component
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-015, SDK-043
- Baseline: §10, §52, §61

V1 onboarding is clone-to-running Component: `os new`, the published SDK, and the window / chooser / decode / render sample. SDK owns the sample and generator; DOC publishes the tutorial used by daily-driving.

<!-- covers: INV-0978 -->

#### Out of scope
`os new` scaffolding (SDK-043). Sample source (SDK-062). Host SDK (SDK-039). Developer image install (INS-003).

#### Acceptance criteria
- [ ] The tutorial lists a closed sequence from installing the host SDK to a running Component that opens a window and renders a user-chosen image via UserSelected.
- [ ] Every command in the sequence is a published `os` or SDK command, not an unpublished script.
- [ ] The tutorial states the SDK is Layer 3 and does not present POSIX file APIs as native.
- [ ] Following the tutorial on the host SDK produces the sample binary that SDK-062 defines.

#### Verification
- Review: SDK lead sign-off recorded on the pull request.
- Integration: the tutorial's command list is executed in the SDK onboarding job and the sample starts under QEMU.
- Manual: the page is linked from the developer-guide landing page.

#### Evidence
- none

### DOC-017 · Write the POSIX and Win32 to native concept map
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-015, ABI-041
- Baseline: §3, §7, §10, §21, §46, §48
- Invariants: I-006, I-007

Every developer arriving has a Unix or Windows mental model. This map records file descriptors to Capabilities, fork to Components, signals to cancellation, sockets to Channels, and Win32 handles to Capabilities, as a concept map only. Native software never sees POSIX or Win32 (I-006, I-007). V3 migration guides expand it.

<!-- covers: GAP-0455 -->

#### Out of scope
Personality implementations (LNX, WIN). Object mapping table authorship (ABI-041). V3 migration guides (DOC-031). Worked ports of real applications beyond the map (SDK).

#### Acceptance criteria
- [ ] A table maps POSIX process, file descriptor, path, signal, socket, thread and environment variable to native Component, Capability, UserSelected, cancellation, Channel, Task and Capability-scoped configuration.
- [ ] A second table maps Win32 process, handle, path, registry and overlapped I/O to the same native terms.
- [ ] Each row states the Personality that still exposes the foreign term and that native crates must not import it.
- [ ] ABI-041 is cited for the Object<T> terminus of each row.

#### Verification
- Review: ABI, LNX and WIN leads sign off that no row claims a POSIX or Win32 API is native.
- Manual: both tables are published on the V1 site under the developer class.

#### Evidence
- none

### DOC-018 · Package documentation into the SystemGeneration store
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-010, DOC-013, PKG-025, PKG-021
- Baseline: §28, §30, §56.5
- Invariants: I-036

Offline `os help` and man-page equivalent pages must ship as an immutable Package in the SystemGeneration so rollback restores the docs that match that generation. PKG owns the store; DOC owns the docs Package.

<!-- covers: EXTRA-040 -->

#### Out of scope
Store substrate and install semantics (PKG-025, PKG-038). `os help` CLI (SDK-067). Man-page viewer (LNX).

#### Acceptance criteria
- [ ] A docs Package is pinned in the SystemGeneration alongside shell and services, identified by content hash (I-036).
- [ ] Rolling back to the previous generation restores the previous docs Package hash, verified by `os inspect` or package list.
- [ ] Installing the generation writes docs only as store objects; a filesystem diff of shared directories is empty.
- [ ] The Package contains the generated corpus used by DOC-020, not a live network index.

#### Verification
- Integration: generation switch and rollback on `qemu-x86_64` (H-001) shows the docs Package hash tracking the generation.
- Review: PKG lead confirms reserved signing fields and no shared-directory writes.

#### Evidence
- none

### DOC-019 · Ship man-page equivalent docs inside the Linux Personality
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-018, DOC-010
- Baseline: §3, §46, §56.5
- Invariants: I-006

EXTRA-040's second half is a man-page equivalent for the Linux personality, generated from the same corpus so native and personality help cannot diverge. LNX installs the viewer. Native Components do not see `man` as a native Interface (I-006).

<!-- covers: EXTRA-040 -->

#### Out of scope
Personality viewer and `MANPATH` (LNX). Native `os help` (DOC-020, SDK-067). Docs Package (DOC-018).

#### Acceptance criteria
- [ ] Every page in the docs Package has a man-page equivalent record generated from the same source, including `os` subcommands.
- [ ] A Linux-personality `man os-inspect` (or the name the generator assigns) shows the same command semantics as native `os help inspect`.
- [ ] A native Component without the Linux personality has no `man` binary and no man database in its address space.
- [ ] Regenerating the corpus updates both native and personality pages in one build.

#### Verification
- Integration: personality fixture on `qemu-x86_64` opens the inspect page via the LNX viewer; a native-only Component listing finds no man database.
- Review: LNX lead confirms the viewer remains inside the Personality.

#### Evidence
- none

### DOC-020 · Ship offline os help from the generated docs Corpus
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-018, DOC-013, DOC-006
- Baseline: §56.5, §62, §64

V2 desktop preview is the earliest rung that needs on-device help with no network. Content is the generated corpus in the docs Package; SDK supplies the `os help` command. Search over that corpus uses the same index family as the site where the toolchain Decision allows it.

<!-- covers: EXTRA-040 -->

#### Out of scope
`os help` CLI implementation (SDK-067). Docs Package (DOC-018). Site search (DOC-006). Man-page equivalent (DOC-019).

#### Acceptance criteria
- [ ] `os help inspect` (and the other V1 subcommands) renders the CLI reference from the on-device Package with the network interface down.
- [ ] A missing page name exits non-zero with a typed error, not a network fetch.
- [ ] Rolling back the SystemGeneration changes `os help` output to that generation's corpus.
- [ ] Help text matches the generated site page for the same snapshot identity.

#### Verification
- Integration: `sdk:tests/os_help_offline_*` on `qemu-x86_64` with network denied, including rollback of the docs Package.
- Review: SDK CLI lead confirms the command reads the Package, not a URL.
- Demo: `os help inspect` on H-002 with the NIC down.

#### Evidence
- none

### DOC-021 · Write Linux and Windows Personality compatibility notes
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-008, DOC-015, GOV-044, LNX-054, WIN-018
- Baseline: §3, §46, §48, §49, §62
- Invariants: I-006, I-007, I-096

V2 gates W1 and L3. Taxonomy names compatibility notes as the first public mapping of what runs where, feeding the V3 compatibility guide. LNX and WIN own ratings; DOC publishes the notes under GOV mark rules.

<!-- covers: INV-1248 -->

#### Out of scope
Corpus ratings (LNX-084, WIN-051). Trademark wording (GOV-044). V3 compatibility guide (DOC-028). Five-minute Linux launch guide authorship (LNX-054). Anti-cheat blocked-title list authorship (WIN-018).

#### Acceptance criteria
- [ ] A published notes page states, for native, Linux personality, Windows personality, fallback VM and not-at-all, what a user can expect at V2, citing C-IDs not pass-rate numbers in prose.
- [ ] Third-party marks (Windows, Steam, Proton) appear only as GOV-044 allows.
- [ ] The notes link WIN-018 for kernel-level anti-cheat and do not claim those titles run.
- [ ] Native APIs are not described as POSIX or Win32 wrappers (I-006, I-007).

#### Verification
- Review: GOV trademark, LNX and WIN leads sign off on the pull request.
- Manual: the notes page is on the V2 site snapshot under the compatibility class.

#### Evidence
- none

### DOC-022 · Write the desktop user guide
- Type: docs
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-008, DOC-015
- Baseline: §9, §25, §30, §62

V2 desktop-preview scope needs a user guide covering launch, Capability grants, rollback, lock, snapshots UI and the store client. The V3 unaided-install study expands the install-adjacent chapters; this task is the desktop book those chapters extend.

<!-- covers: INV-1248 -->

#### Out of scope
Install guide (DOC-030). Permissions and shell chrome (APP). Store client behavior (APP, REL). Rollback mechanism (PKG, INS-014). Administrator guide (DOC-027).

#### Acceptance criteria
- [ ] Chapters exist for launch, grant and revoke, lock and unlock, snapshot and rollback, notifications and store install with capability review.
- [ ] Each chapter names the Settings or shell surface the user actually clicks, not an unpublished command.
- [ ] Rollback is described as selecting a SystemGeneration, not as mutating a shared filesystem.
- [ ] No chapter claims a performance number.

#### Verification
- Review: APP shell lead sign-off recorded on the pull request.
- Manual: the user-guide landing page is on the V2 site snapshot and linked from `os help`.

#### Evidence
- none

### DOC-023 · Generate complete Layer 1 reference pages for every entry point
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-010, ABI-046, CAP-046, CMP-050
- Baseline: §7, §12, §63, §65, §66
- Risks: R-052
- Invariants: I-040

V3-G12 requires a reference page for every Layer 1 ABI entry point. ABI, IPC, CAP, MEM, TSK and CMP author semantics; DOC generates and publishes every page from IDL plus that authored prose. Sibling of ABI-046. Pages remain unfrozen as documents until V4 freeze tasks run (I-040).

#### Out of scope
Normative semantics (ABI-046, CAP-046, CMP-050). Coverage CI gate (DOC-024). Layer 1 freeze (ABI V4 freeze tasks).

#### Acceptance criteria
- [ ] Every Layer 1 entry point listed by ABI-046 has a generated page that includes signature, errors, rights and a link to authored semantics.
- [ ] A newly added prototyped entry point without a page is reported by the generator as missing, not silently omitted.
- [ ] Pages record surface state as prototyped or freeze-candidate as the surface register records; none are labeled frozen.
- [ ] Capability rights pages include every right CAP-046 documents.

#### Verification
- Integration: generator coverage report equals the ABI entry-point list on the V3 docs CI job.
- Review: ABI lead confirms every entry point is present on the pull request.
- Manual: a sample of Capability, Channel, MemoryObject, Component and Task pages render on the current snapshot.

#### Evidence
- none

### DOC-024 · Gate documentation coverage and the docs build in CI
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-012, DOC-023, BLD-001
- Baseline: §63, §65

V3-G12 makes the documentation build part of CI and requires a page for every Layer 1 entry point. This gate fails the merge if a Layer 1 symbol lacks a page or the site build breaks.

#### Out of scope
Generator (DOC-010). V1 generate-only CI (DOC-012). Broken-link and example compilation (DOC-034). ABI entry-point list (ABI).

#### Acceptance criteria
- [ ] A merge that adds a Layer 1 entry point without a reference page fails CI with the symbol name.
- [ ] A merge that breaks the site or generator fails CI.
- [ ] Removing an entry point from the ABI list without dropping its page fails CI as a stale page.
- [ ] The job is required on the merge queue, not nightly-only.

#### Verification
- Integration: `docs:tests/coverage_gate_*` with fixtures for missing page, stale page and broken site.
- Review: BLD lead confirms the job is blocking on the merge queue.

#### Evidence
- none

### DOC-025 · Set up the documentation translation pipeline and string freeze
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-015, DOC-007, GOV-021, TXT-036, TXT-017
- Baseline: §63

GAP-0383 is contributor tooling and per-release freeze for documentation catalogs. UI and OS string catalogs stay TXT. This pipeline consumes that freeze and ships translated docs under the license GOV chose. Language count lives in the verifying TXT and APP tasks, not here.

<!-- covers: GAP-0383 -->

#### Out of scope
UI and OS catalogs (TXT-032, TXT-036). License and translation-contribution terms (GOV-021). Shell and app string shipping (APP-066).

#### Acceptance criteria
- [ ] Documentation catalogs extract from the site sources into the message-catalog format TXT-017 accepted.
- [ ] A per-release docs string freeze is recorded as a catalog revision pinned in the snapshot that ships with that generation.
- [ ] Contributor tooling uploads and downloads documentation catalogs on TXT-036 without write access to unrelated UI catalogs.
- [ ] Translated pages show the license and translator terms from GOV-021.
- [ ] Untranslated strings render in the source language, not as empty pages.

#### Verification
- Integration: extract, freeze, and import a fixture translation and build a snapshot that contains it.
- Review: TXT and GOV licensing leads sign off on catalog separation and license terms.
- Manual: a translated page and a fallback page both render on the snapshot.

#### Evidence
- none

### DOC-026 · Verify public docs suffice for unaided install
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-030, DOC-022, INS-029, INS-039
- Baseline: §63

V3-G02 requires a user without project involvement to install from public media following only public documentation. INS runs the study. DOC verifies that install and user guides were the sole materials and records each gap as a follow-up task rather than a private briefing.

#### Out of scope
Study execution and participant protocol (INS-029). Installer (INS-027). Guide authorship (DOC-030, DOC-022).

#### Acceptance criteria
- [ ] The study report lists every document handed to participants; the set equals pages published by DOC-030 and DOC-022 plus INS-039, with no unpublished notes.
- [ ] Each unaided failure is filed as a DOC or INS task with the page heading that was missing or wrong.
- [ ] The unaided-success count named in INS-029 is met, or the gate remains unsatisfied and this task stays open.
- [ ] No participant received a private walkthrough, chat log or unpublished URL.

#### Verification
- Review: INS usability and DOC leads sign off that materials equal the public snapshot.
- Manual: the study report's document list is diffed against the V3 site snapshot.
- Demo: V3-D01 public install on a Tier 1 laptop uses only those pages.

#### Evidence
- none

### DOC-027 · Write the administrator guide
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-008, DOC-015, BOOT-042, INS-039
- Baseline: §9, §30, §31, §32, §63

Public alpha adds FDE, Secure Boot, multi-user, updater and recovery, so V3 is the earliest gate that needs an administrator guide. STO authors storage chapters later; this book publishes the operational procedures those chapters slot into. Required by V4-G14 (Documentation complete for 1.0): the administrator reference that gate requires is this guide.

#### Out of scope
Storage chapter authorship (STO-084). Secure Boot key path authorship (BOOT-042). Recovery procedures authorship (INS-039). Install guide (DOC-030). Multi-user session objects (SEC, APP-063).

#### Acceptance criteria
- [ ] Chapters exist for FDE recovery keys, Secure Boot enrollment and developer mode, multi-user isolation, updater channels, generation rollback and recovery environment.
- [ ] Each chapter links the owning prefix's authored procedure (BOOT, INS, SEC) rather than restating it with different steps.
- [ ] The guide states what is not promised (fleet management, MDM, domain join) and points at the published non-goals.
- [ ] No chapter claims a performance number.

#### Verification
- Review: BOOT, INS and SEC leads sign off on the pull request.
- Manual: administrator-guide landing page is on the V3 snapshot under the administrator class.

#### Evidence
- none

### DOC-028 · Write the compatibility guide
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-021, LNX-099, WIN-068, GOV-044
- Baseline: §3, §46, §48, §49, §63
- Invariants: I-006, I-007, I-096

V3-G12 requires a published and reviewed compatibility guide. This expands V2 notes into the public book for Linux personality and Windows personality, including what deliberately does not exist. LNX and WIN author chapters; GOV owns trademark wording; DOC owns the book.

<!-- covers: GAP-0477 -->

#### Out of scope
Linux chapters authorship (LNX-099). Windows per-title reports (WIN-068). Trademark rules (GOV-044). VM fallback product (VIRT). 1.0 unsupported matrices (LNX-109, WIN-083).

#### Acceptance criteria
- [ ] A decision tree page answers native, Linux personality, Windows personality, fallback VM or not-at-all for a user-supplied application class, citing C-IDs.
- [ ] Linux personality and Windows personality chapters are the LNX and WIN authored texts, not a DOC rewrite, and mark rules from GOV-044 hold.
- [ ] A "does not exist" chapter names kernel-level anti-cheat, vendor DRM and native POSIX/Win32 APIs as absent from native software.
- [ ] Pass-rate numbers do not appear; thresholds live in corpus registers.

#### Verification
- Review: LNX, WIN, VIRT and GOV trademark leads sign off on the pull request.
- Manual: the guide is on the V3 snapshot and linked from the user guide.

#### Evidence
- none

### DOC-029 · Write the public contribution guide
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-015, GOV-007, GOV-030, GOV-035
- Baseline: §63, §67

V3 public documentation includes a contribution guide. GOV-007 is the V0 repo CONTRIBUTING process; this is the product-facing contributor guide on the docs site (docs, translations, RFCs, first-party Packages).

<!-- covers: INV-1248 -->

#### Out of scope
Repo CONTRIBUTING, CODEOWNERS and AI policy (GOV-007). Code of Conduct (GOV-030). RFC venue (GOV-035). Developer program (DOC-039, GOV-071). Translation tooling (DOC-025).

#### Acceptance criteria
- [ ] The guide tells a stranger how to propose a docs change, a translation, an RFC and a first-party bug, each with a link to the GOV process page rather than a duplicate policy.
- [ ] Style-guide and license requirements are linked, not restated with different rules.
- [ ] The guide does not replace GOV CONTRIBUTING for roadmap task edits.
- [ ] Code of Conduct and RFC venue are linked from the landing page.

#### Verification
- Review: GOV community lead sign-off recorded on the pull request.
- Manual: the page is on the V3 snapshot and linked from the developer guide.

#### Evidence
- none

### DOC-030 · Write the public install guide
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-015, INS-039, INS-027, BOOT-042
- Baseline: §30, §63

V3-G12 requires an install guide published and reviewed. INS owns the installer and authors procedures; DOC owns the public guide the unaided-install study uses. Dual-boot, BitLocker and Secure Boot warnings come from INS and BOOT texts.

<!-- covers: INV-1248 -->

#### Out of scope
Installer (INS-027). Procedure authorship (INS-039). Secure Boot path (BOOT-042). Unaided study (INS-029, DOC-026). Media creation (INS-035).

#### Acceptance criteria
- [ ] The guide covers download verification, media write, Secure Boot, disk selection, FDE default-on, first boot and where to find recovery, using only published INS and BOOT steps.
- [ ] Dual-boot and BitLocker warnings from INS-039 appear before any disk-write step.
- [ ] Hardware Compatibility List lookup is described as a pre-commit step, not as a promise of universal PC support.
- [ ] The unaided-install study can be run using only this guide plus the user guide; no other DOC page is required for install.

#### Verification
- Review: INS and BOOT leads sign off on the pull request.
- Manual: the guide is on the V3 snapshot and is the document list entry for install in DOC-026.

#### Evidence
- none

### DOC-031 · Write migration guides for users and SDK porters
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-017, DOC-022, DOC-014, APP-062
- Baseline: §3, §25, §52, §63

GAP-0382 is user guides from Linux distributions and Windows plus developer porting to the native SDK. The V1 concept map is the vocabulary; this is the public-alpha migration set. APP owns the migration assistant UI; INS owns import engines.

<!-- covers: GAP-0382 -->

#### Out of scope
Import engines (INS-049, INS-033). Migration assistant UI (APP-062). Concept map (DOC-017). VM fallback (VIRT).

#### Acceptance criteria
- [ ] A user guide chapter walks Linux-distribution and Windows users through what the migration assistant imports, what stays in a Personality, and what must be re-entered.
- [ ] A developer chapter walks a POSIX or Win32 application through the concept map to a first native Component, with one worked native rewrite outline that does not claim the foreign API is native.
- [ ] Each chapter states what deliberately does not migrate (kernel-level anti-cheat, vendor DRM, ambient home access).
- [ ] The assistant UI labels in APP-062 match the guide's step names.

#### Verification
- Review: APP, INS, SDK and LNX/WIN leads sign off on the pull request.
- Manual: both chapters are on the V3 snapshot and linked from user and developer landings.

#### Evidence
- none

### DOC-032 · Complete and review the SDK guide
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-014, PKG-085, SVC-021, UIP-031, SCH-040
- Baseline: §35, §52, §63, §66

V3-G12 requires the SDK guide published and reviewed. This promotes the V1 developer guide to the public-alpha SDK guide, folding in packaging, `os env`, supervised services and the UI protocol chapters those prefixes author.

#### Out of scope
Chapter authorship for packaging (PKG-085), environments (ENV-030), services (SVC-021), UI protocol (UIP-031) and extra languages (SDK-086). C SDK complete book (DOC-038). V1 first cut (DOC-014).

#### Acceptance criteria
- [ ] The published SDK guide includes Rust, C and the extra language SDK-086 names, plus packaging, environment and service chapters linked from those prefixes.
- [ ] A review record lists the SDK, PKG, ENV, SVC and UIP leads and the snapshot identity they reviewed.
- [ ] Samples listed in the guide compile on the V3 SDK CI job.
- [ ] Layer 3 versus Layer 1 is stated on the landing page; no POSIX-shaped native API is documented.

#### Verification
- Review: SDK, PKG, ENV, SVC and UIP leads sign off on the pull request.
- Integration: listed samples compile in SDK CI.
- Manual: the guide is on the V3 snapshot and linked from getting-started.

#### Evidence
- none

### DOC-033 · Publish the platform security guide
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-015, SEC-002, GOV-061
- Baseline: §9, §9.1, §51, §63
- Invariants: I-021, I-060

V3 public alpha is the earliest audience for a user-facing platform security guide. It states what the OS protects against, and does not, in powered-off, suspended, locked, unlocked and developer-mode states. SEC authors the threat model; GOV owns privacy policy; DOC publishes the guide.

<!-- covers: GAP-0241 -->

#### Out of scope
Threat model (SEC-002). Privacy policy (GOV-061). Disclosure and advisories (REL-047). Permissions UI (APP).

#### Acceptance criteria
- [ ] A table for powered off, suspended, locked, unlocked and developer mode states what is protected (FDE, lock, Capability store, secrets) and what is not, citing T-IDs from SEC-002.
- [ ] The guide states no ambient authority (I-021) and that deny lists are not the primary mechanism (I-060).
- [ ] Crash reports, HCL submissions and telemetry are described as opt-in with a link to GOV-061, not as always-on.
- [ ] No superiority claim versus Linux or Windows lock or encryption appears without a cited report.

#### Verification
- Review: SEC and GOV leads sign off on the pull request.
- Manual: the guide is on the V3 snapshot under the user or administrator class and linked from the admin guide.

#### Evidence
- none

### DOC-034 · Add broken-link and outdated-example checks to docs CI
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-024, DOC-015, SDK-092
- Baseline: §52, §66

V4-G14 requires broken-link and outdated-example checks to pass in CI. This extends V1/V3 docs CI with link checking and snippet compilation against the freeze-candidate SDK.

#### Out of scope
Coverage of Layer 1 symbols (DOC-024). SDK crate publication (SDK). Claim lint (BEN-004).

#### Acceptance criteria
- [ ] A page with a broken internal or published-snapshot link fails CI with the URL and page path.
- [ ] A fenced snippet tagged as compilable that does not build against the freeze-candidate SDK fails CI with the snippet identity.
- [ ] External-link failures are reported and do not fail the merge unless the link is in the allowlisted first-party set.
- [ ] The job is required on the merge queue.

#### Verification
- Integration: `docs:tests/links_examples_*` with fixtures for a broken internal link, a stale snippet and a live first-party link.
- Review: SDK lead confirms snippets compile against the freeze-candidate crate.

#### Evidence
- none

### DOC-035 · Complete remaining user, admin, SDK, compatibility and ABI docs
- Type: docs
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-027, DOC-032, DOC-038, DOC-028, DOC-033, DOC-023, DOC-022, DOC-030, ABI-050
- Baseline: §63, §65, §66

V4-G14 requires user, administrator, SDK (Rust and C), compatibility, security and ABI references complete. This editorial pass closes holes left by per-guide tasks before sign-off.

#### Out of scope
Per-guide authorship (named dependencies). Review sign-off (DOC-036). Extra-language guide (DOC-037). ABI freeze Decision (ABI).

#### Acceptance criteria
- [ ] A coverage checklist lists user, administrator, SDK Rust, SDK C, compatibility, security and ABI reference landings, each with a snapshot URL and no `TODO` headings.
- [ ] Every hole filed during V3 unaided install or SDK review is either closed on that checklist or dropped with a named follow-up task outside the 1.0 definition.
- [ ] ABI reference pages match ABI-050 terminology for freeze-candidate versus frozen.
- [ ] Style-guide glossary casing holds on every landing heading.

#### Verification
- Review: DOC, SDK, ABI and APP leads sign off on the checklist pull request.
- Manual: the seven landings render on the V4 snapshot with no `TODO` headings.

#### Evidence
- none

### DOC-036 · Record documentation review sign-off for beta
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-035, DOC-034
- Baseline: §66

V4-G14 requires a documentation review sign-off recorded. This is the process gate over the complete user, administrator, SDK, compatibility, security and ABI set after CI link and example checks are green.

#### Out of scope
Editorial completion (DOC-035). Link and example CI (DOC-034). Developer-program docs (DOC-039).

#### Acceptance criteria
- [ ] A review record names the DOC, SDK, ABI, SEC and APP reviewers and the snapshot identity they reviewed.
- [ ] The record states DOC-034 is green on that snapshot.
- [ ] The record is linked from the V4 docs landing and from the gate evidence, not only from a chat message.
- [ ] Open issues on the reviewed snapshot are P2 or lower, or named as blocking with a task ID.

#### Verification
- Review: named leads sign off on the pull request that adds the record.
- Manual: the record is reachable from the V4 snapshot landing.

#### Evidence
- none

### DOC-037 · Write the additional-language SDK guide
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-038, SDK-024, SDK-086
- Baseline: §50, §52, §66

V4-G15 needs SDK bindings for C and one additional language, with developer-program documentation. DOC writes the guide for the language SDK-024 selected; SDK owns the binding.

#### Out of scope
Binding implementation (SDK-063 and later language tasks). C guide (DOC-038). Language order Decision (SDK-024). Developer program terms (GOV-071).

#### Acceptance criteria
- [ ] A guide for the additional language named by SDK-024 covers project layout, IDL stubs, Capability handles and the getting-started equivalent.
- [ ] Samples compile with that language's SDK package on the V4 CI job.
- [ ] The guide states the language is Layer 3 over the Native ABI and does not add Layer 1 entry points.
- [ ] Native software in that language still does not import POSIX or Win32 as the native API.

#### Verification
- Review: SDK bindings lead sign-off recorded on the pull request.
- Integration: listed samples compile in the additional-language CI job.
- Demo: V4-D06 uses this guide's language.

#### Evidence
- none

### DOC-038 · Write the C SDK guide
- Type: docs
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-032, SDK-033, SDK-034
- Baseline: §50, §52, §66

Required by V4-G14 (Documentation complete for 1.0): SDK guides for Rust and C, re-checked at 1.0-G11. C bindings ship at V1; the complete C guide is the V4 documentation gate. SDK owns wrappers and headers; DOC publishes the book.

#### Out of scope
C binding and headers (SDK-033, SDK-034). Rust guide (DOC-032). Crate reference completion (SDK-092).

#### Acceptance criteria
- [ ] The C guide covers headers, IDL-generated stubs, handle ownership, error codes and a chooser sample equivalent to the Rust getting-started path.
- [ ] Listed samples compile with SDK-034 on host and native toolchains.
- [ ] The guide states C is a Layer 3 binding over Layer 1 and does not document Linux syscalls as native.
- [ ] Ownership and drop rules are explicit so a leaked handle is described as a bug, not as garbage collection.

#### Verification
- Review: SDK C-binding lead sign-off recorded on the pull request.
- Integration: listed samples compile in the C SDK CI job.
- Manual: the C guide landing is on the V4 snapshot beside the Rust guide.

#### Evidence
- none

### DOC-039 · Write developer-program documentation
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-015, GOV-071, GOV-063, SDK-080, GOV-073
- Baseline: §56.5, §66

V4-G15 requires developer-program documentation published. GOV owns programme terms and publisher identity; REL owns the repository; SDK owns `os publish`; DOC publishes the onboarding docs.

#### Out of scope
Programme terms (GOV-071). Intermediary and developer agreement (GOV-063). `os publish` (SDK-080). Publisher pipeline (REL-021). Ecosystem counts (GOV-073).

#### Acceptance criteria
- [ ] The docs walk a third-party developer from SDK install through `os publish`, capability review and listing, linking GOV terms rather than restating legal text.
- [ ] Publisher identity, namespace and agreement links match GOV-071 and GOV-063.
- [ ] The docs state that capability review can deny optional Capabilities and the application must still launch degraded where it declared them optional.
- [ ] No package-count target is stated in prose; GOV-073 owns the metric.

#### Verification
- Review: GOV and REL leads sign off on the pull request.
- Manual: the pages are on the V4 snapshot and linked from the SDK guide.
- Demo: V4-D06 can be followed from these pages plus the additional-language guide.

#### Evidence
- none

### DOC-040 · Snapshot the 1.0 documentation set per release
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-007, DOC-035
- Baseline: §66, §70

1.0 documentation completeness reuses GAP-0380 snapshot machinery: freeze the 1.0 docs set so later 1.x snapshots are comparable and the ABI reference stays pinned to the stability statement.

#### Out of scope
Site builder (DOC-007). Editorial completion (DOC-035). Release notes authorship (DOC-042). Channel publication (REL-065).

#### Acceptance criteria
- [ ] A snapshot identity named for 1.0 is immutable after publication: fetching it after a later 1.x snapshot still returns the 1.0 pages.
- [ ] The 1.0 snapshot includes ABI reference, SDK Rust and C guides, user, administrator, compatibility, security guides and the 1.0 release notes.
- [ ] The snapshot records the SystemGeneration and SDK version it was built from.
- [ ] Rebuilding the snapshot from the same sources yields the same content hashes for generated pages.

#### Verification
- Integration: publish 1.0 then a fixture 1.x snapshot and fetch a 1.0 ABI page that still matches the frozen hash.
- Review: REL lead confirms the snapshot is what the stable channel links.

#### Evidence
- none

### DOC-041 · Verify documentation completeness for the 1.0 Gate
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-035, DOC-036, DOC-034, DOC-042, DOC-040, DOC-038, SDK-096
- Baseline: §56.5, §63, §70
- Invariants: I-061

1.0-G11 requires Layer 1 ABI reference at full coverage, SDK guides for Rust and C, user, administrator, compatibility and security guides, release notes, and documentation CI green. This task is the gate-verifying checklist over the V3/V4 set.

<!-- covers: INV-1110, INV-1248 -->

#### Out of scope
Authoring (named dependencies). Accessibility and localization hold (ACC, TXT, APP). Support-policy documents (GOV-075).

#### Acceptance criteria
- [ ] A checklist records a 1.0 snapshot URL for ABI reference, SDK Rust, SDK C, user, administrator, compatibility, security and release notes, each present.
- [ ] DOC-034 and DOC-024 are green on that snapshot.
- [ ] DOC-036 is recorded against the same snapshot identity or a successor that only adds release notes.
- [ ] The release announcement set contains no performance claim without a B-ID link (I-061).

#### Verification
- Review: DOC, SDK, ABI and REL leads sign off on the checklist pull request.
- Manual: 1.0-G11 evidence links this checklist.
- Integration: CI green jobs are cited by job identity on the checklist.

#### Evidence
- none

### DOC-042 · Write 1.0 release notes with V3 and V4 migration paths
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-014, DOC-015, APP-068, LNX-109, WIN-083, GOV-083
- Baseline: §54, §57, §70
- Invariants: I-050, I-061

1.0-G11 requires release notes with a migration path from V3 and V4, plus the explicit non-promises list. REL owns the channel and generated notes from tasks; DOC authors the 1.0 narrative. No performance claim without a B-ID (I-061, I-050).

#### Out of scope
Generated task-diff notes (REL-014). Channel publication (REL-065). Non-goals authorship (APP-068, LNX-109, WIN-083). Stability contract (GOV-083). Claim audit (BEN-062).

#### Acceptance criteria
- [ ] Release notes include a V3-to-1.0 and V4-to-1.0 migration path covering generations, ABI freeze and personality runtimes.
- [ ] A non-promises section lists native filesystem, native GPU stack, native browser, native IDE, ARM64, kernel-level anti-cheat, vendor DRM and distributed interfaces, linking APP, LNX and WIN non-goal pages.
- [ ] Every performance sentence cites a B-ID and a published report path; a fixture sentence without a B-ID is rejected by style review.
- [ ] REL-014 output is included as the task-level changelog, not rewritten by hand into conflicting facts.

#### Verification
- Review: REL, GOV and BEN leads sign off on the pull request, including I-061.
- Manual: notes are in the 1.0 snapshot and linked from the download site.
- Integration: BEN-062 can consume the notes file as an input.

#### Evidence
- none
