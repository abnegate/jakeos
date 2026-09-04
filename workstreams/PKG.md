# PKG · Packages, dependencies, generations, history
- Prefix: PKG
- Lead: none
- Baseline: §28, §29, §30, §31, §34

<!-- roadmap:generated:begin summary -->
Tasks: 91 live, 0 done, 0 in-progress, 91 todo, 0 dropped. Ready: 2. Blocked: 89. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

PKG owns immutable Packages as the unit of software distribution, the Package
manifest (identity, Components, Interfaces, RequestedCapabilities, Dependencies,
Resources, and reserved signing fields), the content-addressed Package object
model over the STO store, per-Package dependency binding, SystemGeneration
composition and switching, the typed history log, restore of generations and
Package sets, the repository client, and personality packaging into immutable
store objects. Native installation is make-available: objects enter the store
and the next SystemGeneration, and nothing is mutated in place (§28, §30, §67
Principle 7). Launch maps known Package objects; it does not look up paths or
resolve dependencies (§34, §53).

## Out of scope

STO owns the content-store substrate, snapshots, ApplicationData and the
filesystem choice. BOOT owns bootloader entries, the generation boot menu, boot
counting and measured boot. REL owns the repository server, signing keys,
advisories and emergency pull. INS owns the image builder, installer, updater
client, recovery environment and rollback UX. CMP maps Package objects into a
Component address space. MEM owns file-backed mapping and shared pages. CAP and
SEC own Capability rights, grant taxonomy and install-time review. APP owns the
store client, consent UI and the running-app update prompt. SVC owns native init
and the default-application registry. LNX and WIN own personality runtimes.
SDK owns `os` CLI printing and cargo Package emission. BLD owns bit-for-bit
rebuild CI. GOV owns licensing policy. ENV owns environment.yaml resolution.

## Tasks

### PKG-001 · Benchmark SystemGeneration rollback time
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-015, PKG-020
- Baseline: §30, §54, §60
- Benchmarks: B-022

Measure wall time to roll back to the previous SystemGeneration, including
boot-menu selection of generation N after N+1 was the default, on the V0.5
hardware scope. Results publish under B-022; this task owns the rollback
segment of that metric.

#### Out of scope
BOOT-006 owns the reboot-into-previous-generation
segment. Generation switch time is PKG-002.

#### Acceptance criteria
- [ ] A B-022 report exists for H-001 and H-002 covering rollback time.
- [ ] The harness rolls back a system that has at least ten retained generations.
- [ ] The published report names the ostree and nixos-rebuild baselines from the register.

#### Verification
- Bench: B-022 on H-001 and H-002; target per register.
- Integration: `pkg:tests/generation/rollback_bench_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-002 · Benchmark SystemGeneration switch time
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-016, PKG-020
- Baseline: §30, §54, §60
- Benchmarks: B-022

Measure wall time to compose SystemGeneration N+1 from an updated Package
set and to switch the next-boot default to it, plus store growth per
generation, on the V0.5 hardware scope. Results publish under B-022.

#### Out of scope
Rollback time (PKG-001). INS image-builder time.

#### Acceptance criteria
- [ ] A B-022 report exists for H-001 and H-002 covering compose time, switch time and store growth per generation.
- [ ] The harness runs on a system that already retains ten generations.
- [ ] Disk overhead is reported as store growth, not as a full image copy.

#### Verification
- Bench: B-022 on H-001 and H-002; target per register.
- Integration: `pkg:tests/generation/switch_bench_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-003 · Benchmark Package install time on the 20-Package Corpus
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-025, PKG-036
- Baseline: §28, §54, §60
- Benchmarks: B-021

Measure wall time to make each Package in the 20-Package corpus available
in the content-addressed store without activation, from a local source.
Results publish under B-021. Dedup ratio is PKG-004.

#### Out of scope
Repository fetch (PKG-064). STO ingest microbenchmarks.

#### Acceptance criteria
- [ ] A B-021 report exists for H-001 and H-002 covering install time on the 20-Package corpus.
- [ ] The harness installs from local Package files, not from a network repository.
- [ ] The published report names the dnf/apt, flatpak and nix-env baselines from the register.

#### Verification
- Bench: B-021 on H-001 and H-002; target per register.
- Integration: `pkg:tests/install/install_bench_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-004 · Benchmark store deduplication ratio on the 20-Package Corpus
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-038, PKG-036, PKG-042, STO-008, STO-009, PKG-003, BEN-007
- Baseline: §27, §29, §54
- Benchmarks: B-021

Measure the ratio of unique stored bytes to total Package bytes after
installing the 20-Package corpus that shares dependencies. Results publish
under B-021. PKG-004 owns the store-ingest view of the
same metric.

<!-- covers: INV-0547, INV-0512 -->

#### Out of scope
Install wall time (PKG-003). STO object-ingest time.

#### Acceptance criteria
- [ ] A B-021 report exists for H-001 and H-002 covering unique-stored-bytes over total Package bytes for the 20-Package corpus.
- [ ] Store size is measured before and after the corpus install.
- [ ] Identical dependency objects across Packages contribute once to unique stored bytes.

#### Verification
- Bench: B-021 on H-001 and H-002; target per register.
- Integration: `pkg:tests/store/dedup_bench_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-005 · Decide the content hash algorithm and chunking strategy for the store
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-040, PKG-041
- Baseline: §27, §28
- Decision: D-0213

Decide the content hash algorithm and chunking strategy used in Package
identifiers and signatures. Hash identity cannot change later without
re-identifying every object (§27). Options are sha256 versus BLAKE3, and
whole-object hashing versus content-defined chunking, using the spike
measurements. STO-013 records the store identifier format;
this adr records the Package-identity consequence.

<!-- covers: GAP-0526, INV-0514, INV-0540 -->

#### Out of scope
Store directory layout (PKG-014). Signature scheme
(REL-003).

#### Acceptance criteria
- [ ] Options evaluated include sha256 whole-object, BLAKE3 whole-object, sha256 content-defined chunking and BLAKE3 content-defined chunking.
- [ ] The accepted option cites PKG-040 for dedup, update size and hash throughput.
- [ ] The decision records that Package identity is derived from content so two builds of identical content share identity.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: ABI and PKG leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-006 · Decide dependency resolution semantics and lockfile location
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-013, PKG-005
- Baseline: §29, §53
- Decision: D-0214

Decide whether a Package manifest pins Dependencies by exact content hash
or by version ranges resolved at install time, and where lockfiles live.
§53 forbids launch-time resolution, so the choice must be settled before
the install path is built. Either option still stores resolved objects by
content identity.

<!-- covers: INV-0549 -->

#### Out of scope
Security-fix propagation onto pins (PKG-046).
Launch mapping (PKG-037).

#### Acceptance criteria
- [ ] Options evaluated include exact content-hash pins in the manifest, version ranges resolved at install into a lockfile next to the manifest, and version ranges resolved into a generation-level lock.
- [ ] The accepted option records that launch performs no dependency resolution.
- [ ] The decision names the lockfile location or records that pins live only in the manifest.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG and SDK leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-007 · Decide what is excluded from a SystemGeneration and how mutable state is separated
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-041, PKG-009, Q-023
- Baseline: §30, §31
- Decision: D-0216

Decide what is excluded from a SystemGeneration (user data, ApplicationData,
logs, caches) and how mutable state is separated from the immutable image
so rollback never rewrites user data. This answers Q-023 and is a
prerequisite for generation compose and the 1.0 rollback guarantee.

<!-- covers: INV-0565 -->

#### Out of scope
Materialising the generation tree (PKG-008).
ApplicationData implementation (STO-002).

#### Acceptance criteria
- [ ] Options evaluated include excluding user data, ApplicationData, logs and caches from the generation tree, snapshotting selected mutable trees into the generation, and a hybrid with explicit restorable classes.
- [ ] The accepted option states that rolling back a generation leaves excluded state intact.
- [ ] The decision lists each excluded class and its owning prefix.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG, STO and INS leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-008 · Decide how a SystemGeneration is materialised on disk
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-014, PKG-041, STO-016
- Baseline: §26, §30
- Decision: D-0217
- Risks: R-018
- Invariants: I-044

Decide how a SystemGeneration is materialised on disk: a profile tree over
the store, a filesystem snapshot per generation, or a verified image per
generation. V0.5 exit requires this generation-switching mechanism adr.
The choice sits on the STO filesystem decision and does not invent a
native filesystem (§26, I-044).

<!-- covers: INV-0564 -->

#### Out of scope
BOOT boot-entry format (BOOT-007). Store layout
(PKG-014).

#### Acceptance criteria
- [ ] Options evaluated include a profile tree over the content store, a filesystem snapshot per generation, and a verified image per generation.
- [ ] The accepted option records that generation N remains bootable after N+1 is composed.
- [ ] ZFS is listed as rejected on licence grounds before measurement, matching the STO substrate adr.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG, STO and BOOT leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-009 · Decide that Package mutation is replaced by immutable Packages and SystemGenerations
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-041
- Baseline: §2, §28, §30, §67
- Decision: D-0218
- Invariants: I-022, I-036

Record the §2 replacement of package mutation by immutable Packages and
SystemGenerations. The standing rules are: no in-place mutation of Package
contents, no shared-filesystem writes on install, and versioned generations
plus explicit history instead of an accumulation of mutable files. The
immutability gate enforces this adr.

<!-- covers: INV-0077, INV-0526, INV-0041, INV-0043, INV-0553, INV-1298 -->

#### Out of scope
Per-Package dependency objects (PKG-013).
The CI gate (PKG-024).

#### Acceptance criteria
- [ ] Options evaluated include immutable Packages plus SystemGenerations, in-place Package mutation with snapshots, and a hybrid writable overlay on immutable bases.
- [ ] The accepted option records that installed Package contents are never mutated in place.
- [ ] The decision lists I-022 and I-036 as standing rules the immutability gate enforces.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: architecture lead sign-off recorded on the pull request that accepts the decision.

#### Evidence
- none

### PKG-010 · Decide how immutable Packages preserve LGPL relinking rights
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-009, PKG-012, GOV-003, SDK-026
- Baseline: §28
- Decision: D-0219
- Invariants: I-069

Decide how content-addressed packaging preserves LGPL section 6
substitution for libraries such as glibc and Wine: dynamic linking as
separate store objects versus shipped object files, and how a user
substitutes a modified library object into a local generation. Coordinated
with GOV licensing policy.

<!-- covers: GAP-0019 -->

