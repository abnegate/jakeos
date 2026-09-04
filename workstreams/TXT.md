# TXT · Text, fonts, input methods, i18n
- Prefix: TXT
- Lead: none
- Baseline: §41, §47, §49, §51, §54, §60, §66, §67
- Baseline gap: §41 names text, shaping, editing and localization as native UI requirements but specifies no font service, IME protocol, glyph atlas, locale data or localization pipeline.

<!-- roadmap:generated:begin summary -->
Tasks: 47 live, 0 done, 0 in-progress, 47 todo, 0 dropped. Ready: 4. Blocked: 43. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

TXT owns the native text stack that every native window uses: font Packages and the FontMatcher Interface, shaping and rasterization, the read-only glyph atlas service, toolkit text layout and editing, the input-method protocol (S-016) registered before the UI protocol becomes a freeze candidate, the sandboxed IME host and engines, locale data and the Locale object, the SDK localization pipeline and translation platform, and complex-script layout (bidi, line breaking, vertical CJK, variable fonts, emoji sequences). Native software never talks fontconfig, Pango or POSIX locales. Personality integration (fontconfig view, Wayland text-input mapping) is specified here and consumed by LNX so compatibility windows match native text.

## Out of scope

UI protocol transport, widgets and input routing (UIP). Accessibility tree and screen reader (ACC). Terminal, Text Editor, shell chrome, emoji picker and first-party catalog shipping (APP). Wayland text-input protocol implementation, fontconfig daemon hosting and personality CJK verification (LNX). Wine font directories and IMM32/TSF (WIN). Font shipping policy and userspace license allowlist (GOV). Content-addressed store and Package format (PKG). Supervisor, locale settings service and time service (SVC). Side-channel statement and V0 threat register (SEC). MemoryObject backing and GPU buffers (MEM, GFX). IDL compiler and Layer 2 evolution rules (IPC). SDK crate publication (SDK). Docs site generation (DOC). CI plumbing (BLD). Benchmark methodology (BEN). Capability rights encoding (CAP). Physical keyboards and layout databases (HW).

## Tasks

### TXT-001 · Benchmark text stack initialisation and first-glyph time inside application startup
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: TXT-013, TXT-006, BEN-016, BEN-009
- Baseline: §34, §41, §54
- Benchmarks: B-016

Font loading, FontMatcher resolution and first shaping sit on the warm-startup path measured by B-016. This harness isolates those stages inside Terminal and Text Editor launch on H-003 and H-002 so later text-stack claims have a published baseline rather than a number in prose.

#### Out of scope
Application-level warm and cold startup publication (BEN-009). Visible-UI boundary (BEN-016).

#### Acceptance criteria
- [ ] A harness named on B-016 records font load, matcher resolution and first-glyph time as separate stages for Terminal and Text Editor.
- [ ] The V0.5 B-016 target is published for those stages on H-003 and H-002 with tracing disabled unless stated.
- [ ] No TXT description, criterion or report restates a numeric startup target; the register holds the target.

#### Verification
- Bench: B-016 on H-003 and H-002; target per register.
- Integration: stage timestamps appear in the B-016 report skeleton under `reports/benchmarks/B-016/`.
- Review: BEN lead confirms the stages are not double-counted against APP-001.

#### Evidence
- none

### TXT-002 · Decide the default system font set and publish its script coverage matrix
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-009
- Baseline: §41
- Decision: D-0316
- Invariants: I-070

The first immutable V0.5 image redistributes fonts. This adr selects the default system set covering Latin, Cyrillic, Greek, Arabic, Hebrew, Devanagari, Thai, CJK, monospace and COLRv1 color emoji, including metric-compatible substitutes for Windows core fonts, and records a script coverage matrix. I-070 forbids bundling Microsoft fonts. GOV-009 decides what may be redistributed; this adr names the families.

<!-- covers: GAP-0247, GAP-0030 -->

#### Out of scope
Shipping-versus-substitutes policy (GOV-009). Per-font license inventory (TXT-005). Package objects (TXT-007).

#### Acceptance criteria
- [ ] Option A (named open-licensed families covering the script list, with metric-compatible substitutes for Windows core fonts) and Option B (Latin-first core set with optional per-script Packages) are evaluated with consequences.
- [ ] The accepted option names every shipped family, its scripts and whether it is a metric-compatible substitute, and the coverage matrix is in the decision file.
- [ ] The accepted option lists no Microsoft core fonts (I-070).
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: GOV licensing and TXT leads sign off on the pull request that accepts the decision file.
- Manual: coverage matrix rows exist for Latin, Cyrillic, Greek, Arabic, Hebrew, Devanagari, Thai, CJK, monospace and COLRv1 emoji.

#### Evidence
- none

### TXT-003 · Decide the shaping and rasterisation libraries for the native text stack
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: TXT-011
- Baseline: §41, §67
- Decision: D-0321
- Risks: R-019
- Invariants: I-009

Every V0.5 application renders text through one shaping and rasterization stack. This adr decides that stack, using the spike report, and applies §67 Principle 15 so a mature library is not replaced without a recorded benefit. It defines the text-stack scope §41 left open. R-019 is the risk that a minimal path hardens into the permanent design.

<!-- covers: GAP-0245, GAP-0518, INV-0781 -->

#### Out of scope
In-Component versus shared-service placement (TXT-004). Toolkit layout integration (TXT-013).

#### Acceptance criteria
- [ ] Option A (retain HarfBuzz plus FreeType) and Option B (Rust-native rustybuzz, swash or cosmic-text) are evaluated against the spike report, including unsafe surface and Principle 15.
- [ ] The accepted option names the libraries, the crate or package boundary, and the rejected option with reasons.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: TXT lead sign-off recorded on the pull request that accepts the decision file.
- Report: `reports/spikes/TXT-011.md` is cited as evidence in the decision options.

#### Evidence
- none

### TXT-004 · Decide whether shaping runs in-Component or in a shared text service Component
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: TXT-008, TXT-011
- Baseline: §41, §10, §51
- Decision: D-0323
- Risks: R-019, R-071
- Threats: T-030
- Invariants: I-083

Placement changes isolation and latency for every UI Component. This adr decides whether shaping and rasterization run as an in-Component library, as a shared system text service, or as a hybrid where the library shapes and a service mints read-only caches. I-083 and T-030 constrain writable sharing. The font threat model is an input because a shared service is a font-parsing attack surface.

<!-- covers: GAP-0245, GAP-0518 -->

#### Out of scope
Library choice (TXT-003). Atlas sharing model at V1 (TXT-015). Supervisor (SVC).

#### Acceptance criteria
- [ ] Option A (in-Component library), Option B (shared system text service) and Option C (hybrid: library shaping, service-minted read-only caches) are evaluated against T-030, I-083 and the spike latency notes.
- [ ] The accepted option states where untrusted font bytes are parsed and that no shared writable atlas is created.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: TXT and SEC leads sign off on the pull request that accepts the decision file.
- Manual: the decision file cites TXT-008 and T-030.

#### Evidence
- none

### TXT-005 · Audit and record licences of every shipped font with redistribution terms
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: TXT-002, GOV-016
- Baseline: §41
- Invariants: I-068, I-070

The first immutable V0.5 image redistributes font files. This document records license, source and redistribution terms for each shipped family under GOV policy and the userspace allowlist, and is the input to BLD license and SBOM CI. Microsoft core fonts are absent (I-070).

<!-- covers: GAP-0030 -->

