# STO · Storage and user-selected authority
- Prefix: STO
- Lead: none
- Baseline: §4, §7, §9.1, §16, §18, §25, §26, §27, §31, §57, §60, §62, §67

<!-- roadmap:generated:begin summary -->
Tasks: 85 live, 0 done, 0 in-progress, 85 todo, 0 dropped. Ready: 1. Blocked: 84. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope
STO owns the native storage model and the user-space Storage service that mediates it. Native software holds typed File, Directory, Collection, Blob, ApplicationData and UserSelected objects through Capabilities; humans keep files and folders through privileged holders such as the File Browser and the OS-owned chooser. The workstream chooses a mature Linux filesystem substrate (CDDL ZFS is rejected for in-kernel use), maps a content-addressed store onto it, and ships copy-on-write clones, snapshots, checksums, atomic replacement, StorageTransaction, change-notification Operations, quotas, disk health, backup, and foreign, removable, network and cloud volumes as capability-scoped Collections.

## Out of scope
Package and SystemGeneration composition (PKG). Chooser UI, File Browser chrome, launcher search and snapshot settings (APP). Capability rights encoding, persistence and audit (CAP). File-backed MemoryObject mapping (MEM). Disk-encryption mechanism, secrets and grant taxonomy (SEC). Personality path, drive-letter and portal views (LNX, WIN). Installer, image builder and recovery generation (INS). ResourceDomain budgets and intent (SCH). Service supervision (SVC). Device enumeration for USB, SMART and MTP (HW). Network stack (NET). Benchmark methodology and publication (BEN). Kernel fork and retained-mechanism inventory (KRN). Boot verity and boot-counter wiring (BOOT). Signing and repository (REL). Documentation pipeline (DOC). Licensing policy (GOV). Operation transport (TSK). Object registry and Native ABI (ABI). Development-environment composition (ENV). Inspect CLI rendering (OBS, SDK).

## Tasks

### STO-001 · Expose File as a typed kernel Object reachable only via Capability<File, Rights>
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-005, CAP-005, CAP-003, CAP-004, CMP-005
- Baseline: §7, §25
- Threats: T-001, T-003
- Invariants: I-016, I-021

V0 proves the native execution model with a File object sufficient for the capability gates: a holder of `Capability<File, ReadWrite>` derives `Capability<File, Read>`, forged handles and Admin derivation fail, and revocation walks a derivation tree. Native Components receive no filesystem namespace. Raw block and filesystem passthrough remain for the Linux personality; the user-space Storage service and the full Operation surface wait for V0.5.

<!-- covers: INV-0479, INV-0478 -->

#### Out of scope
Directory, Blob, chooser and content store (STO V0.5). Rights encoding (CAP). Personality path views (LNX).

#### Acceptance criteria
- [ ] `Capability<File, ReadWrite>` derives `Capability<File, Read>` and the derived handle cannot Write.
- [ ] Deriving Admin or forging a File handle returns `Error::Rights` and allocates no handle.
- [ ] Revoking a File capability makes every derived capability fail within one Operation at derivation depth 8.
- [ ] A freshly created native Component holds no File capability and no filesystem namespace.