#### Out of scope
GOV license firewall (GOV-003). Native linking model
(SDK-026). Wine prefix layout
(PKG-080).

#### Acceptance criteria
- [ ] Options evaluated include dynamically linked separate store objects the user can replace, shipping relinkable object files inside the Package, and a documented local-generation substitution flow.
- [ ] The accepted option shows a user substituting a modified LGPL library object without mutating the original Package identity.
- [ ] The decision records I-069 as the standing rule.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: GOV and PKG leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-011 · Decide the Package manifest schema shape and its Layer 2 evolution rules
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-009, SEC-007, SEC-004, CAP-007, IPC-002
- Baseline: §11, §12, §28, §66
- Decision: D-0220
- Risks: R-017

Decide the Package manifest schema shape (identity, version, Components,
Interfaces, RequestedCapabilities, Dependencies, Resources) and the Layer 2
evolution rules that apply to it. This is a hard-to-change Layer 2 decision
and depends on the SEC authority-source and grant-taxonomy adrs so
capability requests match the grant model. Surface S-018 is the freeze
candidate.

<!-- covers: INV-0527, INV-0233 -->

#### Out of scope
Implementing the schema (PKG-031). Grant runtime
(SEC-006). Component Inputs/Outputs binding
(CMP-025).

#### Acceptance criteria
- [ ] Options evaluated include a single typed manifest document, a split Package-plus-Component manifest pair matching S-018 and S-019, and an IDL-defined manifest served as a Layer 2 interface.
- [ ] The accepted option requires reserved signature, signer and trust-policy fields before the first install.
- [ ] The accepted option describes multi-Component graphs with per-Component capability requirements and connections.
- [ ] Evolution rules are those of S-014 as prototyped by IPC-002, not a Package-only dialect.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG, SEC and IPC leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-012 · Decide the on-disk and on-wire Package format and its relation to the store
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-014, PKG-005, PKG-009
- Baseline: §27, §28
- Decision: D-0221

Decide the on-disk and on-wire Package format and its relation to the
content store: a content-addressed tree with a manifest versus a single
signed archive. The format must be decided before the first immutable
install.

<!-- covers: INV-0539 -->

#### Out of scope
Manifest schema (PKG-011). Signing scheme
(REL-003).

#### Acceptance criteria
- [ ] Options evaluated include a content-addressed tree with a manifest, a single signed archive whose payload unpacks into the store, and a hybrid archive that is also a store object.
- [ ] The accepted option names how the format maps onto the chosen store layout.
- [ ] The decision records that two builds of identical content produce the same Package identity.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG and REL leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-013 · Decide that global dependency installation is replaced by per-Package dependency objects
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-009, PKG-041
- Baseline: §2, §29
- Decision: D-0222
- Invariants: I-036

Record the §2 replacement of global dependency installation by immutable
per-Package dependency objects referenced by content identity. Installing
any Package must never break another installed Package; there is no global
conflict state (§29).

<!-- covers: INV-0078, INV-0544, INV-0546 -->

#### Out of scope
Resolution and lockfiles (PKG-006). The
conflicting-library test (PKG-042).

#### Acceptance criteria
- [ ] Options evaluated include per-Package content-identity dependency objects, a global shared library directory, and a generation-wide dependency set with conflict resolution.
- [ ] The accepted option records that dependencies are referenced by content identity, never by mutable global name lookup.
- [ ] The decision records I-036 as the standing no-global-conflict rule.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: architecture lead sign-off recorded on the pull request that accepts the decision.

#### Evidence
- none

### PKG-014 · Decide the Content-addressed store layout for Packages and SystemGenerations
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-040, PKG-041, PKG-005
- Baseline: §27, §30
- Decision: D-0228

Decide the persistent on-disk layout shared by Packages, SystemGenerations
and development environments: Nix-style store paths, an OSTree-style object
repository, or a casync-like chunk store. V0.5 exit requires an accepted
content-addressing scheme adr. STO-017 then maps the
layout onto the chosen filesystem.

<!-- covers: GAP-0525, INV-0516 -->

#### Out of scope
Hash algorithm (PKG-005). Filesystem substrate
(STO-016). Generation materialisation
(PKG-008).

#### Acceptance criteria
- [ ] Options evaluated include Nix-style store paths, an OSTree-style object repository and a casync-like chunk store.
- [ ] The accepted option cites PKG-040 for dedup ratio and update size.
- [ ] The decision records that a new SystemGeneration costs only its changed objects.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG, STO and ENV leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-015 · Add the automated boot-menu test restoring previous kernel, compositor and Packages
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-020, PKG-018, PKG-021, PKG-025
- Baseline: §30, §60
- Risks: R-018

Add the automated boot-menu test that is the V0.5 generation demo: install
a Package, compose generation N+1, select N at the boot menu, and observe
that the Package is gone and the previous kernel, compositor and Packages
are intact (§60).

<!-- covers: INV-0568, INV-1186, INV-0560 -->

#### Out of scope
BOOT-014 owns menu presentation. BLD-022
owns the harness plumbing.

#### Acceptance criteria
- [ ] After installing a Package into N+1 and booting N, `os package list` does not name that Package.
- [ ] The kernel image hash and compositor Package identity match generation N.
- [ ] The test runs on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Verification
- Integration: `pkg:tests/generation/boot_rollback_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Demo: install a Package, roll back at the boot menu, Package gone and previous state intact, on H-002.

#### Evidence
- none

### PKG-016 · Compose SystemGeneration N+1 as a content-addressed tree sharing objects with N
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: PKG-019, PKG-018, PKG-008, STO-008
- Baseline: §27, §30
- Invariants: I-022

Compose SystemGeneration N+1 as a content-addressed tree that shares
objects with N. An update creates N+1 and leaves N intact and bootable.
The new generation costs only its changed objects (§27, §30). INS
image builder consumes this compose path.

<!-- covers: INV-0560, INV-0522, INV-0516, INV-0043 -->

#### Out of scope
INS-001 consumes the composed tree. BOOT selection of N versus
N+1. Atomic power-cut commit test (PKG-082).

#### Acceptance criteria
- [ ] Composing N+1 leaves generation N listed and bootable.
- [ ] Unchanged kernel, compositor and Package objects in N+1 have the same content identity as in N.
- [ ] Store growth after a one-Package update equals that Package's unique objects, not a full tree copy.
- [ ] A write into the composed tree of N fails with a typed error.

#### Verification
- Unit: `pkg:tests/generation/compose_*` on CI matrix entry `qemu-x86_64`.
- Integration: `pkg:tests/generation/share_with_n_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-017 · Separate ApplicationData, logs and caches from the immutable SystemGeneration tree
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-007, PKG-019, STO-002
- Baseline: §25, §30

Put ApplicationData, logs and caches outside the immutable SystemGeneration
tree so rollback never touches user data. Implements
PKG-007. STO owns ApplicationData storage; PKG
owns the generation membership boundary.

<!-- covers: INV-0565 -->

#### Out of scope
ApplicationData implementation (STO-002). User-data
snapshots (STO).

#### Acceptance criteria
- [ ] A file written into ApplicationData while generation N+1 is default remains after boot of generation N.
- [ ] Logs and caches written during N+1 remain after rollback to N.
- [ ] `os inspect` on a SystemGeneration lists no ApplicationData, log or cache objects as generation members.

#### Verification
- Integration: `pkg:tests/generation/excluded_state_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Manual: write a file in ApplicationData, roll back, confirm the file is present.

#### Evidence
- none

### PKG-018 · Pin the kernel image and modules in a SystemGeneration with BOOT selection
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-019, PKG-008, BOOT-007, BOOT-011
- Baseline: §30
- Risks: R-018

Pin the kernel image and modules in a SystemGeneration so selecting a
previous generation at boot restores the previous kernel (§30). BOOT owns
the bootloader entry format; PKG owns the pin inside the generation object.

<!-- covers: INV-0555 -->

#### Out of scope
BOOT-007. BOOT-011. Module signing
(KRN).

#### Acceptance criteria
- [ ] A SystemGeneration object names a kernel image identity and a module-set identity.
- [ ] Selecting generation N at boot loads the kernel image pinned by N, not the N+1 image.
- [ ] Changing the running kernel's on-disk bytes does not change the pinned identity of a sealed generation.

#### Verification
- Integration: `pkg:tests/generation/kernel_pin_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: BOOT lead confirms the pin fields match BOOT-007.

#### Evidence
- none

### PKG-019 · Define SystemGeneration as the platform Object naming one bootable configuration
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-009, PKG-014, PKG-029
- Baseline: §7, §30

Define SystemGeneration as the platform object that names one complete
bootable configuration: a content-addressed tree with reserved signing
fields (§30). This is the fundamental versioned-OS-state abstraction.

<!-- covers: INV-0056, INV-0554 -->

#### Out of scope
Kernel and userspace pins (PKG-018,
PKG-021). BOOT offering the object at the menu.

#### Acceptance criteria
- [ ] `os inspect` on a SystemGeneration prints identity, parent generation, kernel pin, userspace pins and Package set.
- [ ] Two generations with identical contents share object identity.
- [ ] The generation manifest includes reserved signature, signer and trust-policy fields that V0.5 leaves empty.
- [ ] Creating a generation does not mutate an existing generation object.

#### Verification
- Unit: `pkg:tests/generation/object_*` on CI matrix entry `qemu-x86_64`.
- Integration: `pkg:tests/generation/inspect_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-020 · Build `os Generation list|switch|previous` setting the next-boot default
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-016, BOOT-007, SDK-006
- Baseline: §30, §60
- Risks: R-018

Build `os generation list`, `os generation switch` and `os generation
previous` so a developer can create, list and select the next-boot default
(§60). Switch writes the BOOT entry; it does not mutate generation N.

<!-- covers: INV-0568, INV-1186 -->

#### Out of scope
Boot-menu presentation (BOOT-014). `os restore`
(PKG-060). SDK printing helpers.

#### Acceptance criteria
- [ ] `os generation list` names every retained generation with identity and current/next-boot flags.
- [ ] `os generation switch <id>` sets the next-boot default to that id and leaves the running generation unchanged.
- [ ] `os generation previous` sets the next-boot default to the parent of the current default.
- [ ] Switching to an unknown id returns a typed error and writes no boot entry.

#### Verification
- Integration: `pkg:tests/generation/cli_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Manual: list, switch, reboot, confirm the selected generation is running.

#### Evidence
- none

### PKG-021 · Pin shell, compositor, system services and installed Packages in a SystemGeneration
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-019
- Baseline: §30

