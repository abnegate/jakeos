# REL · Release engineering and security response
- Prefix: REL
- Lead: none
- Baseline: §27, §28, §30, §31, §49, §54, §56.4, §62, §63

<!-- roadmap:generated:begin summary -->
Tasks: 67 live, 0 done, 0 in-progress, 67 todo, 0 dropped. Ready: 1. Blocked: 66. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

REL owns how JakeOS leaves the build farm and how the installed base stays honest: the signing key hierarchy and custody, Package and SystemGeneration signatures, repository server and metadata (including TUF-style roles), update channels and promotion of identical artifacts, corresponding-source and notices publication, SBOM and provenance publication, CVE ingestion and advisory feeds, emergency withdrawal, staged rollouts, the Hardware Compatibility List and public compatibility database, the crash and opt-in telemetry intake pipeline, and the 1.0 stable-channel launch. Installation makes Packages available; an update creates a new SystemGeneration rather than mutating the running image (§27, §28, §30). Native software never sees POSIX, Linux syscalls or Win32 (§3, §57). Kernel live-patching is out; generations plus reboot are the update model (I-086).

## Out of scope

Package format, store layout, repository client, generation compose and client-side revocation (PKG). Image builder, installer, updater client, recovery environment and crash-report consent UI (INS). Hermetic builds, SBOM generation, symbol upload, release-qualification CI jobs and promote-without-rebuild mechanics (BLD). Bootloader, Secure Boot strategy, SBAT enforcement in firmware and shim integration (BOOT). License firewall, legal entity, telemetry policy, repository intermediary terms and published support contract (GOV). Threat model, CNA, SECURITY.md and identity (SEC). Crash capture format, local metrics and inspect (OBS). Store client and consent chrome (APP). Probe schema, live-image checker and lab SKUs (HW). `os publish` and host SDK (SDK). Kernel CVE backport (KRN). Benchmark register and methodology (BEN). Personality corpora and failure reasons (LNX, WIN). VM-offer action (VIRT). Docs site (DOC). Content-store substrate (STO). Lab racking (LAB).

## Tasks

### REL-001 · Decide release, SystemGeneration and Channel versioning
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §28, §30, §66
- Decision: D-0240
- Risks: R-017
- Invariants: I-080

Packages, SystemGenerations and public release names need one vocabulary before the first immutable install (§28, §30). Layer evolution rules stay with ABI and SDK (§66); this Decision only names how a generation number, a channel and a public release relate, so V0.5 manifests can reserve identity fields without a later format break (I-080).

<!-- covers: GAP-0061, GAP-0344 -->

#### Out of scope
Layer 1 and Layer 2 evolution rules (ABI). SDK crate semver (SDK). Channel promotion criteria (REL-004).

#### Acceptance criteria
- [ ] Options evaluated include (A) generation counters plus channel names for OS artifacts and semver for Layer 2 and Layer 3 interfaces, (B) semver for every artifact including generations, and (C) calendar-free public names plus opaque generation IDs.
- [ ] The accepted option states how a SystemGeneration number relates to a public release name and to a channel.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and PKG leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-002 · Decide the signing key hierarchy and custody model
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-001
- Baseline: §28, §30, §51, §63
- Decision: D-0244
- Risks: R-030
- Threats: T-028, T-029
- Invariants: I-080

Every update, rollback and later Secure Boot chain rests on who can sign a SystemGeneration (§30, §63). This Decision fixes an offline root, per-channel intermediates and publisher keys before V1 devices enroll trust, and records custody so a leaked key is a rehearsed incident rather than an improvised one (T-028, T-029).

<!-- covers: GAP-0322 -->

#### Out of scope
Wire-format signatures (REL-003). Production ceremony text (REL-032). Bootloader verification (BOOT-027). Legal-entity holder of the root (GOV-024).

#### Acceptance criteria
- [ ] Options evaluated include (A) offline root with HSM-backed threshold intermediates per channel plus publisher keys, (B) software-held root with delayed HSM migration, and (C) a single online root for all artifacts.
- [ ] The accepted option names which keys sign bootloader, kernel, SystemGeneration, Package and repository metadata, and who holds quorum shares.
- [ ] The accepted option records rotation and compromise as follow-ups, not as unstated assumptions.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL, BOOT and GOV leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-003 · Decide Package and SystemGeneration signing scheme
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-002, PKG-029
- Baseline: §27, §28, §30
- Decision: D-0245
- Risks: R-030
- Threats: T-006, T-007, T-019
- Invariants: I-036, I-080

V1 daily-driving requires a signed remote repository whose clients reject tampering before a Package becomes available (§27, §28). The scheme is fixed before the first signed artifacts so V3 TUF roles, if chosen, extend the same identity rather than replacing it.

<!-- covers: GAP-0542, INV-0540 -->

#### Out of scope
TUF role implementation (REL-037). Client verification (PKG-055). Key ceremony (REL-032).

#### Acceptance criteria
- [ ] Options evaluated include (A) a signed content-addressed index at V1 with TUF roles added at V3, (B) TUF root, targets, snapshot and timestamp from the first channel, and (C) per-Package signatures only with no repository metadata roles.
- [ ] The accepted option states what a client must verify before activation and how a mix-and-match or replayed index is detected, or records that detection waits for REL-037.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and PKG leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-004 · Define update channels and promotion criteria
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-001
- Baseline: §30, §61
- Decision: D-0248
- Invariants: I-022, I-086

Internal daily-driving needs a nightly channel and a written path to testing, then later stable and LTS, so promotion is not renegotiated per release (§30). Live-patching remains a non-goal; a channel promote publishes a new SystemGeneration (I-086).

<!-- covers: GAP-0338 -->

#### Out of scope
Nightly and testing implementation (REL-005). Stable and LTS operation (REL-057). Client channel picker (INS-046).

#### Acceptance criteria
- [ ] Options evaluated include (A) nightly and testing at V1 with stable and LTS added later, (B) all four channels from V1, and (C) a single rolling channel with named tags.
- [ ] The accepted option names soak criteria, who may promote, and that promotion never rebuilds the artifact.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and INS leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-005 · Implement nightly and testing update channels
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-004, REL-007
- Baseline: §30, §61
- Invariants: I-022, I-086

V1 scope is a signed remote repository and the first update channels for daily-driving developers. This task implements nightly and testing only; stable waits for V4. A promote copies already-signed objects into the channel head and never mutates a published generation (§30).

#### Out of scope
Stable and LTS (REL-057). Artifact pipeline without rebuild (REL-019). Updater client (INS-045).

#### Acceptance criteria
- [ ] Nightly and testing channel heads exist as signed repository metadata a PKG-064 fetch can name.
- [ ] Promoting an artifact from nightly to testing leaves content hashes unchanged.
- [ ] A client subscribed to testing does not receive a nightly-only generation.
- [ ] Channel names match REL-004.

#### Verification
- Integration: `rel:tests/channels/dev_promote` on CI matrix entry `qemu-x86_64` (H-001).
- Review: REL lead confirms hashes are identical across the promote.

#### Evidence
- none

### REL-006 · Automate CVE ingestion against the forked kernel tree
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-030, KRN-031, PKG-046
- Baseline: §56.4
- Risks: R-062
- Threats: T-007

§56.4 makes inherited CVE response a permanent cost of the fork. REL matches upstream Linux security fixes against the forked tree, flags ones that touch diverged code, and opens tracked CVE items with policy deadlines; KRN owns backport.

<!-- covers: GAP-0126 -->

#### Out of scope
Backport into diverged subsystems (KRN-045). Advisory publication (REL-044). Userspace crate CVEs (BLD).

#### Acceptance criteria
- [ ] An ingested upstream High or Critical kernel CVE opens a tracked CVE item that names the affected fork paths or records non-applicability with the ledger check KRN-030 produces.
- [ ] Fixes that touch diverged code are labeled distinctly from clean cherry-picks.
- [ ] A withheld feed item does not appear on a public dashboard.
- [ ] The pipeline runs on the V1 nightly matrix without manual copy-paste of CVE IDs.

#### Verification
- Integration: `rel:tests/cve/ingest_fork_match` on CI matrix entry `qemu-x86_64` (H-001) using a recorded upstream sample.
- Review: KRN CVE owner confirms diverged-code labels against the subsystem map.

#### Evidence
- none

### REL-007 · Operate a signed developer repository that rejects tampering
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: REL-003, REL-008, PKG-064, PKG-029
- Baseline: §27, §28, §30, §61
- Risks: R-030
- Threats: T-006, T-007, T-019, T-028
- Invariants: I-036, I-080

V1 exit requires Packages that are content-addressed, signed and verified before activation, and a repository update that creates a new SystemGeneration rather than mutating the running tree (§27, §28, §30). This task operates that repository with development keys from REL-002.

<!-- covers: INV-0540 -->

#### Out of scope
Public mirrors and CDN (REL-050). Client verification internals (PKG-055). Generation compose (PKG-016).