#### Verification
- Unit: `storage:tests/file_v0_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: V0 isolation demo typed denial on H-001 and H-002.
- Review: ABI and CAP leads confirm File is an Object in the registry with no path-keyed check.

#### Evidence
- none

### STO-002 · Provide ApplicationData private storage granted by default at launch
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-019, SEC-007, PKG-007, CAP-025
- Baseline: §9.1, §25

Every V0.5 native application needs prompt-free private persistent storage. ApplicationData is a per-application Directory granted at launch under the SEC grant taxonomy, reclaimed with the Component's package identity, and excluded from SystemGeneration trees so rollback never mutates it.

<!-- covers: INV-0484 -->

#### Out of scope
UserSelected grants (STO-034). Settings objects (SVC). Generation compose (PKG).

#### Acceptance criteria
- [ ] A launched Component receives `Capability<ApplicationData, ReadWrite>` without a user prompt.
- [ ] The ApplicationData tree is not visible to another Component and is not part of the SystemGeneration object set.
- [ ] Destroying the Component leaves ApplicationData intact for the same Package identity and reclaims temporary-storage.

#### Verification
- Unit: `storage:tests/application_data_*` on `qemu-x86_64`.
- Integration: four V0.5 demo apps write ApplicationData and relaunch on H-001 and H-003.

#### Evidence
- none

### STO-003 · Implement atomic replacement of a file or tree with no partial reads
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-020, STO-031, STO-011
- Baseline: §26

Generation switching and settings writes require readers never to observe a partial update. Atomic replacement swaps a File or Directory tree so concurrent Read Operations complete against either the previous or the new sealed object.

<!-- covers: INV-0501 -->

#### Out of scope
Multi-tree transactions (STO-051). Generation compose (PKG).

#### Acceptance criteria
- [ ] A reader concurrent with replacement observes only the previous object or the new sealed object, never a mix.
- [ ] A failed replacement leaves the previous object in place and returns a typed error.
- [ ] Directory tree replacement is atomic for the subtree named by the Directory capability.

#### Verification
- Unit: `storage:tests/atomic_replace_*` on `qemu-x86_64`.
- Fuzz: `storage:fuzz/atomic_replace` concurrent readers and replacers without panic.

#### Evidence
- none

### STO-004 · Benchmark Capability-scoped storage Object I/O against direct Linux file I/O
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-020, STO-029, STO-019, BEN-007, BEN-014
- Baseline: §25, §54
- Benchmarks: B-037

STO owns the harness that drives File, Directory and content-store Operations through the Storage service. BEN owns methodology and publication. V0.5 is publish-only; later rungs use the register regression bands.

#### Out of scope
Methodology, dashboards and cross-OS tables (BEN). Package install time (PKG).

#### Acceptance criteria
- [ ] The harness runs File Read/Write, object open/map and Directory listing shapes named in B-037 on H-001 and H-002.
- [ ] Each run records the Linux read/pread/pwrite, mmap and io_uring baselines on the same volume.
- [ ] A report file exists per in-scope H-ID meeting the V0.5 publish target for B-037.

#### Verification
- Bench: B-037 on H-001, H-002; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### STO-005 · Define Blob as a sealed immutable byte Object identified by content hash
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-013, STO-029, STO-020
- Baseline: §25, §26, §27

Packages and SystemGenerations need sealed immutable byte objects with content identifiers. A Blob cannot be modified once sealed; its identity is the content-hash form chosen by STO-013.

<!-- covers: INV-0482, INV-0499, INV-0509 -->

#### Out of scope
Store service and reference counts (STO-009). Package format (PKG).

#### Acceptance criteria
- [ ] Sealing a Blob returns a content identifier in the decided form and rejects further Write with a typed error.
- [ ] Opening by identifier returns `Error::Integrity` when stored bytes do not match the identifier.
- [ ] Two Blobs with identical content compare equal by identifier.

#### Verification
- Unit: `storage:tests/blob_*` on `qemu-x86_64`.
- Fuzz: `storage:fuzz/blob_seal` malformed and truncated payloads without panic.

#### Evidence
- none

### STO-006 · Surface data and metadata checksum failures as typed corruption errors on read
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-016, STO-020
- Baseline: §26

Map substrate checksum detection into the storage Operation error model so applications and the later scrub UX receive typed corruption errors instead of silent bad bytes.

<!-- covers: INV-0498 -->

#### Out of scope
User-facing scrub UX (STO-064). Filesystem repair (STO-061).

#### Acceptance criteria
- [ ] A Read of data with a mismatched checksum completes with a typed corruption error and does not return the bad bytes.
- [ ] Metadata checksum failure is a distinct typed error from data checksum failure.
- [ ] The error names the object identity so `os inspect` can show it.

#### Verification
- Unit: `storage:tests/checksum_corruption_*` on `qemu-x86_64`.
- Integration: fault-injected bit flip on a File in the V0.5 image on H-001.

#### Evidence
- none

### STO-007 · Prove files.choose<Image>() works with no Capability to the containing directory
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-034, STO-022
- Baseline: §9.1, §25
- Threats: T-001, T-002
- Invariants: I-021, I-035

V0.5 exit: Image Viewer opens a chosen image and cannot enumerate or open any other file in the same directory. The test is retained permanently as a regression.

<!-- covers: INV-0489, INV-0217 -->

#### Out of scope
Chooser UI (APP-002). SDK `files.choose` wrapper (SDK-017).

#### Acceptance criteria
- [ ] After `files.choose<Image>()`, the holder reads the chosen object and holds no Directory capability for its parent.
- [ ] Enumerate or Open of a sibling path returns `Error::Rights` and allocates no handle.
- [ ] The test remains in CI and fails if a path-keyed follow-up check is introduced.

#### Verification
- Unit: `storage:tests/chooser_isolation_*` on `qemu-x86_64`.
- Integration: V0.5 Image Viewer scenario on H-001 and H-003.

#### Evidence
- none

### STO-008 · Deduplicate identical objects across packages, generations and caches
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-010
- Baseline: §26, §27

V0.5 exit verifies deduplication by store size: two Packages that share a Blob occupy that Blob once. Dedup is a property of the content-addressed store, not a second copy of the substrate.

<!-- covers: INV-0512 -->

#### Out of scope
User-data idle reflink (STO-053). Dedup-ratio publication (PKG-004).

#### Acceptance criteria
- [ ] Ingesting a second copy of an identical Blob does not increase unique stored bytes.
- [ ] Two coexisting library versions that share unchanged objects report a single stored copy of each shared object.
- [ ] Reference counts keep a shared object alive until every Package and generation releases it.

#### Verification
- Unit: `storage:tests/store_dedup_*` on `qemu-x86_64`.
- Integration: two-library-version coexistence on H-001.

#### Evidence
- none

### STO-009 · Build the content-addressed Object store with reference counting
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-017, STO-005, STO-029, STO-021
- Baseline: §27
- Threats: T-006

V0.5 exit: installing a Package adds objects to the content-addressed store and two library versions coexist. The service stores, retrieves and reference-counts immutable objects by hash on the mapping chosen by STO-017.

<!-- covers: INV-0511, INV-0509 -->

#### Out of scope
Garbage collection policy (STO-041). Package install orchestration (PKG). Signing (REL).

#### Acceptance criteria
- [ ] Put of a Blob by content identifier is idempotent and increments the reference count.
- [ ] Get returns the object bytes or `Error::NotFound`; a mismatched hash returns `Error::Integrity`.
- [ ] Dropping the last live reference is observable through `os inspect storage` as an unreferenced object, not as an immediate delete.
- [ ] The store volume contains no writable shared directory that native applications can open by path.

#### Verification
- Unit: `storage:tests/content_store_*` on `qemu-x86_64`.
- Integration: PKG local Package install into the store on H-001 and H-003.
- Fuzz: `storage:fuzz/content_store_put` truncated and colliding identifiers without panic.

#### Evidence
- none

### STO-010 · Recompute hashes on ingest and on Verification requests
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-013
- Baseline: §26, §27

Integrity on ingest protects the store from V0.5. On-demand verification backs the V1 verity walk.

<!-- covers: INV-0513 -->

#### Out of scope
Boot-time generation verity (STO-052). `os verify` CLI (STO-045).

#### Acceptance criteria
- [ ] Ingest of bytes that do not match the claimed identifier is rejected and stores nothing.
- [ ] A verification request recomputes the hash and returns a typed pass or `Error::Integrity`.
- [ ] Verification does not mutate the object.

#### Verification
- Unit: `storage:tests/store_verify_*` on `qemu-x86_64`.
- Integration: tampered object file on the store volume is reported on H-001.

#### Evidence
- none

### STO-011 · Expose copy-on-write clone Operations for files and trees over reflink
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-016, STO-020, STO-019
- Baseline: §26

Copy-on-write and cheap clones over the chosen substrate. Snapshots, SystemGenerations and the V1 environment snapshot consume these Operations; the implementation sits on reflink rather than a new filesystem.

<!-- covers: INV-0495, INV-0500 -->

#### Out of scope
Snapshot catalog (STO-025). Overlay stacks (ENV nongoal).

#### Acceptance criteria
- [ ] Clone of a File returns a new File capability; mutating the clone does not change the source bytes.
- [ ] Clone of a Directory clones the subtree and does not grant the source Directory to the caller twice.
- [ ] Clone is implemented with substrate reflink, not a full byte copy, on the chosen filesystem.

#### Verification
- Unit: `storage:tests/cow_clone_*` on `qemu-x86_64`.
- Integration: clone-then-write isolation on H-001 and H-002.

#### Evidence
- none

### STO-012 · Decide replacing the global namespace with Capability-scoped storage objects
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-001, STO-030, CAP-007, STO-027
- Baseline: §25, §67
- Decision: D-0282
- Threats: T-001
- Invariants: I-016, I-021

The §25 model ADR: applications see typed objects, humans keep files and folders through privileged holders (File Browser, chooser). Options include per-component roots, object graphs, and hybrid path facades reserved for personalities.

<!-- covers: INV-0073, INV-0477 -->

#### Out of scope
Implementation of File and Directory (STO build tasks). Personality path synthesis (LNX, WIN).

#### Acceptance criteria
- [ ] The Decision evaluates at least per-component roots, object graphs, and hybrid path facades for personalities.
- [ ] The accepted option states that native software receives no universal filesystem namespace by default.
- [ ] The accepted option states that humans continue to see files and folders through privileged holders.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: ABI and SEC leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-013 · Decide the content-hash algorithm, identifier format, chunking and upgrade path
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-040, STO-030
- Baseline: §27
- Decision: D-0283

V0.5 requires an accepted content-addressing ADR before Packages and SystemGenerations are stored. One Decision covers algorithm, identifier form, large-object chunking and how a later algorithm is introduced without rewriting history.

<!-- covers: INV-0510 -->

#### Out of scope
Store layout (PKG-014). Filesystem mapping (STO-017).

#### Acceptance criteria
- [ ] The Decision evaluates at least SHA-256 and BLAKE3, including identifier form and chunking.
- [ ] The accepted option names the upgrade path for a second algorithm.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: PKG and REL leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-014 · Decide GPT partition and volume layout for store, generations, user data, swap and recovery
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-016
- Baseline: §26, §30, §32
- Decision: D-0287

Layout must be fixed before V0.5 SystemGenerations exist. The Decision includes registered GPT partition type GUIDs so foreign tools identify JakeOS partitions, and places the content-addressed store, user data, swap or hibernate and recovery volumes.

<!-- covers: GAP-0412 -->

#### Out of scope
Image builder consumption (INS-001). Installer wipe-versus-alongside policy (INS).

#### Acceptance criteria
- [ ] The Decision evaluates at least separate GPT partitions versus filesystem subvolumes versus a mixed layout.
- [ ] The accepted option lists volumes for store, generations, user data, swap or hibernate, and recovery, with GPT type GUIDs.
- [ ] Foreign partition tools can identify each JakeOS partition type from the GUID table in the Decision.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: INS and BOOT leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-015 · Decide how an application gains authority to create one new file in a user-chosen place
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-012, SEC-007, Q-017
- Baseline: §9.1, §25
- Decision: D-0289

Text Editor at V0.5 must save without Directory access. Options include a chooser-created File capability versus a single-use `Capability<Directory, CreateOne>`.

<!-- covers: INV-0492 -->

#### Out of scope
Implementation (STO-024). Chooser UI (APP). Persistent grants (CAP).

#### Acceptance criteria
- [ ] The Decision evaluates at least chooser-created File versus single-use `Capability<Directory, CreateOne>`.
- [ ] The accepted option does not grant the containing Directory to the application.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: SEC and APP leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-016 · Decide the initial Linux filesystem under the native storage layer
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-026, GOV-003, SEC-005
- Baseline: §26, §57
- Decision: D-0292
- Risks: R-024
- Invariants: I-044, I-067

V0.5 requires an accepted filesystem-choice ADR against the nine §26 properties. ZFS is listed as rejected on CDDL grounds before measurement. The Decision records that no native filesystem or object store is built before 1.0.

<!-- covers: INV-0504, GAP-0023, INV-0505, INV-1116 -->

#### Out of scope
Native-filesystem criteria re-evaluation (STO-066). Encryption layering (STO-039).

#### Acceptance criteria
- [ ] The Decision evaluates at least btrfs, bcachefs and XFS with reflink, each against the nine §26 properties.
- [ ] ZFS is a rejected option with the CDDL and I-067 rationale recorded before any measurement row.
- [ ] The accepted option states that a native filesystem is not in scope before 1.0.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: KRN, GOV and SEC leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-017 · Decide how the content store maps onto the chosen filesystem without double storage
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-016, STO-013, PKG-014
- Baseline: §27
- Decision: D-0293

Choose hardlinked object directory, reflinked files, or filesystem-native dedup so the content-addressed store does not keep a second copy of Package bytes.

<!-- covers: INV-0525 -->

#### Out of scope
Store service implementation (STO-009). Package manifest format (PKG).

#### Acceptance criteria
- [ ] The Decision evaluates at least hardlinked object directory, reflinked files and filesystem-native dedup.
- [ ] The accepted option states how a Package Blob is stored once across Packages and SystemGenerations.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: PKG lead sign-off recorded on the pull request that accepts the Decision.

#### Evidence
- none

### STO-018 · Decide the platform type registry behind choose<T>, UserSelected<T> and file.type
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-012, Q-038, Q-042
- Baseline: §25, §45, §52
- Decision: D-0296

One Decision: where typed object kinds such as Image come from and how they map to content types, so SEM and SDK share one type system. Answers the V0.5 chooser questions and the later automation `file.type` question.

<!-- covers: INV-0490, INV-0985, INV-0848 -->

#### Out of scope
Chooser grant implementation (STO-034). Semantic registry (SEM). MIME databases inside the Linux personality (LNX).

#### Acceptance criteria
- [ ] The Decision evaluates at least a platform type registry, MIME sniffing, and per-chooser filter tables.
- [ ] The accepted option is the single source for `choose<T>`, `UserSelected<T>` and `file.type`.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: SDK and SEM leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-019 · Define Object<Directory> whose Capability scopes access to its subtree
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-029, STO-012, STO-020
- Baseline: §7, §25

Required by the File Browser and by the temporary-storage and ApplicationData roots. A Directory capability names a subtree; holding it does not grant sibling or parent objects.

<!-- covers: INV-0160, INV-0480 -->

#### Out of scope
Collection (STO-036). File Browser UI (APP-006).

#### Acceptance criteria
- [ ] List, Open and Create child Operations succeed only for names inside the Directory capability.
- [ ] Open of a parent or sibling returns `Error::Rights` and allocates no handle.
- [ ] Deriving a child File from a Directory records the parent for revocation.

#### Verification
- Unit: `storage:tests/directory_*` on `qemu-x86_64`.
- Fuzz: `storage:fuzz/directory_names` without panic or namespace escape.

#### Evidence
- none

### STO-020 · Define Object<File> with Read, Write and metadata Operations and its rights set
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-029, STO-001, TSK-011, ABI-009, MEM-020
- Baseline: §7, §18, §25

Complete the V0 File object into the §7 typed object with async Operations, typed errors and the rights taxonomy used by the chooser. Mapping coherence with MemoryObject is decided by MEM; STO exposes the File Operations those mappings attach to.

<!-- covers: INV-0159 -->

#### Out of scope
Persistent MemoryObject property (STO-046). Mapping implementation (MEM).

#### Acceptance criteria
- [ ] Read, Write, Stat and SetMetadata are Operations that complete with typed results.
- [ ] A Read-only capability cannot Write; the denial is `Error::Rights`.
- [ ] Metadata includes size, kind and content type from the typed-kind registry Decision.

#### Verification
- Unit: `storage:tests/file_ops_*` on `qemu-x86_64`.
- Integration: Text Editor save and Image Viewer open on H-001 and H-003.

#### Evidence
- none

### STO-021 · Implement the partition and volume layout library used by the image builder
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-014, STO-016
- Baseline: §26, §30

INS consumes this library in the V0.5 image builder. STO supplies GPT layout, mkfs and subvolume creation with idempotency tests so a second run does not destroy an existing JakeOS volume set.

<!-- covers: GAP-0412 -->

#### Out of scope
Image compose and bootable media (INS). Bootloader entries (BOOT).

#### Acceptance criteria
- [ ] The library creates the decided GPT layout, filesystems and subvolumes from a blank disk image.
- [ ] A second invocation on an already-laid-out disk is idempotent and does not wipe user-data or store volumes.
- [ ] Created partitions carry the registered GPT type GUIDs from the Decision.

#### Verification
- Unit: `storage:tests/partition_layout_*` on `qemu-x86_64`.
- Integration: INS scripted install into a QEMU disk on H-001.

#### Evidence
- none

### STO-022 · Enforce no path-based authority and no default filesystem namespace with lint and tests
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-001, ABI-003, ABI-018
- Baseline: §9.1, §25, §67
- Threats: T-001
- Invariants: I-016, I-021

Principle 8 keeps human-facing files and folders without making paths the security model. A CI lint rejects native authority checks keyed on filesystem paths, and a conformance test proves a freshly created Component holds no filesystem namespace once the V0 File object exists.

<!-- covers: INV-0037, INV-0478, INV-1299 -->

#### Out of scope
Chooser isolation regression (STO-007). POSIX path synthesis (LNX).

#### Acceptance criteria
- [ ] CI fails a native crate that grants or checks authority from a filesystem path string.
- [ ] A newly created Component's capability table contains no File, Directory or namespace handle by default.
- [ ] The lint allowlist is empty for native crates; personality crates are out of the lint's input set.

#### Verification
- Unit: `storage:tests/path_authority_lint_*` on `qemu-x86_64`.
- Review: GOV lint owners confirm the rule is merge-blocking.

#### Evidence
- none

### STO-023 · Inventory retained Linux storage drivers and filesystems with divergence status
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-017
- Baseline: §1, §2, §5.1

Collapse NVMe, SATA, storage drivers and the mature filesystems (ext4, btrfs, xfs, vfat) into one inventory that lists each mechanism, its Phase A-E fate and the personality that depends on it. The document feeds KRN's whole-kernel retained-mechanism inventory and is the input to the storage regression matrix.

<!-- covers: INV-0018, INV-0019, INV-0028, INV-0066, INV-0130, INV-0137 -->

#### Out of scope
Regression tests (STO-028). Kernel fork tracking (KRN).

#### Acceptance criteria
- [ ] The inventory names NVMe, SATA, generic block/SCSI, ext4, btrfs, xfs and vfat with a Phase A-E fate each.
- [ ] Each row names the personality or native object that depends on the mechanism.
- [ ] CDDL ZFS is listed as excluded from in-kernel use, citing I-044 and I-067.

#### Verification
- Review: KRN retained-mechanism owners confirm the storage rows merge into the kernel inventory without duplication.

#### Evidence
- none

### STO-024 · Implement the save-as grant so an application creates one file without directory access
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-015, STO-034
- Baseline: §9.1, §25
- Threats: T-002
- Invariants: I-035

Implements STO-015 for the V0.5 Text Editor acceptance scenario.

<!-- covers: INV-0492 -->

#### Out of scope
Chooser UI (APP). Persistent grant store (CAP).

#### Acceptance criteria
- [ ] After save-as, the application holds a File capability for the new object and no Directory capability for its parent.
- [ ] A second create in the same parent without a new grant returns `Error::Rights`.
- [ ] The audit log records exactly one create grant for the operation.

#### Verification
- Unit: `storage:tests/save_as_*` on `qemu-x86_64`.
- Integration: Text Editor save-as scenario on H-001 and H-003.

#### Evidence
- none

### STO-025 · Expose subtree and system-volume snapshot Operations
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-011, STO-016
- Baseline: §26

V0.5 storage model over a mature CoW filesystem with snapshots, consumed by PKG SystemGenerations. Subtree snapshots and a whole-system-volume snapshot are Operations on Directory and volume objects.

<!-- covers: INV-0496 -->

#### Out of scope
User-visible catalog and restore UI (STO-070). Generation compose (PKG).

#### Acceptance criteria
- [ ] Snapshot of a Directory returns a read-only snapshot object that does not change when the live tree is written.
- [ ] Snapshot of the system volume is a distinct Operation used by PKG generation materialisation.
- [ ] Snapshot creation does not grant the live Directory to the caller beyond the rights it already holds.

#### Verification
- Unit: `storage:tests/snapshot_ops_*` on `qemu-x86_64`.
- Integration: PKG generation compose uses a system-volume snapshot on H-001.

#### Evidence
- none

### STO-026 · Measure btrfs, bcachefs and XFS-reflink on snapshots, clones, checksums, dedup and crash safety
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: GOV-003, BEN-019, STO-023, STO-030
- Baseline: §26, §57
- Risks: R-024

Precedes STO-016. Uses BEN's filesystem harness and adds crash-consistency runs. ZFS appears only as a license-excluded reference row, never as a measured candidate.

<!-- covers: GAP-0524 -->

#### Out of scope
The filesystem Decision (STO-016). Energy methodology (BEN).

#### Acceptance criteria
- [ ] The report measures snapshot create, clone, checksum overhead, dedup and crash consistency for btrfs, bcachefs and XFS-reflink on H-001 and H-002.
- [ ] ZFS is present only as a CDDL-excluded reference row with no benchmark run.
- [ ] The report recommends one substrate against the nine §26 properties or states the remaining gaps.

#### Verification
- Report: which candidate meets the nine §26 properties; what crash-consistency failures remain; whether any gap would later justify a native store; confirmation that ZFS was not measured.
- Bench: B-037 candidate runs on H-001, H-002 as input to the report.

#### Evidence
- none

### STO-027 · Prototype File, Directory and UserSelected Interfaces
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-001
- Baseline: §25, §26
- Explores: S-027, S-033

Prototype `Capability<File>`, `Capability<Directory>`, UserSelected and a typed change-notification Operation so STO-012 is informed by running code (§25). Surface S-027 remains open. Native software never sees a global filesystem path as ambient authority.

#### Out of scope
The Decision (STO-012). Freeze of S-027 (STO-082). Chooser UI (APP-002).

#### Acceptance criteria
- [ ] A prototype File and Directory Capability pair scopes access to a subtree on `qemu-x86_64`.
- [ ] A prototype UserSelected grant opens one object and cannot enumerate siblings in the same Directory.
- [ ] Surface S-027 remains `open` or `prototyped`, never `frozen`.
- [ ] The report records how the UserSelected chooser hands authority to the caller and which Component renders it, so S-033 is explored by running code.

#### Verification
- Report: how Directory scoping composes with UserSelected, where StorageTransaction durability is expressed, and which options STO-012 must evaluate.
- Integration: the prototype runs on `qemu-x86_64`.

#### Evidence
- none

### STO-028 · Add NVMe, SATA and ext4/btrfs/xfs/vfat regression tests to the CI matrix
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-023, KRN-014, KRN-010
- Baseline: §1, §2, §5.1

The second of the two retain-item tasks: boot and I/O tests on virtio and the reference desktop NVMe/SATA, plus mount and LTP filesystem subsets for the four retained filesystems, so V0 Linux compatibility intact covers storage. Native software still does not see those filesystems as APIs.

<!-- covers: INV-0018, INV-0019, INV-0028, INV-0066, INV-0130, INV-0137 -->

#### Out of scope
Native storage objects (STO-001). kselftest hosting (BLD, KRN).

#### Acceptance criteria
- [ ] CI boots and performs block I/O on virtio-blk on H-001 and NVMe on H-002.
- [ ] CI mounts ext4, btrfs, xfs and vfat and runs the named LTP filesystem subset on H-001.
- [ ] A regression in any named row fails the KRN retained-subsystem merge gate.

#### Verification
- Integration: `storage:tests/regression_matrix_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-001 storage-related LTP subset on H-001.