Pin the desktop shell, compositor, system services and installed Package
set in a SystemGeneration (§30). SVC native init starts the pinned
services from the selected generation.

<!-- covers: INV-0556, INV-0557, INV-0558, INV-0559 -->

#### Out of scope
SVC-007 starts the pins. APP session host. Package install
(PKG-025).

#### Acceptance criteria
- [ ] A SystemGeneration names identities for shell, compositor, each system service and each installed Package.
- [ ] Selecting generation N at boot starts the compositor Package pinned by N.
- [ ] Adding a Package records it in N+1 and not in N.

#### Verification
- Integration: `pkg:tests/generation/userspace_pins_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: SVC lead confirms init can read the pin list.

#### Evidence
- none

### PKG-022 · Implement the durable typed history event log for system state changes
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-007
- Baseline: §31

Implement the durable typed history event log so all system state changes
are explicit events rather than inferred from files (§31). V0.5 records
the log; V1 `os history` lists it.

<!-- covers: INV-0571 -->

#### Out of scope
CLI listing (SDK-019, PKG-059). Restore
(PKG-060). Event catalogue
(PKG-023).

#### Acceptance criteria
- [ ] Appending an event survives process restart and reboot.
- [ ] Each event has a type, timestamp source, actor identity and payload schema.
- [ ] The log lives outside the immutable generation tree per PKG-007.
- [ ] Readers without the history Capability receive `Error::Rights`.

#### Verification
- Unit: `pkg:tests/history/log_*` on CI matrix entry `qemu-x86_64`.
- Integration: `pkg:tests/history/durable_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-023 · Record Package install, update and OS update events in history
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-022, PKG-025, PKG-016
- Baseline: §31

Record Package install, Package update and OS-update (new SystemGeneration)
events in the history log as the V0.5 event catalogue (§31).

<!-- covers: INV-0573, INV-0574, INV-0576 -->

#### Out of scope
Driver and environment events (PKG-053). CLI listing.

#### Acceptance criteria
- [ ] Installing a Package appends a typed install event naming Package identity.
- [ ] Replacing a Package in N+1 appends a typed update event naming old and new identities.
- [ ] Composing a SystemGeneration appends a typed OS-update event naming N and N+1.

#### Verification
- Integration: `pkg:tests/history/package_events_*` on CI matrix entry `qemu-x86_64`.

#### Evidence
- none

### PKG-024 · Add the CI Gate proving store and Generation trees are never mutated in place
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-009, PKG-038, PKG-016
- Baseline: §30, §67
- Invariants: I-022, I-036

Add the CI gate that enforces Principle 7 and the §30 invariants: writes
into the store or the active generation fail with a typed error, and a lint
forbids mutable paths in PKG code.

<!-- covers: INV-1298, INV-0043, INV-0553, INV-0526 -->

#### Out of scope
Install filesystem-diff test (STO-032). STO volume
harness (STO-032).

#### Acceptance criteria
- [ ] Opening a store object for write returns a typed error and changes no bytes.
- [ ] Opening an active generation tree for write returns a typed error.
- [ ] CI fails a PKG change that introduces a mutable path into store or generation code.
- [ ] The gate runs on every merge to main.

#### Verification
- Unit: `pkg:tests/immutability/write_denied_*` on CI matrix entry `qemu-x86_64`.
- Review: lint allowlist is empty except for documented test doubles.

#### Evidence
- none

### PKG-025 · Implement install as make-available into the store and next SystemGeneration
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: PKG-038, PKG-016, PKG-033, PKG-031, PKG-006, STO-031
- Baseline: §28, §60
- Threats: T-002, T-006
- Invariants: I-020

Implement install as make-available: fetch objects into the content store,
register the Package in the current or next SystemGeneration, and expose
its launcher entry (§28). This is the V0.5 install exit criterion. V0.5
installs from local files only.

<!-- covers: INV-0533, INV-1183, INV-0041 -->

#### Out of scope
Repository fetch (PKG-064). Uninstall
(PKG-043). CLI (PKG-034). APP
launcher chrome.

#### Acceptance criteria
- [ ] Installing a local Package file registers it in generation N+1 and leaves generation N unchanged.
- [ ] The Package's launcher entry is visible to the session host after N+1 is selected.
- [ ] Install without `Capability<Package, Install>` returns `Error::Rights` and allocates no store objects.
- [ ] Install of a Package whose Dependencies are missing returns a typed error and commits no generation.

#### Verification
- Integration: `pkg:tests/install/make_available_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Demo: cold boot to the four native applications running from immutable Packages on H-002.

#### Evidence
- none

### PKG-026 · Implement manifest Dependencies as content-addressed Package references
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-013, PKG-006, PKG-031
- Baseline: §28, §29

Implement manifest Dependencies as content-addressed Package references,
never as mutable global name lookup (§29).

<!-- covers: INV-0532, INV-0544 -->

#### Out of scope
Binding multiple versions (PKG-032).
Resolution lockfile location (PKG-006).

#### Acceptance criteria
- [ ] A valid manifest Dependency field is a content identity accepted by PKG-005.
- [ ] A Dependency that is a mutable name without identity fails manifest validation.
- [ ] `os inspect` on a Package lists each Dependency by content identity.

#### Verification
- Unit: `pkg:tests/manifest/dependencies_*` on CI matrix entry `qemu-x86_64`.

#### Evidence
- none

### PKG-027 · Build the Package manifest validator and CI lint
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-031, PKG-028
- Baseline: §21, §28

Build the Package manifest validator and CI lint. It rejects manifests
that request persistent services without an explicit capability grant
(§21) and enforces schema validity for the four V0.5 applications.

<!-- covers: INV-0536, INV-1182 -->

#### Out of scope
Grant runtime (SEC-006). Background-execution Capability
(CAP-017).

#### Acceptance criteria
- [ ] A manifest that requests a persistent service without an explicit background-execution Capability fails validation.
- [ ] A schema-invalid manifest for Terminal, File Browser, Text Editor or Image Viewer fails CI.
- [ ] A valid four-app manifest passes the lint with zero findings.

#### Verification
- Unit: `pkg:tests/manifest/lint_*` on CI matrix entry `qemu-x86_64`.
- Review: APP lead confirms the four V0.5 app manifests pass.

#### Evidence
- none

### PKG-028 · Implement RequestedCapabilities in the manifest and launcher-time initial grant
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-011, PKG-031, SEC-007, SEC-004
- Baseline: §9.1, §28
- Risks: R-017
- Threats: T-006
- Invariants: I-021

Implement RequestedCapabilities in the Package manifest. The launcher
grants the small initial set at start (§9.1). The field drives SEC
install-time review and CAP grant binding.

<!-- covers: INV-0531, INV-0212 -->

#### Out of scope
Optional versus required (PKG-075). Permissions
UI (APP, SEC). CAP-025 attaches the granted set.

#### Acceptance criteria
- [ ] A Package manifest lists RequestedCapabilities as typed names matching the SEC grant taxonomy.
- [ ] Image Viewer starts with UI and GPU capabilities only, matching its manifest, and holds no filesystem Capability.
- [ ] A request for a Capability type not in the taxonomy fails manifest validation.
- [ ] `os inspect` on a launched Component lists exactly the granted subset of RequestedCapabilities.

#### Verification
- Integration: `pkg:tests/manifest/requested_caps_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: SEC lead confirms taxonomy names match SEC-007.

#### Evidence
- none

### PKG-029 · Reserve signature, signer and trust-policy fields in Package and Generation manifests
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-011
- Baseline: §28, §30
- Invariants: I-080

Reserve signature, signer and trust-policy fields in Package and
SystemGeneration manifests before the first immutable install so V1
signing is not a breaking format migration.

<!-- covers: EXTRA-067, GAP-0323 -->

#### Out of scope
Filling the fields (PKG-055). REL key
hierarchy.

#### Acceptance criteria
- [ ] Package and generation manifest schemas include signature, signer and trust-policy fields.
- [ ] V0.5 install accepts empty reserved fields and rejects unknown keys in their place.
- [ ] A golden-file test fails if the reserved field names are renamed or removed.

#### Verification
- Unit: `pkg:tests/manifest/reserved_signing_fields_*` on CI matrix entry `qemu-x86_64`.

#### Evidence
- none

### PKG-030 · Implement Package Resources and read-only PackageData access
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-031, STO-005
- Baseline: §25, §28

Implement Package Resources (assets, localisation data) and read-only
PackageData access for the owning Components (§25, §28).

<!-- covers: INV-0529, INV-0483 -->

#### Out of scope
ApplicationData (STO-002). TXT localisation
pipeline.

#### Acceptance criteria
- [ ] A Package can bundle named Resources addressable as PackageData.
- [ ] The owning Component reads PackageData; a write returns a typed error.
- [ ] A Component that is not an owner of the Package receives `Error::Rights` on PackageData.

#### Verification
- Unit: `pkg:tests/manifest/packagedata_*` on CI matrix entry `qemu-x86_64`.
- Integration: Image Viewer reads a bundled asset via PackageData on `qemu-x86_64`.

#### Evidence
- none

### PKG-031 · Implement the Package manifest schema with identity, version, Components and Interfaces
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-011, PKG-029
- Baseline: §11, §28, §60
- Risks: R-017

Implement the Package manifest schema: identity, version, Components
(entry points, code objects, per-Component capability requirements,
connections) and exposed typed Interfaces (§28). Native applications are
Component graphs; the format describes multiple Components (§11).

<!-- covers: INV-0527, INV-1182, INV-0055, INV-0528, INV-0233, INV-0530 -->

#### Out of scope
RequestedCapabilities (PKG-028). Dependencies
(PKG-026). CMP graph instantiation.

#### Acceptance criteria
- [ ] A Package manifest names identity, version, one or more Components and zero or more Interfaces.
- [ ] Each Component entry lists an entry point, code-object identity and per-Component capability requirements.
- [ ] The four V0.5 application Packages parse as valid manifests.
- [ ] A manifest that omits identity or Components fails validation with a typed error.

#### Verification
- Unit: `pkg:tests/manifest/schema_core_*` on CI matrix entry `qemu-x86_64`.
- Review: CMP lead confirms Component fields match CMP-025.

#### Evidence
- none

### PKG-032 · Bind each Package to its own dependency versions so multiple library versions coexist
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-026, PKG-038
- Baseline: §29

Bind each Package to its own dependency versions so App A can use Library
1.4 while App B uses Library 2.1, with no global conflict state (§29).