#### Acceptance criteria
- [ ] A Package whose bytes or signature do not match repository metadata is rejected before activation and allocates no handle.
- [ ] Fetching a repository update and applying it creates SystemGeneration N+1 and leaves N bootable.
- [ ] Objects in the repository are addressed by content hash as specified in REL-008.
- [ ] Development signing keys are not the V3 production root.
- [ ] `os inspect` on the activated Package names the signer recorded in the reserved manifest fields.

#### Verification
- Integration: `rel:tests/repo/tamper_reject` and `rel:tests/repo/update_new_generation` on `qemu-x86_64` (H-001).
- Manual: flip one byte of a published Package and confirm PKG-055 refuses activation.

#### Evidence
- none

### REL-008 · Specify the repository wire protocol and metadata format
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-003, PKG-005, PKG-014
- Baseline: §27, §28
- Invariants: I-036, I-047

PKG-064 cannot be built without a content-addressed, chunk-fetchable, statically mirrorable server contract. §27 content addressing is the integrity model; any HTTP host can serve the bytes because authenticity lives in signatures, not in the transport (I-047).

<!-- covers: GAP-0330 -->

#### Out of scope
Client implementation (PKG-064). TUF roles (REL-037). CDN topology (REL-024).

#### Acceptance criteria
- [ ] A committed protocol document names the index format, chunk-level fetch, delta hints and signature envelope REL-003 selected.
- [ ] The document states that a static file tree is sufficient to mirror, with no privileged origin API required for fetch.
- [ ] A fixture repository served as static files is consumed by a PKG-064 test on H-001.
- [ ] Native clients never use a POSIX path as the trust root.

#### Verification
- Review: REL and PKG leads sign off on the protocol document.
- Integration: `pkg:tests/repository/static_mirror_fetch` on `qemu-x86_64` (H-001).

#### Evidence
- none

### REL-009 · Measure delta versus full SystemGeneration update sizes
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-014, PKG-005, REL-008
- Baseline: §27, §30, §54

Update bandwidth and rollback speed determine whether generations feel first-class. This spike measures delta versus full SystemGeneration sizes on the PKG store format so INS chunk-level deltas and the V3 B-035 gate rest on a report rather than analogy. It does not restate a size target.

<!-- covers: GAP-0543 -->

#### Out of scope
Client delta fetch (INS-025). Store-level object diff (PKG-083). B-035 publication (BEN-@bench-delta-rollback).

#### Acceptance criteria
- [ ] The report records delta bytes and full generation bytes for the same consecutive generation pair on the chosen store format.
- [ ] The report compares whole-object hashing against content-defined chunking on that pair.
- [ ] The report names the delivery mechanism REL recommends to INS and whether that recommendation is corpus-specific.

#### Verification
- Report: answers the delta-to-full ratio on the chosen store format, whether content-defined chunking beats whole-object hashing enough to justify the extra index, and which mechanism INS-025 implements.
- Bench: B-035 method notes recorded in the report even though the V1 run is not a gate.

#### Evidence
- none

### REL-010 · Deliver a rotatable trust store inside SystemGenerations
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-002, REL-003, PKG-016, PKG-029
- Baseline: §30, §63
- Risks: R-030
- Threats: T-028, T-029
- Invariants: I-022

A leaked release key must be replaceable with a normal update before public alpha enrolls devices (T-029). The trust store is an immutable object inside the SystemGeneration PKG composes, so rotation is a generation switch rather than an out-of-band enrollment.

<!-- covers: GAP-0324 -->

#### Out of scope
Production HSM operation (REL-041). Boot-chain certificate rotation drill (BOOT-047). Client verification cache (PKG-055).

#### Acceptance criteria
- [ ] A SystemGeneration includes a trust-store object naming the currently valid intermediate keys and revoked keys.
- [ ] Installing a successor generation that rotates an intermediate causes PKG-055 to accept signatures under the new key and reject signatures under the revoked key.
- [ ] Rotation does not require the user to enroll a firmware key.
- [ ] `os inspect` on the generation names the trust-store object hash.

#### Verification
- Integration: `rel:tests/trust_store/rotate_intermediate` on `qemu-x86_64` (H-001) and H-002.
- Review: REL and PKG leads confirm revoked keys fail closed.

#### Evidence
- none

### REL-011 · Define hardware support tiers as the HCL unit
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-003, HW-043, HW-047
- Baseline: §62, §63
- Decision: D-0236
- Invariants: I-095

§62 constrains target hardware; V2's three machines seed Tier 1. REL owns the published Hardware Compatibility List unit so the V3 community database and installer verdict share one schema. Unlisted hardware is unsupported (I-095).

<!-- covers: GAP-0145 -->

#### Out of scope
SKU selection (HW-003). Probe tools (HW-066). HCL publication (REL-048). Promotion automation (HW-083).

#### Acceptance criteria
- [ ] Options evaluated include (A) Tier 1 lab-gated, Tier 2 community-reported, unsupported otherwise, (B) lab-only until 1.0, and (C) three tiers including a periodic-CI middle tier.
- [ ] The accepted option is the unit of the published Hardware Compatibility List and names the promotion path into Tier 1.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and HW leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-012 · Decide the repository model and source trust display
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-003, REL-004
- Baseline: §28, §62, §63
- Decision: D-0243
- Threats: T-006
- Invariants: I-021

Curated store versus open repository versus third-party remotes and sideloading shapes Package identity, signing and the permissions UI before third parties publish at V3. The V2 store client needs the trust-level display contract so users can consent to Capability requests from a named source (§62).

<!-- covers: GAP-0461, GAP-0329 -->

#### Out of scope
Publisher identity scheme (REL-025). Prebuilt versus rebuild (REL-016). Store client chrome (APP-045).

#### Acceptance criteria
- [ ] Options evaluated include (A) curated store plus explicit sideloading with trust-level display, (B) open repository with third-party remotes, and (C) first-party Packages only until 1.0.
- [ ] The accepted option states how trust level is shown before install and that a sideloaded source is never ambient (I-021, T-006).
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL, PKG and APP leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-013 · Expose a repository search and metadata API
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-012, REL-008, REL-007, BLD-055
- Baseline: §9.1, §28, §62
- Threats: T-006
- Invariants: I-021

The V2 native-repository gate requires the store client to show declared Capabilities, publisher, SBOM and compatibility before install. REL serves that metadata; APP-045 consumes it. Selecting a Package never grants a directory (I-021).

<!-- covers: GAP-0336, INV-1228 -->

#### Out of scope
Store client UI (APP-045). SBOM generation (BLD-055). Public third-party catalogue (REL-050).

#### Acceptance criteria
- [ ] A query for a Package returns publisher, declared RequestedCapabilities, SBOM locator and compatibility status before any install Operation.
- [ ] Trust level from REL-012 is a field on the response.
- [ ] A Package that requests a Capability the user has not granted still returns metadata and does not install.
- [ ] APP-045 can render the response without a second catalogue fetch.

#### Verification
- Integration: `rel:tests/metadata/pre_install_fields` on `qemu-x86_64` (H-001).
- Review: APP store-client owner confirms the response is sufficient for install-time Capability review.

#### Evidence
- none

### REL-014 · Generate release notes from tasks and Generation diffs
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-001, PKG-022, BEN-004
- Baseline: §31, §54, §62
- Invariants: I-061

Desktop-preview users need hardware-facing changelogs. The task repository and generation diff already hold the facts; this generator emits notes that cite B-IDs rather than inventing performance claims (I-061).

<!-- covers: GAP-0379 -->

#### Out of scope
1.0 migration guide prose (REL-065). Docs site publishing (DOC). History log (PKG-022).

#### Acceptance criteria
- [ ] A generation switch produces a notes artifact listing merged task IDs and Package identity diffs.
- [ ] The generator fails CI if a notes draft contains a performance number without a B-ID citation (I-061).
- [ ] Hardware-facing entries name Reference machines by H-ID rather than informal nicknames.

#### Verification
- Unit: `rel:tests/notes/from_tasks_and_diff`.
- Review: BEN claim-lint owner confirms the notes path is covered by BEN-004.

#### Evidence
- none

### REL-015 · Publish a versioned application compatibility Corpus
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-084, WIN-051, WIN-009, GOV-040
- Baseline: §46, §48, §49, §62
- Corpora: C-004, C-007
- Invariants: I-096

V2 L3 and W1 gates need a public per-generation pass/fail corpus fed by CI. LNX and WIN own scenarios; REL publishes the product database so compatibility is verifiable rather than anecdotal (§49, I-096).

<!-- covers: GAP-0451 -->

#### Out of scope
Scenario scripts (LNX, WIN). Community submissions (REL-022). Dataset license (GOV-040). VM-offer action (VIRT-013).

#### Acceptance criteria
- [ ] A versioned corpus document per SystemGeneration lists C-004 and C-007 entries with pass or fail and the generation ID.
- [ ] CI corpus jobs from LNX-084 and WIN-051 are the only writers of official rows.
- [ ] The published dataset license matches GOV-040.
- [ ] A third party can fetch the document without a login.

#### Verification
- Compat: C-004 and C-007 rows published for the V2 generation on H-002.
- Review: LNX and WIN corpus owners confirm ratings match `reports/compat/`.

#### Evidence
- none