#### Out of scope
Allowlist policy (GOV-016). CI enforcement (BLD-023). Family selection (TXT-002).

#### Acceptance criteria
- [ ] A committed table lists every shipped font Package identity, upstream source, license SPDX identifier and redistribution terms.
- [ ] Every row is on the userspace allowlist; AGPL, SSPL and BUSL do not appear (I-068).
- [ ] No row is a Microsoft core font (I-070).
- [ ] BLD-023 can consume the table path without a hand-maintained duplicate.

#### Verification
- Review: GOV licensing sign-off recorded on the pull request.
- Integration: `BLD-023` reads the table and fails a fixture that adds a disallowed license.

#### Evidence
- none

### TXT-006 · Build the typed FontMatcher Interface with family, weight, script fallback chains
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-007, TXT-003, IPC-035
- Baseline: §41, §12, §66

Native applications resolve family, weight, width, style and script to a fallback chain over installed font Packages through a typed Layer 2 Interface, not fontconfig. The V0.5 toolkit and the four demo applications call only this Interface. State is prototyped; versions lock at V4.

<!-- covers: GAP-0246, INV-0781 -->

#### Out of scope
fontconfig generation for the Linux personality (TXT-019). Variable-font axes (TXT-039). IDL compiler (IPC).

#### Acceptance criteria
- [ ] `FontMatcher.match` returns an ordered fallback chain of font Package objects for family, weight, width, style and script, and a missing family yields the next chain entry rather than tofu without a recorded fallback.
- [ ] A native Component without fontconfig, Fontconfig or `FONTCONFIG_PATH` still resolves the default system set.
- [ ] Terminal, File Browser, Text Editor and Image Viewer resolve fonts only through FontMatcher, verified by a trace that lists the Interface calls.
- [ ] The Interface is listed in the Layer 2 registry with a version identity.

#### Verification
- Unit: `text:tests/font_matcher_*` on CI matrix entries `qemu-x86_64` (H-001) and `hw-h002`.
- Integration: four-app launch on H-003 with `os inspect` showing FontMatcher Channel use and no fontconfig files in the Component address space.
- Review: IPC lead confirms the Interface follows S-014 evolution rules as prototyped.

#### Evidence
- none

### TXT-007 · Ship fonts as immutable content-addressed Packages with per-user installation
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-002, TXT-005
- Baseline: §28, §41, §27

Fonts are Package objects in the content-addressed store: the default set is part of the SystemGeneration, and a user installs additional fonts as Packages without writing outside the store. This is the V0.5 immutable-package exit applied to the text stack.

<!-- covers: GAP-0246, INV-0781 -->

#### Out of scope
Store substrate (STO). `os package` CLI (PKG-034). FontMatcher (TXT-006).

#### Acceptance criteria
- [ ] Installing the default font set adds only content-addressed objects; a filesystem diff of shared directories is empty.
- [ ] A per-user font Package is visible to that user's FontMatcher and not to another user on the same machine.
- [ ] Uninstalling a user font Package leaves the system set and other users' fonts intact, including shared content still referenced.
- [ ] Two versions of the same family coexist and FontMatcher can name each by Package identity.

#### Verification
- Integration: install and uninstall fixtures on H-003 against PKG-038, with store-size dedup check.
- Unit: `text:tests/font_packages_*` on `qemu-x86_64`.
- Review: PKG lead confirms manifests use reserved signing fields and no shared-directory writes.

#### Evidence
- none

### TXT-008 · Write the font parsing and text service threat model
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002
- Baseline: §41, §51, §9.1
- Risks: R-071, R-019
- Threats: T-015, T-030
- Invariants: I-083

A shared glyph atlas is a font-parsing attack surface and a cross-domain channel (T-030, T-015). V0.5 applications parse fonts from Packages and user-installed objects, so trust boundaries, who parses untrusted bytes, and what I-083 requires of a minted atlas must be written before placement and atlas decisions. The V1 side-channel statement (SEC-029) is later; this document names the TXT surface that statement must cover.

<!-- covers: EXTRA-015 -->

#### Out of scope
SEC V0 threat register (SEC-002). V1 side-channel position statement (SEC-029). Atlas implementation (TXT-020).

#### Acceptance criteria
- [ ] The document names assets (font bytes, atlas MemoryObjects, shaped-run caches), actors (malicious font Package, malicious GUI Component) and vectors T-030 and T-015.
- [ ] Trust boundaries state which Component parses untrusted font bytes and that the atlas object is read-only (I-083).
- [ ] Placement and atlas adrs are listed as consumers of this document.
- [ ] No shared writable atlas is described as an accepted design.

#### Verification
- Review: SEC and TXT leads sign off on the pull request.
- Manual: T-015 and T-030 are cited by heading ID in the document.

#### Evidence
- none

### TXT-009 · Build the monospace grid text fast path for the Terminal
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §41, §34, §60
- Benchmarks: B-016

The V0.5 Terminal paints a cell grid. Cell-width metrics and a no-reshape glyph path keep shaping off the per-keystroke and warm-startup paths measured by B-016, while still using FontMatcher for the monospace family.

<!-- covers: INV-0771 -->

#### Out of scope
Terminal application, PTY and tabs (APP-004). GPU text atlas at V1 (TXT-020).

#### Acceptance criteria
- [ ] A monospace FontMatcher request yields cell width and height used by a grid painter that does not reshape on ASCII cell updates.
- [ ] A scripted Terminal scenario that fills the grid with ASCII and then types one character does not invoke the full paragraph shaper on the last character, verified by a trace counter.
- [ ] Combining marks and wide East-Asian cells take the slow path and still occupy the documented cell count.
- [ ] B-016 stage timestamps for Terminal include this path as a named stage.

#### Verification
- Unit: `text:tests/mono_grid_*` on `qemu-x86_64` and `hw-h002`.
- Integration: APP Terminal scripted acceptance on H-003 using this path.
- Bench: B-016 Terminal warm startup on H-002; target per register.

#### Evidence
- none

### TXT-010 · Register the input-method protocol shape as a Layer 2 Surface in UI protocol v0
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-013, IPC-038
- Baseline: §41, §66

S-016 must exist as optional UI-protocol messages (preedit, commit, surrounding text, cursor rectangle, candidate placement) before the UI protocol is a V1 freeze candidate, even though engines ship at V2. Registering the shape in protocol v0, then bumping with UIP-015, avoids an incompatible V1 revision. This task does not freeze S-016.

<!-- covers: EXTRA-066 -->

#### Out of scope
Toolkit field implementation (TXT-022). Engine host (TXT-029). UI protocol freeze (UIP). Freeze of S-016 (TXT-045).

#### Acceptance criteria
- [ ] UI protocol v0 IDL declares optional TextInput messages for preedit, commit, surrounding text, cursor rectangle and candidate placement, owned as S-016.
- [ ] `registers/surfaces.md` entry S-016 lists those five operations in its prose and remains `open` or `prototyped`, not `frozen`.
- [ ] An old client that omits the optional methods still opens a window after the v0.1 bump test owned by UIP.
- [ ] Native IDL contains no Wayland `zwp_text_input` or IBus types (I-048).

#### Verification
- Unit: IDL parse of the TextInput methods in `uip:protocol` CI on `qemu-x86_64`.
- Integration: UIP v0 to v0.1 bump regression still passes with the optional methods present.
- Review: UIP and TXT leads confirm S-016 is referenced from the protocol IDL.

#### Evidence
- none