<!-- covers: INV-0545, INV-0546, INV-0544 -->

#### Out of scope
The two-app test (PKG-042). Personality
runtime pinning (PKG-076).

#### Acceptance criteria
- [ ] Two installed Packages can name different identities for the same library name.
- [ ] Launching either Package maps only its bound dependency identity.
- [ ] Installing the second Package does not change the first Package's bound identities.

#### Verification
- Integration: `pkg:tests/deps/multi_version_bind_*` on CI matrix entry `qemu-x86_64`.

#### Evidence
- none

### PKG-033 · Define Object<Package> and Capability<Package> rights including a distinct install right
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-009, ABI-005, ABI-013, CAP-010
- Baseline: §7, §11
- Threats: T-002
- Invariants: I-021

Define Object<Package> and Capability<Package, Rights> with a distinct
install right so ImageDecoder and the other V0.5 applications hold no
package-installation authority (§7, §11). Coordinated with ABI object
registry and CAP rights encoding.

<!-- covers: INV-0171, INV-0243 -->

#### Out of scope
CAP-036 scales rights declarations at V1. Install
implementation (PKG-025).

#### Acceptance criteria
- [ ] `Capability<Package, Install>` is a distinct right; Read does not imply Install.
- [ ] ImageDecoder holds no `Capability<Package, Install>`; an install attempt returns `Error::Rights` and allocates no handle.
- [ ] Terminal, File Browser, Text Editor and Image Viewer each fail the same install attempt.
- [ ] `os inspect` on a Package Capability prints type, rights and object identity.

#### Verification
- Unit: `pkg:tests/object/package_rights_*` on CI matrix entry `qemu-x86_64`.
- Integration: `pkg:tests/object/imagedecoder_no_install_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-034 · Build `os Package install|remove|list` for local Package files
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-025, PKG-043, SDK-006
- Baseline: §28, §60

Build `os package install`, `os package remove` and `os package list` for
local Package files. V0.5 has no remote repository; the four apps and the
generation demo need this CLI.

<!-- covers: INV-0533, INV-0542 -->

#### Out of scope
`os package update|query` (PKG-062). SDK cargo
emission (SDK-020).

#### Acceptance criteria
- [ ] `os package install <file>` makes a local Package available in the next SystemGeneration.
- [ ] `os package remove <id>` removes it from the next SystemGeneration.
- [ ] `os package list` names every Package in the current and next generations.
- [ ] Install of a corrupt file returns a typed error and writes no generation.

#### Verification
- Integration: `pkg:tests/cli/package_v0_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Demo: install a Package from a local file on H-002.

#### Evidence
- none

### PKG-035 · Build `os Package build` producing deterministic content-addressed Packages
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-038, PKG-031, PKG-005
- Baseline: §27, §28
- Threats: T-007

Build `os package build` producing deterministic content-addressed Packages
needed for the four V0.5 application Packages. Two builds of identical
content yield the same identity (§27).

<!-- covers: INV-0055, INV-0514 -->

#### Out of scope
SDK-020 emits a Package from cargo. BLD determinism flags.
V1 two-builder CI (PKG-065).

#### Acceptance criteria
- [ ] Two `os package build` runs on identical inputs produce the same Package identity.
- [ ] The four V0.5 application Packages build with this tool.
- [ ] A changed Resource byte changes Package identity.

#### Verification
- Unit: `pkg:tests/build/identity_*` on CI matrix entry `qemu-x86_64`.
- Integration: rebuild Terminal twice and compare identities on `qemu-x86_64`.

#### Evidence
- none

### PKG-036 · Assemble the 20-Package benchmark Corpus from native apps and dependencies
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-035, PKG-034
- Baseline: §29, §54

Assemble the 20-Package benchmark corpus from native applications and
shared dependencies so V0.5 install-time and dedup-ratio gates measure a
fixed set. The corpus is a PKG artefact, not a compatibility corpus C-ID.

#### Out of scope
B-021 harnesses (PKG-003,
PKG-004). LNX corpora.

#### Acceptance criteria
- [ ] The corpus contains exactly twenty Packages including the four V0.5 apps and shared library Packages.
- [ ] At least two Packages share an identical dependency object.
- [ ] The corpus is pinned by content identity in the repository and rebuilds to those identities.

#### Verification
- Integration: `pkg:tests/corpus/twenty_packages_*` on CI matrix entry `qemu-x86_64`.
- Review: BEN lead confirms the corpus matches the B-021 method.

#### Evidence
- none

### PKG-037 · Precompute the Package load map so Components map objects without launch-time lookup
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-045, PKG-038, PKG-031, PKG-006, SDK-026
- Baseline: §34, §53
- Invariants: I-039

Precompute the Package load map at build and install time so a Component
maps known immutable objects directly, with no path lookup and no
dependency resolution at launch (§53). CMP performs the mapping; PKG
supplies the map.

<!-- covers: INV-0999, INV-1013, INV-0628, INV-1009, INV-1011, INV-0629 -->

#### Out of scope
CMP-017 and CMP-027 perform the
map. Path-based loaders (SDK-026).

#### Acceptance criteria
- [ ] A built Package contains a load map naming every code and dependency object by content identity.
- [ ] A trace of Terminal launch contains no path lookup and no dependency-resolution step.
- [ ] A missing load-map entry fails install, not launch, with a typed error.

#### Verification
- Integration: `pkg:tests/launch/load_map_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: CMP lead confirms the map is sufficient for CMP-017.

#### Evidence
- none

### PKG-038 · Store Packages, binaries, assets and dependencies as deduplicated content-addressed objects
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: PKG-012, PKG-014, PKG-005, STO-009, STO-005, STO-013
- Baseline: §27, §28, §29
- Invariants: I-036

Store Packages, binaries, assets and dependencies as deduplicated
content-addressed objects. PKG owns the tree and manifest encoding; STO
owns the storage substrate (§27).

<!-- covers: INV-0518, INV-0519, INV-0520, INV-0521, INV-0547, INV-0514 -->

#### Out of scope
STO-009. STO-008. Package builder
(PKG-035).

#### Acceptance criteria
- [ ] Putting the same bytes twice yields one store object identity.
- [ ] A Package, a binary, an asset and a dependency each exist as store objects inspectable by identity.
- [ ] Removing a Package that still shares a dependency object with another Package leaves that object in the store.
- [ ] Object identity matches PKG-005.

#### Verification
- Unit: `pkg:tests/store/schema_*` on CI matrix entry `qemu-x86_64`.
- Integration: `pkg:tests/store/dedup_put_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-039 · Share already-verified pages of identical objects across launches and applications
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-044, PKG-037, MEM-016, MEM-023
- Baseline: §34, §53
- Invariants: I-039

Share already-verified pages of identical objects across launches and
applications so deduplicated dependency objects are loaded once (§34,
§53). MEM owns the mapping; PKG owns the verified-object identity that
makes sharing safe.

<!-- covers: INV-1014, INV-0634 -->

#### Out of scope
MEM-016. CMP-029.

#### Acceptance criteria
- [ ] Two applications bound to the same library object share physical pages for that object.
- [ ] A second launch of the same Package does not re-hash objects present in the verification cache.
- [ ] `os inspect` reports a single resident object identity mapped into both Components.

#### Verification
- Integration: `pkg:tests/launch/shared_pages_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: MEM lead confirms page identity checks match MEM-016.

#### Evidence
- none

### PKG-040 · Measure dedup ratio, update size and hash throughput on a realistic Package set
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §27, §28, §29

Measure deduplication ratio, typical update size and hash throughput on a
realistic Linux package corpus so the V0.5 store-layout and content-hash
decisions rest on numbers rather than analogy. Compare sha256 against BLAKE3
and whole-object hashing against content-defined chunking. The run uses
existing Linux package corpora and does not require native ABI surfaces.

<!-- covers: GAP-0525, GAP-0526 -->

#### Out of scope
Store layout and hash algorithm decisions (PKG-014,
PKG-005). STO content-store implementation.

#### Acceptance criteria
- [ ] The report records dedup ratio, update size and hash throughput for sha256 and BLAKE3 on the same corpus.
- [ ] The report records the same metrics for whole-object hashing and content-defined chunking.
- [ ] The report names which combination the later adrs treat as the default candidate and why.

#### Verification
- Report: answers which hash and chunking pair wins on dedup ratio, update size and hash throughput; whether Nix-style, OSTree-style or casync-like chunking is the measurement leader; and which results are corpus artefacts rather than layout properties.
- Bench: B-021 method notes recorded in the report even though the V0 run is not a gate.

#### Evidence
- none

### PKG-041 · Study Nix, OSTree and casync stores, generations and rollback for PKG design
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §27, §30, §58
- Explores: S-018

Study Nix/NixOS content-addressed stores and generations, OSTree object
repositories and casync chunk stores, including rollback and sharing of
unchanged objects. The report is the research input to every V0.5 PKG adr
on store layout, hashing and generation materialisation (§58). It does not
gate V0 execution-model work.

<!-- covers: INV-1140 -->

#### Out of scope
Choosing the store layout (PKG-014). Implementing a store.

#### Acceptance criteria
- [ ] The report describes how Nix, OSTree and casync identify objects, share unchanged bytes across generations and roll back.
- [ ] The report lists what each system mutates in place and what JakeOS must not copy.
- [ ] The report names open questions each V0.5 PKG adr must close.

#### Verification
- Report: answers how each system encodes identity, generations and rollback; which mechanisms §2 would preserve; and which semantics (in-place mutation, global dependency installation) the PKG adrs must replace.

#### Evidence
- none

### PKG-042 · Add the two-application conflicting-library test with dedup verified by store size
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-032, PKG-025, PKG-038
- Baseline: §29
- Invariants: I-036

Add the V0.5 exit test: two applications with conflicting library versions
both run, share no mutable state, and identical content is deduplicated as
verified by store size (§29).

<!-- covers: INV-0548, INV-0547, INV-0545 -->

#### Out of scope
Binding implementation (PKG-032). Dedup
benchmark (PKG-004).

#### Acceptance criteria
- [ ] Application A bound to Library 1.4 and application B bound to Library 2.1 both launch and run their scripted scenarios.
- [ ] The two library objects have different identities and neither Component maps the other's object.
- [ ] Store size after installing both is less than the sum of the two Packages when they share other identical objects.

#### Verification
- Integration: `pkg:tests/deps/conflicting_versions_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-043 · Implement uninstall as removal from the next SystemGeneration without deleting shared content
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-025
- Baseline: §28, §30