### REL-016 · Decide whether the repository accepts publisher prebuilts or rebuilds every Package
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-012, BLD-054
- Baseline: §27, §28, §51
- Decision: D-0234
- Threats: T-006, T-007
- Invariants: I-036

Public third-party Packages force a rebuild-versus-prebuilt choice that determines whether reproducibility and SBOMs are project-wide or publisher-dependent. This Decision lands before REL-021 accepts submissions.

<!-- covers: GAP-0329 -->

#### Out of scope
SBOM generation (BLD-055). Publisher pipeline (REL-021). Non-free channel consent (REL-028).

#### Acceptance criteria
- [ ] Options evaluated include (A) rebuild every Package from source on project infrastructure, (B) accept publisher prebuilts with provenance and SBOM, and (C) prebuilts only on the non-free channel.
- [ ] The accepted option states how a client distinguishes a rebuilt object from a prebuilt object in metadata.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and BLD leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-017 · Automate release qualification as a CI checklist
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-027, REL-019, BLD-067, BEN-053
- Baseline: §54, §63
- Invariants: I-061, I-088

Public alpha cannot rest on a human-run checklist that drifts. Qualification evaluates milestone gates, reproducibility, Tier 1 hardware, unexplained benchmark regressions and upgrade plus rollback chains as a mechanical gate. Numbers live in B-IDs (I-088).

<!-- covers: GAP-0148 -->

#### Out of scope
CI job plumbing (BLD-067). Go/no-go roles (REL-027). Artifact signing (REL-019).

#### Acceptance criteria
- [ ] A qualification job fails when any cited V3 gate verifier is not done or when B-051 reports an unexplained regression versus V2.
- [ ] The job fails when BLD-041 (or the V3 successor) reports mismatched generation hashes.
- [ ] The job records Tier 1 hardware pass or fail per H-ID in the V3 hardware scope.
- [ ] Upgrade and rollback chain results from INS-047 are inputs, not restated numbers.

#### Verification
- Integration: `rel:tests/qualify/checklist_fail_on_red_gate` in the BLD-067 job.
- Review: REL release owner confirms the checklist matches REL-027.

#### Evidence
- none

### REL-018 · Publish inherited-CVE time-to-Generation measurements
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-006, KRN-052, REL-044, BEN-007
- Baseline: §54, §56.4
- Benchmarks: B-051
- Risks: R-062

V3 security-response gates require inherited High and Critical CVE latency measured and published. The harness records time from disclosure to a shipped SystemGeneration. The target lives in the benchmark register owned by BEN; this task does not invent a B-ID or restate a number.

<!-- covers: GAP-0054 -->

#### Out of scope
Kernel backport (KRN-045). Published SLA text (REL-060). Advisory format (REL-044).

#### Acceptance criteria
- [ ] A committed report lists, for each ingested High or Critical inherited kernel CVE in the window, disclosure identity and the generation that shipped the fix, or an explicit non-applicable record.
- [ ] The report names the harness and the measurement method BEN will register.
- [ ] No REL description, criterion or announcement restates a latency number; the register holds the target once allocated.

#### Verification
- Bench: time from inherited High or Critical CVE disclosure to a shipped SystemGeneration, published per the BEN method; target per register once BEN allocates the B-ID.
- Report: distribution used by KRN-052 matches the REL harness rows.
- Review: BEN lead confirms the method is register-shaped and cites no number in REL prose.

#### Evidence
- none

### REL-019 · Build once, sign and promote identical release artifacts
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: REL-041, REL-004, BLD-065, BLD-061, INS-001
- Baseline: §27, §30, §63
- Threats: T-007, T-028
- Invariants: I-022, I-086

Generation images, installer media, repository objects and SDK artifacts must be signed once and promoted through channels without rebuild, or the testing that justified promotion is void. BLD produces; REL signs and publishes. Rebuild-per-channel is a supply-chain defect (T-007).

<!-- covers: GAP-0147 -->

#### Out of scope
Image compose (INS-001, BLD-061). Channel policy (REL-004). Client apply (INS-045).

#### Acceptance criteria
- [ ] Signing a generation image, installer medium, repository index and SDK tarball produces four signatures over the same content hashes BLD emitted.
- [ ] Promoting those artifacts from nightly to testing to a later channel leaves every hash unchanged.
- [ ] A rebuilt artifact with a different hash cannot occupy the same generation ID.
- [ ] The pipeline refuses to sign an artifact whose SBOM or provenance is missing once REL-051 exists; until then it records the gap in the qualification log.

#### Verification
- Integration: `rel:tests/pipeline/promote_without_rebuild` using BLD-065 outputs on `qemu-x86_64` (H-001).
- Review: BLD and REL leads confirm hashes match across channels.

#### Evidence
- none

### REL-020 · Build a download site with signatures and Verification instructions
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-024, REL-041, REL-050, REL-046
- Baseline: §63
- Threats: T-019
- Invariants: I-061

Public alpha begins with a verifiable download: signature files, checksums and mirror or torrent distribution consistent with REL-024. The site cites B-IDs for any performance claim and never states a number in marketing copy (I-061).

<!-- covers: GAP-0376 -->

#### Out of scope
CDN and volunteer-mirror topology (REL-024). Installer media creation (INS-035). Docs site chrome (DOC).

#### Acceptance criteria
- [ ] The download page for a generation offers the image, detached signatures, checksums and a verification procedure a stranger can follow.
- [ ] Mirror and torrent links, if present, serve the same content hashes as the origin.
- [ ] Corresponding-source links from REL-046 are reachable from the same page.
- [ ] A mutated image fails the published verification procedure.

#### Verification
- Manual: follow the published verification steps on a clean host against H-001-built media.
- Integration: `rel:tests/download/signature_and_checksum` against the origin fixture.

#### Evidence
- none

### REL-021 · Build the publisher submission, review and signing pipeline
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: REL-025, REL-028, REL-016, REL-041, GOV-063
- Baseline: §9.1, §28, §63
- Risks: R-049
- Threats: T-006
- Invariants: I-021

The V3 public repository must gate submissions: identity enrollment, manifest and Capability lint, malware and license scan, reproducibility check, human review and appeal. SDK `os publish` is the client; REL is the service. A repository without this gate is a malware channel (T-006, R-049).

<!-- covers: GAP-0335, GAP-0460 -->

#### Out of scope
`os publish` CLI (SDK-080, PKG-084). Intermediary legal terms (GOV-063). Emergency pull (REL-036).

#### Acceptance criteria
- [ ] A first publish requires publisher key attestation and namespace ownership per REL-025.
- [ ] Submissions with invalid manifests, disallowed licenses or failed malware scan never reach the signed index.
- [ ] RequestedCapabilities are linted and queued for human review when they exceed the curation policy.
- [ ] An appeal path exists and is recorded on rejected submissions.
- [ ] A Package from outside the core team can complete the pipeline in REL-033.

#### Verification
- Integration: `rel:tests/publish/reject_malware_and_bad_manifest` plus `rel:tests/publish/identity_attestation` on `qemu-x86_64` (H-001).
- Review: GOV intermediary-terms owner confirms the queue matches published developer terms.

#### Evidence
- none

### REL-022 · Collect opt-in Personality compatibility reports
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-015, LNX-091, WIN-064, WIN-068, GOV-061, VIRT-013
- Baseline: §49, §63
- Corpora: C-004, C-007
- Threats: T-023
- Invariants: I-021, I-096

V3 community compatibility reports feed the public database: app identity, Personality version, result and workarounds. When an ELF or PE binary fails under a Personality the OS explains why, consults this database and points at known workarounds; VIRT-013 owns the VM-offer action (§49).

<!-- covers: GAP-0366, GAP-0450 -->

#### Out of scope
Official CI corpus rows (REL-015). PE failure reason emission (WIN-064). VM fallback (VIRT-013). Privacy policy (GOV-061).

#### Acceptance criteria
- [ ] An opt-in report contains app identity, Personality version, result and optional workaround text and is reviewable by the user before upload (T-023).
- [ ] A failed PE launch surfaces the WIN-064 code plus a database lookup; a silent failure is a test fail.
- [ ] Community rows are marked distinct from CI rows in the published database.
- [ ] The telemetry agent that uploads reports holds only the upload Capability it needs (I-021).

#### Verification
- Integration: `rel:tests/compat/opt_in_report_and_pe_reason` on `qemu-x86_64` (H-001).
- Compat: C-004 and C-007 community-row schema accepted beside official rows.
- Manual: redact a field before upload and confirm the stored report matches the redaction.

#### Evidence
- none

### REL-023 · Build crash deduplication, grouping and a triage dashboard
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-038, OBS-049
- Baseline: §24, §63
- Risks: R-048
- Threats: T-023

Raw crash streams are useless at alpha scale. Reports appear in the tracker within the V3 gate window, grouped by OBS signatures and linked to SystemGenerations and roadmap task IDs, completing the REL end-to-end pipeline.

<!-- covers: GAP-0364 -->

#### Out of scope
On-device signatures (OBS-049). Consent and redaction (INS-021). Intake (REL-038).