### TXT-011 · Prototype HarfBuzz+FreeType against Rust shaping and raster stacks on a script Corpus
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §41, §67
- Explores: S-016
- Risks: R-019
- Invariants: I-009

A minimal text path that ships for V0.5 apps can harden into the permanent stack (R-019). This spike builds HarfBuzz plus FreeType and a Rust-native candidate (rustybuzz, swash or cosmic-text) behind one shaping trait, measures correctness on a multi-script corpus, throughput, binary size, first-glyph cost and unsafe surface, and rules options out before the library and placement adrs.

<!-- covers: GAP-0245, GAP-0518 -->

#### Out of scope
The library decision (TXT-003). Placement decision (TXT-004). Toolkit integration (TXT-013).

#### Acceptance criteria
- [ ] Both candidate stacks implement one shaping trait and render the same corpus (Latin, Arabic, Devanagari, Thai, CJK) to inspected glyph IDs.
- [ ] The report records per-stack unsafe line count, binary size delta and first-glyph cost on H-002 without stating a numeric target in the task text.
- [ ] The report names at least one option ruled out and why, citing Principle 15.
- [ ] `reports/spikes/TXT-011.md` exists with the spike skeleton headings.

#### Verification
- Report: Which stacks produce identical glyph IDs on the corpus; what unsafe surface each has; what first-glyph and binary-size cost each has on H-002; which options are ruled out under Principle 15; whether a shared service is required for isolation.
- Unit: corpus harness on `qemu-x86_64`.
- Bench: first-glyph stage published beside B-016 method notes; no absolute target.

#### Evidence
- none

### TXT-012 · Build the toolkit text editing model with cursor, selection and surrounding text
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-010
- Baseline: §41, §60

Caret movement by grapheme, selection, undo and surrounding-text access are the primitives Text Editor and every text field use at V0.5. A reserved preedit span is part of the model so the IME protocol attaches without redesign. Widgets live in UIP; this is the buffer and caret model.

<!-- covers: INV-0771 -->

#### Out of scope
Text field widgets (UIP-023). IME preedit rendering (TXT-022). UAX #9 bidi (TXT-038).

#### Acceptance criteria
- [ ] Caret moves by extended grapheme cluster; a test with a combining sequence and an emoji ZWJ sequence places the caret on cluster boundaries only.
- [ ] Selection, insert, delete and undo round-trip on a Latin buffer and preserve a reserved preedit span that insert does not merge.
- [ ] Surrounding-text access returns the documented window around the caret for the IME protocol's surrounding-text message.
- [ ] Text Editor scripted acceptance uses this model for typing, select-all and undo.

#### Verification
- Unit: `text:tests/edit_model_*` on `qemu-x86_64` and `hw-h002`.
- Integration: Text Editor scripted scenario on H-003.
- Review: UIP text-widget task can bind without changing the buffer API.

#### Evidence
- none

### TXT-013 · Integrate shaping and rasterisation into the toolkit retained text layout
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: TXT-003, TXT-004, TXT-006, TXT-012
- Baseline: §41, §60, §39

V0.5 exit requires a native application to render declarative UI. This task delivers shaped, rasterized, high-DPI-aware text runs as retained render nodes for Latin and simple scripts, with wrapping and alignment, on the decided stack. Complex scripts wait for V2. UIP widgets consume the nodes.

<!-- covers: INV-0771, INV-0781 -->

#### Out of scope
Widget set (UIP-024, UIP-023). Compositor (GFX). Bidi and line-break UAX suites (TXT-038). Shared atlas service (TXT-020).

#### Acceptance criteria
- [ ] A declarative text node shapes and rasterizes Latin, Cyrillic and Greek through FontMatcher and the decided libraries, wrapping to a width and aligning start, center and end.
- [ ] At 1x and 2x scale factors on H-002, glyph pixels are produced from the rasterizer and presented by the compositor without CPU bitmap scaling of the whole run.
- [ ] Terminal, File Browser, Text Editor and Image Viewer render their labels and bodies through these nodes, verified by the four-app scripted scenarios.
- [ ] No `unsafe` outside the decided rasterization crate boundary listed by TXT-003.

#### Verification
- Integration: four-app scenarios on H-003 and H-002.
- Unit: `text:tests/layout_v0_*` on `qemu-x86_64`.
- Demo: Text Editor and Terminal showing shaped Latin text on H-002.
- Review: UIP lead confirms widgets consume the retained nodes without a second shaper.

#### Evidence
- none

### TXT-014 · Benchmark per-Component text memory and shaping CPU with shared versus private caches
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: TXT-020
- Baseline: §10, §41, §54
- Benchmarks: B-008, B-016

Cheap isolation means many Components render text. The shared read-only atlas exists on a memory and startup claim; this harness publishes per-Component text-cache memory and shaping CPU for N Components with private caches and with the service-minted atlas, feeding B-008 and B-016. Targets stay in the register.

<!-- covers: GAP-0248 -->

#### Out of scope
Atlas implementation (TXT-020). Idle-Component memory methodology (BEN).

#### Acceptance criteria
- [ ] The harness reports resident text-cache memory and shaping CPU for a documented N of otherwise identical Components, private versus shared atlas.
- [ ] Reports exist for H-001 and H-002 under B-008 and B-016 method notes.
- [ ] No TXT prose states a numeric memory or CPU target.

#### Verification
- Bench: B-008 and B-016 on H-001 and H-002; target per register.
- Integration: both cache modes are selectable in the harness without rebuilding applications.

#### Evidence
- none

### TXT-015 · Decide the cross-Component glyph atlas and shaped-text cache sharing model
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: TXT-008, TXT-004, SEC-029
- Baseline: §41, §51, §67
- Decision: D-0317
- Risks: R-071, R-080
- Threats: T-015, T-030
- Invariants: I-083

V1 daily-driving runs many text-rendering Components. This adr decides how glyph rasters and shaped runs are shared: per-Component caches, a read-only atlas minted by the text service, or a shared writable atlas. I-083 and T-030 constrain the writable option. SEC-029 is an input because a shared atlas is a named cross-domain channel.

<!-- covers: GAP-0248 -->

#### Out of scope
Parser sandbox (TXT-018). Service implementation (TXT-020).

#### Acceptance criteria
- [ ] Option A (per-Component caches), Option B (read-only atlas minted by the text service) and Option C (shared writable atlas) are evaluated against I-083, T-030 and T-015.
- [ ] The accepted option states the MemoryObject rights of atlas pages and that clients cannot map them writable.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: TXT and SEC leads sign off on the pull request that accepts the decision file.
- Manual: the decision cites SEC-029, T-015 and T-030.

#### Evidence
- none

### TXT-016 · Decide the locale data source between ICU/CLDR and an ICU4X port
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §41, §67
- Decision: D-0319
- Invariants: I-009

Formatting, collation, plural rules and time zones need a CLDR-backed implementation before the V2 Locale object, localization pipeline and CLDR/tzdata Package. This adr decides ICU/CLDR versus an ICU4X port, applying Principle 15.

<!-- covers: GAP-0257 -->

#### Out of scope
Locale object (TXT-031). Data Package updates (TXT-024). System locale settings (SVC-025).

#### Acceptance criteria
- [ ] Option A (ICU with CLDR) and Option B (ICU4X port) are evaluated for data freshness, binary size, sandboxability and Principle 15.
- [ ] The accepted option names the crate or library, the data-load path, and that tzdata/CLDR can update without a new SystemGeneration.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: TXT and SDK leads sign off on the pull request that accepts the decision file.

#### Evidence
- none