Implement uninstall as removal from the next SystemGeneration without
deleting content that other generations or Packages still reference
(§28, §30).

<!-- covers: INV-0542 -->

#### Out of scope
Store GC (PKG-067). Generation retention
(PKG-052).

#### Acceptance criteria
- [ ] `os package remove` drops the Package from N+1 and leaves it in N.
- [ ] A dependency object still referenced by another Package remains in the store.
- [ ] Remove without `Capability<Package, Install>` returns `Error::Rights`.

#### Verification
- Integration: `pkg:tests/install/uninstall_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-044 · Verify Package content once and cache the result keyed by Object identity
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-045, PKG-038, PKG-005
- Baseline: §34

Verify Package content once (hash at V0.5) and cache the result keyed by
object identity so subsequent launches skip re-verification (§34). V1
install-signature-verification extends the cache to signatures.

<!-- covers: INV-0630 -->

#### Out of scope
Signature verification (PKG-055). STO ingest
hash (STO-010).

#### Acceptance criteria
- [ ] First map of an object hashes it and records a cache entry keyed by identity.
- [ ] Second map of the same identity performs no hash and still treats the object as verified.
- [ ] A cache entry whose bytes no longer match identity is rejected with a typed error and the entry is dropped.

#### Verification
- Unit: `pkg:tests/verify/cache_*` on CI matrix entry `qemu-x86_64`.
- Integration: `pkg:tests/verify/second_launch_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-045 · Decide verified-once launch trust for cached Package objects
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-014, PKG-005, PKG-041
- Baseline: §34
- Decision: D-0230

Decide how launch trusts cached Package objects so mapping can skip
re-hash: dm-verity-style block verification, a signed content-store index,
or a per-launch hash of a small manifest. Must precede V0.5 immutable
Package mapping. V1 PKG-050 revisits the choice
for signed activation.

<!-- covers: INV-0639 -->

#### Out of scope
The cache implementation (PKG-044). STO generation verity
(STO-052).

#### Acceptance criteria
- [ ] Options evaluated include dm-verity-style block verification, a signed content-store index, and a per-launch hash of a small manifest.
- [ ] The accepted option records what a V0.5 launch may skip after a cache hit.
- [ ] The decision names what V1 signed activation must still prove.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG, STO and SEC leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-046 · Decide how security fixes reach a library pinned by many Packages
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-006, PKG-032, Q-021
- Baseline: §29
- Decision: D-0215

Decide how a security fix reaches a library pinned by many Packages:
rebuild and republish dependents, grafting or substitution rules, or
runtime relinking, without global mutation. Needed before REL's first
CVE response. Answers Q-021.

<!-- covers: INV-0550 -->

#### Out of scope
The rebuild pipeline (PKG-051). REL advisory
publication.

#### Acceptance criteria
- [ ] Options evaluated include rebuild-and-republish of dependents, grafting a substitute object into a new generation, and runtime relinking to a patched object.
- [ ] The accepted option records that no global in-place mutation of a pinned library occurs.
- [ ] The decision names how a Package that cannot take the fix is reported.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG and REL leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-047 · Decide how Linux and Windows compatibility applications are packaged immutably
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-009, PKG-012, Q-020
- Baseline: §3, §28, §36
- Decision: D-0223
- Invariants: I-036

Decide how Linux and Windows compatibility applications are packaged as
immutable Packages (OCI images, Flatpak-like bundles, Wine prefixes) while
still satisfying the no-mutation rule. V1 L2 corpus needs OCI and Flatpak.
Answers Q-020. Native software still sees none of those formats (§3).

<!-- covers: INV-0541 -->

#### Out of scope
LNX-039 and LNX-058. Wine hosting
(WIN-013). Import implementation (PKG-058).

#### Acceptance criteria
- [ ] Options evaluated include storing OCI layers as content-addressed objects, wrapping Flatpak bundles as Packages, and treating Wine prefixes as immutable base layers plus ApplicationData overlays.
- [ ] The accepted option records that personality install writes no mutable global prefix or image store.
- [ ] The decision names which formats V1 must import and which wait for V2.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG, LNX and WIN leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-048 · Decide which state classes are restorable at each Milestone and in scope for 1.0
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-022, PKG-007
- Baseline: §31
- Decision: D-0225

Decide which state classes (OS, apps, packages, configuration, user data,
workspaces, application state) are restorable at each milestone and which
are in scope for 1.0. Must precede V2 restore UI work.

<!-- covers: INV-0587 -->

#### Out of scope
Application-state achievability (PKG-069).
Selective restore (PKG-071).

#### Acceptance criteria
- [ ] Options evaluated include restoring OS and Packages only through 1.0, adding configuration at V3, and taking every §31 class into 1.0.
- [ ] The accepted option lists each class against V1, V2, V3, V4 and 1.0 as in-scope, deferred or non-goal.
- [ ] Application state is marked pending PKG-069 rather than silently included.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG, STO and INS leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-049 · Decide running-application behaviour when its Package is replaced by a new Generation
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-016, PKG-037
- Baseline: §30, §34
- Decision: D-0226
- Threats: T-034

Decide running-application behaviour when its Package is replaced by a new
SystemGeneration: old objects stay mapped until exit, a restart prompt, or
deferred activation. No running Component may observe a mixed-version tree
(T-034).

<!-- covers: EXTRA-017 -->

#### Out of scope
Implementation and mixed-version test (PKG-066).
APP-047.

#### Acceptance criteria
- [ ] Options evaluated include old objects staying mapped until exit, a restart prompt, and deferred activation of N+1 for that Package.
- [ ] The accepted option records that a running Component never observes a mixed-version tree.
- [ ] The decision names what APP must prompt and what PKG must test.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG, CMP and APP leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-050 · Decide the verified-once launch trust mechanism
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-045, PKG-044
- Baseline: §34
- Decision: D-0229

Revisit verified-once launch trust for V1 signed activation. Hash caching
is PKG-044; generation verity is STO-052.
Options are dm-verity-style block verification, a signed content-store
index, or a per-launch hash of a small manifest.

<!-- covers: INV-0639 -->

#### Out of scope
Signature verification against trust roots
(PKG-055). STO verity chain.

#### Acceptance criteria
- [ ] Options evaluated include dm-verity-style block verification, a signed content-store index, and a per-launch hash of a small manifest.
- [ ] The accepted option records how it composes with PKG-045 and STO-052.
- [ ] The decision names what a tampered object fails with at activation.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG, STO and SEC leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-051 · Implement the pinned-dependency fix propagation chosen by the security-updates adr
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: PKG-046, PKG-064, PKG-032
- Baseline: §29

Implement the pinned-dependency fix propagation chosen by
PKG-046. A fixed library must reach every
dependent Package through the store without global mutation. V1 signed
repository is how the fix is delivered.

<!-- covers: INV-0550 -->

#### Out of scope
REL advisory publication. Rebuild of the library itself (BLD).

#### Acceptance criteria
- [ ] Publishing a fixed library identity produces a new SystemGeneration in which every dependent Package that can take the fix is bound to that identity.
- [ ] A Package that cannot take the fix remains bound to its previous identity and is listed as remaining-vulnerable.
- [ ] No existing Package object is mutated in place.

#### Verification
- Integration: `pkg:tests/deps/rebuild_pipeline_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: REL lead confirms the pipeline matches the accepted security-updates option.

#### Evidence
- none

### PKG-052 · Implement SystemGeneration garbage collection with user-controlled retention
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-016, PKG-020
- Baseline: §30

Implement SystemGeneration garbage collection with a user-controlled
retention policy. Daily-driving at V1 accumulates generations (§30).

<!-- covers: INV-0566 -->

#### Out of scope
Store object GC (PKG-067). BOOT-013
owns ESP kernel-image retention.

#### Acceptance criteria
- [ ] A retention setting keeps at least the current, next-boot and parent generations.
- [ ] GC of a generation not in the retention set removes it from `os generation list` and leaves referenced store objects for remaining generations.
- [ ] GC of the running or next-boot generation returns a typed error.
- [ ] The user can pin a generation so GC skips it.

#### Verification
- Integration: `pkg:tests/generation/retention_gc_*` on CI matrix entries `qemu-x86_64` and `hw-h004`.

#### Evidence
- none

### PKG-053 · Record driver update and environment events in history
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-022, PKG-023, ENV-011
- Baseline: §31

Accept driver-update and environment events into the history log so V1
`os history` lists generation, Package and environment events (§31). ENV
emits environment events. Firmware update events are emitted later by
HW-046 into the same typed slots.

<!-- covers: INV-0577 -->

#### Out of scope
HW-046. ENV-011 payload contents.

#### Acceptance criteria
- [ ] An environment-change event from ENV appears in the history log with a typed payload.
- [ ] The log schema includes a driver-update event type that HW can append later.
- [ ] Unknown event types are stored and listed without failing the log.

#### Verification
- Integration: `pkg:tests/history/v1_event_types_*` on CI matrix entry `qemu-x86_64`.

#### Evidence
- none

### PKG-054 · Write the install and update path threat model citing the threat Register
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002, PKG-055
- Baseline: §9, §28, §51
- Risks: R-030
- Threats: T-006, T-007, T-028

Write the install and update path threat model as the PKG contribution to
the system threat model. SEC owns the system document; this task is the
supply-chain section covering malicious Packages, build-worker compromise
and update-channel compromise.

#### Out of scope
SEC-002. INS-012.

#### Acceptance criteria
- [ ] The document cites T-006, T-007 and T-028 by ID and maps each to a PKG control.
- [ ] The document names the reserved signing fields, verification cache and repository client as controls.
- [ ] A Review line names who accepts the document.

#### Verification
- Review: SEC lead sign-off recorded on the pull request.

#### Evidence
- none

### PKG-055 · Verify Package signatures against trust roots before activation and reject tampering
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-050, PKG-044, PKG-029, REL-003
- Baseline: §28, §34
- Risks: R-030
- Threats: T-006

Verify Package signatures against trust roots before activation and reject
tampering. Fills the reserved signing fields and extends the verification
cache to signatures. V1 exit: a tampered Package is rejected.

<!-- covers: INV-0630, GAP-0323, INV-0540 -->

#### Out of scope
REL-007 owns keys and the server. BOOT-027
verifies generation manifests at boot.

#### Acceptance criteria
- [ ] A Package whose signature does not match the trust root is not activated and no generation is composed.
- [ ] A Package whose signed identity does not match content identity is rejected with a typed error.
- [ ] A second activation of a signature-verified object hits the verification cache and does not re-check the signature bytes.
- [ ] Empty reserved fields are rejected at V1 activation.

#### Verification
- Integration: `pkg:tests/verify/signature_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Fuzz: `pkg:fuzz/manifest_signature` one hour nightly without panic.