#### Evidence
- none

### STO-029 · Build the user-space storage service mediating Capability-scoped access to the substrate
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-012, STO-016, SVC-015, KRN-001, IPC-012, STO-001
- Baseline: §4, §25, §26
- Threats: T-002

The §4 user-space Storage service hosts every storage object below and is supervised by SVC. Native Components never open the substrate by path; they hold Capabilities the service mints and checks.

<!-- covers: INV-0109, INV-0508 -->

#### Out of scope
Supervision policy (SVC). Object registry (ABI). Chooser UI (APP).

#### Acceptance criteria
- [ ] The service starts as a supervised Component with a declared capability set and no ambient filesystem namespace.
- [ ] Every File and Directory Operation is mediated by the service; a native crate cannot open the substrate path from outside it.
- [ ] Killing and restarting the service rebinds clients without granting new storage authority.
- [ ] `os inspect` shows the service Component, its storage objects and their holders.

#### Verification
- Unit: `storage:tests/service_skeleton_*` on `qemu-x86_64`.
- Integration: supervised restart on H-001 and H-003.
- Review: SVC lead confirms the service manifest and restart policy.

#### Evidence
- none

### STO-030 · Write the storage threat analysis: chooser spoofing, TOCTOU across snapshots, store poisoning
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002
- Baseline: §9.1, §25, §27
- Threats: T-001, T-002, T-006, T-012