### TXT-017 · Decide the message catalog format between Fluent and gettext
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §41, §52, §66
- Decision: D-0320

First-party applications accumulate string sites from V0.5. Deciding the catalog format and the SDK string API beside SDK v1 prevents rewriting every UI string site before the V2 localization framework.

<!-- covers: GAP-0258 -->

#### Out of scope
Pipeline implementation (TXT-032). Translation platform (TXT-036). SDK crate publication (SDK-059).

#### Acceptance criteria
- [ ] Option A (Fluent) and Option B (gettext) are evaluated for plurals, gender, bi-directional isolation, extraction tooling and SDK ergonomics.
- [ ] The accepted option names the on-disk catalog format, the SDK lookup API, and that catalogs ship as Package resources.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: TXT and SDK leads sign off on the pull request that accepts the decision file.

#### Evidence
- none

### TXT-018 · Parse untrusted fonts in an isolated Component before atlas admission
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-008, TXT-015
- Baseline: §10, §41, §51
- Threats: T-030
- Invariants: I-083

User-installed and personality-supplied fonts are untrusted bytes. A capability-scoped Component parses and sanity-checks them with no display or file authority beyond the font object, so the text service that mints shared atlases never parses those bytes in its own address space.

<!-- covers: GAP-0248 -->

#### Out of scope
Atlas minting (TXT-020). Package installation (TXT-007). Personality font directories (LNX, WIN).

#### Acceptance criteria
- [ ] A parser Component started with only `Capability<FontObject, Read>` parses a fixture font and returns a validated face token; it holds no GPU, display or directory capability, verified by `os inspect`.
- [ ] A malformed font fixture causes the parser Component to exit with a typed error and does not crash the text service or compositor, verified by a kill-or-panic test.
- [ ] Atlas admission accepts only tokens produced by a successful parse.
- [ ] Fuzzing the parser for the documented nightly window produces no host process panic.

#### Verification
- Unit: `text:tests/font_parse_sandbox_*` on `qemu-x86_64`.
- Integration: `os inspect` dump of the parser Component on H-003 showing the capability set.
- Fuzz: `text:fuzz/font_parse` nightly without host panic.
- Review: SEC lead confirms T-030 is addressed for the parse step.

#### Evidence
- none

### TXT-019 · Expose the native font store to the Linux Personality through fontconfig
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-006, TXT-007, LNX-052, LNX-024
- Baseline: §47, §49
- Corpora: C-003

V1 L2 integration scores scaling, and §49 requires compatibility windows to feel native. Generating fontconfig configuration and font directories from the native store gives browser, IDE and Wine (consumed by WIN) the same families and fallback chains as native applications. Native software still does not load fontconfig.

<!-- covers: GAP-0246, GAP-0251 -->

#### Out of scope
Personality process hosting (LNX-052). Wine font directories (WIN). Native FontMatcher (TXT-006).

#### Acceptance criteria
- [ ] A generated fontconfig view lists the same family, weight and script fallbacks as FontMatcher for the default set.
- [ ] A Linux-personality process using that view, and a native Component using FontMatcher, resolve the same file identity for a Latin default-family request.
- [ ] A native Component address space contains no fontconfig library or `FONTCONFIG_PATH`, verified by `os inspect` on Terminal.
- [ ] C-003 browser and IDE entries on H-002 use the generated view without shipping a second font set.

#### Verification
- Integration: FontMatcher versus fontconfig resolution fixture on H-003.
- Compat: C-003 browser and IDE entries on H-002 using the generated view.
- Unit: `text:tests/fontconfig_bridge_*` on `qemu-x86_64`.

#### Evidence
- none

### TXT-020 · Build the text service minting read-only glyph atlas and shaped-run MemoryObjects
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: TXT-015, TXT-018, TXT-013, MEM-024, SVC-015
- Baseline: §16, §32, §41, §51
- Risks: R-071
- Threats: T-015, T-030
- Invariants: I-083

V1 daily-driving multiplies text-rendering Components. The text service shapes and rasterizes once per system and hands clients read-only MemoryObject atlas pages and shaped-run cache entries with an eviction and generation protocol. Clients never map atlas pages writable (I-083).

<!-- covers: GAP-0248 -->

#### Out of scope
Restart and rebind (TXT-023). GPU Buffer objects (GFX). MemoryObject GPU backing (MEM-024). Parser Component (TXT-018).

#### Acceptance criteria
- [ ] Two Components requesting the same glyph at the same size receive MemoryObjects whose physical pages match and whose mapping rights are read-only; a writable map returns `Error::Rights`.
- [ ] Eviction of an atlas page increments a generation; a client using a stale generation is told to re-request and does not read another Component's pixels.
- [ ] Untrusted fonts reach the atlas only through TXT-018 tokens.
- [ ] `os inspect` on the text service shows atlas pages, generations and client Components.

#### Verification
- Unit: `text:tests/atlas_readonly_*` on `qemu-x86_64` and `hw-h002`.
- Integration: two-Component identical-run fixture with page-identity check on H-002.
- Fuzz: generation and eviction protocol without panic, nightly.
- Review: SEC lead confirms T-030 is not reopened by writable mappings.

#### Evidence
- none

### TXT-021 · Route Capability<TextInputFocus> to the IME Component for the focused field only
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-022, UIP-012, CAP-003
- Baseline: §9.1, §41
- Threats: T-001
- Invariants: I-021

The compositor mints a per-focus `Capability<TextInputFocus>` for the focused field. An IME Component without that capability receives no key events. A permanent denial test proves no global key stream exists, which the capability model requires (T-001, I-021).

<!-- covers: GAP-0252, INV-0780 -->

#### Out of scope
Focus arbitration policy (UIP-005). Engine host (TXT-029). Rights encoding (CAP-010).

#### Acceptance criteria
- [ ] Focusing a field mints `Capability<TextInputFocus>` to the IME Component; blurring revokes it and the next key event is not delivered to that IME, verified within one Operation.
- [ ] An IME Component that never held the capability records zero key events during a typed sentence in another field.
- [ ] `os inspect` shows the capability holder equal to the focused field's IME and no other Component.
- [ ] A regression named `ime_no_global_keys` is retained permanently.

#### Verification
- Unit: `text:tests/ime_focus_cap_*` on `qemu-x86_64`.
- Integration: denial scenario on H-003 with `os inspect` dump.
- Review: CAP lead confirms attenuation and revocation match CAP-004.

#### Evidence
- none

### TXT-022 · Implement the native input-method protocol in the toolkit text fields
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: TXT-010, TXT-012, TXT-013
- Baseline: §41, §66

Preedit rendering, commit, surrounding text, cursor rectangle and candidate placement are implemented against S-016 so real fields exercise the protocol before the V1 freeze candidate. Engines arrive at V2; a test IME Component is enough here.

<!-- covers: GAP-0252, INV-0780 -->

#### Out of scope
Hosted engines (TXT-029). Focus capability minting (TXT-021). Widgets other than text fields (UIP).

#### Acceptance criteria
- [ ] A test IME driving a toolkit field shows preedit in the reserved span, commits replace preedit, and surrounding-text queries match the editing model.
- [ ] Cursor rectangle and candidate-placement messages are emitted on caret move, with coordinates in the protocol's surface space.
- [ ] A field without an IME still types Latin through the editing model.
- [ ] The V1 UI-protocol freeze-candidate suite includes these optional methods as exercised, not frozen.