#### Evidence
- none

### PKG-056 · Declare handled content types and URL schemes in the Package manifest
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-031
- Baseline: §28

Declare handled content types and URL schemes in the Package manifest.
This is the manifest side of the default-application registry
(SVC-019). V1 daily-driving needs file and link
opening to route through Capabilities.

#### Out of scope
SVC-019. APP File Browser and launcher.

#### Acceptance criteria
- [ ] A Package manifest can list handled content types and URL schemes.
- [ ] Terminal and Text Editor manifests declare at least one handler each.
- [ ] A malformed type string fails manifest validation.

#### Verification
- Unit: `pkg:tests/manifest/handlers_*` on CI matrix entry `qemu-x86_64`.
- Review: SVC lead confirms the fields match the default-application registry.

#### Evidence
- none

### PKG-057 · Declare exposed semantic interfaces in the manifest Interfaces section
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-031
- Baseline: §28, §42

Declare exposed semantic interfaces in the Package manifest Interfaces
section so Terminal.run and Editor.open are discoverable at V1 (§42).
SEM-007 consumes the declarations.

<!-- covers: INV-0801, INV-0530 -->

#### Out of scope
SEM-007. APP implementations of Terminal.run and Editor.open.

#### Acceptance criteria
- [ ] Terminal and Text Editor manifests declare their semantic Interfaces by name and version.
- [ ] A declaration that names an unknown Interface fails validation.
- [ ] `os inspect` on a Package lists declared semantic Interfaces.

#### Verification
- Unit: `pkg:tests/manifest/semantic_ifaces_*` on CI matrix entry `qemu-x86_64`.
- Review: SEM lead confirms the declarations match SEM-007.

#### Evidence
- none

### PKG-058 · Import OCI images and Flatpak bundles into the store as immutable Packages
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: PKG-047, PKG-038, PKG-025
- Baseline: §28, §36, §46

Import OCI images and Flatpak bundles into the store as immutable Packages.
V1 L2 corpus requires an OCI container runtime and Flatpak; LNX owns the
runtime, PKG owns storing layers as content-addressed objects. Native
software never sees OCI or Flatpak APIs.

<!-- covers: INV-0541 -->

#### Out of scope
LNX-039. LNX-058. Wine prefixes
(PKG-080).

#### Acceptance criteria
- [ ] Importing an OCI image creates store objects per layer and a Package whose Dependencies are those layer identities.
- [ ] Importing a Flatpak bundle creates an immutable Package; unpacking does not write a mutable extra-data directory into the Package.
- [ ] Re-importing the same image yields the same Package identity.
- [ ] Native Components cannot open OCI or Flatpak APIs; only the Linux personality runtime consumes the imported Package.

#### Verification
- Integration: `pkg:tests/personality/oci_import_*` on CI matrix entry `qemu-x86_64`.
- Compat: C-003 container-runtime and Flatpak entries consume the imported Packages on H-002.

#### Evidence
- none

### PKG-059 · Build `os history` listing Generation, Package and environment events
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-022, PKG-023, PKG-053, SDK-019
- Baseline: §31, §61, §64

Build `os history` listing generation, Package and environment events.
V1 exit is system history v1. SDK-019 prints; this task is
the PKG query surface the CLI calls.

<!-- covers: INV-0571 -->

#### Out of scope
SDK-019 formatting. Restore (PKG-060).

#### Acceptance criteria
- [ ] `os history` lists generation, Package install/update and environment events in order.
- [ ] Filtering by type returns only matching events.
- [ ] A caller without the history Capability receives `Error::Rights`.

#### Verification
- Integration: `pkg:tests/history/cli_v1_*` on CI matrix entries `qemu-x86_64` and `hw-h004`.

#### Evidence
- none

### PKG-060 · Build `os restore` to a previous SystemGeneration verified after a simulated bad update
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-020, PKG-022, PKG-048, SVC-030
- Baseline: §31, §61

Build `os restore` to a previous SystemGeneration. V1 exit: restore
kernel, Packages and system configuration after a simulated bad update.
SDK-044 is the CLI wrapper.

<!-- covers: INV-0579 -->

#### Out of scope
Package-set restore (PKG-077). Configuration restore
(PKG-086). SDK CLI wrapper.

#### Acceptance criteria
- [ ] After a simulated bad update to N+1, `os restore` to N boots N with its kernel and Package set.
- [ ] ApplicationData written during N+1 remains after restore to N.
- [ ] Restore of an unknown generation id returns a typed error and changes no boot default.
- [ ] The CI test runs after a simulated bad update on `qemu-x86_64`.

#### Verification
- Integration: `pkg:tests/restore/generation_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-061 · Write the Package format and manifest reference for SDK v1
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-011, PKG-012, PKG-031
- Baseline: §28, §52, §66

Write the Package format and manifest reference for SDK v1. DOC generates
IDL docs; this format reference is PKG-authored so developers can package
applications.

#### Out of scope
DOC IDL generation. SDK crate API (S-031). Public packaging guide
(PKG-085).

#### Acceptance criteria
- [ ] The reference documents identity, version, Components, Interfaces, RequestedCapabilities, Dependencies, Resources and reserved signing fields.
- [ ] The reference documents the on-disk and on-wire format chosen by PKG-012.
- [ ] A Review line names who accepts the document.

#### Verification
- Review: SDK and DOC leads sign off on the pull request.

#### Evidence
- none

### PKG-062 · Build `os Package update|query` creating a new SystemGeneration per update
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-064, PKG-016, PKG-034, PKG-055
- Baseline: §30, §61

Build `os package update` and `os package query`. Update transactions land
in generation N+1 (§61). Query reads repository metadata without composing
a generation.

<!-- covers: INV-1201, INV-0560 -->

#### Out of scope
APP store client. INS updater orchestration.

#### Acceptance criteria
- [ ] `os package query` lists available updates without writing a generation.
- [ ] `os package update <id>` fetches objects, verifies signatures and composes N+1.
- [ ] A failed update leaves generation N as the next-boot default.
- [ ] Update without `Capability<Package, Install>` returns `Error::Rights`.

#### Verification
- Integration: `pkg:tests/cli/update_query_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-063 · Support content-addressed precompiled artefacts in Packages
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-038, WASM-005
- Baseline: §34

Support content-addressed precompiled artefacts in Packages: prelinked
images, precompiled shaders and AOT Wasm stored as objects (§34). WASM
and GFX produce them; PKG stores them.

<!-- covers: INV-0633 -->

#### Out of scope
WASM-005 compilation. GFX shader cache service.

#### Acceptance criteria
- [ ] A Package can name prelinked-image, shader and AOT-Wasm objects by content identity.
- [ ] Identical artefacts across Packages are stored once.
- [ ] Launch maps a named artefact without rebuilding it.

#### Verification
- Unit: `pkg:tests/artefacts/store_*` on CI matrix entry `qemu-x86_64`.
- Integration: a Wasm Component Package maps its AOT artefact on `qemu-x86_64`.

#### Evidence
- none

### PKG-064 · Build the repository client fetching Package objects and metadata from signed repositories
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: REL-008, PKG-038, PKG-016, PKG-055
- Baseline: §27, §61
- Risks: R-030
- Threats: T-028

Build the repository client that fetches Package objects and metadata from
signed repositories. V1 exit: an update delivered from the repository
creates a new SystemGeneration. REL owns the server and signing.

<!-- covers: INV-1201 -->

#### Out of scope
REL-007. REL-008. Delta transfer
(PKG-083).

#### Acceptance criteria
- [ ] Fetching a Package from the developer repository verifies metadata signatures before any object is activated.
- [ ] A successful fetch plus compose yields a new SystemGeneration listing the Package.
- [ ] A MITM-altered metadata document is rejected with a typed error and no generation is composed.
- [ ] The client fetches by content identity and does not trust a mutable name as identity.

#### Verification
- Integration: `pkg:tests/repo/client_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Fuzz: `pkg:fuzz/repo_metadata` one hour nightly without panic.

#### Evidence
- none

### PKG-065 · Add the two-builder identical Package identity CI check
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-035, BLD-041
- Baseline: §27
- Threats: T-007

Add the CI check that two builders produce identical Package identity.
V1 self-hosting requires bit-for-bit image reproducibility across two
machines; BLD owns rebuild, PKG checks identity equality.

<!-- covers: INV-0514 -->

#### Out of scope
BLD-041. Independent rebuilders at V4.

#### Acceptance criteria
- [ ] CI builds the four V0.5 application Packages on two builders and asserts identity equality.
- [ ] A non-deterministic Package fails the job with the two identities printed.
- [ ] The check runs on every merge to main.

#### Verification
- Integration: `pkg:tests/build/two_builder_identity_*` in the BLD double-build job.

#### Evidence
- none

### PKG-066 · Implement running-app update behaviour and the no-mixed-version-tree test
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-049, PKG-016, PKG-037
- Baseline: §30, §34
- Threats: T-034

Implement the accepted running-app update behaviour and the test that no
running Component observes a mixed-version tree after a Package is replaced
under it. APP-047 is the user prompt.

<!-- covers: EXTRA-017 -->

#### Out of scope
APP-047. CMP mapping.

#### Acceptance criteria
- [ ] After N+1 replaces a Package, a still-running Component continues to map only N objects.
- [ ] A test fails if a running Component maps at least one object from N and one from N+1 of the same Package.
- [ ] Activation of N+1 objects for that Component occurs only after the accepted restart or deferral point.

#### Verification
- Integration: `pkg:tests/update/no_mixed_version_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-067 · Implement unreferenced store Object collection safe for running Components
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-052, PKG-043, STO-041
- Baseline: §27, §30

Collect store objects no longer referenced by any SystemGeneration or
mapped Component. Complements generation retention. STO-037
is the substrate collector; this task defines the Package and generation
root set.

<!-- covers: INV-0542, INV-0566 -->

#### Out of scope
STO-037. STO-041.

#### Acceptance criteria
- [ ] An object referenced only by a collected generation is eligible for collection after no Component maps it.
- [ ] An object mapped by a running Component is not collected.
- [ ] `os inspect` on a collected identity returns a typed not-found error.