#### Acceptance criteria
- [ ] Two reports that share an OBS-049 signature are grouped as one cluster.
- [ ] A cluster names SystemGeneration ID and the roadmap task ID when a matching test exists.
- [ ] Operators cannot read redacted fields the user removed (T-023).
- [ ] A new unique signature opens a tracked item in the configured tracker.

#### Verification
- Integration: `rel:tests/crash/dedup_group_and_link` against the intake fixture.
- Review: OBS signature owner confirms grouping keys match the capture format.

#### Evidence
- none

### REL-024 · Decide origin, CDN and volunteer-mirror topology
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-012, GOV-041
- Baseline: §27, §63
- Decision: D-0235
- Risks: R-053
- Threats: T-019
- Invariants: I-047

Public alpha volume exceeds a single origin. Content addressing and signed metadata make untrusted mirrors safe, so a project-owned global mirror network buys little. Health checks must not be able to tamper with signed objects (T-019).

<!-- covers: GAP-0332, GAP-0397 -->

#### Out of scope
Download site (REL-020). Repository protocol (REL-008). Funding (GOV-041).

#### Acceptance criteria
- [ ] Options evaluated include (A) origin plus commercial CDN plus verified volunteer mirrors with no project-owned global mirror network, (B) origin plus CDN only, and (C) a project-operated global mirror network.
- [ ] The accepted option states how mirror health is checked without granting mirrors signing authority.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and GOV leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-025 · Decide publisher identity and Package naming
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-012, GOV-056
- Baseline: §28, §63
- Decision: D-0237
- Threats: T-006, T-033
- Invariants: I-080

A public repository without namespace ownership, key attestation and a name-squatting rule cannot stop impersonation. Trademark policy remains GOV; this Decision fixes the operational identity scheme grants will key on (T-033).

<!-- covers: GAP-0328, GAP-0462 -->

#### Out of scope
Trademark policy (GOV-056). Grant continuity (CAP). Publisher pipeline (REL-021).

#### Acceptance criteria
- [ ] Options evaluated include (A) reverse-DNS namespace with key attestation at first publish, (B) flat names with first-come ownership, and (C) publisher IDs without package-name ownership.
- [ ] The accepted option states how a key change is attested and how a name-squatting dispute is resolved operationally.
- [ ] Persistent grants remain keyed on package identity plus publisher, not content hash (T-033).
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and GOV trademark owners sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-026 · Declare fleet-management and paid-app non-goals for 1.0
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-012
- Baseline: §63
- Decision: D-0238
- Invariants: I-092

Collapse REL 1.0 nongoals into one Decision before the public repository: no MDM or fleet provisioning, and no paid applications or payouts. SEC already scopes directory join out of 1.0; this Decision keeps REL from growing a commerce or fleet stack that would consume release engineering (I-092).

<!-- covers: GAP-0396, GAP-0467 -->

#### Out of scope
Enterprise directory (SEC). Developer-program onboarding (GOV-071). Repository mechanics (REL-050).

#### Acceptance criteria
- [ ] Options evaluated include (A) defer MDM, fleet provisioning, paid applications and payouts past 1.0, and (B) build a store commerce and fleet-provisioning stack for 1.0.
- [ ] The accepted option lists the nongoals in language a V3 reviewer can lint against new REL tasks.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and GOV leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-027 · Define release-readiness gates and freeze policy
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-004
- Baseline: §63
- Decision: D-0239

V3 public releases need blocker definition, go/no-go roles and per-channel freeze rules so qualification is not renegotiated under time pressure. REL-017 implements the mechanical half.

<!-- covers: GAP-0378 -->

#### Out of scope
CI checklist (REL-017). Support lifecycle (REL-053). Feature freeze governance (GOV-074).

#### Acceptance criteria
- [ ] Options evaluated include (A) mechanical CI checklist with named go/no-go roles and per-channel freeze, (B) human-run checklist only, and (C) automatic promote when CI is green with no freeze.
- [ ] The accepted option defines a blocker bug and who may override a red qualification job.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL release owner and GOV process owner sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-028 · Define repository curation and free versus non-free channels
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-012, REL-016, GOV-022
- Baseline: §28, §63
- Decision: D-0241
- Risks: R-049
- Threats: T-006
- Invariants: I-021

Steam and proprietary drivers need an explicit non-free channel with consent. Review criteria, approval roles, appeals and free/non-free separation are operational REL policy; GOV owns redistribution licensing.

<!-- covers: GAP-0076 -->

#### Out of scope
Firmware blob policy (GOV-022, HW-023). Publisher pipeline (REL-021). Store consent UI (APP-025).

#### Acceptance criteria
- [ ] Options evaluated include (A) automated scan plus audit with a non-free channel requiring explicit consent, (B) full human review of every Package, and (C) no non-free channel.
- [ ] The accepted option names approval roles, appeal path and how non-free is displayed before install (I-021).
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and GOV leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-029 · Decide repository retention of past generations
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-004, PKG-052
- Baseline: §30, §31
- Decision: D-0242
- Invariants: I-022

Rollback and restore promises are only honest if referenced objects remain downloadable. This Decision bounds the 1.0 N-previous-generations guarantee and PKG garbage collection.

<!-- covers: GAP-0334 -->

#### Out of scope
Client GC (PKG-052, PKG-067). Upgrade-path tests (REL-067). ESP kernel retention (BOOT-013).

#### Acceptance criteria
- [ ] Options evaluated include (A) retain every object referenced by a supported generation plus a published N-previous floor, (B) retain only the current channel head, and (C) retain everything forever.
- [ ] The accepted option states what a client may still fetch after a generation leaves the channel head.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and PKG leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-030 · Decide whether releases use a transparency log
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-003
- Baseline: §51, §63
- Decision: D-0247
- Threats: T-007, T-028

Binary transparency is the defence against a compromised pipeline silently shipping targeted builds. This Decision chooses whether clients verify inclusion proofs before the public repository hardens; operation of the log is V4.

<!-- covers: GAP-0325 -->

#### Out of scope
Log operation (REL-058). Key hierarchy (REL-002).

#### Acceptance criteria
- [ ] Options evaluated include (A) a Sigstore or Rekor-style log with client inclusion proofs on the stable path, (B) a log without client proofs, and (C) no transparency log for 1.0.
- [ ] The accepted option states which artifacts are logged (generations, Packages, repository metadata) and whether verification is mandatory on stable.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and SEC leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-031 · Define release infrastructure as code with a status page
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-024, GOV-041, BLD-034
- Baseline: §63
- Risks: R-053, R-070

Repository or CDN outages stop updates and security fixes for the installed base. Infrastructure is declared as code with a public status page. Hosting REL origin and signing on JakeOS waits for installer and updater (R-070); this tree records the host, it does not choose a new kernel boundary.

<!-- covers: GAP-0392 -->

#### Out of scope
CI runners (BLD). Lab machines (LAB). Funding (GOV-041).

#### Acceptance criteria
- [ ] Repository, signing-service, CDN and status-page definitions live in the repository and recreate the public endpoints from those definitions.
- [ ] A status page names origin, CDN and mirror health without exposing embargoed jobs.
- [ ] The as-code tree states whether REL origin, signing and status-page hosts run on JakeOS or on a retained Linux host, and names the owning prefix for that host.
- [ ] A configuration change is reviewable as an ordinary pull request.

#### Verification
- Review: REL and BLD infra owners sign off on the as-code tree.
- Manual: recreate the origin fixture from the repository on a clean host.

#### Evidence
- none

### REL-032 · Define root signing key ceremony, quorum and rotation
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-002, GOV-066, GOV-024
- Baseline: §51, §63
- Risks: R-066
- Threats: T-029
- Invariants: I-080

Production keys for public alpha need ceremony, HSM storage, signing quorum, rotation, revocation and named governance roles. REL writes the operational runbook; GOV names which roles hold shares.

<!-- covers: GAP-0045 -->

#### Out of scope
HSM operation (REL-041). Governance role list (GOV-066). Legal entity (GOV-024).

#### Acceptance criteria
- [ ] A committed ceremony document names root storage, quorum, rotation, revocation and the GOV roles that hold shares.
- [ ] The document states what happens on suspected compromise without requiring a calendar date.
- [ ] Development keys used at V1 are listed as distinct from the production root.

#### Verification
- Review: REL, GOV and BOOT leads sign off on the ceremony document.

#### Evidence
- none

### REL-033 · Rehearse third-party submit, review, sign, publish and revoke
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-021, REL-036, REL-050, SDK-082
- Baseline: §28, §63
- Threats: T-006

V3 exit: a third-party submission is reviewed, signed, published and revoked in a drill, and Packages from outside the core team are in the repository. This verifies the publisher pipeline and emergency pull.

#### Out of scope
Publisher service (REL-021). Client revoke (PKG-087). SDK publish CLI (SDK-080).

#### Acceptance criteria
- [ ] A Package submitted by an identity outside the core team is reviewed, signed and present in the public index.
- [ ] The same Package is withdrawn through REL-036 and PKG-087 refuses activation afterward.
- [ ] The drill record names the reviewer and the revocation generation.