#### Verification
- Unit: `text:tests/ime_protocol_field_*` on `qemu-x86_64` and `hw-h002`.
- Integration: test IME against Text Editor and a Settings text field on H-003.
- Review: UIP lead confirms S-016 messages match the IDL registered at V0.5.

#### Evidence
- none

### TXT-023 · Make the text service restartable with client rebind and cache regeneration
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-020, SVC-009
- Baseline: §32, §41
- Benchmarks: B-024
- Risks: R-023
- Invariants: I-037

SVC supervision from V0.5 requires every shared service to restart and rebind. A crashed text service must not blank text in running applications during V1 daily-driving: clients rebind, revalidate atlas generations and re-request evicted runs.

#### Out of scope
Supervisor policy (SVC-005). Atlas minting (TXT-020). Compositor rebind (GFX).

#### Acceptance criteria
- [ ] Killing the text service N times in CI on H-003 leaves Terminal and Text Editor presenting text after rebind, with no application exit.
- [ ] After rebind, clients refuse stale atlas generations and re-request runs; a stale map does not display.
- [ ] B-024 is published for the text service on H-003.
- [ ] `os inspect` shows the rebound Channel and a new atlas generation.

#### Verification
- Integration: kill loop on H-003 and H-002 using BLD-020.
- Bench: B-024 on H-003; target per register.
- Unit: `text:tests/atlas_rebind_*` on `qemu-x86_64`.

#### Evidence
- none

### TXT-024 · Ship CLDR, tzdata and Unicode data as a Package updating independently of SystemGenerations
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-016, TXT-031, PKG-038
- Baseline: §28, §30, §41

Locale and time-zone data change faster than the OS. A content-addressed data Package with hot reload in the text and locale services, coordinated with PKG independent update and SVC time settings, keeps formatting and zones current between SystemGenerations.

<!-- covers: GAP-0257 -->

#### Out of scope
Time sync client (SVC-032). Generation switch (PKG). Locale API (TXT-031).

#### Acceptance criteria
- [ ] Installing a newer tzdata/CLDR Package does not create a new SystemGeneration and does not write outside the store.
- [ ] Locale formatting after reload uses the new data for a documented timezone fixture without restarting applications that rebound the data Channel.
- [ ] Rolling back the data Package restores previous formatting, verified by a fixture timestamp.
- [ ] `os inspect` shows the data Package identity held by the locale service.

#### Verification
- Integration: data-Package update and rollback on H-003.
- Unit: `text:tests/cldr_reload_*` on `qemu-x86_64`.
- Review: PKG lead confirms independent update does not mutate the running generation's package tree.

#### Evidence
- none

### TXT-025 · Render emoji ZWJ sequences and COLRv1 colour glyphs
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-002, TXT-020, TXT-039
- Baseline: §41

Emoji presentation selectors, modifier and ZWJ sequences, and COLRv1 rasterization into the shared atlas use the emoji font selected in the font-set decision. V2 shell and messaging-class Layer 3 applications expose emoji everywhere text runs.

<!-- covers: GAP-0249, GAP-0247 -->

#### Out of scope
Emoji picker UI (APP-028). Font-set decision (TXT-002).

#### Acceptance criteria
- [ ] A ZWJ sequence in the Unicode emoji test list renders as a single cluster using the COLRv1 emoji font, not as separate monochrome glyphs.
- [ ] Skin-tone modifiers affect the documented sequences and are stored as one atlas key.
- [ ] Presentation selectors choose emoji versus text style as the Unicode tables specify.
- [ ] Atlas pages for color glyphs remain read-only MemoryObjects (I-083).

#### Verification
- Unit: `text:tests/emoji_zwj_*` and `text:tests/colrv1_*` on `qemu-x86_64` and `hw-h002`.
- Integration: Text Editor and a shell surface showing a ZWJ sequence on H-002.
- Review: coverage matrix from TXT-002 includes the emoji family.

#### Evidence
- none

### TXT-026 · Decide hosting existing IME engines versus native engines and the 1.0 language list
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: TXT-022, TXT-021
- Baseline: §41, §67
- Decision: D-0318
- Invariants: I-009

CJK, Indic and Vietnamese users cannot type without engines. This adr decides sandboxed hosting of existing engines (librime, libpinyin, anthy, libhangul, m17n) versus writing native engines, and records the 1.0 input-language list. Principle 15 argues against a rewrite; the V4 CJK gate needs engines from V2.

<!-- covers: GAP-0253 -->

#### Out of scope
Host Component (TXT-029). Chinese engine bring-up (TXT-028). Remaining engines (TXT-042).

#### Acceptance criteria
- [ ] Option A (sandboxed hosting of librime, libpinyin, anthy, libhangul, m17n) and Option B (native-written engines) are evaluated against Principle 15, sandboxability and the V4 CJK gate.
- [ ] The accepted option lists the 1.0 input languages and which engine covers each.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: TXT lead sign-off recorded on the pull request that accepts the decision file.

#### Evidence
- none

### TXT-027 · Decide hinting, subpixel positioning and gamma policy across scale factors
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: TXT-013, TXT-019
- Baseline: §41, §49
- Decision: D-0322
- Risks: R-042

V2 introduces fractional and per-display scaling. Mixed-fidelity text between native and compatibility windows is the visible signal that a compat app is not native, which §49 forbids. This adr decides hinting, subpixel positioning and gamma so both paths render the same at every fractional scale.

<!-- covers: GAP-0250 -->

#### Out of scope
Pixel-parity implementation (TXT-035). Compositor scaling (GFX, UIP).

#### Acceptance criteria
- [ ] Option A (one policy applied natively and projected into personality FreeType/fontconfig) and Option B (independent native and personality policies) are evaluated against §49.
- [ ] The accepted option names hinting mode, subpixel positioning and gamma at 1x, 1.25x, 1.5x and 2x.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: TXT and LNX leads sign off on the pull request that accepts the decision file.

#### Evidence
- none

### TXT-028 · Ship the first hosted IME engine for Chinese pinyin input
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-029
- Baseline: §41
- Benchmarks: B-045

The first engine through the host proves protocol, capability routing and candidate window end to end at V2. Remaining 1.0 languages wait for V3 so the V4 CJK gate has a full milestone of soak. Keystroke-to-glyph is published under B-045.

<!-- covers: GAP-0253, INV-0780 -->

#### Out of scope
Host lifecycle (TXT-029). Japanese, Korean and remaining engines (TXT-042).

#### Acceptance criteria
- [ ] Pinyin input into a native text field commits Chinese characters through S-016, with candidates placed at the cursor rectangle.
- [ ] The engine Component holds `Capability<TextInputFocus>` and dictionary Package access only, verified by `os inspect`.
- [ ] A scripted sentence on H-002 and H-004 commits the documented character sequence.
- [ ] B-045 keystroke-to-glyph is published for this path on H-002.

#### Verification
- Integration: scripted pinyin scenario on H-002 and H-004.
- Bench: B-045 on H-002; target per register.
- Unit: `text:tests/ime_pinyin_*` on `qemu-x86_64`.

#### Evidence
- none

### TXT-029 · Build the sandboxed IME host Component with candidate window presentation
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: TXT-026, TXT-021, SVC-015
- Baseline: §9.1, §10, §41
- Threats: T-001
- Invariants: I-021

The IME host holds only `Capability<TextInputFocus>` and dictionary Package access, runs engine lifecycle and per-session switching, and places a candidate window at the protocol's cursor rectangle. The V2 localization framework and V4 CJK gate depend on it. No global key stream (I-021).