SEC's V0 threat model requires storage designs to cite the threats they address before being fixed. This document feeds the V0.5 storage ADRs and the threat register.

<!-- covers: EXTRA-048 -->

#### Out of scope
System-wide threat model (SEC). Chooser trusted-UI surfaces (GFX, APP).

#### Acceptance criteria
- [ ] The document maps chooser spoofing, confused-deputy grants, snapshot TOCTOU and store poisoning onto existing T-IDs.
- [ ] Each V0.5 storage ADR cites at least one T-ID from this document.
- [ ] No new threat is invented that is not in `registers/threats.md`.

#### Verification
- Review: SEC threat-model owners sign off on the pull request.

#### Evidence
- none

### STO-031 · Add the StorageTransaction Operation kind with commit and abort completion
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-029, TSK-013, ABI-014, STO-020
- Baseline: §18, §26

§18 Operation kind. V0.5 delivers single-volume atomic grouping used by Package install and generation build, coordinated with TSK's Operation model. Multi-tree commit waits for V1.

<!-- covers: INV-0356 -->

#### Out of scope
Multi-object transactions (STO-051). Durability contract (STO-038). Operation transport (TSK).

#### Acceptance criteria
- [ ] A StorageTransaction groups Writes on one volume and completes with Commit or Abort.
- [ ] Abort leaves no partial Write visible to subsequent Read.
- [ ] Cancel of an in-flight StorageTransaction completes with Cancelled and does not commit.

#### Verification
- Unit: `storage:tests/storage_transaction_*` on `qemu-x86_64`.
- Integration: PKG install uses a StorageTransaction on H-001.

#### Evidence
- none

### STO-032 · Add a filesystem-diff test that Package install writes nothing outside the store
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-009, STO-021, PKG-025
- Baseline: §27, §28
- Invariants: I-020

V0.5 exit: installing a Package adds it to the content-addressed store without writing outside the store, verified by filesystem diff. PKG performs the install; STO owns the diff harness over its volumes.

<!-- covers: INV-0041, INV-0526, INV-0534, INV-0535, INV-0537, INV-0538, INV-1183, INV-0536 -->

#### Out of scope
Install orchestration and immutability lint (PKG). User-data volumes (expected to stay unchanged, asserted here).

#### Acceptance criteria
- [ ] After a local Package install, a recursive diff of every volume except the store and the next SystemGeneration is empty.
- [ ] The test fails if any shared directory or user-data volume gains a file.
- [ ] The harness is merge-blocking in CI.

#### Verification
- Integration: `storage:tests/store_write_boundary_*` on H-001 and H-003.

#### Evidence
- none

### STO-033 · Provide the Component-scoped temporary-storage Capability granted at launch
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-019, CAP-025
- Baseline: §9.1

§9.1 launch capability set includes temporary-storage alongside UI and GPU. The Directory is scoped to the Component and reclaimed when the Component is destroyed.

<!-- covers: INV-0215 -->

#### Out of scope
ApplicationData (STO-002). ResourceDomain quotas (SCH).

#### Acceptance criteria
- [ ] A launched Component receives a temporary-storage Directory capability without a user prompt.
- [ ] Destroying the Component deletes the temporary-storage tree and revokes the capability.
- [ ] Another Component cannot Open the temporary-storage tree.

#### Verification
- Unit: `storage:tests/temporary_storage_*` on `qemu-x86_64`.
- Integration: Component destroy reclaims bytes on H-001.

#### Evidence
- none

### STO-034 · Implement UserSelected<T> and the chooser grant returning Capability<Image, ReadWrite>
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-018, STO-012, SEC-007
- Baseline: §9.1, §25, §60
- Threats: T-001, T-002, T-012
- Invariants: I-035

Authority side of the OS-owned chooser. APP owns the UI; STO mints the narrow capability, records the audit event and returns the typed object to the SDK call.

<!-- covers: INV-0485, INV-0216, INV-1184 -->

#### Out of scope
Chooser window and trusted-UI (APP, GFX). SDK `files.choose` (SDK-017). Persistent grants (CAP).

#### Acceptance criteria
- [ ] A successful choose of kind Image returns `UserSelected<Image>` carrying `Capability<Image, ReadWrite>` for that object only.
- [ ] The grant does not include the containing Directory.
- [ ] The audit log records the grant with object identity, rights and requesting Component.

#### Verification
- Unit: `storage:tests/user_selected_*` on `qemu-x86_64`.
- Integration: Image Viewer choose scenario on H-001 and H-003.
- Review: SEC grant-taxonomy owners confirm the implicit-chooser class.

#### Evidence
- none

### STO-035 · Define a typed change-notification Operation for File, Directory and Collection
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-020, STO-019, STO-036, TSK-013
- Baseline: §18, §25

IDEs in the V1 L2 corpus need watch semantics. STO ships a typed change-notification Operation and a bridge library that the Linux personality wires to its retained watch mechanism. Native software never sees that mechanism; it holds an Operation on File, Directory or Collection.

<!-- covers: EXTRA-005 -->

#### Out of scope
Personality watch retention (LNX-055). Search indexer (STO-065).

#### Acceptance criteria
- [ ] Watch on a File completes when that File's bytes or metadata change, with a typed event.
- [ ] Watch on a Directory or Collection does not deliver events for objects outside the capability.
- [ ] The bridge library is consumed only by the Linux personality crate, not by native SDK crates.

#### Verification
- Unit: `storage:tests/change_notification_*` on `qemu-x86_64`.
- Integration: native watch plus personality bridge on H-001 and H-004.
- Fuzz: `storage:fuzz/change_notification` event storms without panic.

#### Evidence
- none

### STO-036 · Define Collection as a typed user-meaningful group of items independent of layout
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-019, STO-012, STO-018
- Baseline: §25

§25 Collection is the foundation for removable volumes, network shares and cloud providers surfacing as capability-scoped Collections. Membership is independent of Directory layout.

<!-- covers: INV-0481 -->

#### Out of scope
Removable media (STO-062). Provider interface (STO-073).

#### Acceptance criteria
- [ ] A Collection capability lists members and Open of a member returns a typed object without granting sibling members.
- [ ] Adding or removing a member does not require a Directory capability for any backing tree.
- [ ] A Collection has a kind from the typed-kind registry.

#### Verification
- Unit: `storage:tests/collection_*` on `qemu-x86_64`.
- Integration: File Browser lists a Collection on H-001.

#### Evidence
- none

### STO-037 · Implement content-store garbage collection with pinning and `os store gc`
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-041, STO-008, PKG-052
- Baseline: §27

Implements STO-041 so daily-driving at V1 does not grow the store without bound. Roots include live SystemGenerations, installed Packages and user-pinned objects.

<!-- covers: INV-0524 -->

#### Out of scope
Generation retention policy (PKG). Cache API (STO-044).

#### Acceptance criteria
- [ ] `os store gc` deletes objects unreachable from the decided root set and reports bytes reclaimed via `os inspect storage`.
- [ ] A user-pinned object survives GC until unpinned.
- [ ] GC does not delete an object referenced by a live SystemGeneration or installed Package.

#### Verification
- Unit: `storage:tests/store_gc_*` on `qemu-x86_64`.
- Integration: create generations, pin, gc, verify roots on H-001 and H-004.

#### Evidence
- none

### STO-038 · Decide when Write and StorageTransaction data is power-loss safe
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-031, STO-020
- Baseline: §18, §26
- Decision: D-0284

One Decision for the point at which Write and StorageTransaction data is power-loss safe. Options include fsync-on-commit, group-commit with a bounded window, and an explicit Durable flag on Write.

<!-- covers: EXTRA-007 -->

#### Out of scope
Power-cut test (STO-048). Operation transport (TSK).

#### Acceptance criteria
- [ ] The Decision evaluates at least fsync-on-commit, group-commit with a bounded window, and an explicit Durable flag.
- [ ] The accepted option states the completion condition that means power-loss safety.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: TSK and KRN leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-039 · Decide encryption layering across the verified system store and encrypted user data
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-005, STO-017, STO-009
- Baseline: §26, §27, §51
- Decision: D-0285

Must precede V3 FDE installer. SEC decides the encryption mechanism; this Decision states whether the immutable, publicly known system store is verified-but-unencrypted while user and application data are encrypted, and how deduplication interacts with encryption.

<!-- covers: GAP-0195 -->

#### Out of scope
LUKS/dm-crypt versus fscrypt (SEC-005). Installer FDE UI (INS).

#### Acceptance criteria
- [ ] The Decision evaluates at least verified-unencrypted system store plus encrypted user data, encrypt-everything, and encrypt-store with convergent encryption.
- [ ] The accepted option states how deduplication behaves under encryption.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: SEC and PKG leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-040 · Decide persistent MemoryObject semantics: storage backing, crash consistency, content addressing
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-020, STO-038, STO-005, Q-006
- Baseline: §16, §26, §27
- Decision: D-0288

Answers the §16 open question before the persistent property ships. Options include file-backed CoW, a content-addressed Blob, and a hybrid that seals to a Blob on durability.

<!-- covers: INV-0318 -->

#### Out of scope
Implementation (STO-046). Mapping coherence (MEM).

#### Acceptance criteria
- [ ] The Decision evaluates at least file-backed CoW, content-addressed Blob, and a hybrid seal-on-durable path.
- [ ] The accepted option states crash-consistency relative to the durability contract.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: MEM lead sign-off recorded on the pull request that accepts the Decision.

#### Evidence
- none

### STO-041 · Decide content-store garbage collection: root set, policy and user control
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-009, PKG-007, Q-019
- Baseline: §27
- Decision: D-0294

Answers the §27 GC question before the V1 signed repository produces many SystemGenerations. Options include generation-count roots, age-based collection and user-pinned roots.

<!-- covers: INV-0524 -->

#### Out of scope
Implementation (STO-037). Generation retention (PKG).