#### Verification
- Demo: third-party submit, review, sign, publish and revoke on H-001.
- Integration: `rel:tests/publish/drill_revoke` on `qemu-x86_64` (H-001).

#### Evidence
- none

### REL-034 · Exercise the security-response process with public postmortems
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-047, REL-044, REL-039, REL-036, BLD-062
- Baseline: §56.4, §63
- Risks: R-062

V3 exit: the process is exercised via at least two real or simulated advisories with public postmortems. This verifies disclosure policy, the advisory feed and embargo handling without leaking embargoed builds through public CI.

#### Out of scope
Policy text (REL-047). Kernel backport (KRN-052). Embargoed compile path (BLD-062).

#### Acceptance criteria
- [ ] Two advisory exercises exist, each with a signed advisory, a shipped or explicitly withheld generation, and a public postmortem.
- [ ] Embargoed artifacts used in the exercise are built on BLD-062 and do not appear on the public dashboard.
- [ ] Each postmortem names what REL-047 required and what actually ran.

#### Verification
- Review: REL and KRN CVE owners sign off on both postmortems.
- Manual: confirm the public site shows the signed advisories and not the embargoed trees.

#### Evidence
- none

### REL-035 · Ship a hardware probe that produces anonymized fingerprints for the HCL
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-011, HW-047, HW-068
- Baseline: §9.1, §62, §63
- Threats: T-042
- Invariants: I-078, I-095

Feeds the compatibility database and Hardware Compatibility List. REL owns the published fingerprint format and upload; HW owns live-image and host-side checkers. Submissions contain no serial numbers or network identifiers (T-042, I-078).

<!-- covers: GAP-0369 -->

#### Out of scope
Live-image checker (HW-066). Host-side foreign-OS probe (HW-067). Conformance suite (HW-064). HCL site (REL-048).

#### Acceptance criteria
- [ ] The probe emits a record matching HW-047 with PCI, USB, ACPI and firmware identifiers and no serial, MAC or TPM EK (I-078).
- [ ] The user reviews the exact record before upload.
- [ ] Upload is opt-in and holds only the upload Capability (T-042).
- [ ] An unlisted machine is labeled unsupported rather than silently promoted (I-095).

#### Verification
- Integration: `rel:tests/hcl/probe_redacts_identifiers` on H-002.
- Review: HW HCL-privacy owner confirms the record matches HW-068.

#### Evidence
- none

### REL-036 · Implement emergency Generation and Package withdrawal
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-037, PKG-087, BOOT-040, REL-044
- Baseline: §30, §63
- Threats: T-006, T-022
- Invariants: I-022

Security response must withdraw a published generation or malicious Package without violating anti-rollback, mark affected generations, and coordinate third-party disclosure. PKG-087 is the client; BOOT-040 is the watermark policy (T-022).

<!-- covers: GAP-0340, GAP-0466 -->

#### Out of scope
Client refuse-activation (PKG-087). Anti-rollback watermark (BOOT-038). Publisher pipeline (REL-021).

#### Acceptance criteria
- [ ] Withdrawing a generation from a channel stops new clients from fetching it and publishes a successor generation that clients can step to.
- [ ] The withdrawal does not require installing a generation older than the security-fix watermark BOOT-040 selected.
- [ ] Affected generations are marked in repository metadata PKG-087 reads.
- [ ] A third-party Package withdrawal emits a coordinated-disclosure record on the advisory feed.

#### Verification
- Integration: `rel:tests/pull/withdraw_generation_and_package` on `qemu-x86_64` (H-001).
- Review: BOOT anti-rollback owner confirms the pull path cannot install below the watermark.

#### Evidence
- none

### REL-037 · Implement TUF-style repository metadata roles
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-003, REL-008, REL-007
- Baseline: §27, §28, §63
- Threats: T-019, T-028
- Invariants: I-036

Signed Packages alone do not stop replay of an old valid generation. Root, targets, snapshot and timestamp roles make stale and mix-and-match metadata detectable by clients.

<!-- covers: GAP-0331 -->

#### Out of scope
Signing scheme Decision (REL-003). Client fetch (PKG-064). Transparency log (REL-058).

#### Acceptance criteria
- [ ] Repository metadata includes root, targets, snapshot and timestamp roles, or the subset REL-003 required if it chose TUF from V1.
- [ ] A client presented with a rewind of timestamp or a mix of old targets and new snapshot fails closed and activates nothing.
- [ ] Role keys match REL-002 intermediates, not the offline root on the fast path.
- [ ] `os inspect` on a refused update names the metadata role that failed.

#### Verification
- Integration: `rel:tests/tuf/replay_and_mix_match` on `qemu-x86_64` (H-001).
- Fuzz: `rel:fuzz/tuf_metadata` on nightly without panic.

#### Evidence
- none

### REL-038 · Build the crash-report intake pipeline from Component state
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: OBS-029, OBS-026, OBS-049, OBS-028, BLD-038, INS-020, GOV-055, WIN-063, LNX-027
- Baseline: §24, §51, §63
- Risks: R-048
- Threats: T-023
- Invariants: I-021, I-077

V2 scoped the crash pipeline out; the V3 gate requires symbolicated, deduplicated reports. OBS captures, INS consents, REL intakes structured Component, TaskGroup, Capability graph and trace context rather than raw core dumps. Symbol artifacts come from BLD and OBS; REL hosts the intake and the debuginfod-style lookup the pipeline needs. Dumps never contain disk keys or unlocked secrets (I-077).

<!-- covers: GAP-0361, GAP-0365 -->

#### Out of scope
Capture format (OBS-029). Consent and redaction UI (INS-020, INS-021). Local metrics (OBS-051). Kernel dumps under lockdown (SEC).

#### Acceptance criteria
- [ ] Intake accepts an OBS structured report with Component, TaskGroup, Capability graph and trace window and rejects a raw core dump that contains disk-key material (I-077).
- [ ] Symbolication uses BLD-038 objects addressed by content hash.
- [ ] The intake agent holds only the Capabilities GOV-055 listed; ambient filesystem or network besides the intake endpoint fails the isolation test (I-021, GAP-0365).
- [ ] A report the user redacted arrives with those fields absent.
- [ ] Personality crashes mapped by WIN-063 and LNX-027 land in the same intake schema.

#### Verification
- Integration: `rel:tests/crash/intake_structured_and_scrub` on `qemu-x86_64` (H-001).
- Review: SEC threat-model owner confirms T-023 controls on the intake agent.

#### Evidence
- none

### REL-039 · Join pre-disclosure lists and name embargo holders
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-047, GOV-024, GOV-065
- Baseline: §56.4, §63
- Risks: R-062

Without linux-distros and vendor programs the fork learns of kernel and GPU or browser issues at the same time as attackers. This task records membership and who may hold embargoed information.

<!-- covers: GAP-0355 -->

#### Out of scope
CNA application (SEC). Advisory feed (REL-044). Embargoed builds (BLD-062).

#### Acceptance criteria
- [ ] Membership or a documented application in progress exists for linux-distros and the GPU and browser vendor programs the entity can join.
- [ ] Named GOV roles are the only holders of embargoed information.
- [ ] The document states that public CI must not build embargoed trees.

#### Verification
- Review: REL, KRN and GOV security-response roles sign off on the membership record.

#### Evidence
- none

### REL-040 · Maintain SBAT Generation numbers and run shim-review if chosen
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-031, BOOT-036, REL-041
- Baseline: §51, §63
- Risks: R-047
- Threats: T-008, T-028

If BOOT chooses shim, REL runs shim-review and maintains SBAT numbers so compromised bootloader or kernel builds revoke without dbx churn. If shim is rejected, this task records the no-op path and still publishes SBAT-equivalent generation numbers for the enrolled project keys.

<!-- covers: GAP-0180 -->

#### Out of scope
Shim integration in the boot chain (BOOT-044). Secure Boot strategy (BOOT-031). Submission plan (BOOT-036).

#### Acceptance criteria
- [ ] When shim is the accepted BOOT option, a shim-review submission artifact exists and SBAT generation numbers increment on each signed bootloader and kernel that must be revocable.
- [ ] When shim is rejected, the task record states the no-op and project-key revocation still names generation numbers.
- [ ] A revoked SBAT generation is refused by the signed boot chain test BOOT owns.

#### Verification
- Integration: `rel:tests/sbat/increment_and_revoke` coordinated with BOOT-044 on H-002.
- Review: BOOT Secure Boot owner confirms the chosen path.

#### Evidence
- none

### REL-041 · Operate HSM-backed signing for kernel, generations and Packages
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: REL-032, REL-003, REL-010, GOV-024, GOV-066
- Baseline: §51, §63
- Risks: R-030, R-053, R-066
- Threats: T-007, T-028, T-029
- Invariants: I-080

§63 secure signing: offline root, HSM intermediates for bootloader, kernel, SystemGenerations, Packages and repository metadata, with documented rotation and a compromise runbook. Development keys from V1 do not sign public alpha.

<!-- covers: GAP-0181, INV-1243 -->

#### Out of scope
Ceremony document (REL-032). Trust-store object (REL-010). Boot verification (BOOT-027).