#### Verification
- Integration: `pkg:tests/store/object_gc_*` on CI matrix entries `qemu-x86_64` and `hw-h004`.

#### Evidence
- none

### PKG-068 · Benchmark Package-set restore time
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-077
- Baseline: §31, §54
- Benchmarks: B-036

Measure wall time to restore a previous Package set from history, on the
V2 hardware scope. STO owns snapshot creation time of user data; this
task owns Package-set restore under B-036.

<!-- covers: INV-0586 -->

#### Out of scope
STO snapshot creation. BEN-042 methodology.

#### Acceptance criteria
- [ ] A B-036 report exists for H-002, H-004 and H-005 covering Package-set restore time.
- [ ] Restore uses content-addressed sharing; the report records store growth, not a full image copy.
- [ ] The published report names the register baselines.

#### Verification
- Bench: B-036 on H-002, H-004 and H-005; target per register.

#### Evidence
- none

### PKG-069 · Decide whether application-state restore is a 1.0 goal or non-goal
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-079, PKG-048
- Baseline: §31
- Decision: D-0212

Decide whether application-state restore is a 1.0 goal or a non-goal,
using the spike report. §31 names it a long-term ambition; V4 feature
freeze needs the scope settled. Answers Q-056.

<!-- covers: GAP-0544, INV-0587 -->

#### Out of scope
The spike (PKG-079). Window restore (APP).

#### Acceptance criteria
- [ ] Options evaluated include in-scope for 1.0 via checkpointing, in-scope via cooperative state interfaces, and an explicit 1.0 non-goal.
- [ ] The accepted option cites PKG-079 evidence.
- [ ] If the non-goal is accepted, the decision names the owning later work and does not leave a silent gap in 1.0 restore.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG and APP leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-070 · Decide whether SystemGeneration switches may apply without reboot
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-020, BOOT-008, Q-052
- Baseline: §30
- Decision: D-0224
- Invariants: I-086

Decide whether SystemGeneration switches may apply without reboot: kexec
into the new kernel, a userspace-only live switch, or reboot-only.
Lockdown forbids unsigned kexec and measured-boot values would need
re-derivation. Answers Q-052. Constrains BOOT PCR policy and the V2
update UX promise. Live-patching remains a non-goal (I-086).

<!-- covers: GAP-0244 -->

#### Out of scope
INS-009. BOOT lockdown and PCR policy.

#### Acceptance criteria
- [ ] Options evaluated include kexec into the new kernel, a userspace-only live switch, and reboot-only apply.
- [ ] The accepted option records the lockdown and PCR consequences for BOOT.
- [ ] The decision states that kernel live-patching is not an option.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG, BOOT and INS leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-071 · Decide selective restore semantics and how partial restore avoids inconsistency
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-048, PKG-060, Q-026
- Baseline: §31
- Decision: D-0227

Decide selective restore semantics (restore only apps, only OS) and how
partial restore avoids unbootable or mismatched combinations. Answers
Q-026.

<!-- covers: INV-0589 -->

#### Out of scope
Implementation (PKG-078). User-data snapshots (STO).

#### Acceptance criteria
- [ ] Options evaluated include restore-OS-only, restore-Packages-only, restore-configuration-only, and forbidding combinations the decision lists as inconsistent.
- [ ] The accepted option names the consistency checks that reject an unbootable combination.
- [ ] The decision records that a rejected partial restore leaves the current generation selected.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: PKG and INS leads sign off on the pull request that accepts the decision.

#### Evidence
- none

### PKG-072 · Track and publish counts of native and Personality Packages in the repository
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-064, PKG-031
- Baseline: §56.5

Track and publish counts of native and personality Packages in the
repository as the §56.5 ecosystem metric. Kind metadata lives in the
manifest; REL publishes the number.

<!-- covers: INV-1108 -->

#### Out of scope
REL publication channel. APP store client.

#### Acceptance criteria
- [ ] Each Package manifest carries a kind of native, linux-personality or windows-personality.
- [ ] A query over the developer repository returns counts per kind.
- [ ] The counts are published with the V2 repository status, not as a performance claim.

#### Verification
- Unit: `pkg:tests/manifest/kind_*` on CI matrix entry `qemu-x86_64`.
- Review: REL lead confirms the published counts match the query.

#### Evidence
- none

### PKG-073 · Implement SystemGeneration health states so a failed boot auto-selects the previous one
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-019, PKG-020, BOOT-018
- Baseline: §30

Implement SystemGeneration health states (unknown, booting, good, bad) so
a failed boot auto-selects the previous generation. BOOT boot counter and
SVC boot-success reporting feed the state machine. V2 exit: a
fault-injected broken update boots the previous generation on all three
machines.

#### Out of scope
BOOT-018. BOOT-028. INS
rollback UX.

#### Acceptance criteria
- [ ] A generation that fails boot counting is marked bad and is not the next-boot default.
- [ ] The parent generation becomes the next-boot default when the current one is marked bad.
- [ ] `os inspect` on a SystemGeneration prints its health state.
- [ ] A good mark cannot be applied by a Component that lacks the boot-success Capability.

#### Verification
- Integration: `pkg:tests/generation/health_state_*` on CI matrix entries `qemu-x86_64`, `hw-h002`, `hw-h004` and `hw-h005`.

#### Evidence
- none

### PKG-074 · Expose a typed SystemGeneration management Interface for settings and rollback UI
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-020, PKG-060, PKG-077, IPC-035
- Baseline: §12, §30, §62

Expose a typed SystemGeneration management interface so Settings and the
INS rollback UX can list, switch and restore generations without parsing
CLI output. V2 exit: restore a previous generation from the settings UI.

#### Out of scope
INS-014. APP Settings chrome. `os restore` CLI.

#### Acceptance criteria
- [ ] The interface lists generations, health state and Package-set summaries over a Channel.
- [ ] Restore of a named generation is a typed Operation that composes no new objects when contents already exist.
- [ ] A client without the generation-management Capability receives `Error::Rights`.
- [ ] The interface is registered as a Layer 2 platform interface with a version identity.

#### Verification
- Integration: `pkg:tests/generation/mgmt_iface_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: IPC lead confirms evolution-rule compliance.

#### Evidence
- none

### PKG-075 · Distinguish optional from required RequestedCapabilities and support degraded launch
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-028
- Baseline: §9.1, §28

Distinguish optional from required RequestedCapabilities. V2 exit:
capability requests denied at install still launch the application in
degraded mode where it declared optional capabilities. CMP and SEC
consume the distinction.

<!-- covers: INV-0531 -->

#### Out of scope
CMP-043. SEC-038.
APP store client.

#### Acceptance criteria
- [ ] Each RequestedCapability is marked required or optional in the manifest.
- [ ] A Package with a denied optional Capability still produces a launchable Component graph.
- [ ] A denied required Capability fails install with a typed error and composes no generation.
- [ ] `os inspect` on the Package lists denied optional Capabilities.

#### Verification
- Unit: `pkg:tests/manifest/optional_caps_*` on CI matrix entry `qemu-x86_64`.
- Integration: `pkg:tests/manifest/degraded_install_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-076 · Pin Personality runtime versions per application via Package multi-version
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-032, PKG-047
- Baseline: §29, §48

Pin personality runtime versions per application via Package multi-version
so multiple Wine/Proton builds coexist and each Windows application is
bound to one. WIN surfaces the pin in UI.

<!-- covers: INV-0545 -->

#### Out of scope
WIN-040. PKG-080.

#### Acceptance criteria
- [ ] Two Windows applications can bind to two different Wine Package identities.
- [ ] Launching either application maps only its pinned Wine identity.
- [ ] Installing a newer Wine Package does not retarget an existing application's pin.

#### Verification
- Integration: `pkg:tests/personality/runtime_pin_*` on CI matrix entry `qemu-x86_64`.
- Review: WIN lead confirms the pin is visible to WIN-040.

#### Evidence
- none

### PKG-077 · Implement restore of installed application set and Package versions from history
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-060, PKG-048, PKG-023
- Baseline: §31
- Invariants: I-036

Implement restore of the installed application set and Package versions
from history. V2 exit: restore a previous Package set from the UI,
relying on content-addressed sharing rather than duplicate images.

<!-- covers: INV-0580, INV-0581, INV-0586 -->

#### Out of scope
INS rollback UX. Selective restore (PKG-078). User-data
snapshots (STO).

#### Acceptance criteria
- [ ] Restoring a history point's Package set produces a SystemGeneration whose Package identities match that point.
- [ ] Shared objects already in the store are not duplicated; store growth is limited to identities not present.
- [ ] ApplicationData is not rewritten by Package-set restore.

#### Verification
- Integration: `pkg:tests/restore/package_set_*` on CI matrix entries `qemu-x86_64`, `hw-h002` and `hw-h004`.

#### Evidence
- none

### PKG-078 · Implement selective restore with consistency checks
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-071, PKG-077
- Baseline: §31

Implement selective restore with the consistency checks from
PKG-071 so partial restore cannot produce unbootable
or mismatched combinations.

<!-- covers: INV-0589, INV-0586 -->

#### Out of scope
INS Settings checkboxes. Full generation restore
(PKG-060).

#### Acceptance criteria
- [ ] Restore-Packages-only against a kernel that the decision marks inconsistent returns a typed error and leaves the current generation selected.
- [ ] Restore-OS-only leaves the current Package set when that combination is allowed.
- [ ] A successful selective restore composes a generation that boots on `qemu-x86_64`.