#### Acceptance criteria
- [ ] The Decision evaluates at least generation-count roots, age-based collection and user-pinned roots.
- [ ] The accepted option names the root set and the user control surface.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: PKG lead sign-off recorded on the pull request that accepts the Decision.

#### Evidence
- none

### STO-042 · Decide three-view mapping of user data across native and personalities
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-036, STO-012
- Baseline: §25, §46, §48
- Decision: D-0295

One object graph must back native Collections, the Linux personality home and the Windows personality profile so a Documents folder is one set of objects. personality-view-api implements the chosen mapping; INS migration imports consume it.

<!-- covers: GAP-0416 -->

#### Out of scope
View API (STO-047). POSIX home (LNX). Windows profile (WIN). Migration import (STO-068, INS).

#### Acceptance criteria
- [ ] The Decision evaluates at least one object graph with two path facades, copy-on-first-use, and adopt-in-place per personality.
- [ ] The accepted option states that native software never receives path strings as authority.
- [ ] The accepted option is the mapping INS imports and personality views consume.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: LNX, WIN and INS leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-043 · Provide the StorageSnapshot primitive used by `os env` environments
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-025, STO-011
- Baseline: §26, §35
- Invariants: I-043

V1 `os env enter` cached startup needs pre-created copy-on-write snapshots of project and dependency storage. §35 forbids an overlay stack; ENV creates the snapshot, STO owns the primitive.

<!-- covers: INV-0496 -->

#### Out of scope
Environment compose and `os env` CLI (ENV, SDK). Overlayfs in the Linux personality (LNX).

#### Acceptance criteria
- [ ] Creating a StorageSnapshot of a project Directory returns a writable CoW view that does not mutate the source.
- [ ] The implementation uses snapshot and clone Operations, not an overlay filesystem stack.
- [ ] Destroying the snapshot reclaims its exclusive bytes and leaves the source intact.

#### Verification
- Unit: `storage:tests/environment_snapshot_*` on `qemu-x86_64`.
- Integration: ENV compose-storage-snapshot path on H-001 and H-004.

#### Evidence
- none

### STO-044 · Provide a content-hash keyed cache API for build outputs, decoded assets and precompiled code
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-009, STO-041, STO-013
- Baseline: §27

ENV cached build outputs and PKG precompiled artefacts share a hash-keyed cache with GC integration so cached results are safe to reuse.

<!-- covers: INV-0515 -->

#### Out of scope
Compiler and decoder producers (ENV, PKG, SDK). GC implementation (STO-037).

#### Acceptance criteria
- [ ] Put/get of a cache object is keyed by the decided content identifier.
- [ ] A cache object is a GC root while pinned by an environment or Package and is eligible after unpin.
- [ ] Get of a corrupted cache object returns `Error::Integrity` and does not return the bytes.

#### Verification
- Unit: `storage:tests/hash_cache_*` on `qemu-x86_64`.
- Integration: ENV warm enter reuses a cache object on H-001.

#### Evidence
- none

### STO-045 · Provide on-demand and boot-time integrity Verification of stored objects and generations
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-010, STO-052, PKG-016
- Baseline: §26, §27, §64

`os verify` walks a SystemGeneration against expected hashes. Boot-time check reports to BOOT's boot counter; STO owns the walk, not the counter.

<!-- covers: INV-0503 -->

#### Out of scope
Boot counter and fallback (BOOT). Signing (REL). Verity layout (STO-052).

#### Acceptance criteria
- [ ] `os verify` of a live SystemGeneration reports every mismatched object identity.
- [ ] A boot-time verification failure is emitted as a typed event consumed by the boot counter.
- [ ] Verification of a passing generation exits without rewriting store objects.

#### Verification
- Unit: `storage:tests/os_verify_*` on `qemu-x86_64`.
- Integration: tampered generation fails verify on H-001 and H-004.

#### Evidence
- none

### STO-046 · Implement the persistent MemoryObject property backed by durable storage
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-040, MEM-023, STO-038
- Baseline: §16, §26

Implements STO-040. Jointly tested with MEM's mapping coherence so a persistent MemoryObject survives process restart under the durability contract.

<!-- covers: INV-0313 -->

#### Out of scope
Mapping implementation (MEM). Encryption of MemoryObject (SEC).

#### Acceptance criteria
- [ ] A MemoryObject created with the persistent property is reachable after Component restart via its capability or content identifier as decided.
- [ ] A crash before the durability completion condition leaves either the previous persistent bytes or a typed incomplete error, never a torn mix.
- [ ] Mapping a persistent MemoryObject matches the MEM file-mapping Decision.

#### Verification
- Unit: `storage:tests/persistent_memory_*` on `qemu-x86_64`.
- Integration: joint MEM mapping coherence on H-001 and H-004.

#### Evidence
- none

### STO-047 · Expose a Capability-scoped view API for personalities to synthesise path namespaces
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-042, STO-036, STO-019
- Baseline: §3, §25, §46
- Invariants: I-016

The Linux and Windows personalities synthesise POSIX paths and drive letters over native storage. STO supplies a scope-preserving view API so those personalities cannot mint authority the enclosing Component does not hold. Native software never sees those paths.

<!-- covers: INV-0493 -->

#### Out of scope
POSIX home view (LNX-047). Windows filesystem layer (WIN-028). Mapping Decision (STO-042).

#### Acceptance criteria
- [ ] A view opened with a Directory or Collection capability cannot Open an object outside that capability.
- [ ] Native crates have no API that takes a path string as authority.
- [ ] Personality view handles are not transferable to native Components as path authority.

#### Verification
- Unit: `storage:tests/personality_view_*` on `qemu-x86_64`.
- Integration: LNX home view over a granted Collection on H-001.

#### Evidence
- none

### STO-048 · Prove the durability contract with a fault-injected power-cut test on NVMe
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-038, STO-051, LAB-003
- Baseline: §26

Uses LAB remote power on the reference desktop and QEMU device-reset injection in CI. After a cut, committed StorageTransactions are present and aborted ones are not.

<!-- covers: EXTRA-007 -->

#### Out of scope
Lab PDU architecture (LAB). Durability Decision (STO-038).

#### Acceptance criteria
- [ ] QEMU device-reset during commit leaves either the pre-transaction state or the committed state, never a mix.
- [ ] The same assertion holds on H-002 NVMe under LAB power cut.
- [ ] Uncommitted Writes are absent after replay.

#### Verification
- Integration: QEMU device-reset harness on H-001.
- Manual: LAB power-cut procedure on H-002 recorded in the pull request.

#### Evidence
- none

### STO-049 · Add `os inspect storage` and trace events for storage Operations and snapshots
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-029, STO-025, OBS-019, SDK-007
- Baseline: §24, §64
- Invariants: I-034

V1 requires `os trace` export of a session and the debugger to show async Operations. Storage Operations and snapshot events must appear in OBS's schema; SDK renders them.

#### Out of scope
Inspect CLI (SDK). Trace transport (OBS).

#### Acceptance criteria
- [ ] `os inspect storage` lists live File, Directory, Collection, Blob, snapshot and store objects with holders and rights.
- [ ] `os trace` includes submit, complete, commit, abort and snapshot events for storage Operations.
- [ ] A Component without inspect rights cannot read another Component's storage inspect records.

#### Verification
- Unit: `storage:tests/inspect_storage_*` on `qemu-x86_64`.
- Integration: V1 debugger session shows a StorageTransaction on H-001.

#### Evidence
- none

### STO-050 · Add old-client/new-service versioning tests for every storage L2 Interface
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-029, IPC-035, IPC-033, IPC-042
- Baseline: §12, §66

V1 freezes L2 evolution rules. Storage interfaces are SDK v1 freeze candidates and need the interface-evolution test the V4 gate later requires.

Required by V4-G02 (Layer 2 interface versions for 1.x are locked): the interface-evolution test passes for every core interface, and STO-082 runs it for storage.

#### Out of scope
Locking 1.x versions (STO-082). IDL compiler (IPC).

#### Acceptance criteria
- [ ] Every storage Layer 2 interface is registered with a version identity.
- [ ] An old client talks to a new storage service for each registered interface without a typed protocol error on unchanged methods.
- [ ] A new client talks to an old storage service and receives a negotiated subset.

#### Verification
- Integration: `storage:tests/l2_versioning_*` on `qemu-x86_64`.
- Review: IPC evolution-rules owners confirm the tests match the frozen rules.

#### Evidence
- none

### STO-051 · Complete multi-Object StorageTransaction with all-or-nothing commit across trees
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-031, STO-038, TSK-044
- Baseline: §18, §26

§26 transactions backing the §18 Operation. V1 SystemGeneration updates from the signed repository and `os restore` rely on grouped atomic changes across trees.

<!-- covers: INV-0502, INV-0356 -->

#### Out of scope
Single-volume grouping (STO-031). Generation compose (PKG).

#### Acceptance criteria
- [ ] A StorageTransaction spanning two Directory trees commits all changes or none.
- [ ] Commit completion matches the durability contract.
- [ ] Abort or cancel of a multi-tree transaction leaves both trees at their pre-transaction state.

#### Verification
- Unit: `storage:tests/storage_txn_multi_*` on `qemu-x86_64`.
- Integration: PKG generation update uses a multi-tree transaction on H-001 and H-004.
- Fuzz: `storage:fuzz/storage_txn_multi` without panic or partial commit.

#### Evidence
- none

### STO-052 · Apply verity to the immutable system store with the root hash in the signed manifest
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-010, PKG-029, REL-003, STO-039
- Baseline: §26, §27, §51

V1 exit: Packages are content-addressed, signed and verified before activation. Completes the firmware-to-userspace chain with PKG (manifest), BOOT (measured boot) and REL (signing). A checksum without a signed root is integrity, not authenticity.

<!-- covers: GAP-0173, INV-0503 -->