#### Acceptance criteria
- [ ] Production signatures on bootloader, kernel, SystemGeneration, Package and repository metadata verify against the hierarchy REL-002 accepted.
- [ ] The offline root is not present on CI workers.
- [ ] A compromise runbook exists and is the one REL-052 drills.
- [ ] V1 development keys are rejected by the V3 trust store.

#### Verification
- Integration: `rel:tests/signing/prod_verify_and_dev_key_reject` on H-001 and H-002.
- Review: GOV key-governance owner confirms quorum matches the ceremony document.
- Manual: signing ceremony performed once for the public-alpha root with the documented quorum.

#### Evidence
- none

### REL-042 · Operate opt-in telemetry intake for crash-free and boot rates
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-055, GOV-061, INS-021, OBS-051, REL-038
- Baseline: §54, §63
- Benchmarks: B-041, B-042, B-043
- Risks: R-048, R-057
- Threats: T-023
- Invariants: I-021, I-061

V3 gates need opt-in telemetry from the alpha fleet with crash-free session rate measured and published. REL stores only the consented schema and feeds staged rollouts. Targets live in B-041, B-042 and B-043; this task does not restate them (I-061).

<!-- covers: GAP-0360 -->

#### Out of scope
Policy (GOV-055). On-device counters (OBS-051). Redaction UI (INS-021). BEN publication (BEN-049, BEN-051, BEN-052).

#### Acceptance criteria
- [ ] Intake accepts only fields listed in GOV-055 and drops others.
- [ ] Crash-free, panic and update-success aggregates are exported to the B-041, B-042 and B-043 harnesses.
- [ ] The agent holds only consented Capabilities (I-021, T-023).
- [ ] Machines that did not opt in contribute zero rows.

#### Verification
- Bench: B-041, B-042 and B-043 ingest paths on the alpha fleet export; targets per register.
- Integration: `rel:tests/telemetry/schema_and_opt_in` on `qemu-x86_64` (H-001).
- Review: GOV privacy-policy owner confirms the stored schema matches the published policy.

#### Evidence
- none

### REL-043 · Promote consecutive alpha SystemGenerations through channels
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-019, REL-005, REL-017, INS-045
- Baseline: §30, §63
- Benchmarks: B-043
- Invariants: I-022, I-086

V3 updater gate: consecutive alpha releases delivered through the update channel using REL-019 promotion without rebuild. INS applies generations; REL publishes them.

#### Out of scope
Client apply and rollback UX (INS-045, INS-043). Qualification checklist (REL-017).

#### Acceptance criteria
- [ ] At least six consecutive alpha generations are published on the testing or alpha channel with unchanged hashes from the signed pipeline.
- [ ] Each promote is recorded as a PKG history event.
- [ ] B-043 counts those promotes as update attempts on H-001 and every V3 Tier 1 machine the soak includes.

#### Verification
- Integration: `rel:tests/promote/consecutive_alpha` with INS-043.
- Bench: B-043 on H-001; target per register.

#### Evidence
- none

### REL-044 · Publish machine-readable security advisories and a feed
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-047, REL-006, PKG-081, BLD-062
- Baseline: §56.4, §63
- Threats: T-007

OSV and CSAF/VEX so the updater and third-party scanners can match exposure. PKG-081 consumes the feed. Prose advisories are not sufficient.

<!-- covers: GAP-0351 -->

#### Out of scope
Client matching (PKG-081). Disclosure policy (REL-047). Embargoed compilation (BLD-062).

#### Acceptance criteria
- [ ] Each published advisory is available as OSV and CSAF/VEX and is signed under REL-041.
- [ ] A fixture generation with a known CVE is matched by PKG-081 to the fixing generation.
- [ ] Embargoed advisories are absent from the public feed until the disclosure policy releases them.

#### Verification
- Integration: `rel:tests/advisory/osv_csaf_match` on `qemu-x86_64` (H-001).
- Review: KRN CVE owner confirms inherited-kernel rows name fork applicability.

#### Evidence
- none

### REL-045 · Publish AppStream catalogue metadata for the repository
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-013, REL-050
- Baseline: §46, §49, §63
- Invariants: I-025

Linux-personality software centres and offline media must consume the repository without a second catalogue. Native software still does not see POSIX or AppStream APIs; this is Personality-facing metadata beside the native metadata API (I-025).

<!-- covers: GAP-0464 -->

#### Out of scope
Native metadata API (REL-013). Linux software-centre hosting (LNX). Offline media (INS-038).

#### Acceptance criteria
- [ ] An AppStream catalog is published beside the native metadata API and is mirrorable as static files.
- [ ] Native Components cannot open the catalog as a POSIX directory; Personality software centres can.
- [ ] Capability and publisher fields in AppStream match REL-013 for the same Package.

#### Verification
- Integration: `rel:tests/appstream/mirrorable_catalog` on `qemu-x86_64` (H-001).
- Review: LNX owner confirms a personality software centre can parse the catalog.

#### Evidence
- none

### REL-046 · Publish corresponding source beside every binary Generation
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-019, GOV-059, GOV-003
- Baseline: §5.1, §27, §63
- Invariants: I-067

GPLv2 corresponding-source obligations begin with the first public binary. REL publishes exact commits, patches, build scripts and toolchain beside the binary channel, reachable from `os history`. GOV states the written-offer policy INS and APP display.

<!-- covers: GAP-0013, GAP-0374 -->

#### Out of scope
Written-offer policy (GOV-059). About UI (INS-037, APP-057). License firewall (GOV-003).

#### Acceptance criteria
- [ ] Every signed public generation has a corresponding-source bundle naming commits, patches, scripts and toolchain hashes.
- [ ] `os history` on that generation names the bundle locator.
- [ ] A missing bundle fails REL-017.

#### Verification
- Integration: `rel:tests/source/bundle_beside_generation` on `qemu-x86_64` (H-001).
- Review: GOV licensing owner confirms the bundle satisfies the written-offer policy.

#### Evidence
- none

### REL-047 · Publish vulnerability disclosure policy and signed advisory format
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-031, GOV-065, SEC-002
- Baseline: §56.4, §63
- Risks: R-062
- Threats: T-007

§56.4 and §63 public security response: contact, embargo, signed advisories covering the kernel fork, bootloader, Personalities and the package repository. SEC owns CNA and SECURITY.md; REL owns the operational policy for distributed artifacts.

<!-- covers: GAP-0239, INV-1249, GAP-0046, GAP-0350 -->

#### Out of scope
CNA and SECURITY.md (SEC). Kernel triage steps (KRN-031). Advisory feed implementation (REL-044). GOV sign-off roles (GOV-065).

#### Acceptance criteria
- [ ] A public policy names the contact, encrypted channel, embargo rules and the signed advisory format for kernel, bootloader, Personalities and repository artifacts.
- [ ] The policy names which GOV roles may sign advisories.
- [ ] The policy is linked from the download site and the repository.

#### Verification
- Review: REL, SEC and GOV security-response roles sign off on the published policy.

#### Evidence
- none

### REL-048 · Publish the hardware compatibility database with community reports
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-035, REL-011, HW-065, HW-047, GOV-040, GOV-061
- Baseline: §62, §63
- Risks: R-085
- Threats: T-042
- Invariants: I-095

§63 and V3 exit: Hardware Compatibility List with Tier 1 plus community submissions. HW owns probe tools; REL publishes and accepts reports under REL-011. Unlisted hardware is unsupported (I-095).

<!-- covers: INV-1241, GAP-0369 -->

#### Out of scope
Probe emission (REL-035, HW-066). Community ingest validation (HW-065). Installer warning (INS-028).

#### Acceptance criteria
- [ ] The published Hardware Compatibility List includes every V3 Tier 1 machine with probe data.
- [ ] Community submissions that pass HW-065 appear as Tier 2 and contain no serials or network identifiers (T-042).
- [ ] The installer-readable verdict format matches what INS-028 consumes.
- [ ] Dataset license matches GOV-040.

#### Verification
- Integration: `rel:tests/hcl/publish_tier1_and_community` against HW-047.
- Review: HW and GOV privacy owners confirm redaction and license.

#### Evidence
- none

### REL-049 · Publish the third-party notices bundle for every Generation
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-019, GOV-003, GOV-059, BLD-023
- Baseline: §5.1, §63
- Invariants: I-067, I-068

Workstreams.md gives REL the notices bundle. INS and APP display it; REL ships it beside every signed generation so the About view is not hand-assembled.

#### Out of scope
About UI (INS-037, APP-057). License allowlists (GOV, BLD-023). Corresponding source (REL-046).

#### Acceptance criteria
- [ ] Every signed public generation includes a notices bundle listing third-party licenses for shipped objects.
- [ ] The bundle is the file INS-037 displays.
- [ ] A generation that fails BLD-023 cannot receive a notices bundle signature.

#### Verification
- Integration: `rel:tests/notices/bundle_per_generation` on `qemu-x86_64` (H-001).
- Review: GOV licensing owner confirms the bundle matches the allowlist.

#### Evidence
- none