<!-- covers: GAP-0253, GAP-0252, INV-0780 -->

#### Out of scope
Individual engines (TXT-028, TXT-042). Wayland text-input (TXT-030). Focus minting (TXT-021).

#### Acceptance criteria
- [ ] The host Component's `os inspect` capability set is `TextInputFocus` plus dictionary Package read, with no filesystem, network or global input capability.
- [ ] Switching engines mid-session delivers subsequent commits from the new engine without restarting the client field.
- [ ] Candidate window geometry matches the last cursor-rectangle message, verified by a layout dump.
- [ ] Killing the host restarts it under SVC and restores IME after rebind without application exit.

#### Verification
- Integration: host kill/rebind on H-003; candidate placement on H-002.
- Unit: `text:tests/ime_host_*` on `qemu-x86_64`.
- Review: SEC lead confirms T-001 is not reopened.

#### Evidence
- none

### TXT-030 · Bridge text-input-v3 and input-method-v2 in the Wayland bridge to the native IME
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-029, LNX-006
- Baseline: §47, §49, §41
- Corpora: C-004

Linux GUI applications type through the native IME host via Wayland text-input protocols implemented inside LNX's bridge. TXT owns the mapping from those protocols onto S-016. §47 integration and the V4 gate that CJK input works in Linux-personality applications depend on this mapping.

<!-- covers: INV-0893 -->

#### Out of scope
Wayland compositor hosting (LNX-006, LNX-064). Wine IMM32/TSF (WIN). Native protocol (TXT-022).

#### Acceptance criteria
- [ ] A Wayland client using `zwp_text_input_v3` commits through the native IME host; the host still holds only `TextInputFocus` for that surface.
- [ ] `input-method-v2` candidate placement maps onto the S-016 cursor rectangle.
- [ ] A Linux-personality GTK text field on H-002 types pinyin via the native host without a second IME daemon in the personality.
- [ ] Native IDL still contains no Wayland types.

#### Verification
- Integration: GTK text-field pinyin scenario on H-002 beside a native field using the same host.
- Compat: a C-004 GUI entry that types CJK on H-002.
- Review: LNX lead confirms the bridge owns protocol code and TXT owns the mapping table.

#### Evidence
- none

### TXT-031 · Build the typed Locale Object with per-application override and formatting APIs
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-016, SVC-025
- Baseline: §41, §52

Locale is a typed SDK object for numbers, dates, plurals, collation and measurement, with per-application override and a session default from SVC settings storage. The V2 localization framework and §41 locale formatting require it. Native software does not read POSIX `LANG`.

<!-- covers: GAP-0257, INV-0775 -->

#### Out of scope
Settings service (SVC-025). Catalog lookup (TXT-032). Data Package (TXT-024). Personality `LANG` projection (LNX).

#### Acceptance criteria
- [ ] Formatting APIs for number, date, plural and collation return the decided CLDR-backed results for a fixture locale.
- [ ] An application override does not change another application's formatting, verified by two Components in one session.
- [ ] A Component with no locale capability receives the session default and cannot read another app's override.
- [ ] No native path reads `LANG`, `LC_*` or `/etc/locale.conf`.

#### Verification
- Unit: `text:tests/locale_format_*` on `qemu-x86_64`.
- Integration: two-Component override fixture on H-003 with SVC session default.
- Review: SDK lead confirms the object is Layer 3 over the Layer 2 locale settings Interface.

#### Evidence
- none

### TXT-032 · Build the SDK localisation pipeline with catalog extraction and packaged resources
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: TXT-017, TXT-031, TXT-038, PKG-030
- Baseline: §41, §28, §52
- Risks: R-042, R-058

Build-time string extraction, catalogs shipped as Package resources, runtime lookup with plural and bidi-aware formatting through the Locale object, and the SDK API. V2 exit requires the localization framework in place so the three-language shell is not a bolt-on (R-042).

<!-- covers: GAP-0258, INV-0775 -->

#### Out of scope
Public translation platform (TXT-036). Docs translation (DOC-025). First-party catalog filling (APP, TXT-033).

#### Acceptance criteria
- [ ] Extraction tooling emits catalogs in the decided format from SDK string sites, and a missing-string CI check fails a fixture that adds an unmarked literal in a UI crate.
- [ ] Catalogs are Package resources; runtime lookup returns the plural form selected by the Locale object.
- [ ] An RTL locale applies bidi isolation to interpolated values, verified by a fixture string.
- [ ] Terminal, File Browser, Text Editor and Image Viewer load catalogs through this API.

#### Verification
- Unit: `text:tests/catalog_lookup_*` on `qemu-x86_64`.
- Integration: extracted catalogs packaged and loaded on H-003.
- Review: SDK lead confirms the API is on the v1 crate with semver.

#### Evidence
- none

### TXT-033 · Translate the shell and four native applications into three languages
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-032, TXT-038, TXT-036
- Baseline: §41, §62
- Risks: R-042

V2 scope requires the shell translated into at least three languages, including one RTL language to exercise bidi. CI coverage and screenshot review of layout under translated strings make this a gate, not a screenshot-only hope.

<!-- covers: INV-0775 -->

#### Out of scope
Five-language expansion (TXT-041). Ten-language gate (TXT-046). APP string authorship beyond applying catalogs (APP).

#### Acceptance criteria
- [ ] Shell and the four V0.5 applications ship catalogs for three languages, one of which is RTL.
- [ ] CI reports string coverage per language for those surfaces; a missing catalog key fails the job.
- [ ] Screenshot review of an RTL layout on H-002 shows mirrored chrome and no clipped primary actions in the scripted settings scenario.
- [ ] Switching session locale via SVC-025 reloads catalogs without reinstalling Packages.

#### Verification
- Integration: locale-switch scenario on H-002 and H-004.
- Manual: screenshot review recorded on the pull request for the RTL language.
- Unit: coverage reporter fixture with a dropped key.

#### Evidence
- none

### TXT-034 · Build the spell-checking and hyphenation service with dictionary Packages
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: TXT-007, TXT-022, PKG-038
- Baseline: §41

A typed Interface over Hunspell dictionaries and hyphenation patterns, installed as per-language Packages, is consumed by toolkit text fields (underlines, suggestions, per-field opt-out for secrets). A V2 polished desktop needs this in every text field that opts in.

<!-- covers: GAP-0260 -->

#### Out of scope
Text fields (UIP). Package store (PKG). Secret fields' policy UX (APP, SEC).

#### Acceptance criteria
- [ ] A text field with spell-check on underlines a documented misspelling and returns ranked suggestions from the language's dictionary Package.
- [ ] Hyphenation points for a documented paragraph match the pattern Package for that language.
- [ ] A field marked secret sends no buffer contents to the service, verified by a Channel trace.
- [ ] Installing a dictionary Package enables that language without a SystemGeneration; uninstall removes suggestions for it.

#### Verification
- Unit: `text:tests/spell_*` and `text:tests/hyphen_*` on `qemu-x86_64`.
- Integration: Text Editor underline and suggestion scenario on H-003; secret-field trace.
- Review: SEC lead confirms secret fields do not cross the service Channel.

#### Evidence
- none

### TXT-035 · Apply the text rendering policy natively and in the Linux Personality with pixel parity tests
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-027, TXT-019
- Baseline: §41, §49
- Corpora: C-004

Implements subpixel positioning and fractional-scale rasterization in the toolkit, projects the same hinting, gamma and scale settings into personality fontconfig and FreeType properties, and adds a pixel-compare test at 1x, 1.25x, 1.5x and 2x that scores in the L3 integration check.