#### Out of scope
Measured-boot PCR policy (BOOT). Signing keys (REL). Encryption mechanism (SEC).

#### Acceptance criteria
- [ ] The immutable system store carries a verity root hash recorded in the signed SystemGeneration manifest.
- [ ] Activating a generation whose store bytes do not match the root hash fails closed and does not boot that generation.
- [ ] The user-data volume is not required to be in the same verity tree.

#### Verification
- Unit: `storage:tests/store_verity_*` on `qemu-x86_64`.
- Integration: tampered store object rejected before activation on H-001 and H-004.

#### Evidence
- none

### STO-053 · Deduplicate identical content in user data with an idle-intent reflink pass
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-011, SCH-010, SCH-041
- Baseline: §22, §26

§26 deduplication beyond the content store. Runs as a Background-intent Component within SCH budgets so Interactive file access is not starved.

<!-- covers: INV-0497 -->

#### Out of scope
Content-store dedup (STO-008). Quota objects (STO-067).

#### Acceptance criteria
- [ ] An idle reflink pass collapses identical user-data files into shared extents without changing File identities.
- [ ] The pass runs under Background intent and is charged to a ResourceDomain I/O budget.
- [ ] A Write to one File after dedup does not change the other's bytes.

#### Verification
- Unit: `storage:tests/user_data_dedup_*` on `qemu-x86_64`.
- Integration: Background-intent pass on H-004 under SCH I/O budget.

#### Evidence
- none

### STO-054 · Benchmark snapshot creation and restore time on the three target machines
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-070, STO-025, BEN-042, BEN-007
- Baseline: §26, §54, §62
- Benchmarks: B-036

V2 publishes snapshot creation and restore time. STO owns the mechanism harness; BEN owns methodology and publication. Runs on the three V2 target machines.

<!-- covers: INV-0496, INV-1227 -->

#### Out of scope
Catalog UI (APP). Methodology (BEN).

#### Acceptance criteria
- [ ] The harness creates and restores a user-data snapshot per the B-036 method on H-002, H-004 and H-005.
- [ ] Each run records the Linux snapshot baseline on the same volume.
- [ ] A report file exists per in-scope H-ID meeting the V2 publish target for B-036.

#### Verification
- Bench: B-036 on H-002, H-004, H-005; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### STO-055 · Decide how the storage model degrades on foreign filesystems lacking its metadata
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-012, STO-036
- Baseline: §25, §26
- Decision: D-0286

Precedes NTFS and exFAT support: xattr fallback, case-insensitivity, forbidden characters, timestamp precision and permission bits. Native Collections on foreign volumes must not lose metadata silently.

<!-- covers: GAP-0428 -->

#### Out of scope
NTFS driver (STO-060). exFAT support (STO-058).

#### Acceptance criteria
- [ ] The Decision evaluates at least xattr fallback, sidecar metadata, and refuse-unsupported.
- [ ] The accepted option states behavior for case-insensitivity, forbidden characters, timestamp precision and permission bits.
- [ ] Silent metadata drop is a rejected option or an explicit, user-visible degradation.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: LNX and WIN leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-056 · Decide how configuration and application state become versioned restorable objects
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-002, SVC-006, Q-025
- Baseline: §31
- Decision: D-0290

V2 exit restores OS, Packages and configuration from the UI. Options are a structured settings store versus snapshotting ApplicationData.

<!-- covers: INV-0588 -->

#### Out of scope
Settings service (SVC). Generation restore (PKG). Snapshot catalog (STO-070).

#### Acceptance criteria
- [ ] The Decision evaluates at least a structured settings store and snapshotting ApplicationData.
- [ ] The accepted option states what `os restore` reverts for configuration versus user files.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: SVC and PKG leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-057 · Build disk health: TRIM/discard policy and SMART monitoring with notifications
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-029, STO-006, SVC-015
- Baseline: §26, §32

Runs as a supervised SVC service with typed device capabilities from HW. TRIM/discard policy and SMART monitoring emit notifications; repair stays in the recovery environment.

<!-- covers: EXTRA-035 -->

#### Out of scope
Filesystem repair (STO-061). Scrub UX (STO-064). Device enumeration (HW).

#### Acceptance criteria
- [ ] Discard policy is applied to the system and user-data volumes and is visible in `os inspect storage`.
- [ ] SMART attributes that cross the service's failure threshold emit a typed notification Capability event.
- [ ] The service holds only the device capabilities it is granted and cannot Open user Files.

#### Verification
- Unit: `storage:tests/disk_health_*` on `qemu-x86_64`.
- Integration: virtio SMART injection on H-001; NVMe SMART on H-002.

#### Evidence
- none

### STO-058 · Support exFAT and FAT32 removable media with formatting and known limitations
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-055, STO-062
- Baseline: §25, §26

Cameras, SD cards and consoles are exFAT. Timestamp and character-set limitations surface per the foreign-filesystem Decision. Native software sees Collections, not mount paths.

<!-- covers: GAP-0427 -->

#### Out of scope
Automount policy (STO-062). Personality drive letters (WIN, LNX).

#### Acceptance criteria
- [ ] An exFAT and a FAT32 volume open as Collections with the decided metadata degradation applied.
- [ ] Format of a blank removable disk as exFAT or FAT32 succeeds and the new Collection is empty.
- [ ] Limitations of timestamp precision and forbidden characters are returned as typed errors, not silent rename.

#### Verification
- Integration: QEMU virtual removable disks on H-001.
- Manual: SD card and USB stick on H-004 and H-005.

#### Evidence
- none

### STO-059 · Support SMB, NFS and WebDAV as foreign network filesystems
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-055, STO-036, STO-047
- Baseline: §25

Foreign and network filesystems are STO, not NET. Capability-scoped mounts, not a global filesystem hierarchy. V3 provider Collections with secrets come later.

<!-- covers: GAP-0433, GAP-0436 -->

#### Out of scope
Network stack (NET). Secrets (SEC). Provider Collections (STO-076).

#### Acceptance criteria
- [ ] An SMB, NFS or WebDAV share opens as a Collection only after an explicit grant.
- [ ] Native Components without that Collection cannot Open objects on the share.
- [ ] Personality views of the share do not exceed the Collection capability.

#### Verification
- Integration: loopback SMB and NFS in QEMU on H-001.
- Manual: NAS share on H-002.

#### Evidence
- none

### STO-060 · Retain in-kernel NTFS read/write with dirty-volume and ACL mapping policy
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-055, STO-023
- Baseline: §2, §25, §26

Dual-boot shared data partitions. INS owns Fast Startup guidance; STO owns driver retention and mapping into the native layer. Dirty or hibernated volumes are not written.

<!-- covers: GAP-0426 -->

#### Out of scope
Fast Startup installer guidance (INS). Windows personality filesystem (WIN).

#### Acceptance criteria
- [ ] A clean NTFS volume opens read/write as a Collection with ACL mapping per the foreign-filesystem Decision.
- [ ] A dirty or hibernated NTFS volume opens read-only or is refused with a typed error, never read/write.
- [ ] Native software holds a Collection, not a path.

#### Verification
- Integration: NTFS image in QEMU on H-001.
- Manual: shared data partition on H-002 where dual-boot exists.

#### Evidence
- none

### STO-061 · Provide filesystem repair and store consistency check from the recovery environment
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-016, STO-009, STO-006, INS-013
- Baseline: §26, §32

Third part of disk-health work. Ships inside INS's resident recovery generation and repairs substrate and store metadata without booting a normal SystemGeneration.

<!-- covers: EXTRA-035 -->

#### Out of scope
Recovery generation compose (INS). SMART monitoring (STO-057).

#### Acceptance criteria
- [ ] From the recovery environment, a substrate repair Operation runs on the system volume and reports a typed result.
- [ ] A store consistency check lists unreferenced, missing and hash-mismatched objects.
- [ ] Repair does not require the main Storage service to be running.

#### Verification
- Integration: recovery image on H-001 with a corrupted store.
- Manual: recovery generation on H-002.

#### Evidence
- none

### STO-062 · Surface removable and MTP volumes as consented Collections with automount and safe eject
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-036, STO-047, STO-055, STO-034
- Baseline: §9.1, §25
- Threats: T-001
- Invariants: I-021

USB sticks, SD cards and phones appear only as user-consented capability-scoped Collections. `/media` and drive letters exist only inside personalities via the view API.

<!-- covers: GAP-0316, GAP-0429 -->

#### Out of scope
exFAT format (STO-058). USB device enumeration (HW). Personality path views (LNX, WIN).

#### Acceptance criteria
- [ ] Inserting removable media does not grant any native Component a Collection until the user consents.
- [ ] Safe eject waits for outstanding Operations and then revokes the Collection; a subsequent Open returns `Error::Rights`.
- [ ] Personality `/media` or drive-letter views are absent from native inspect output.

#### Verification
- Integration: QEMU USB storage hot-plug on H-001.
- Manual: USB stick and SD card on H-004 and H-005.
- Demo: consented Collection in File Browser on H-002.

#### Evidence
- none

### STO-063 · Take scheduled user-data snapshots with a retention policy
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-025, STO-067
- Baseline: §26, §62

Browse-and-restore and Trash need a snapshot history. Retention is bounded by storage quotas.

<!-- covers: INV-1227 -->

#### Out of scope
Catalog UI (APP). Quota objects (STO-067). Trash (STO-069).

#### Acceptance criteria
- [ ] A scheduled snapshot of user data appears in the snapshot catalog without a user prompt at the scheduled interval.
- [ ] Retention deletes snapshots beyond the policy and never deletes a user-pinned snapshot.
- [ ] Snapshot storage is charged against the user's quota.

#### Verification
- Unit: `storage:tests/scheduled_snapshots_*` on `qemu-x86_64`.
- Integration: accelerated schedule on H-002.

#### Evidence
- none