### REL-050 · Publish a public Package repository with integrity Verification
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: REL-024, REL-037, REL-021, REL-041, GOV-063
- Baseline: §27, §28, §63
- Threats: T-006, T-019
- Invariants: I-036

§63 and V3 exit: public repository with mirrors and integrity verification. Clients verify TUF metadata and signatures. Intermediary terms are GOV; REL operates the repo those terms govern.

<!-- covers: INV-1242 -->

#### Out of scope
TUF implementation (REL-037). Publisher pipeline (REL-021). Client (PKG-064). Legal terms (GOV-063).

#### Acceptance criteria
- [ ] A public origin serves Packages and metadata matching REL-008.
- [ ] A mirror configured per REL-024 serves the same hashes and cannot substitute a payload under a valid signature (T-019).
- [ ] Tampered metadata is rejected by PKG-064 before activation.
- [ ] Developer terms from GOV-063 are linked from the repository.

#### Verification
- Integration: `rel:tests/repo/public_mirror_integrity` on `qemu-x86_64` (H-001).
- Manual: fetch via a volunteer-mirror fixture and verify signatures.

#### Evidence
- none

### REL-051 · Publish SBOM and signed provenance for every release artifact
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-070, BLD-055, BLD-068, REL-041, REL-019
- Baseline: §27, §51, §63
- Threats: T-007

BLD generates SBOMs; REL publishes signed SLSA-style provenance covering vendored crates, inherited Linux sources and firmware blobs so a 1.0 OS can answer which sources produced which shipped bytes.

<!-- covers: GAP-0095 -->

#### Out of scope
SBOM generation (BLD-055, BLD-070). Format Decision (BLD-054). Corresponding source (REL-046).

#### Acceptance criteria
- [ ] Every signed public generation, installer medium and SDK artifact has an SBOM and a signed provenance attestation.
- [ ] Attestations cover vendored crates, inherited Linux sources and redistributed firmware blobs.
- [ ] A missing SBOM fails BLD-068 and REL-017.

#### Verification
- Integration: `rel:tests/sbom/signed_provenance_per_artifact` on `qemu-x86_64` (H-001).
- Review: BLD SBOM owner confirms attestations match CI outputs.

#### Evidence
- none

### REL-052 · Write and rehearse supply-chain incident runbooks
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-041, REL-032, REL-036, REL-031
- Baseline: §51, §63
- Risks: R-030
- Threats: T-007, T-028, T-029

Compromised signing keys, build infrastructure or repository hosts need rehearsed recovery, not improvisation while users are exposed. Complements REL-041.

<!-- covers: GAP-0357 -->

#### Out of scope
HSM operation (REL-041). Emergency pull mechanism (REL-036). Key rotation drill on testing (REL-054).

#### Acceptance criteria
- [ ] Runbooks exist for compromised signing keys, compromised build infrastructure and compromised repository hosts.
- [ ] Each runbook is rehearsed once with a written outcome that names the actual steps taken.
- [ ] The key-compromise runbook uses REL-010 rotation rather than out-of-band enrollment.

#### Verification
- Review: REL, BLD and GOV leads sign off on runbooks and rehearsal records.
- Manual: tabletop or staged rehearsal of the three incidents with artifacts attached as Evidence when done.

#### Evidence
- none

### REL-053 · Decide release cadence, LTS window and support lifecycle
- Type: adr
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-004, REL-001, GOV-075
- Baseline: §56.4, §66
- Decision: D-0246
- Risks: R-061, R-062
- Invariants: I-086

V4 support-policy drafts: time-based versus feature-based cadence, LTS window, which layers receive backports, security-only phase and end-of-life communication. The 1.0 support promise is this Decision accepted. Numbers for windows live in the GOV contract and the CVE SLA register entry, not in this prose (R-061).

<!-- covers: GAP-0062, GAP-0345 -->

#### Out of scope
Published contract text (GOV-075, GOV-083). CVE SLA register row (REL-060). Stable channel operation (REL-057).

#### Acceptance criteria
- [ ] Options evaluated include (A) time-based releases with an LTS window and a security-only phase, (B) feature-based releases with no LTS, and (C) rolling stable only.
- [ ] The accepted option names which layers receive backports and how end-of-life is communicated without a calendar date in roadmap sources.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: REL and GOV leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### REL-054 · Execute a signing-key rotation drill on the testing Channel
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-041, REL-010, BOOT-047, REL-057
- Baseline: §30, §63
- Threats: T-028, T-029

V4 exit: a signing-key rotation drill on testing without breaking updates, using the generation trust store. BOOT-047 covers the boot chain; this task covers repository, Package and SystemGeneration intermediates.

#### Out of scope
Boot-chain certificates (BOOT-047). Ceremony document (REL-032). Transparency log (REL-058).

#### Acceptance criteria
- [ ] An intermediate release key is rotated on the testing channel via a new SystemGeneration trust store.
- [ ] Clients on testing continue to verify new generations and refuse the revoked intermediate.
- [ ] Previous generations that remain in retention stay bootable and verifiable.
- [ ] The drill record names the old and new key identities.

#### Verification
- Integration: `rel:tests/signing/rotate_testing_intermediate` on H-001 and H-002.
- Demo: rotation on the testing channel shown with BOOT-047 on H-002.

#### Evidence
- none

### REL-055 · Publish fleet crash-free and panic-rate dashboards
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-042, REL-023, OBS-053, BEN-049, BEN-051
- Baseline: §54, §63
- Benchmarks: B-041, B-042
- Risks: R-057
- Invariants: I-061

V4 fleet gate and demo: opt-in machines, crash-free session rate and kernel panic rate published. Targets live in B-041 and B-042; this dashboard does not restate them (I-061).

#### Out of scope
Intake (REL-042). BEN publication (BEN-049, BEN-051). Panic attribution (OBS-053).

#### Acceptance criteria
- [ ] A public dashboard shows B-041 and B-042 for opt-in machines with machine count and generation ID.
- [ ] Rows from machines that did not opt in are absent.
- [ ] A generation whose rates miss the register target is visible as a failing series without a number copied into REL prose.

#### Verification
- Bench: B-041 and B-042 on the V4 fleet export; targets per register.
- Review: BEN lead confirms the dashboard reads harness reports rather than a parallel calculation.

#### Evidence
- none

### REL-056 · Implement staged rollouts with automatic halt on crash rate
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: REL-042, REL-023, REL-057, REL-036
- Baseline: §30, §54, §63
- Benchmarks: B-041, B-042, B-043
- Risks: R-057
- Threats: T-028
- Invariants: I-061, I-086

A bad kernel or compositor generation must stop before the whole installed base. Halt thresholds cite B-041, B-042 and B-043, not prose numbers (I-061). Halt publishes a withdrawal plus a successor generation rather than live-patching (I-086).

<!-- covers: GAP-0339 -->

#### Out of scope
Telemetry intake (REL-042). Emergency pull mechanism (REL-036). Client apply (INS).

#### Acceptance criteria
- [ ] A stable or testing channel rollout can be limited to a percentage of opted-in machines.
- [ ] Crossing the B-041 or B-042 halt threshold stops further promotion and triggers REL-036 for the bad generation.
- [ ] Halted machines remain on the previous generation and can fetch the successor.
- [ ] B-043 records halted attempts distinctly from successful boots.

#### Verification
- Integration: `rel:tests/rollout/halt_on_injected_crash_rate` on H-001 with fault-injected telemetry.
- Bench: B-041, B-042 and B-043 halt path; targets per register.

#### Evidence
- none

### REL-057 · Operate the stable Channel and LTS branch infrastructure
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-053, REL-004, REL-019, REL-037, REL-017
- Baseline: §30, §66
- Invariants: I-022, I-086

V4 REL scope: stable channel infrastructure and LTS branch policy implementation from REL-053 and REL-004. Promotion still does not rebuild.

#### Out of scope
Cadence Decision (REL-053). Staged halt (REL-056). Kernel 1.x branch (KRN-058).

#### Acceptance criteria
- [ ] A stable channel head exists and accepts only artifacts that passed REL-017.
- [ ] An LTS branch or channel exists if REL-053 selected LTS, or the task record states the no-LTS path.
- [ ] Promoting to stable leaves content hashes unchanged from testing.

#### Verification
- Integration: `rel:tests/channels/stable_promote` on `qemu-x86_64` (H-001).
- Review: REL release owner confirms the LTS path matches the accepted Decision.

#### Evidence
- none

### REL-058 · Operate a public transparency log for signing actions
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-030, REL-041
- Baseline: §51, §63
- Threats: T-007, T-028

Beta users need independently auditable evidence that signing keys were not misused. Implements REL-030; clients verify inclusion proofs on the stable path when that option was accepted. If the Decision rejected a log, this task records the no-op and still publishes a signed append-only list of signing events.

<!-- covers: GAP-0083 -->

#### Out of scope
Decision (REL-030). Key operation (REL-041). Client inclusion-proof code (PKG).

#### Acceptance criteria
- [ ] When the Decision requires a log, every production signature on a generation, Package or repository metadata has an inclusion proof a client can verify.
- [ ] When the Decision rejects a log, a signed append-only event list is still published and the no-op is recorded on this task.
- [ ] A signature missing from the log or event list fails stable-path verification if proofs were mandatory.