<!-- covers: GAP-0250 -->

#### Out of scope
Policy decision (TXT-027). Compositor fractional scaling (GFX, UIP). Settings portal (LNX-077).

#### Acceptance criteria
- [ ] Native toolkit rasterization applies the decided hinting, subpixel and gamma at 1x, 1.25x, 1.5x and 2x.
- [ ] Personality FreeType and fontconfig properties receive the same settings, verified by a dump compared to the native policy object.
- [ ] A pixel-compare of a documented Latin string at those scales between a native surface and a Linux-personality surface meets the match rule recorded in the decision.
- [ ] The compare job is part of C-004 integration scoring on H-002.

#### Verification
- Integration: pixel-compare harness on H-002 at the four scales.
- Compat: C-004 integration scoring includes the compare on H-002 and H-005.
- Unit: policy projection dump test on `qemu-x86_64`.

#### Evidence
- none

### TXT-036 · Stand up the public translation platform and catalog sync for first-party apps
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-032
- Baseline: §41
- Risks: R-058

Community translation is how the V3 five-language and V4 ten-language targets are reached. A public platform with automated catalog round-trip into first-party repositories and per-language coverage reporting makes those gates mechanical.

<!-- covers: GAP-0258 -->

#### Out of scope
Docs translation pipeline (DOC-025). Catalog format (TXT-017). Filling the first three languages (TXT-033).

#### Acceptance criteria
- [ ] Translators submit catalogs in the decided format; CI round-trips them into the first-party app repositories without hand copies.
- [ ] A coverage report per language and per Package is published with each merge to the translation branch.
- [ ] A rejected catalog that breaks placeholders fails CI before merge.
- [ ] The three V2 languages are imported through this path, not through a one-off dump.

#### Verification
- Integration: round-trip fixture in CI that lands a catalog and fails a broken placeholder.
- Review: DOC and TXT leads confirm docs translation remains a separate pipeline.
- Manual: coverage report URL or generated path recorded on the pull request.

#### Evidence
- none

### TXT-037 · Run Unicode and text-rendering conformance suites as a CI Gate
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-038, TXT-013
- Baseline: §41

BidiTest, BidiCharacterTest, LineBreakTest, GraphemeBreakTest, WordBreakTest and the text-rendering-tests corpus run in BLD CI against the layout engine and shaping stack, making GAP-0249's Unicode conformance claim a permanent gate.

<!-- covers: GAP-0249 -->

#### Out of scope
Layout implementation (TXT-038). CI runner platform (BLD).

#### Acceptance criteria
- [ ] The five UAX test files and the text-rendering-tests corpus run on every post-merge CI of the text crates.
- [ ] A known-fail allowlist is empty of default-ignore cases that the layout claims to implement; adding a fail without an adr task is a CI failure.
- [ ] Failures name the UAX test ID in the log.
- [ ] The job is required, not advisory.

#### Verification
- Integration: post-merge job on H-001 using BLD CI.
- Unit: a sabotaged grapheme fixture fails the job in a branch test.
- Review: BLD lead confirms the job is in the required tier.

#### Evidence
- none

### TXT-038 · Implement UAX #29 segmentation, UAX #9 bidi and UAX #14 line breaking in layout
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: TXT-013, TXT-012
- Baseline: §41

Grapheme and word segmentation, bidirectional resolution and reordering, and line breaking in the toolkit layout engine are required for the V2 three-language shell (including an RTL script) and the §41 localization requirement of bidi.

<!-- covers: GAP-0249, INV-0775 -->

#### Out of scope
Conformance CI gate (TXT-037). Vertical CJK (TXT-040). Editing model grapheme caret (TXT-012).

#### Acceptance criteria
- [ ] Grapheme and word boundaries match UAX #29 for the default test file except explicitly listed, numbered exceptions.
- [ ] BidiTest and BidiCharacterTest pass for the levels the layout claims, with reordering visible in a retained-node dump.
- [ ] LineBreakTest passes for the classes the layout claims; an RTL paragraph wraps with the decided base direction.
- [ ] An RTL fixture in Text Editor shows caret movement on grapheme boundaries after reordering.

#### Verification
- Unit: `text:tests/uax9_*`, `text:tests/uax14_*`, `text:tests/uax29_*` on `qemu-x86_64` and `hw-h002`.
- Integration: RTL Text Editor scenario on H-002.
- Review: coverage of claimed UAX classes is listed in the crate docs.

#### Evidence
- none

### TXT-039 · Support variable font axes and OpenType feature selection end to end
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-006, TXT-020, TXT-013
- Baseline: §41

Variation axes and feature tags flow from FontMatcher requests through shaping, the atlas key and rasterization. The V2 polished shell relies on weight axes for typography, and the toolkit exposes features to applications.

<!-- covers: GAP-0249 -->

#### Out of scope
FontMatcher family fallback (TXT-006). COLRv1 emoji (TXT-025).

#### Acceptance criteria
- [ ] A FontMatcher request with a weight axis value selects a variable-font instance; two weights produce distinct atlas keys and distinct rasters.
- [ ] Feature tags (liga, smcp, or the decided set) enable and disable in shaping, verified by glyph-ID dumps.
- [ ] Atlas eviction treats axis plus feature tags as part of the key so two instances never share a stale raster.
- [ ] Shell typography uses a weight axis rather than a second static family for the documented style.

#### Verification
- Unit: `text:tests/vf_axes_*` and `text:tests/ot_features_*` on `qemu-x86_64`.
- Integration: shell typography screenshot vs two weights on H-002.
- Review: FontMatcher IDL includes axis and feature fields.

#### Evidence
- none

### TXT-040 · Support vertical CJK text layout and orientation features in the toolkit
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-038, TXT-039
- Baseline: §41

Vertical writing mode with vert/vrt2 features and upright or rotated Latin runs in the layout engine is needed for CJK text fidelity before the V4 CJK gate and the ten-language target.

<!-- covers: GAP-0249 -->

#### Out of scope
IME engines (TXT-042). Conformance CI (TXT-037).

#### Acceptance criteria
- [ ] A vertical writing-mode node places CJK glyphs in the vertical flow and applies vert or vrt2 substitutions when the font provides them.
- [ ] Latin runs in that node are upright or rotated per the decided orientation feature, verified by glyph advances.
- [ ] Line breaking in vertical mode uses the UAX #14 classes claimed by TXT-038.
- [ ] A documented Japanese fixture renders without overlapping glyphs on H-002.

#### Verification
- Unit: `text:tests/vertical_cjk_*` on `qemu-x86_64` and `hw-h002`.
- Integration: Japanese vertical fixture on H-002.
- Review: CJK coverage matrix from the font-set decision includes a font with vert/vrt2.

#### Evidence
- none

### TXT-041 · Bring the shell and native applications to five languages via the translation platform
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-033, TXT-036
- Baseline: §41, §63
- Risks: R-058

V3 scope caps localization at five languages. Reaching five through the public platform validates community translation before the V4 ten-language gate and the V3 public documentation.

<!-- covers: INV-0775 -->

#### Out of scope
Ten-language coverage gate (TXT-046). Documentation catalogs (DOC-025).

#### Acceptance criteria
- [ ] Shell and shipped native applications have catalogs for five languages imported through TXT-036.
- [ ] CI coverage reports five languages; a language below the V3 threshold recorded in the verifying report fails the job.
- [ ] At least one of the five is RTL and reuses the V2 RTL layout review.
- [ ] No language is added by a one-off dump that bypasses the platform.