### STO-064 · Schedule checksum scrubs and report corruption to the user with recovery actions
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-006, STO-025, STO-057
- Baseline: §26

Turns typed checksum errors into a user-facing report with snapshot-based recovery suggestions. APP renders; STO owns the scrub schedule and the typed report.

<!-- covers: EXTRA-035 -->

#### Out of scope
Notification chrome (APP). Repair in recovery (STO-061).

#### Acceptance criteria
- [ ] A scheduled scrub walks user-data and store objects and records typed corruption events.
- [ ] A corruption event names the object and lists snapshot restore as an available action when a snapshot exists.
- [ ] Scrub runs under Background intent.

#### Verification
- Unit: `storage:tests/scrub_*` on `qemu-x86_64`.
- Integration: injected corruption on H-002.

#### Evidence
- none

### STO-065 · Provide a consent-scoped file content index provider for desktop search
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-035, STO-036, STO-034
- Baseline: §9, §25, §44
- Threats: T-001
- Invariants: I-021

Storage half of desktop search. APP owns the service and launcher. The indexer covers only granted Collections, driven by change-notification Operations, with no ambient home access.

<!-- covers: GAP-0315 -->

#### Out of scope
Launcher search UI (APP-027). Semantic interfaces (SEM).

#### Acceptance criteria
- [ ] The index contains content only from Collections the indexer Component holds.
- [ ] Revoking a Collection removes its documents from subsequent query results.
- [ ] A query API returns typed hits and does not return path strings as authority.

#### Verification
- Unit: `storage:tests/search_index_*` on `qemu-x86_64`.
- Integration: grant, index, revoke, query on H-002.

#### Evidence
- none

### STO-066 · Define and re-evaluate the criteria that would justify a native filesystem or Object store
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-016, STO-004
- Baseline: §26, §57
- Invariants: I-044

§26 and §57: measured semantic gaps and performance shortfalls recorded against the substrate at each gate. No build task for a native filesystem is planned before 1.0.

<!-- covers: INV-0506 -->

#### Out of scope
Any native filesystem implementation. Substrate choice (already decided).

#### Acceptance criteria
- [ ] The report lists remaining §26 semantic gaps on the chosen substrate with evidence from V0.5 and V1.
- [ ] The report cites B-037 results versus Linux file I/O without restating register numbers in prose.
- [ ] The report states whether any gap justifies a native filesystem before 1.0, with a negative default.

#### Verification
- Report: which §26 properties the substrate still misses; whether B-037 shortfalls are semantic or implementation; the go/no-go for a native store before 1.0.
- Review: KRN and GOV leads sign off that no native-filesystem build task is opened.

#### Evidence
- none

### STO-067 · Provide storage quotas per user and per ResourceDomain as typed objects
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-002, SCH-034, STO-037
- Baseline: §23, §25
- Threats: T-016

Quota objects consumed by APP's settings UI and SCH's ResourceDomain budgets. Distinct from content-store garbage collection.

<!-- covers: EXTRA-034 -->

#### Out of scope
Settings UI (APP). ResourceDomain enforcement engine (SCH). Store GC (STO-037).

#### Acceptance criteria
- [ ] A per-user quota and a per-ResourceDomain quota are typed objects inspectable via `os inspect storage`.
- [ ] A Write that would exceed a quota completes with a typed quota error and allocates no additional durable bytes.
- [ ] Quota usage excludes unreclaimed GC-eligible store objects until GC runs, as documented on the object.

#### Verification
- Unit: `storage:tests/quotas_*` on `qemu-x86_64`.
- Integration: SCH domain storage quota on H-002 and H-004.

#### Evidence
- none

### STO-068 · Decide how user data maps across native, Linux home and Windows profile views
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-042, STO-047, STO-036
- Baseline: §25, §46, §48
- Decision: D-0297

STO-042 fixes the live object graph. This Decision fixes how INS migration imports an existing Linux home or Windows profile into that graph without producing three diverging copies.

<!-- covers: GAP-0416 -->

#### Out of scope
Live mapping (STO-042). Migration assistant UI (APP). Import engines (INS).

#### Acceptance criteria
- [ ] The Decision evaluates at least import-as-copy into native Collections, adopt-in-place on first personality launch, and dual-write during import.
- [ ] The accepted option is consistent with the live mapping from STO-042.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: LNX, WIN and INS leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-069 · Implement Trash and undo-delete on storage snapshots
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-025, STO-063, STO-036
- Baseline: §25, §26

V2 desktop preview expects a recycle bin built on snapshot Operations rather than a hidden directory of ambient files.

<!-- covers: GAP-0316 -->

#### Out of scope
File Browser chrome (APP). Snapshot schedule (STO-063).

#### Acceptance criteria
- [ ] Delete of a File moves it to a Trash Collection and does not grant the Trash Collection to the deleting application.
- [ ] Undo-delete restores the File identity from a snapshot or Trash member.
- [ ] Empty Trash is a distinct Operation that requires a Trash capability.

#### Verification
- Unit: `storage:tests/trash_*` on `qemu-x86_64`.
- Integration: File Browser delete and undo on H-002.

#### Evidence
- none

### STO-070 · Provide the snapshot catalog and restore Operations behind browse-and-restore UI
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-063, STO-056, STO-025
- Baseline: §26, §31, §62

V2 exit: snapshots and rollback in the settings UI. APP renders; STO owns the catalog, diff and restore of user data and application state.

<!-- covers: INV-1227 -->

#### Out of scope
Settings UI (APP). Generation rollback (PKG, BOOT). Backup to external targets (STO-071).

#### Acceptance criteria
- [ ] The catalog lists user-data and application-state snapshots with creation time and pinned state via `os inspect storage`.
- [ ] Restore of a snapshot reverts user data through snapshot Operations and takes a safety snapshot first.
- [ ] Diff of two snapshots returns typed change records without exposing ungranted objects.

#### Verification
- Unit: `storage:tests/snapshot_catalog_*` on `qemu-x86_64`.
- Integration: restore from settings UI on H-002, H-004 and H-005.
- Demo: browse-and-restore on H-002.

#### Evidence
- none

### STO-071 · Build encrypted scheduled snapshot-based backup to external and network targets
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-063, STO-039, SEC-027, STO-070
- Baseline: §26, §31

Encrypted, snapshot-based, scheduled user-data backup to external or network targets as the counterpart of machine-to-machine restore. Keys live in SEC's secrets service. Incremental snapshot send.

<!-- covers: EXTRA-036 -->

#### Out of scope
Backup UI (APP-060). Secrets service (SEC). Network stack (NET).

#### Acceptance criteria
- [ ] A scheduled backup sends incremental snapshots to an external Collection or network target using keys from the secrets service.
- [ ] Restoring from that target recreates user data without granting the backup Component ambient home authority.
- [ ] Ciphertext on the target is unreadable without the secrets-service key.

#### Verification
- Integration: local external disk backup and restore on H-002.
- Manual: network target backup on H-004.
- Review: SEC secrets owners confirm key handling.

#### Evidence
- none

### STO-072 · Decide whether BitLocker volumes are readable via user-space dislocker-style support
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-016, STO-074
- Baseline: §25, §51
- Decision: D-0281

The Decision half of reading previous-OS encrypted disks. Depends on GOV licensing review of the candidate implementation.

<!-- covers: GAP-0431 -->

#### Out of scope
Linux LUKS/ext4/btrfs/XFS unlock (STO-074). FDE for JakeOS volumes (SEC, INS).

#### Acceptance criteria
- [ ] The Decision evaluates at least user-space dislocker-style support, no BitLocker support, and deferral past 1.0.
- [ ] The accepted option cites the userspace license allowlist outcome.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV and SEC leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-073 · Decide the native storage-provider Interface for network and cloud Collections
- Type: adr
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-036, STO-059
- Baseline: §25, §27
- Decision: D-0291

Scheduled before network-share providers (V3) and cloud providers (V4) so both use one provider contract with on-demand hydration. Also decides which cloud providers ship in 1.0.

<!-- covers: GAP-0437 -->

#### Out of scope
Provider implementations (STO-076, STO-078). Secrets (SEC).

#### Acceptance criteria
- [ ] The Decision evaluates at least a single hydration-capable provider Interface versus per-protocol Interfaces.
- [ ] The accepted option names the 1.0 cloud provider set or an explicit empty set.
- [ ] The accepted option states on-demand hydration and capability scope for Collections.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: APP and NET leads sign off on the pull request that accepts the Decision.

#### Evidence
- none

### STO-074 · Unlock and mount LUKS, ext4, Btrfs and XFS volumes from other Linux installs
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-039, STO-036, STO-055, SEC-018, STO-085
- Baseline: §25, §26, §51

Prerequisite for INS in-place migration. Unlock uses SEC's key-slot API; volumes surface as Collections, not ambient mounts.

<!-- covers: GAP-0431 -->

#### Out of scope
BitLocker Decision (STO-072). Migration assistant (INS).

#### Acceptance criteria
- [ ] A LUKS volume from another Linux install unlocks via the disk-unlock API and opens as a Collection.
- [ ] ext4, btrfs and XFS volumes open with foreign-filesystem degradation applied.
- [ ] Failed unlock returns a typed error and does not leave a writable Collection.

#### Verification
- Integration: LUKS plus ext4/btrfs/XFS images on H-001.
- Manual: second Linux disk on H-002.

#### Evidence
- none

### STO-075 · Provide per-user storage roots and separately encrypted user volumes
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-039, SEC-042, STO-002, SEC-017
- Baseline: §9.1, §25, §51

V3 exit: two users with separate sessions and separate encrypted data. SEC decides the multi-user model; STO provides the volume and key-slot layout under the encryption-layering Decision.

#### Out of scope
Session switching UI (APP). Identity and key slots (SEC).