#### Verification
- Integration: `rel:tests/tlog/inclusion_or_event_list` on `qemu-x86_64` (H-001).
- Review: SEC owner confirms the deployed path matches REL-030.

#### Evidence
- none

### REL-059 · Qualify in-place V3 to V4 upgrades through the update Channel
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-043, REL-057, PKG-089, INS-054, REL-029
- Baseline: §30, §31
- Benchmarks: B-043
- Invariants: I-022

V4 exit: community V3 installs upgrade with user data preserved and rollback to V3 possible. REL promotes the artifacts; INS and PKG apply generations. Retention must still serve V3 objects (REL-029).

#### Out of scope
Client upgrade path (INS-054). Store format migration (PKG-089). Rollback UX (INS-014).

#### Acceptance criteria
- [ ] A V3 generation in the retained set can fetch the V4 generation from the update channel.
- [ ] Rollback from the V4 generation to the V3 generation boots and leaves user data intact.
- [ ] B-043 records the in-place upgrade attempts on V4 hardware scope.

#### Verification
- Integration: `rel:tests/upgrade/v3_to_v4_and_rollback` with INS-054 on H-001 and H-002.
- Bench: B-043 on H-002; target per register.

#### Evidence
- none

### REL-060 · Set published security-response targets per stable release
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-053, REL-018, GOV-075
- Baseline: §54, §56.4
- Risks: R-061, R-062
- Invariants: I-061, I-088

1.0 is judged on patch latency; the target is a register entry, not a number in this task. V4 support-policy drafts include the CVE SLA that 1.0 then meets. BEN owns the B-ID once allocated.

<!-- covers: GAP-0354 -->

#### Out of scope
Harness (REL-018). GOV contract (GOV-075). Trailing-window proof (REL-064).

#### Acceptance criteria
- [ ] A benchmark-register entry (or a BEN task that adds one) names inherited High and Critical CVE time-to-generation with method, harness and per-milestone target kinds.
- [ ] This task's prose and the Decision it cites contain no latency number (I-088).
- [ ] GOV-075 cites the same B-ID.

#### Verification
- Review: BEN and GOV leads sign off that the SLA is a register target, not a sentence in REL.md.

#### Evidence
- none

### REL-061 · Run organized pre-release testing of installer, updater and rollback
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-017, REL-057, INS-027, INS-045, LNX-107, WIN-080
- Baseline: §49, §63
- Invariants: I-096

Structured community rounds with published cases for installer, updater, rollback and Personalities catch hardware the lab matrix misses before each stable.

<!-- covers: GAP-0398 -->

#### Out of scope
Lab soak matrix (BLD-076). Installer implementation (INS-027). Corpus scenarios (LNX, WIN).

#### Acceptance criteria
- [ ] Published test cases cover installer, updater, rollback and Personality launch for the RC under test.
- [ ] Community results are filed against generation ID and H-ID or an unsupported-hardware label.
- [ ] A P0 from this round fails REL-017 for that RC.

#### Verification
- Manual: one organized round on H-002, H-004 and H-005 using the published cases.
- Review: INS and REL leads confirm P0 routing into qualification.

#### Evidence
- none

### REL-062 · Publish crash-free-session and boot-success rates per Channel
- Type: benchmark
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-066, REL-055, REL-042, BEN-049, BEN-052
- Baseline: §54
- Benchmarks: B-041, B-043
- Risks: R-057
- Invariants: I-061, I-088

1.0 stability gate: crash-free-session and boot-success rates that stable-channel generations must meet. Targets and method live in B-041 and B-043; this task publishes the soak measurement per channel and does not restate numbers.

<!-- covers: GAP-0367 -->

#### Out of scope
Dashboard (REL-055). Soak operation (REL-066). Methodology (BEN-007).

#### Acceptance criteria
- [ ] A B-041 report exists for the 1.0 soak window on the stable channel with machine count.
- [ ] A B-043 report exists for the same window covering boot-success of stable-channel generations.
- [ ] No REL announcement text contains a rate number except by citing those reports (I-061).

#### Verification
- Bench: B-041 and B-043 on the 1.0 soak fleet; targets per register.
- Review: BEN lead confirms the per-channel split matches the harness method.

#### Evidence
- none

### REL-063 · Sign and launch the 1.0 stable Channel with the final HCL
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-066, REL-057, REL-041, REL-048, HW-088, REL-064
- Baseline: §62, §63
- Invariants: I-095

1.0 scope: final signing, stable channel launch and final Hardware Compatibility List publication for Tier 1 and community Tier 2 with a promotion path. Reuses REL-057 and REL-048.

#### Out of scope
HCL content authorship (HW-088). Soak (REL-066). Notes (REL-065).

#### Acceptance criteria
- [ ] The 1.0 generation is signed with production keys and is the stable channel head.
- [ ] The published Hardware Compatibility List at launch matches HW-088 for Tier 1 and names the Tier 2 promotion path (I-095).
- [ ] Development keys cannot sign the stable head.

#### Verification
- Integration: `rel:tests/launch/stable_head_signatures` on the 1.0 candidate.
- Review: HW and REL leads confirm the launched HCL matches the lab sign-off.
- Demo: signed 1.0 image install on H-002 with the published HCL verdict.

#### Evidence
- none

### REL-064 · Publish trailing inherited-CVE response against the SLA
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-060, REL-018, KRN-057
- Baseline: §54, §56.4
- Risks: R-061, R-062
- Invariants: I-061, I-088

1.0 support commitment: trailing inherited High and Critical CVE response meets the published SLA for the required fraction, with the distribution published. Uses REL-018 and REL-060. No latency number appears in this task.

#### Out of scope
Register target (REL-060). Kernel compliance report (KRN-057). Ingestion (REL-006).

#### Acceptance criteria
- [ ] A published distribution of inherited High and Critical CVE time-to-generation covers the trailing window the register names.
- [ ] The report states pass or fail against the REL-060 target without copying the number into REL.md.
- [ ] KRN-057 and this report cite the same B-ID.

#### Verification
- Bench: inherited-CVE time-to-generation versus the register target; B-ID per REL-060.
- Review: KRN and BEN leads confirm the trailing window matches.

#### Evidence
- none

### REL-065 · Publish 1.0 release notes and the V3/V4 migration guide
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-014, REL-063, BEN-062
- Baseline: §31, §54, §63
- Invariants: I-061

1.0 scope: release notes and migration guide from V3 and V4 installs. Generated from REL-014 plus a written migration path. No performance claims without BEN links (I-061).

#### Out of scope
Notes generator (REL-014). Docs site (DOC). Claim lint (BEN-062).

#### Acceptance criteria
- [ ] 1.0 notes list task IDs and generation diffs produced by REL-014.
- [ ] A migration guide names the supported V3-to-1.0 and V4-to-1.0 paths REL-067 verifies.
- [ ] BEN-062 reports zero uncited performance claims in the notes.

#### Verification
- Review: REL, DOC and BEN claim-audit owners sign off on the notes and guide.

#### Evidence
- none

### REL-066 · Soak the 1.0 release candidate on Tier 1 and the beta fleet
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-057, REL-061, REL-017, BLD-079, HW-087
- Baseline: §54, §63
- Benchmarks: B-041, B-042, B-043
- Risks: R-057, R-063
- Invariants: I-061

1.0 exit: release-candidate soak on the full Tier 1 fleet and beta fleet with no open P0 or P1. REL owns the soak channel and go/no-go against REL-027. Rates are B-IDs, not sentences.

#### Out of scope
CI matrix jobs (BLD-079). Hardware sign-off (HW-087). Rate publication (REL-062).

#### Acceptance criteria
- [ ] The candidate is the only generation on the soak channel for the window B-041 names.
- [ ] Qualification remains green and HW-087 reports zero open P0 or P1 on Tier 1.
- [ ] B-041, B-042 and B-043 reports exist for the soak window.

#### Verification
- Bench: B-041, B-042 and B-043 on the soak fleet; targets per register.
- Integration: BLD-079 remains green for the candidate.
- Review: REL go/no-go recorded against REL-027.

#### Evidence
- none

### REL-067 · Define and CI-test supported upgrade paths between stables
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-029, REL-059, INS-047, PKG-091
- Baseline: §30, §31
- Benchmarks: B-043
- Invariants: I-022

1.0 users must reach later 1.x without reinstall. N to N+1 and skip-version paths run in CI on real generation histories, honouring REL-029.

<!-- covers: GAP-0346 -->

#### Out of scope
Retention Decision (REL-029). Client chains (INS-047). Rollback guarantee (PKG-091).

#### Acceptance criteria
- [ ] CI runs N-to-N+1 and skip-version upgrades on stored generation histories that REL-029 still serves.
- [ ] A skip-version upgrade that needs a missing object fails closed with a typed error rather than a partial apply.
- [ ] B-043 includes those CI attempts.

#### Verification
- Integration: `rel:tests/upgrade/n_to_n1_and_skip` with INS-047 on H-001.
- Bench: B-043 on H-001; target per register.

#### Evidence
- none