#### Verification
- Integration: `pkg:tests/restore/selective_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### PKG-079 · Prototype application-state restore via checkpointing and cooperative state interfaces
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-022, PKG-048
- Baseline: §31

Prototype application-state restore via checkpointing and via cooperative
state Interfaces so PKG-069 can put it in or
out of 1.0. Evidence is required before scoping.

<!-- covers: GAP-0544 -->

#### Out of scope
The 1.0 scope adr (PKG-069). APP window
restore.

#### Acceptance criteria
- [ ] The report records a checkpointing prototype against at least one V0.5 application.
- [ ] The report records a cooperative state-interface prototype against Terminal or Text Editor.
- [ ] The report recommends in-scope, deferred or non-goal for 1.0 with failure modes named.

#### Verification
- Report: answers whether checkpointing restores usable state, whether cooperative Interfaces are sufficient, what cannot be restored, and the 1.0 recommendation.

#### Evidence
- none

### PKG-080 · Package Wine prefixes as immutable base layers with ApplicationData overlays
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-047, PKG-076, STO-002
- Baseline: §28, §48

Package Wine prefixes as immutable base layers with ApplicationData
overlays. Prefix mutation lands in ApplicationData, never in the Package.
V2 Windows personality W1 gates consume this layout.

<!-- covers: INV-0541 -->

#### Out of scope
WIN-040. WIN-054. Native Win32 APIs.

#### Acceptance criteria
- [ ] A Wine prefix Package is immutable; a write into it returns a typed error.
- [ ] Prefix mutation (registry, installed redistributables) lands in ApplicationData.
- [ ] Two applications using the same Wine base Package do not share ApplicationData.
- [ ] Re-installing the base Package does not wipe ApplicationData.

#### Verification
- Integration: `pkg:tests/personality/wine_prefix_*` on CI matrix entry `qemu-x86_64`.
- Review: WIN lead confirms the overlay matches WIN-040.

#### Evidence
- none

### PKG-081 · Match installed SystemGenerations against advisories and Surface fixing generations
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-019, PKG-059
- Baseline: §31, §63

Match installed SystemGenerations against advisories on the client and
surface vulnerable Packages with the generation that fixes them. REL
publishes the feed; PKG matches locally.

<!-- covers: GAP-0353 -->

#### Out of scope
REL-044. APP Settings presentation.

#### Acceptance criteria
- [ ] Given an advisory that names a Package identity, the client lists each installed generation that contains it.
- [ ] The client names the newest installed or repository generation that does not contain the vulnerable identity, when one exists.
- [ ] A malformed advisory is rejected and does not mark generations vulnerable.

#### Verification
- Integration: `pkg:tests/advisory/match_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: REL lead confirms field mapping against the advisory feed.

#### Evidence
- none

### PKG-082 · Add the power-cut fault-injection test for atomic SystemGeneration commit
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-016, STO-031
- Baseline: §30

Add the power-cut fault-injection test for atomic SystemGeneration commit.
An interrupted update leaves either generation N or a complete N+1, never
a partial tree. INS-056 is the hardware demo; this
task is the unit and QEMU test.

<!-- covers: INV-0560 -->

#### Out of scope
INS-056. BOOT fallback.

#### Acceptance criteria
- [ ] Interrupting compose before commit leaves generation N as the only bootable new state.
- [ ] Interrupting after commit leaves a complete N+1 that boots.
- [ ] No test run leaves a generation that `os generation list` names but that fails to map its kernel pin.

#### Verification
- Integration: `pkg:tests/generation/atomic_commit_*` on CI matrix entry `qemu-x86_64` with fault injection.
- Review: STO lead confirms the test uses StorageTransaction commit points.

#### Evidence
- none

### PKG-083 · Implement delta Object transfer for repository updates
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-064, PKG-005, PKG-014
- Baseline: §27, §63
- Threats: T-028

Implement delta object transfer for repository updates: store-level chunk
or object diff fetch used by the INS updater. V3 scope includes delta
updates.

#### Out of scope
INS-025 client orchestration. REL delta encoding
on the server.

#### Acceptance criteria
- [ ] Fetching N+1 when N is present transfers only objects not in N.
- [ ] A truncated delta does not activate N+1 and leaves N selected.
- [ ] Reconstructed objects match their content identity before activation.

#### Verification
- Integration: `pkg:tests/repo/delta_fetch_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Fuzz: `pkg:fuzz/delta_object` one hour nightly without panic.

#### Evidence
- none

### PKG-084 · Build `os Package publish` for third-party submission to the public repository
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-035, PKG-064, REL-021
- Baseline: §56.5, §63

Build `os package publish` for third-party submission to the public
repository. V3 exit: a third-party developer publishes a native Package
from the SDK. REL owns review and signing; SDK-080 is the
developer-facing wrap.

#### Out of scope
REL-021. SDK-080. Signing keys.

#### Acceptance criteria
- [ ] `os package publish` uploads Package objects and a manifest to the publisher pipeline.
- [ ] Publish of an unsigned or schema-invalid Package returns a typed error.
- [ ] A successful submit is queryable as pending-review, not as activated.

#### Verification
- Integration: `pkg:tests/repo/publish_*` on CI matrix entry `qemu-x86_64`.
- Demo: a third-party native Package submitted from the SDK path on H-002.

#### Evidence
- none

### PKG-085 · Write the public packaging guide for native and Personality applications
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-061, PKG-084, PKG-047
- Baseline: §28, §56.5, §63

Write the public packaging guide for native and personality applications.
V3 exit: strangers package and publish unaided. DOC publishes the site;
PKG authors the packaging content.

#### Out of scope
DOC site. SDK hello-component guide. GOV-054.

#### Acceptance criteria
- [ ] The guide documents native Package build, manifest fields, RequestedCapabilities and publish.
- [ ] The guide documents personality packaging for the formats PKG-047 accepted.
- [ ] A Review line names who accepts the document.

#### Verification
- Review: SDK, DOC and REL leads sign off on the pull request.

#### Evidence
- none

### PKG-086 · Implement restore of system and application configuration from history
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-048, PKG-022, SVC-029
- Baseline: §31

Implement restore of system and application configuration from history.
1.0 requires `os restore` to cover configuration; SVC owns the settings
objects and emits the events.

<!-- covers: INV-0582 -->

#### Out of scope
SVC-013. User-data restore (STO). Application-state
restore.

#### Acceptance criteria
- [ ] Restoring a history point reapplies the settings objects recorded at that point.
- [ ] Package-set and generation pins are not silently changed by configuration restore.
- [ ] A missing settings schema version returns a typed error and applies no partial settings.

#### Verification
- Integration: `pkg:tests/restore/configuration_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: SVC lead confirms the restore path matches SVC-029.

#### Evidence
- none

### PKG-087 · Refuse activation of revoked Packages and flag affected SystemGenerations
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-055
- Baseline: §28, §63
- Threats: T-006

Refuse activation of revoked Packages and flag affected SystemGenerations.
V3 exit includes a revocation drill; the client honours revocation lists
from REL.

#### Out of scope
REL-036. REL-033.

#### Acceptance criteria
- [ ] Activating a Package on the revocation list returns a typed error and composes no generation.
- [ ] `os generation list` flags generations that contain a revoked Package identity.
- [ ] A revocation list whose signature fails is rejected and the previous list remains in force.

#### Verification
- Integration: `pkg:tests/repo/revocation_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Demo: revocation drill with REL-033 on H-002.

#### Evidence
- none

### PKG-088 · Build `os store verify` and repair for corrupted generations in recovery
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-038, PKG-019, STO-010
- Baseline: §26, §27, §63

Build `os store verify` and repair so a recovery environment can restore a
system whose current generation is corrupted. INS owns the recovery
environment; PKG owns walking generation hashes and replacing bad objects
from the repository or from a sibling generation.

#### Out of scope
INS-041. STO-061.

#### Acceptance criteria
- [ ] `os store verify <generation>` reports every object whose bytes do not match identity.
- [ ] Repair replaces a corrupt object from another generation or from the repository and re-verifies identity.
- [ ] Repair of the running generation's mutable excluded state is refused with a typed error.

#### Verification
- Integration: `pkg:tests/store/verify_repair_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: INS lead confirms the tool runs inside the recovery generation.

#### Evidence
- none

### PKG-089 · Implement store and Generation format migration for V3 to V4 upgrades with rollback
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-016, PKG-014, PKG-012, PKG-082
- Baseline: §30, §66

Implement store and generation format migration for V3 to V4 upgrades so
a V3 install can compose a V4 generation and still roll back to V3. INS
owns the updater client path; PKG owns the format rewrite.

#### Out of scope
INS-054. Manifest schema lock (PKG-090).

#### Acceptance criteria
- [ ] A V3 generation is readable after migration and remains bootable as the rollback target.
- [ ] A migrated V4 generation boots and lists the same Package identities.
- [ ] A failed migration leaves the V3 generation selected and writes no partial V4 tree.
- [ ] The migration is itself a new SystemGeneration, not an in-place rewrite.

#### Verification
- Integration: `pkg:tests/generation/format_migration_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: INS lead confirms the updater invokes this path.

#### Evidence
- none

### PKG-090 · Lock Package and Generation manifest schema versions for 1.x with evolution tests
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-031, PKG-011, PKG-041, IPC-062, PKG-005
- Baseline: §12, §28, §66
- Freezes: S-018

Lock Package and generation manifest schema versions served for the 1.x
line, with old-client/new-schema and new-client/old-schema evolution tests.
This freezes S-018 after the V0.5 schema adr and the V3 evolution matrix.

<!-- covers: INV-0527 -->

#### Out of scope
IPC-068 enumerates the register. ABI Layer 1 freeze.

#### Acceptance criteria
- [ ] S-018 is listed with a locked version identity for 1.x.
- [ ] An old v0.5-era manifest reader accepts a 1.x manifest that only adds optional fields.
- [ ] A 1.x reader accepts a V3 manifest of the locked prior version.
- [ ] A breaking field rename fails CI.

#### Verification
- Integration: `pkg:tests/manifest/evolution_lock_*` on CI matrix entry `qemu-x86_64`.
- Review: IPC lead confirms the locked version is in the Layer 2 enumeration.

#### Evidence
- none

### PKG-091 · Verify the update and rollback guarantee on every Tier 1 machine
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-073, PKG-082, PKG-060, PKG-077, PKG-086, PKG-052, PKG-059, BOOT-028
- Baseline: §30, §31, §63

Verify the update and rollback guarantee on every Tier 1 machine: retained
previous generations remain bootable, rollback preserves user data, a
fault-injected failure returns automatically, and `os history` plus
`os restore` cover OS, Packages and configuration. INS and BOOT run the
hardware ceremony; this task is the PKG evidence pack.

#### Out of scope
INS-041. BOOT-049.
INS-056.

#### Acceptance criteria
- [ ] On every Tier 1 machine in the 1.0 hardware scope, at least three previous SystemGenerations remain bootable.
- [ ] Rolling back to a previous generation leaves ApplicationData intact.
- [ ] A fault-injected failing generation is marked bad and the parent boots without a user step.
- [ ] `os history` lists OS, Package and configuration events and `os restore` reapplies each class on one scripted run per machine.

#### Verification
- Integration: `pkg:tests/restore/guarantee_1_0_*` on every Tier 1 machine in the 1.0 hardware scope.
- Demo: PKG evidence pack attached to the 1.0 rollback-guarantee ceremony.
- Review: INS and BOOT leads confirm their ceremony tasks consumed this pack.

#### Evidence
- none