#### Acceptance criteria
- [ ] Each user has a storage root Collection that the other user cannot Open.
- [ ] Each user volume is encrypted under a key unlocked at that user's login.
- [ ] `os inspect storage` as one user does not list the other user's objects.

#### Verification
- Integration: two-user volumes on H-002 and H-004.
- Review: SEC multi-user owners confirm key-slot layout.

#### Evidence
- none

### STO-076 · Provide SMB, NFS, WebDAV and SFTP shares as storage-provider Collections
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-073, STO-059, SEC-027, STO-034
- Baseline: §25

Home NAS and office shares surfaced in the chooser and File Browser with credentials in the secrets service, using the provider contract.

<!-- covers: GAP-0317, GAP-0433, GAP-0436 -->

#### Out of scope
Chooser UI (APP). Secrets storage (SEC). Cloud providers (STO-078).

#### Acceptance criteria
- [ ] SMB, NFS, WebDAV and SFTP shares appear as Collections through the provider Interface.
- [ ] Credentials are read from the secrets service and are not stored in the Collection metadata.
- [ ] A share granted through the chooser does not grant other shares on the same host.

#### Verification
- Integration: loopback providers on H-001.
- Manual: NAS and SFTP on H-002.
- Demo: chooser opens a NAS Collection on H-002.

#### Evidence
- none

### STO-077 · Make `os restore` revert user data through storage snapshots
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-070, PKG-060, STO-056
- Baseline: §31, §64

§31 restore of user data. PKG owns generation restore; STO owns the user-data revert and the pre-restore safety snapshot.

<!-- covers: INV-0583 -->

#### Out of scope
Generation switch (PKG, BOOT). Restore CLI chrome (SDK). Backup service (STO-071).

#### Acceptance criteria
- [ ] `os restore` of a user-data snapshot takes a safety snapshot first, then reverts user data.
- [ ] Generation restore does not revert user data unless the user-data snapshot is named.
- [ ] A failed restore leaves the safety snapshot restorable.

#### Verification
- Unit: `storage:tests/restore_user_data_*` on `qemu-x86_64`.
- Integration: `os restore` user-data path on H-002 and H-004.

#### Evidence
- none

### STO-078 · Ship the 1.0 cloud storage providers with on-demand hydration
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: STO-073, STO-076, SEC-027
- Baseline: §25, §27

Implements the provider Decision for the providers chosen for 1.0, feature-complete before V4 feature freeze. Collections hydrate on demand and stay capability-scoped.

<!-- covers: GAP-0437 -->

#### Out of scope
Provider contract Decision (STO-073). Network stack (NET).

#### Acceptance criteria
- [ ] Each 1.0 cloud provider named by the Decision appears as a Collection with on-demand hydration.
- [ ] An object not yet hydrated cannot be read until fetch completes; the Operation is cancellable.
- [ ] Revoking the Collection stops hydration and drops local exclusive copies that are not pinned.

#### Verification
- Integration: mock provider in CI on H-001.
- Manual: one real 1.0 provider on H-002 if the Decision names any.
- Review: APP File Browser opens a cloud Collection on H-002.

#### Evidence
- none

### STO-079 · Version the on-disk layout and migrate V3 installs in place
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-021, PKG-089
- Baseline: §26, §30

V4 exit: in-place upgrade of V3 installs with data preserved and rollback to V3 possible requires a versioned partition and store layout with a migration step.

#### Out of scope
Generation format migration (PKG). Installer (INS).

#### Acceptance criteria
- [ ] A V3 disk image gains a layout version field and migrates to the V4 layout without copying user data byte-for-byte unless required by the layout Decision.
- [ ] Rolling back the SystemGeneration to V3 leaves user data readable.
- [ ] A second migration on an already-V4 layout is a no-op.

#### Verification
- Integration: V3 image upgrade and rollback on H-001.
- Manual: in-place upgrade on one Tier 1 machine in the V4 hardware scope.

#### Evidence
- none

### STO-080 · Write L1 conformance tests for File, Directory, Blob and StorageTransaction entry points
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-020, STO-019, STO-005, STO-051, ABI-033
- Baseline: §7, §65, §66
- Invariants: I-040

V4 exit: every Layer 1 entry point has a conformance test before the freeze ADR. Storage objects that are Layer 1 participate; Layer 2 storage Interfaces are covered by the L2 lock task.

#### Out of scope
Freeze ADR (ABI). L2 version lock (STO-082).

#### Acceptance criteria
- [ ] Conformance tests exist for File, Directory, Blob and StorageTransaction entry points named in the ABI specification.
- [ ] A binary built against the freeze candidate passes those tests on a subsequent V4 build.
- [ ] Tests do not invoke personality path APIs.

#### Verification
- Integration: ABI conformance runner on H-001 and one Tier 1 machine.
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### STO-081 · Close external audit findings in the storage service, store and chooser authority
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-029, STO-034, STO-009, STO-007
- Baseline: §9.1, §25, §51
- Threats: T-001, T-002, T-006

V4 exit: all High and Critical audit findings fixed and re-verified. Storage mediation is in the audit scope.

#### Out of scope
Kernel capability audit (CAP, KRN). Personality audit (LNX, WIN).

#### Acceptance criteria
- [ ] Every High and Critical finding against the Storage service, content store or chooser authority has a fix and a regression test.
- [ ] Re-verification by the auditor is recorded as Evidence when the task is done.
- [ ] Chooser isolation and store integrity tests remain green.

#### Verification
- Review: external auditor re-verification recorded on the pull request.
- Unit: new regression tests under `storage:tests/audit_*` on `qemu-x86_64`.

#### Evidence
- none

### STO-082 · Enumerate and lock storage L2 Interface versions for 1.x
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-050, IPC-068, STO-027, STO-012, STO-018
- Baseline: §66
- Freezes: S-027, S-033

V4 exit: Layer 2 interface versions for 1.x are enumerated and locked with the evolution test passing for every core interface. Storage Interfaces are among them.

#### Out of scope
IPC lock register (IPC). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] Every storage Layer 2 Interface has a locked 1.x version listed with the IPC lock.
- [ ] Old-client/new-service and new-client/old-service tests pass for each locked storage Interface.
- [ ] Adding a breaking change without a version bump fails CI.

#### Verification
- Integration: `storage:tests/l2_lock_*` on `qemu-x86_64`.
- Review: IPC lead confirms storage rows in the locked version set.

#### Evidence
- none

### STO-083 · Verify rollback preserves user data by fault injection on every Tier 1 machine
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-077, PKG-073, STO-032
- Baseline: §31, §63

1.0 exit: rollback preserves user data on every Tier 1 machine under a fault-injected failing generation. INS, PKG and BOOT own the generation flip; STO owns the user-data assertion.

<!-- covers: INV-0583 -->

#### Out of scope
Generation flip and boot counter (PKG, BOOT, INS). Lab soak calendar (LAB).

#### Acceptance criteria
- [ ] After a fault-injected failing generation on each in-scope Tier 1 machine, user-data hashes match the pre-update snapshot.
- [ ] ApplicationData for a running Package is unchanged by the generation rollback.
- [ ] The assertion is merge-blocking in the 1.0 qualification checklist.

#### Verification
- Integration: fault-injected failing generation on every Tier 1 machine in the 1.0 hardware scope.
- Review: REL qualification owners record the per-machine results.

#### Evidence
- none

### STO-084 · Write the storage user and administrator guides for 1.0
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-027, DOC-022, STO-071, STO-067, STO-076
- Baseline: §25, §26, §63

1.0 documentation exit: user and administrator guides must cover snapshots, quotas, backup, foreign and network volumes and the chooser model. DOC owns the pipeline; STO authors the content.

#### Out of scope
Docs site and translation pipeline (DOC). Settings UI copy (APP).

#### Acceptance criteria
- [ ] The user guide describes chooser grants, snapshots, Trash, quotas and backup without path-based permission steps.
- [ ] The administrator guide describes layout, store GC, disk health, foreign volumes and network providers.
- [ ] Both guides are published through the DOC pipeline and pass docs CI.

#### Verification
- Review: DOC lead editorial sign-off recorded on the pull request.
- Manual: unaided-install study materials include the storage chapters.

#### Evidence
- none

### STO-085 · Decide how untrusted removable and foreign filesystem images are parsed
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-058, HW-002
- Baseline: §26, §51
- Decision: D-0349
- Threats: T-044
- Invariants: I-009

Inherited kernel filesystem drivers parse attacker-controlled bytes in kernel mode whenever a removable drive or a foreign partition is mounted (T-044). Before removable media (STO-058) and foreign Linux volumes (STO-074) are mounted for users, this Decision fixes how untrusted images are parsed: in an isolated user-space filesystem Component that reuses the inherited code, in the kernel with a restricted type allowlist and no auto-mount, or unrestricted as Linux does today. §26 forbids a new filesystem; this is about where the existing parsers run, decided on measured cost per HW-002 criteria.

#### Out of scope
exFAT and FAT32 support (STO-058). Foreign volume unlock and mount (STO-074). Lock-screen device policy (SEC-080).

#### Acceptance criteria
- [ ] Option A (isolated user-space filesystem Component hosting the inherited parsers for removable and foreign volumes), option B (kernel parsers behind a type allowlist with no auto-mount of untrusted volumes), and option C (unrestricted kernel mounting as on Linux) are evaluated with a fuzzing-exposure argument and a measured throughput cost per option.
- [ ] The accepted option states which filesystem types are parsed where, what auto-mount does for an unknown volume, and how the choice appears to the user.
- [ ] The accepted option cites T-044 and the HW-002 residency criteria rather than principle.
- [ ] STO-074 and the removable-media path implement the accepted option; review records STO and SEC lead sign-off on the pull request.

#### Verification
- Review: STO and SEC leads sign off on the pull request that accepts the Decision file.
- Bench: throughput of the accepted option against direct kernel mounting on H-002, published under B-037 with no superiority claim.

#### Evidence
- none