#### Verification
- Integration: coverage job on post-merge CI.
- Manual: platform import log for the two languages added after V2.
- Review: DOC lead confirms user-facing shell strings are not duplicated in docs catalogs.

#### Evidence
- none

### TXT-042 · Ship hosted IME engines for Japanese, Korean and the rest of the 1.0 language list
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-026, TXT-028, TXT-029
- Baseline: §41

Completes the engine list from the IME strategy decision (anthy, libhangul, m17n-based Vietnamese and Indic, and any other language on the 1.0 list) so the V4 gate can test CJK input on Tier 1 with a full milestone of soak.

<!-- covers: GAP-0253, INV-0780 -->

#### Out of scope
Host (TXT-029). Chinese pinyin (TXT-028). V4 CJK gate (TXT-044).

#### Acceptance criteria
- [ ] Each 1.0 input language from the decision has a hosted engine Package that commits through S-016 on H-003.
- [ ] Japanese (anthy or the decided engine) and Korean (libhangul or the decided engine) pass scripted sentences on H-002.
- [ ] Engine Components hold the same capability set as the Chinese engine, verified by `os inspect`.
- [ ] Session switching among Chinese, Japanese and Korean does not leak key events across engines.

#### Verification
- Integration: scripted JP and KR sentences on H-002 and H-004.
- Unit: `text:tests/ime_engines_list_*` on `qemu-x86_64`.
- Review: the 1.0 language list in the decision file is checked off.

#### Evidence
- none

### TXT-043 · Write the SDK guide for fonts, text layout, input methods and localisation
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-032, TXT-006, TXT-022, TXT-031, DOC-014
- Baseline: §41, §52, §56.5

V3 exit requires the SDK guide published and reviewed. FontMatcher, IME protocol, Locale and catalog APIs need developer guidance so third-party Packages localize correctly. DOC generates references; this task authors the TXT chapters.

#### Out of scope
IDL-to-docs generator (DOC-010). Site publishing (DOC). SDK crate API freeze (SDK).

#### Acceptance criteria
- [ ] The guide contains worked examples for FontMatcher, text layout, attaching an IME, Locale formatting and catalog lookup.
- [ ] Every example builds against SDK v1 in CI.
- [ ] Native examples do not call fontconfig, Pango, POSIX locales or Wayland text-input.
- [ ] DOC's developer guide links these chapters as the normative TXT text.

#### Verification
- Review: SDK and DOC leads sign off on the pull request.
- Integration: example crates compile in docs CI.
- Manual: broken-link check includes the new chapters.

#### Evidence
- none

### TXT-044 · Verify CJK input methods in native and Linux-Personality applications on Tier 1
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-042, TXT-030, TXT-040, LNX-064, LNX-107
- Baseline: §41, §47, §49

V4-G11 requires CJK input methods functional in native and Linux-personality applications. Scripted typing scenarios for Chinese, Japanese and Korean run on every Tier 1 machine each RC.

<!-- covers: GAP-0253, INV-0780, INV-0893 -->

#### Out of scope
IBus and text-input host (LNX-064). Engine implementation (TXT-042).

#### Acceptance criteria
- [ ] Scripted Chinese, Japanese and Korean typing scenarios pass on every machine in V4 hardware scope for a native text field.
- [ ] The same three scenarios pass for a Linux-personality GTK or Qt field on those machines via TXT-030.
- [ ] Failures name machine H-ID, language and scenario ID.
- [ ] The job runs each RC, not only once.

#### Verification
- Integration: RC matrix on H-002, H-004, H-005, H-006, H-007, H-008, H-009, H-010, H-011, H-012, H-013, H-014.
- Compat: Linux-personality CJK scenarios scored on the same RC matrix.
- Demo: native and Linux-personality fields committing CJK on H-002.

#### Evidence
- none

### TXT-045 · Lock 1.x versions of FontMatcher, TextInput, Locale and SpellCheck interfaces
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-006, TXT-010, TXT-031, TXT-034, TXT-022, IPC-062, TXT-002, TXT-011, TXT-026
- Baseline: §12, §66
- Freezes: S-016
- Invariants: I-040

V4 exit requires Layer 2 interface versions enumerated and locked, with the old-client/new-service and new-client/old-service evolution test passing for every core Interface. S-016, registered at V0.5, is frozen here and not earlier. FontMatcher, Locale and SpellCheck versions are locked in the same pass even though they are not separate S-IDs.

<!-- covers: EXTRA-066, GAP-0246 -->

#### Out of scope
Layer 1 freeze (ABI, I-040). UI protocol S-015 freeze (UIP). IPC-wide version enumeration (IPC-068).

#### Acceptance criteria
- [ ] S-016 state is `frozen` and `Frozen by` names this task.
- [ ] FontMatcher, TextInput, Locale and SpellCheck 1.x versions are listed and the IPC evolution matrix is green for old-client/new-service and new-client/old-service.
- [ ] A v0.5-era TextInput client still runs against the frozen service.
- [ ] No Layer 1 surface is listed in `Freezes`.

#### Verification
- Integration: IPC-062 entries for the four Interfaces.
- Review: ABI/IPC lead confirms S-016 freeze closure (spike/decision/register) and I-040 is intact.
- Unit: version-negotiation fixture on `qemu-x86_64`.

#### Evidence
- none

### TXT-046 · Reach ten languages at the V4-G11 string-coverage threshold with a CI coverage Gate
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-041, TXT-036
- Baseline: §41, §63
- Risks: R-058

V4-G11 names the language count and coverage threshold in this verifying task: ten languages at 95 percent string coverage for shell and native applications. Coverage measurement is a blocking CI gate. APP ships translations; TXT measures and gates.

<!-- covers: INV-0775 -->

#### Out of scope
APP catalog authorship (APP-066). Platform (TXT-036). Docs languages (DOC).

#### Acceptance criteria
- [ ] CI reports ten languages at or above 95 percent string coverage for shell and shipped native applications; below that fails the required job.
- [ ] Coverage is computed from the translation platform catalogs, not from a hand-written checklist.
- [ ] The five V3 languages remain at or above the V3 threshold.
- [ ] Feature freeze cannot proceed while this job is red.

#### Verification
- Integration: required coverage job on every RC.
- Review: APP lead confirms first-party surfaces are in the measured set.
- Manual: coverage report path recorded per RC.

#### Evidence
- none

### TXT-047 · Re-verify the localisation and IME gates on the 1.0 release build
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: TXT-046, TXT-044, TXT-045
- Baseline: §41, §63

1.0 exit requires the V4 accessibility and localization gates to hold on the release build: ten languages at the V4 coverage threshold, CJK input tested. This task re-runs coverage, Unicode conformance and IME scenarios on the final release candidate across Tier 1.

#### Out of scope
New languages or engines. Layer 2 thaw. APP soak (APP-067).

#### Acceptance criteria
- [ ] The ten-language coverage job is green on the 1.0 release-candidate image for every Tier 1 H-ID in 1.0 hardware scope.
- [ ] Chinese, Japanese and Korean IME scenarios pass on that image for native and Linux-personality fields.
- [ ] Unicode conformance CI is green on that image.
- [ ] S-016 remains `frozen`.

#### Verification
- Integration: 1.0 RC matrix repeating TXT-046 and TXT-044 jobs.
- Review: release engineer records the three job URLs on the 1.0 checklist.
- Compat: Linux-personality CJK scenarios on the RC image.

#### Evidence
- none
