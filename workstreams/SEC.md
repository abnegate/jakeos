# SEC · Security model and hardening
- Prefix: SEC
- Lead: none
- Baseline: §8, §9, §9.1, §11, §40, §43, §44, §51, §63, §64, §67

<!-- roadmap:generated:begin summary -->
Tasks: 79 live, 0 done, 0 in-progress, 79 todo, 0 dropped. Ready: 1. Blocked: 78. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

SEC owns the platform security model: the public threat model and threat-register citation rule; no-ambient-authority conformance; user identity, login, Session, administrator elevation and pluggable authenticators; disk encryption, key-slots, recovery keys, TPM sealing and suspend key policy; the per-application secrets service; grant taxonomy, authority precedence, permission prompts and the permissions UI contract; the typed CA trust store; the microarchitectural side-channel statement; hardware-enforcement hooks and enclave research; and the engineering side of the external audit, hardening evidence pack and 1.0 security-posture gate (§9, §51, §63). Native software holds only Capabilities it was handed.

## Out of scope

Capability table, rights encoding, derivation, revocation walk and persistent grant store (CAP). Consent chrome, greeter, lock screen, Settings grant panels and in-use indicators (APP). Measured boot, Secure Boot, pre-boot unlock UI and generation verity (BOOT). Filesystem substrate, UserSelected minting and encryption layering onto the store (STO). Package manifest schema and signing fields (PKG). Kernel hardening config and unsafe inventory generation (KRN). Fuzz infrastructure and merge lints (BLD). CVE response, disclosure, signing hierarchy and repository trust (REL). Legal commissioning of the auditor (GOV). Trusted-UI Surfaces, capture implementation and compositor lock mode (GFX). Audit log store and `os inspect` CLI (OBS, SDK). Semantic registry and AI broker (SEM). TLS library and network Capability broker (NET). Personality certificate stores as ABI, ptrace and default compat profiles (LNX, WIN). Installer FDE UX (INS). Suspend mechanism (PWR). Session supervision trees (SVC). Intent-class Capability gating (SCH). Encrypted MemoryObject property (MEM). Fingerprint devices, firmware update service and TPM hardware enablement (HW). Handle-word encoding (ABI). Benchmark methodology (BEN). Published documentation site (DOC).

## Tasks

### SEC-001 · Deny ambient process enumeration in native Components
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-005, CAP-001, CMP-005
- Baseline: §9.1, §11
- Threats: T-001, T-038
- Invariants: I-021

V0 isolation demo and §9.1/§11 require a native Component, including a fixture ImageDecoder, to hold no ambient process-enumeration or network Capability. An exploit is confined to the smallest useful unit.

<!-- covers: INV-0206, INV-0239, INV-0242 -->

#### Out of scope
Memory and object isolation (CMP-012). Filesystem and device denials (SEC-003). Syscall filter (ABI).

#### Acceptance criteria
- [ ] A freshly created native Component that enumerates other Components or Tasks receives `Error::Rights` and allocates no handle, on `qemu-x86_64`.
- [ ] The ImageDecoder fixture holds no network Capability; a connect Operation returns `Error::Rights` and no endpoint.
- [ ] The ImageDecoder fixture cannot enumerate other Components; the denial is visible in the Capability audit log.

#### Verification
- Unit: `kernel:tests/sec/isolation_enum_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0-D04 isolation demo on H-001 and H-002 includes the ImageDecoder fixture denials.
- Review: CMP lead confirms the fixture is a Component graph node, not a process.

#### Evidence
- none

### SEC-002 · Publish threat model and threat Register
- Type: docs
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §9, §9.1, §11, §51
- Risks: R-008, R-067

Every CAP, SEC and BOOT Decision cites the threats it addresses before designs freeze. This document names adversaries, assets, trust boundaries, safety layers and per-Component blast radius, and it is the narrative that `registers/threats.md` enumerates as T-IDs.

<!-- covers: EXTRA-048, EXTRA-063, GAP-0165, GAP-0540, INV-0958, INV-0244 -->

#### Out of scope
Compositor section (GFX-011). Font and atlas section (TXT). Installer and updater section (INS-012). Minting new T-IDs outside `registers/threats.md`.

#### Acceptance criteria
- [ ] The document names at least: malicious native app, compromised Component, compromised compatibility app, local unprivileged user, evil-maid device off, evil-maid device suspended, malicious DMA peripheral, network attacker and supply-chain attacker.
- [ ] The document maps each named adversary to one or more existing T-IDs and states the blast radius of an ImageDecoder compromise (§11).
- [ ] The document requires every CAP, SEC and BOOT adr task to cite T-IDs in its `Threats:` field.
- [ ] Review records ABI, CAP and BOOT lead sign-off on the pull request.

#### Verification
- Review: ABI, CAP, BOOT and SEC leads sign off on the pull request.
- Manual: `registers/threats.md` is the exclusive T-ID source; the document introduces no unregistered threat names.

#### Evidence
- none

### SEC-003 · Test no ambient filesystem, home, device, or app data
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-009, SEC-006, CAP-025
- Baseline: §9.1, §11, §25
- Threats: T-001, T-038
- Invariants: I-021, I-016

V0.5 chooser gate: a native application and ImageDecoder start without filesystem, home, device or other-application data authority. Selecting one file never grants the containing directory (§9.1, §25).

<!-- covers: INV-0201, INV-0202, INV-0207, INV-0211, INV-0240 -->

#### Out of scope
Chooser UI (APP-002). UserSelected minting (STO). Camera and microphone extensions (SEC-021, SEC-039).

#### Acceptance criteria
- [ ] A native application started with UI and GPU Capabilities only receives `Error::Rights` on File, Directory, Device and other-application data Operations and allocates no handle, on `qemu-x86_64`.
- [ ] The ImageDecoder fixture has no arbitrary filesystem Capability; a path-shaped open is impossible and the File Operation returns `Error::Rights`.
- [ ] After a UserSelected grant of one File, listing the parent Directory still returns `Error::Rights`.
- [ ] The suite is invoked from SEC-009 on `qemu-x86_64` and H-002.

#### Verification
- Integration: `runtime:tests/sec/ambient_denial_*` on `qemu-x86_64` and `hw-h002`.
- Unit: ImageDecoder fixture graph asserts held Capabilities are Input, Output and ResourceDomain only.
- Review: STO lead confirms no path-keyed check is used.

#### Evidence
- none

### SEC-004 · Decide authority sources and precedence
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-007, SEC-002
- Baseline: §9.1
- Decision: D-0267
- Risks: R-017
- Threats: T-001, T-002
- Invariants: I-021, I-060

§9.1 needs a single precedence among manifest request, launcher policy, user chooser, permissions UI and delegation before Packages request Capabilities. CAP-007 names the V0 sources; this Decision orders them so a confused deputy cannot outrank the user.

<!-- covers: INV-0218 -->

#### Out of scope
Grant duration and UI class (SEC-007). Manifest schema (PKG-011). Launch-time binding (CAP-025). Consent chrome (APP).

#### Acceptance criteria
- [ ] Options evaluated include at least: (A) user chooser always wins over manifest and launcher; (B) launcher policy can deny a chooser grant; (C) manifest request is sufficient without a user step.
- [ ] The accepted option names every source and a total precedence, including delegation from another Component.
- [ ] Each option cites T-001 and T-002 and records whether a privileged service can mint caller authority.
- [ ] Review records CAP and PKG lead sign-off on the pull request.

#### Verification
- Review: CAP and PKG leads sign off on the pull request.
- Manual: decision file lists at least two options and names T-001 and T-002.

#### Evidence
- none

### SEC-005 · Decide disk encryption layer and store interaction
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-002
- Baseline: §26, §27, §51, §57
- Decision: D-0268
- Threats: T-008, T-010
- Invariants: I-044, I-073

Encryption choice precedes the STO filesystem adr (snapshots, dedup, content-addressed store). Implementation stays V1. The Decision reuses a mature mechanism and does not invent a storage layer (§57).

<!-- covers: GAP-0194, GAP-0527 -->

#### Out of scope
Volume implementation (SEC-017). Store-versus-user layering (STO-039). Filesystem substrate (STO-016). Installer UX (INS).

#### Acceptance criteria
- [ ] Options evaluated include at least: LUKS2/dm-crypt block; fscrypt file; filesystem-native; both block and file, without a new storage layer.
- [ ] The accepted option states how snapshots and content-addressed dedup interact with ciphertext.
- [ ] Each option cites T-008 and T-010 and I-044.
- [ ] Review records STO and KRN lead sign-off on the pull request.

#### Verification
- Review: STO and KRN leads sign off on the pull request.
- Manual: decision file lists at least two options and names T-008.

#### Evidence
- none

### SEC-006 · Implement the grant taxonomy in the permission runtime
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-007, SEC-004, CAP-025, PKG-028
- Baseline: §9.1, §28
- Threats: T-001
- Invariants: I-021

V0.5 Packages declare RequestedCapabilities. The runtime enforces the grant-taxonomy Decision so chooser, prompt and settings-only classes are distinct, and one-time, session, persistent and revocable durations are distinct. Surface S-022 stays prototyped.

<!-- covers: EXTRA-068, GAP-0227 -->

#### Out of scope
Taxonomy Decision (SEC-007). Persistent store across reboot (CAP-037). Prompt chrome (APP, SEC-044).

#### Acceptance criteria
- [ ] A chooser-class grant is recorded only after a UserSelected object is minted; a prompt-class grant is not recorded by the chooser path.
- [ ] A settings-only class request with no prior grant returns `Error::Rights` and does not show a prompt.
- [ ] One-time, session and persistent durations are distinguishable in `os inspect` of the Component's grants on `qemu-x86_64`.
- [ ] Revoking a recorded grant makes the next Operation return `Error::Rights` with no leftover mapping.

#### Verification
- Unit: `runtime:tests/sec/grant_runtime_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0.5 Image Viewer launch holds UI and GPU only until a chooser grant.
- Review: CAP lead confirms classes match SEC-007.

#### Evidence
- none

### SEC-007 · Decide user-mediated grant taxonomy
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-007, SEC-002, Q-013
- Baseline: §9.1
- Decision: D-0269
- Risks: R-017, R-041
- Threats: T-001, T-012
- Invariants: I-021, I-060

Chooser, prompt and settings-only classes, plus one-time, session, persistent and revocable durations, must be accepted before the Package capability-request schema. Prompt design decides whether the Capability model is usable or ignored. S-022 is the Layer 2 grant schema this Decision shapes. This Decision also resolves Q-013 by naming which scheduling intents, such as Realtime and LowLatency, are requested as revocable grants and how an unprivileged Component is kept from starving others.

<!-- covers: EXTRA-068, GAP-0227 -->

#### Out of scope
Runtime enforcement (SEC-006). Manifest schema (PKG-011). Anti-fatigue prompt policy (SEC-043). Consent chrome (APP).

#### Acceptance criteria
- [ ] Options evaluated include at least: (A) chooser versus prompt versus settings-only with one-time, session, persistent and revocable durations; (B) prompt-everything; (C) settings-only for all classes.
- [ ] The accepted option names which Capabilities are chooser, prompt or settings-only, with a rationale against prompt fatigue.
- [ ] Each option cites T-001, T-012 and I-060.
- [ ] Surface S-022 remains `prototyped`; the Decision does not freeze it.
- [ ] Review records PKG and APP lead sign-off on the pull request.

#### Verification
- Review: PKG and APP leads sign off on the pull request.
- Manual: decision file lists at least two options, names S-022 and T-012.

#### Evidence
- none

### SEC-008 · Implement scoped Inspect Capability
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-005, CAP-007, OBS-006
- Baseline: §9.1, §24, §64
- Threats: T-001, T-027
- Invariants: I-021, I-034

`os inspect` must not be ambient. `Capability<Inspect>` is scoped to a ResourceDomain or Component subtree so V0 inspect gates print live objects without giving every Component a view of the rest of the system (§64). Holders inspect only the subtree named by the Capability.

<!-- covers: INV-1265 -->

#### Out of scope
CLI rendering (SDK-007). Trace-access policy (OBS-014). Debugger attach (SDK).

#### Acceptance criteria
- [ ] A Component holding `Capability<Inspect>` scoped to Component A prints A's objects via the inspect provider on `qemu-x86_64` and cannot print Component B.
- [ ] A call without `Capability<Inspect>` returns `Error::Rights`, allocates no handle and emits a denial in the Capability audit log.
- [ ] Scoping to a ResourceDomain includes every Component in that domain and no Component outside it.

#### Verification
- Unit: `kernel:tests/sec/inspect_capability_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: V0 `os inspect` suite on H-001 holds a scoped Inspect Capability and fails closed on a sibling Component.
- Review: OBS lead confirms the provider registers under OBS-006.

#### Evidence
- none

### SEC-009 · Build the ambient-authority denial test harness
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-001
- Baseline: §9.1, §11
- Invariants: I-021

Shared harness for filesystem, home, device, app-data and later camera and microphone denials on `qemu-x86_64`. §9.1 constraints become lint and test tasks rather than prose.

<!-- covers: INV-0201 -->

#### Out of scope
Suite contents (SEC-003 and later conformance tasks). Fuzz infrastructure (BLD).

#### Acceptance criteria
- [ ] The harness starts a native Component with an explicit Capability set and records every denied Operation type, object kind and error.
- [ ] Adding a new denial case is a test function; the harness does not hard-code filesystem-only checks.
- [ ] The harness runs on CI matrix entry `qemu-x86_64` and fails the job if any denied Operation succeeds or allocates a handle.

#### Verification
- Unit: `runtime:tests/sec/isolation_harness_*` on `qemu-x86_64`.
- Integration: SEC-003 is invoked through this harness on H-001.
- Review: BLD lead confirms the job is wired into the QEMU matrix.

#### Evidence
- none

### SEC-010 · Record defence-in-depth and authority-design rules
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002
- Baseline: §51, §67
- Invariants: I-060, I-082

§51 and Principle 3 collapse into standing review rules: Rust is not the complete memory-safety strategy, unsafe authority is isolated, and deny-list sandboxes are not the primary mechanism. Unsafe inventory CI remains BLD.

<!-- covers: INV-0949, INV-0950, INV-0956, INV-1294 -->

#### Out of scope
Unsafe inventory generation (BLD-011, KRN-056). Hardware-enforcement hooks (SEC-037). Kernel hardening config (KRN-034).

#### Acceptance criteria
- [ ] The document states that language safety, Capability safety, Component isolation, MemoryObject ownership and hardware enforcement are distinct layers (§51).
- [ ] The document states I-060: blocklist-based sandboxing is not the primary security mechanism.
- [ ] The document is committed in the platform documentation tree and cited by the V0 CAP and SEC review checklist.

#### Verification
- Review: SEC and CAP leads sign off on the pull request.
- Manual: the document names T-001 and I-060.

#### Evidence
- none

### SEC-011 · Require an explicit screen-share Capability
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-010, SEC-007, CAP-005
- Baseline: §9.1, §40
- Threats: T-013
- Invariants: I-085

The V0.5 compositor must not grant unrestricted capture. Capture exists only as an explicit screen-share Capability (S-034). User-facing share UX is V2.

<!-- covers: INV-0751 -->

#### Out of scope
Per-Surface capture implementation (GFX-061). Share UX and indicator (APP-038, APP-031). V2 denial gate (SEC-046).

#### Acceptance criteria
- [ ] A compositor client without a screen-share Capability cannot obtain another client's buffer; the Operation returns `Error::Rights` or a denied Surface.
- [ ] The screen-share Capability is prompt-class per SEC-007.
- [ ] `os inspect` of a V0.5 demo application lists no screen-share Capability unless one was granted.

#### Verification
- Integration: `runtime:tests/sec/screen_share_cap_*` on `qemu-virtio-gpu` and `hw-h002`.
- Review: GFX lead confirms S-034 is the object the compositor checks.

#### Evidence
- none

### SEC-012 · Decide user identity versus Capability roots
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-002, CAP-007
- Baseline: §7, §9.1
- Decision: D-0277
- Threats: T-001, T-026

Pulled to V0.5 so V1 identity and Session work is not waiting on a later Decision. Chooses kernel uid-like identity versus a userspace identity-service mapped to Capability roots, and how login becomes the session root holder. Linux-personality uid/gid derivation is recorded as a consequence, not as a native API.

<!-- covers: GAP-0212, GAP-0541 -->

#### Out of scope
Identity service implementation (SEC-020). Session object (SEC-028). Personality uid mapping implementation (LNX). Multi-user scope (SEC-042).

#### Acceptance criteria
- [ ] Options evaluated include at least: kernel uid-like identity; userspace identity-service mapped to Capability roots; hybrid kernel identifier plus userspace root holder.
- [ ] The accepted option states how login becomes the session Capability root holder and that native software never uses a uid check as authority.
- [ ] Each option cites T-001 and states how the Linux personality derives uid/gid without exporting them natively.
- [ ] Review records CAP and LNX lead sign-off on the pull request.

#### Verification
- Review: CAP and LNX leads sign off on the pull request.
- Manual: decision file lists at least two options and names T-001.

#### Evidence
- none

### SEC-013 · Define administrator versus standard user
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-012
- Baseline: §9.1, §63
- Decision: D-0265
- Threats: T-002, T-026

Pulled to V1 so the identity service can create the first account as administrator and Authorization knows which operations elevate. Replaces sudo and polkit with scoped Capabilities after re-authentication.

<!-- covers: GAP-0221 -->

#### Out of scope
Authorization service (SEC-015). Identity service (SEC-020). Settings chrome (APP).

#### Acceptance criteria
- [ ] Options evaluated include at least: first account is administrator with an elevation Capability; all local accounts equivalent until first elevation; a role bit on the identity object.
- [ ] The accepted option names which operations require elevation and that the first created account is an administrator.
- [ ] Each option cites T-002 and states that elevation never mints a wildcard Capability.
- [ ] Review records SVC and APP lead sign-off on the pull request.

#### Verification
- Review: SVC and APP leads sign off on the pull request.
- Manual: decision file lists at least two options.

#### Evidence
- none

### SEC-014 · Define pluggable authenticator interfaces
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-012
- Baseline: §9.1, §63
- Invariants: I-092

Pluggable authenticators so V3 fingerprint and FIDO2, and later directory login, land without rewriting identity. The password authenticator is the V1 plugin; the Interface stays open for additional methods.

<!-- covers: GAP-0213 -->

#### Out of scope
Identity service account store (SEC-020). Fingerprint plugin (SEC-057). FIDO2 plugin (SEC-056). Directory login (SEC-075).

#### Acceptance criteria
- [ ] A typed Authenticator Interface verifies, enrolls and unenrolls factors without exposing credential bytes to the caller.
- [ ] A second dummy plugin registers beside password and can authenticate a test account on `qemu-x86_64`.
- [ ] Removing a plugin does not require schema changes to the identity store.
- [ ] A caller without the authenticator-admin Capability receives `Error::Rights` on enroll and unenroll.

#### Verification
- Unit: `runtime:tests/sec/authenticator_iface_*` on `qemu-x86_64`.
- Integration: password plugin plus dummy plugin on H-001.
- Review: IPC lead confirms the Interface follows S-014 evolution rules.

#### Evidence
- none

### SEC-015 · Build a re-authentication Authorization service
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-020, SEC-013, SEC-028, CAP-003
- Baseline: §9.1, §63
- Threats: T-002
- Invariants: I-060

V1 admin actions need scoped Capabilities after re-authentication with an audit record, replacing sudo and polkit. Native Packages contain no setuid binaries (SEC-023).

<!-- covers: GAP-0220 -->

#### Out of scope
Setuid lint (SEC-023). Trusted-UI elevation Surface (GFX-040). Durable audit store (OBS-044).

#### Acceptance criteria
- [ ] An elevated Operation without a prior re-authentication returns `Error::Rights` and allocates no handle.
- [ ] Successful re-authentication mints a scoped Capability whose rights are a subset of the administrator role named by SEC-013.
- [ ] The grant is recorded in the Capability audit log with actor, target and rights.
- [ ] Elevation never yields a wildcard Capability; a test requests Admin on an unrelated File and is denied.

#### Verification
- Unit: `runtime:tests/sec/authorization_*` on `qemu-x86_64` and `hw-h002`.
- Integration: install-driver-style elevation on H-004 records the audit event.
- Review: CAP lead confirms minted rights are a subset.

#### Evidence
- none

### SEC-016 · Provide a typed CA trust store with pinning
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-002, CAP-005
- Baseline: §9.1, §51
- Threats: T-019

V1 TLS for Git, cargo and HTTPS needs a typed system CA store with per-application pinning. An application without the store Capability cannot use default system CAs.

<!-- covers: EXTRA-019 -->

#### Out of scope
Native TLS consumption (SEC-022, NET-011). Personality mirrors (SEC-024). Enterprise enrolment (SEC-054).

#### Acceptance criteria
- [ ] `Capability<TrustStore>` lists system CAs and per-application pins; a Component without it cannot read the store.
- [ ] A pin that excludes a CA causes handshake using that CA to fail with a typed trust error.
- [ ] `os inspect` of the store shows pin set and CA identities without exposing private keys.
- [ ] Adding or removing a CA requires an Authorization grant named by SEC-013.

#### Verification
- Unit: `runtime:tests/sec/ca_store_*` on `qemu-x86_64`.
- Integration: pinned versus unpinned fetch against a test CA on H-001.
- Review: NET lead confirms NET-011 can consume this object.

#### Evidence
- none

### SEC-017 · Provide encrypted volumes for developer machines
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-005, SEC-018, STO-016
- Baseline: §51, §61
- Threats: T-008, T-010
- Invariants: I-073

V1 daily-drive: encryption must exist before a lost laptop is possible. V1 scope allows manual full-disk encryption; installer UI is V3. The mechanism is the one named by SEC-005. Required by V3-G01 (Installer completes on Tier 1 with full-disk encryption).

#### Out of scope
Installer FDE UX (INS, SEC-055). TPM PCR seal (SEC-052). Store layering Decision (STO-039).

#### Acceptance criteria
- [ ] A volume created with the chosen mechanism unlocks via SEC-018 and mounts as the user data Collection on H-001 and H-002.
- [ ] Ciphertext on the block device is distinguishable from plaintext of a known pattern after unlock-then-relock.
- [ ] A wrong passphrase does not pivot root and does not log the secret.
- [ ] Manual FDE is documented for developers; no installer UI is required.

#### Verification
- Integration: `runtime:tests/sec/fde_volume_*` on `qemu-x86_64` and `hw-h002`.
- Manual: enroll and unlock on H-004.
- Review: STO lead confirms the volume maps onto the chosen filesystem without a new storage layer.

#### Evidence
- none

### SEC-018 · Expose a pre-boot disk-unlock key-slot API
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-005
- Baseline: §51
- Threats: T-008, T-010

BOOT owns the pre-boot unlock UI. SEC owns key-slots, passphrase verify and recovery-key unlock consumed at V1 manual FDE.

<!-- covers: GAP-0197 -->

#### Out of scope
Text unlock UI (BOOT-026). Recovery-key save UX (SEC-025). TPM unseal (SEC-052).

#### Acceptance criteria
- [ ] Passphrase verify against an enrolled slot unlocks the volume key and returns it only to the unlock caller.
- [ ] Recovery-key verify unlocks the same volume after passphrase failure.
- [ ] A wrong secret increments an attempt counter visible to the UI and does not return the volume key.
- [ ] Multiple enrolled slots are addressable; selecting slot 2 unlocks when slot 1 is empty.

#### Verification
- Unit: `runtime:tests/sec/keyslot_*` on `qemu-x86_64`.
- Integration: BOOT-026 consumes this API on H-001.
- Review: BOOT lead confirms the slot list and remaining-attempt fields match the text UI.

#### Evidence
- none

### SEC-019 · Hide stable hardware identifiers without a Capability
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-009, CAP-005
- Baseline: §9.1
- Threats: T-001
- Invariants: I-078

No ambient machine ID, TPM EK, disk serial or MAC. Per-application derived identifiers are provided where needed.

<!-- covers: GAP-0236 -->

#### Out of scope
TPM service (SEC-053). Network stack MAC handling (NET). HCL probe redaction (REL).

#### Acceptance criteria
- [ ] A native Component without a hardware-identity Capability that requests machine ID, TPM EK, disk serial or MAC receives `Error::Rights` and allocates no handle.
- [ ] A derived per-application identifier is stable for that Component and differs from the identifier issued to a second Component on the same machine.
- [ ] The denial cases run in SEC-009 on `qemu-x86_64`.

#### Verification
- Unit: `runtime:tests/sec/hwid_denial_*` on `qemu-x86_64` and `hw-h002`.
- Integration: isolation harness job includes the four identifier kinds.
- Review: HW lead confirms no sysfs passthrough remains on the native path.

#### Evidence
- none

### SEC-020 · Build a Capability-gated identity service
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-012, SEC-013, SEC-014
- Baseline: §9.1, §61
- Threats: T-010, T-026

V1 daily-driving login: local accounts with Argon2id-hashed credentials, no ambient credential file, typed Interfaces only. The first created account is an administrator per SEC-013.

<!-- covers: GAP-0213 -->

#### Out of scope
Session object (SEC-028). Greeter chrome (APP). Additional authenticators (SEC-057, SEC-056).

#### Acceptance criteria
- [ ] Creating the first account yields an administrator; a second account is a standard user unless elevation is granted.
- [ ] Credential verify uses Argon2id; a Component without the identity Capability cannot enumerate accounts or read hashes.
- [ ] Failed authentication increments a counter visible to `os inspect` of the identity service and does not return the hash.
- [ ] All identity Operations are typed Channel calls; no ambient credential file is readable to native Components.

#### Verification
- Unit: `runtime:tests/sec/identity_*` on `qemu-x86_64` and `hw-h002`.
- Integration: first-boot style account creation on H-004.
- Review: CAP lead confirms the session root handoff matches SEC-012.

#### Evidence
- none

### SEC-021 · Extend isolation suite to ImageDecoder microphone denial
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-003, AUD-003
- Baseline: §9.1, §11
- Threats: T-014, T-038
- Invariants: I-021

Audio exists at V1. ImageDecoder must still hold no microphone Capability.

<!-- covers: INV-0241 -->

#### Out of scope
Microphone grant enforcement in the audio path (AUD-003). Prompt UI (APP). Camera denial (SEC-039).

#### Acceptance criteria
- [ ] The ImageDecoder fixture's microphone Operation returns `Error::Rights`, allocates no stream and appears in the audit log, on `qemu-x86_64`.
- [ ] The case is registered in SEC-009 and fails the job if the Operation succeeds.
- [ ] A Component that holds a microphone Capability is unaffected by the ImageDecoder denial.

#### Verification
- Integration: `runtime:tests/sec/isolation_mic_*` on `qemu-x86_64` and `hw-h002`.
- Review: AUD lead confirms the denied object is the same as AUD-003.

#### Evidence
- none

### SEC-022 · Consume the CA trust store in native TLS
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-016, NET-005
- Baseline: §9.1
- Threats: T-019

Native TLS consumes the typed store. An application without the store Capability cannot use default system CAs. Personality TLS stays in LNX and WIN.

<!-- covers: EXTRA-019 -->

#### Out of scope
TLS library Decision (NET-005). Personality mirrors (SEC-024). Resolver (NET-019).

#### Acceptance criteria
- [ ] A native TLS handshake without `Capability<TrustStore>` fails with a typed trust error and opens no connection.
- [ ] A handshake with the store Capability and a valid pin succeeds against the test server on `qemu-x86_64`.
- [ ] Native software does not open a POSIX socket to complete the handshake.

#### Verification
- Integration: `runtime:tests/sec/native_tls_trust_*` on `qemu-x86_64`.
- Review: NET lead confirms consumption matches NET-011.

#### Evidence
- none

### SEC-023 · Forbid setuid binaries in native Packages
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-015, BLD-011, PKG-028
- Baseline: §9.1, §28
- Invariants: I-060

Native Packages contain no setuid binaries. CI lint rejects setuid bits on native artifacts so elevation stays in the Authorization service.

<!-- covers: GAP-0220 -->

#### Out of scope
Authorization service (SEC-015). Personality packaging (LNX, WIN). License lint (GOV, BLD).

#### Acceptance criteria
- [ ] A native Package whose payload has a setuid bit fails CI with a named lint.
- [ ] A native Package without that bit passes the lint on `qemu-x86_64`.
- [ ] The lint does not apply to Linux-personality payloads.

#### Verification
- Unit: lint fixtures for setuid-present and setuid-absent Packages.
- Review: PKG lead confirms the check runs on native artifacts only.

#### Evidence
- none

### SEC-024 · Mirror the CA trust store into personalities
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-016
- Baseline: §3, §9.1
- Threats: T-011, T-019
- Invariants: I-025

The native trust store is mirrored into Linux and Windows personality certificate stores so pinning and enrolment apply once. Personalities consume the native ABI and never extend it.

<!-- covers: EXTRA-019 -->

#### Out of scope
Native store (SEC-016). Personality TLS stacks (LNX, WIN). Enterprise enrolment (SEC-054).

#### Acceptance criteria
- [ ] A CA added to the native store appears in the Linux personality trust set used by the personality TLS path.
- [ ] A per-application pin on a native Component is not visible as a global personality CA.
- [ ] Removing a CA from the native store removes it from both personality mirrors.

#### Verification
- Integration: `runtime:tests/sec/ca_mirror_*` on `qemu-x86_64`.
- Review: LNX lead confirms the mirror is a consumption of S-030, not an extension of Layer 1.

#### Evidence
- none

### SEC-025 · Require saving an offline recovery key
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-017, SEC-018
- Baseline: §51, §63
- Threats: T-008, T-010
- Invariants: I-073, I-077

Recovery-key path starts when volumes exist: generate an offline recovery key the user must save; no cloud escrow by default.

<!-- covers: GAP-0198 -->

#### Out of scope
Installer confirmation UX (INS-010). TPM seal (SEC-052). Cloud backup (STO).

#### Acceptance criteria
- [ ] Creating an encrypted volume generates a recovery key and refuses to mark the volume ready until a save acknowledgment is recorded.
- [ ] The recovery key unlocks via SEC-018 after passphrase failure.
- [ ] No network Capability is used during generation or save; a test that stubs a cloud endpoint observes no traffic.
- [ ] The key material is not written to crash dumps (I-077).

#### Verification
- Integration: `runtime:tests/sec/recovery_key_*` on `qemu-x86_64` and `hw-h002`.
- Manual: save-to-file acknowledgment on H-004.
- Review: INS lead confirms first-boot can consume the acknowledgment flag.

#### Evidence
- none

### SEC-026 · Prove no application can read another app's secrets
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-027
- Baseline: §9.1
- Threats: T-001
- Invariants: I-021

Enumerate and read of another Component's secrets return `Error::Rights` and allocate no handle.

<!-- covers: GAP-0222 -->

#### Out of scope
Secrets service implementation (SEC-027). SSH agent (SEC-030).

#### Acceptance criteria
- [ ] Component A storing a secret cannot be enumerated or read by Component B; both Operations return `Error::Rights` and allocate no handle, on `qemu-x86_64`.
- [ ] The denial is present in the Capability audit log.
- [ ] The case is registered in SEC-009.

#### Verification
- Integration: `runtime:tests/sec/secrets_isolation_*` on `qemu-x86_64` and `hw-h002`.
- Review: CAP lead confirms no leftover mapping remains after the denied Operation.

#### Evidence
- none

### SEC-027 · Build a per-application secrets service
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-020, SEC-028, CAP-005
- Baseline: §9.1, §61
- Threats: T-001, T-010
- Invariants: I-077

V1 keyring: each application's secrets are Capability-scoped objects under a per-user key unlocked at login. No application can enumerate or read another application's secrets.

<!-- covers: GAP-0222 -->

#### Out of scope
Isolation suite (SEC-026). SSH and Git integration (SEC-030). Hardware-backed lock (SEC-059).

#### Acceptance criteria
- [ ] A secret stored by Component A is readable by A after a session unlock and not by Component B.
- [ ] Locking the Session makes subsequent secret reads return a typed locked error until unlock.
- [ ] Secret bytes do not appear in crash dumps of the secrets service.
- [ ] `os inspect` lists secret identities and holders, not plaintext.

#### Verification
- Unit: `runtime:tests/sec/secrets_*` on `qemu-x86_64` and `hw-h002`.
- Integration: login, store, lock, unlock, read on H-004.
- Review: OBS lead confirms dump redaction for secret objects.

#### Evidence
- none

### SEC-028 · Implement Session linking identity, seat, and root
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-020, SEC-012
- Baseline: §9.1, §32, §61
- Threats: T-009, T-026
- Invariants: I-075

Session binds user, seat, display and the session Capability root, with lock, unlock, idle, suspend-inhibit and end-of-session as typed Interfaces replacing logind for native software.

<!-- covers: GAP-0214 -->

#### Out of scope
Supervision tree (SVC-027). Greeter and lock chrome (APP). Compositor lock mode (GFX-045). Multi-user switching (SEC-060).

#### Acceptance criteria
- [ ] Successful login creates a Session whose Capability root is the holder named by SEC-012.
- [ ] Lock, unlock, idle and end-of-session are typed Operations; lock makes the compositor-facing Session state `locked`.
- [ ] Suspend-inhibit is a Capability, not an ambient D-Bus call; without it the inhibit Operation returns `Error::Rights`.
- [ ] Native software has no logind-shaped Interface.

#### Verification
- Unit: `runtime:tests/sec/session_*` on `qemu-x86_64` and `hw-h002`.
- Integration: login-lock-unlock on H-004 with GFX-045.
- Review: SVC lead confirms the Session object is the one SVC-027 parents.

#### Evidence
- none

### SEC-029 · Publish the microarchitectural side-Channel statement
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002, SEC-010
- Baseline: §9.1, §51
- Risks: R-080
- Threats: T-015, T-030
- Invariants: I-083

State what isolation Capabilities claim, which shared resources (glyph atlas, shared MemoryObjects, SMT siblings) are cross-domain channels, and which mitigations are on by default.

<!-- covers: EXTRA-015 -->

#### Out of scope
Glyph atlas implementation (TXT). Mitigation measurement (KRN-029). Kernel hardening config (KRN-034).

#### Acceptance criteria
- [ ] The statement names what the Capability model claims and what it does not claim about microarchitectural isolation.
- [ ] The statement lists glyph atlas, shared MemoryObjects and SMT siblings as cross-domain channels and cites T-015 and T-030.
- [ ] The statement names which shipped mitigations are on by default, citing B-040 for cost, with no superiority claim.
- [ ] Review records TXT and KRN lead sign-off on the pull request.

#### Verification
- Review: TXT and KRN leads sign off on the pull request.
- Manual: the document cites T-015, T-030, I-083 and B-040.

#### Evidence
- none

### SEC-030 · Back SSH agent and Git credentials with secrets
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-027
- Baseline: §61
- Threats: T-010

V1 self-hosting needs Git over SSH without plaintext keys. Optional TPM- or FIDO2-resident keys come later.

<!-- covers: GAP-0223 -->

#### Out of scope
FIDO2 authenticators (SEC-056). TPM service (SEC-053). Personality sshd (NET, LNX).

#### Acceptance criteria
- [ ] An SSH private key stored as a secret is used to authenticate a Git fetch without writing the key as a plaintext file in the Component's store.
- [ ] A Component without the agent Capability cannot list or sign with another Component's keys; Operations return `Error::Rights`.
- [ ] Git credential helper reads and writes only through the secrets service.

#### Verification
- Integration: `runtime:tests/sec/ssh_git_agent_*` on `qemu-x86_64` and H-004.
- Review: ENV lead confirms self-host Git over SSH uses this path.

#### Evidence
- none

### SEC-031 · Decide disk-key eviction on suspend
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-017, SEC-002, PWR-002
- Baseline: §51, §61
- Decision: D-0275
- Threats: T-009, T-041
- Invariants: I-077

Pulled to V1 because V1 ships suspend and resume. Decides RAM eviction, re-unlock on resume, suspend-then-hibernate, and whether hibernation is allowed under lockdown.

<!-- covers: GAP-0201 -->

#### Out of scope
Implementation (SEC-048). Hibernation policy for the power service (PWR-007). Suspend cycle harness (PWR-014).

#### Acceptance criteria
- [ ] Options evaluated include at least: evict keys from RAM on suspend and re-unlock on resume; keep keys in RAM while locked; suspend-then-hibernate; forbid hibernate under lockdown.
- [ ] The accepted option states what happens to disk keys on suspend and whether an unsigned hibernation image is rejected (T-041).
- [ ] Each option cites T-009 and T-041.
- [ ] Review records PWR and BOOT lead sign-off on the pull request.

#### Verification
- Review: PWR and BOOT leads sign off on the pull request.
- Manual: decision file lists at least two options and names T-009 and T-041.

#### Evidence
- none

### SEC-032 · Evaluate MPK, CET, and LAM for unsafe isolation
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-010, KRN-034
- Baseline: §8, §51
- Invariants: I-058

Pulled to V1 so V2 hardware-enforcement hooks depend on a done spike. Explores x86-64 MPK/PKU, CET shadow stacks and LAM for isolating unsafe code inside Components. Application-visible Capabilities stay conceptually stable (§8).

<!-- covers: INV-0960 -->

#### Out of scope
Hook implementation (SEC-037). Enclave study (SEC-035). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] `reports/spikes/SEC-032.md` records, for MPK/PKU, CET shadow stacks and LAM, whether the mechanism can isolate unsafe code inside a Component without changing application-visible Capabilities.
- [ ] The report names which hooks V2 adds and which mechanisms stay off by default.
- [ ] The report does not freeze an L1 surface and cites no performance number; cost comparisons cite B-040 or stay qualitative.

#### Verification
- Report: which mechanisms isolate unsafe code, what V2 hooks to add, ABI impact, and what remains software.
- Review: KRN and CAP leads sign off on the spike report.

#### Evidence
- none

### SEC-033 · Scope AI Capability grants to project, file, or time
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-034, SEM-010, SEM-029, SEC-006, CAP-041
- Baseline: §44, §57
- Threats: T-017
- Invariants: I-023, I-051

V2 AI demo: grants are scoped and revocable mid-run; the action graph is logged. Implements the AI-principal Decision. No AI work precedes a done semantic-registry task.

<!-- covers: INV-0830 -->

#### Out of scope
Principal Decision (SEC-034). Broker implementation (SEM-010). Mid-run revocation mechanics (CAP-041). Consent chrome (APP-025).

#### Acceptance criteria
- [ ] An AI-broker grant scoped to one File cannot be used on a second File; the Operation returns `Error::Rights`.
- [ ] A time-windowed grant fails the next Operation after expiry with `Error::Rights` and no leftover mapping.
- [ ] Revoking a grant mid-run is visible on the action graph and the next broker call using that grant fails.
- [ ] The broker holds no grant that was not logged.

#### Verification
- Integration: V2 AI demo on H-002 scopes a Project grant, revokes it, and the next step fails.
- Unit: `runtime:tests/sec/ai_grants_*` on `qemu-x86_64`.
- Demo: V2 AI demo action graph logged and one step revoked on H-002.
- Review: SEM lead confirms registry-before-broker ordering.

#### Evidence
- none

### SEC-034 · Decide whether an AI assistant is a distinct principal
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-029, SEM-001, SEC-012, SEC-007, Q-037
- Baseline: §44, §57
- Decision: D-0266
- Risks: R-043
- Threats: T-017
- Invariants: I-023, I-051

V2 AI demo is Capability-only. SEC decides permissioning, scoped grants, and whether the assistant is a distinct principal for audit and revocation, after the SEM registry exists. Answers Q-037.

<!-- covers: INV-0826, INV-0830, INV-0835 -->

#### Out of scope
Scoped grant implementation (SEC-033). Broker (SEM-010). Model execution placement (SEM).

#### Acceptance criteria
- [ ] Options evaluated include at least: assistant is a distinct principal; assistant acts as the user; hybrid with a distinct audit identity and user-held grants.
- [ ] The accepted option states how grants are scoped (project, file, time) and how revocation is attributed.
- [ ] Each option cites T-017, I-051 and SEM-029.
- [ ] Review records SEM and CAP lead sign-off on the pull request.

#### Verification
- Review: SEM and CAP leads sign off on the pull request.
- Manual: decision file lists at least two options, names T-017 and Q-037.

#### Evidence
- none

### SEC-035 · Study secure enclaves as a confidentiality home
- Type: spike
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-032, SEC-002
- Baseline: §8
- Invariants: I-058

Whether SGX/SEV-style enclaves can host Components that need hardware confidentiality. Enablement is LATER; 1.0 does not promise hardware enforcement.

<!-- covers: INV-0194 -->

#### Out of scope
Enablement (SEC-078). CHERI mapping (CAP). Hardware-enforcement hooks (SEC-037).

#### Acceptance criteria
- [ ] `reports/spikes/SEC-035.md` describes at least one enclave mechanism and whether a Component could run inside it without changing application-visible Capabilities.
- [ ] The report states that 1.0 does not promise hardware confidentiality.
- [ ] The report records a keep-software-isolation option and a LATER enablement option.

#### Verification
- Report: which enclave mechanisms exist, ABI impact, 1.0 non-promise, and whether SEC-078 proceeds or drops.
- Review: CAP lead comments on ABI stability.

#### Evidence
- none

### SEC-036 · Prove grant revocation takes effect immediately
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-045, CAP-044, CAP-004
- Baseline: §7, §9.1
- Threats: T-005, T-013, T-014

V2 permissions gate: after revoke, camera, microphone, files, network and screen capture fail on the next Operation with no leftover mapping. CAP owns the object-type matrix; this task is the UI-level proof.

#### Out of scope
Kernel matrix (CAP-044). Settings chrome (APP-029). Device stacks (MED, AUD, NET, GFX).

#### Acceptance criteria
- [ ] Revoking camera, microphone, File, network connect and screen-capture from the permissions UI makes the next Operation of that type return `Error::Rights` on `qemu-x86_64`.
- [ ] No leftover mapping remains; a subsequent retry without a new grant also fails.
- [ ] The five types are one CI job citing CAP-044; a missing type fails the job.

#### Verification
- Integration: `runtime:tests/sec/revoke_ui_*` on `qemu-x86_64` and H-002.
- Review: CAP lead confirms this job consumes the matrix rather than duplicating it.

#### Evidence
- none

### SEC-037 · Add hardware-enforcement hooks in Capability checks
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-032, CAP-005, CAP-010
- Baseline: §8, §51
- Invariants: I-058

§51 hardware layer: design hooks now from the x86-enforcement study, enable on capable hardware later. Does not freeze an L1 surface. Application-visible Capabilities remain conceptually stable (§8).

<!-- covers: INV-0954 -->

#### Out of scope
Enclave enablement (SEC-078). Rights encoding Decision (CAP). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] Capability check paths expose a hook the study named (MPK, CET or LAM) compiled out or no-op on hardware without the feature, on `qemu-x86_64`.
- [ ] Enabling the hook on capable hardware does not change application-visible handle or rights encodings.
- [ ] No L1 surface is marked frozen by this task.
- [ ] `os inspect` reports whether the hook is active on the running kernel.

#### Verification
- Unit: `kernel:tests/sec/hw_enforce_hooks_*` on `qemu-x86_64` and `hw-h002`.
- Review: CAP lead confirms S-003 remains prototyped.
- Manual: spike report SEC-032 is cited on the pull request.

#### Evidence
- none

### SEC-038 · Review and deny optional Capabilities at install
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-006, SEC-004, PKG-075
- Baseline: §9.1, §28
- Threats: T-006

V2 store-client gate: Capability requests are shown and can be denied at install. Store chrome is APP; this task is the review contract.

#### Out of scope
Store client UI (APP-045). Degraded launch (SEC-047, CMP-043). Publisher identity (REL).

#### Acceptance criteria
- [ ] Installing a Package lists requested required and optional Capabilities before commit.
- [ ] Denying an optional Capability records a denial and still allows install to complete.
- [ ] Denying a required Capability aborts install with a typed error and stores no grant.
- [ ] The review is visible in the grant log.

#### Verification
- Integration: `runtime:tests/sec/install_review_*` on `qemu-x86_64` and H-002.
- Review: PKG lead confirms optional-versus-required matches PKG-075.

#### Evidence
- none

### SEC-039 · Extend isolation suite to ambient camera denial
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-021, MED-013
- Baseline: §9.1
- Threats: T-014
- Invariants: I-021

Camera service exists at V2. A native application must not receive camera access automatically.

<!-- covers: INV-0204 -->

#### Out of scope
Camera service (MED-013). In-use indicator (APP-031). Prompt runtime (SEC-044).

#### Acceptance criteria
- [ ] A native application with no camera Capability receives `Error::Rights` on capture, allocates no session and appears in the audit log, on `qemu-x86_64`.
- [ ] The case is registered in SEC-009.
- [ ] A granted camera session is unaffected.

#### Verification
- Integration: `runtime:tests/sec/isolation_camera_*` on `qemu-x86_64` and H-002.
- Review: MED lead confirms the denied object is the Camera Capability MED mints.

#### Evidence
- none

### SEC-040 · Support passphrase change and cryptographic erase
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-018, SEC-017
- Baseline: §51
- Threats: T-010
- Invariants: I-073

Rotate key-slots without re-encrypting. Cryptographic erase is the factory-reset primitive.

<!-- covers: GAP-0200 -->

#### Out of scope
TPM reseal (SEC-051). Installer factory reset UX (INS). Volume creation (SEC-017).

#### Acceptance criteria
- [ ] Changing a passphrase updates the slot and the old passphrase no longer unlocks, without rewriting user data ciphertext, on `qemu-x86_64`.
- [ ] Cryptographic erase destroys the volume key; a subsequent unlock with the old passphrase and recovery key both fail.
- [ ] After erase, a known plaintext pattern is not recoverable from the block device in the test image.

#### Verification
- Integration: `runtime:tests/sec/keyslot_rotate_*` on `qemu-x86_64` and `hw-h002`.
- Review: STO lead confirms no full-volume rewrite is required.

#### Evidence
- none

### SEC-041 · Expose coarse and precise location as Capabilities
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-006, SEC-044
- Baseline: §9.1
- Threats: T-001
- Invariants: I-021

Timezone and Wi-Fi geolocation versus precise location are separate Capabilities, with no ambient location. Shell in-use indicator is APP.

<!-- covers: GAP-0289, INV-0209 -->

#### Out of scope
Time and locale services (SVC). Wi-Fi stack (NET). Settings panel (APP). In-use indicator chrome (APP).

#### Acceptance criteria
- [ ] Coarse and precise location are distinct Capabilities; holding coarse does not satisfy a precise Operation.
- [ ] A native application with neither Capability receives `Error::Rights` on both Operations and allocates no handle.
- [ ] Granting precise location does not grant other-application data or filesystem authority.
- [ ] `os inspect` shows which class is held and whether it is in use.

#### Verification
- Unit: `runtime:tests/sec/location_*` on `qemu-x86_64`.
- Integration: coarse-only versus precise grant on H-004.
- Review: SVC lead confirms timezone detection can consume coarse location.

#### Evidence
- none

### SEC-042 · Decide 1.0 multi-user and per-user encryption scope
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-012, SEC-005
- Baseline: §9.1, §63
- Decision: D-0270
- Threats: T-026
- Invariants: I-092

Must precede V3 multi-user sessions: local account count, fast user switching, concurrent graphical sessions and per-user home keys.

<!-- covers: GAP-0218 -->

#### Out of scope
Implementation (SEC-060, SEC-061). Enterprise directory (SEC-075). Greeter chrome (APP).

#### Acceptance criteria
- [ ] Options evaluated include at least: single graphical session with multiple accounts; fast user switching of one graphical session; concurrent graphical sessions with per-user home keys.
- [ ] The accepted option states local account count policy, whether concurrent graphical sessions exist, and whether per-user home data is encrypted under login-unlocked keys.
- [ ] Each option cites T-026 and I-092.
- [ ] Review records APP, SVC and STO lead sign-off on the pull request.

#### Verification
- Review: APP, SVC and STO leads sign off on the pull request.
- Manual: decision file lists at least two options and names T-026.

#### Evidence
- none

### SEC-043 · Decide permission prompt policy against fatigue
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-007, SEC-002
- Baseline: §9.1
- Decision: D-0273
- Risks: R-041
- Threats: T-012
- Invariants: I-060

Complements V0.5 grant taxonomy with default, persistence and anti-fatigue rules for the prompt runtime. Prompt-everything reintroduces a deny-list UX.

#### Out of scope
Prompt runtime (SEC-044). Trusted-UI Surfaces (GFX-040). Consent chrome (APP-025).

#### Acceptance criteria
- [ ] Options evaluated include at least: prompt every time; session default with a settings escape; chooser silent, prompt once-per-session, settings-only never prompt.
- [ ] The accepted option states default persistence and how repeated prompts are coalesced without granting silently.
- [ ] Each option cites T-012, R-041 and I-060.
- [ ] Review records APP and GFX lead sign-off on the pull request.

#### Verification
- Review: APP and GFX leads sign off on the pull request.
- Manual: decision file lists at least two options and names T-012.

#### Evidence
- none

### SEC-044 · Implement trusted permission prompts
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-043, GFX-040, SEC-006
- Baseline: §9.1, §40
- Threats: T-012
- Invariants: I-060

V2 prompts for Capability requests. GFX owns trusted-UI Surfaces. SEC owns prompt state, grant recording and deny-without-crash behavior.

#### Out of scope
Trusted-UI compositor protection (GFX-040). Consent chrome widgets (APP-025). Permissions list UI (SEC-045).

#### Acceptance criteria
- [ ] A prompt-class request renders through the trusted-UI Surface; the requesting application cannot overlay or inject input into it.
- [ ] Deny returns `Error::Rights` to the requester, allocates no handle and does not abort the Component.
- [ ] Allow records a grant whose duration matches SEC-043.
- [ ] A second request coalesced by policy does not show a second prompt unless the policy requires it.

#### Verification
- Integration: `runtime:tests/sec/prompt_runtime_*` on `qemu-virtio-gpu` and H-002.
- Review: GFX lead confirms the Surface is the trusted-UI class.
- Demo: V2 camera/microphone/files/network/screen-capture prompt deny-without-crash on H-002.

#### Evidence
- none

### SEC-045 · Ship permissions UI to view and revoke grants
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-006, APP-012, CAP-037, OBS-043
- Baseline: §9.1, §63
- Threats: T-001
- Invariants: I-021

V2 gate: permissions UI lists every active grant and revocation takes effect immediately, tested with camera, microphone, files, network and screen capture. APP owns Settings chrome; this task is the list/revoke contract.

#### Out of scope
Settings chrome (APP-029). Immediate-revoke proof job (SEC-036). Audit history v2 (SEC-062).

#### Acceptance criteria
- [ ] Every active grant for a Component is listed with type, duration and source.
- [ ] Revoke of a listed grant is recorded and the next Operation of that type returns `Error::Rights`.
- [ ] Camera, microphone, files, network and screen capture each appear when granted.
- [ ] A caller without the permissions-admin Capability cannot revoke another user's grants.

#### Verification
- Integration: `runtime:tests/sec/permissions_ui_v1_*` on `qemu-x86_64` and H-002.
- Review: APP lead confirms APP-029 consumes this contract.
- Demo: V2 permissions UI on H-002 lists and revokes a camera grant.

#### Evidence
- none

### SEC-046 · Verify screen-share denial on the V2 Gate
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-011, GFX-061, APP-031
- Baseline: §9.1, §40
- Threats: T-013
- Invariants: I-085

V2 screen-sharing gate: without the screen-share Capability the application receives a black or denied Surface; sharing shows a persistent indicator (APP, GFX).

#### Out of scope
Capture implementation (GFX-061). Indicator chrome (APP-031). Capability type (SEC-011).

#### Acceptance criteria
- [ ] An application without the screen-share Capability receives a black or denied Surface and no other client's pixels, on H-002.
- [ ] While sharing is active the persistent indicator is on; ending the grant turns it off.
- [ ] The case is registered in SEC-009.

#### Verification
- Integration: `runtime:tests/sec/screen_share_gate_*` on H-002 and `qemu-virtio-gpu`.
- Demo: V2 screen-share denial on H-002.
- Review: GFX lead confirms the denied Surface is S-034.

#### Evidence
- none

### SEC-047 · Launch Packages in degraded mode when optional grants are denied
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-038, PKG-075
- Baseline: §9.1, §34
- Invariants: I-021

V2 store gate: denying optional Capabilities still launches the application in a declared degraded mode. CMP implements the launch path; this task is the security contract.

#### Out of scope
Launch path (CMP-043). Manifest schema (PKG). Store chrome (APP-045).

#### Acceptance criteria
- [ ] A Package that declares an optional camera Capability and is denied it still starts; a camera Operation returns `Error::Rights`.
- [ ] Required Capability denial still aborts install and does not start the Component.
- [ ] Degraded mode is visible in `os inspect` of the Component.

#### Verification
- Integration: `runtime:tests/sec/degraded_launch_*` on `qemu-x86_64`.
- Review: CMP lead confirms CMP-043 implements this contract.

#### Evidence
- none

### SEC-048 · Implement suspend key-eviction and hibernate policy
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-031, SEC-017, PWR-014, PWR-007
- Baseline: §51, §61
- Threats: T-009, T-041
- Invariants: I-077

Implements the V1 suspend-key-eviction Decision on laptops, including authenticated hibernation images if lockdown allows hibernate.

#### Out of scope
Suspend cycles (PWR). Hibernation image layout (STO, PWR-020). Pre-boot unlock UI (BOOT).

#### Acceptance criteria
- [ ] On H-004, suspend follows the accepted eviction option: keys are absent from RAM after suspend if eviction was chosen, proven by a resume that requires unlock before reading the volume.
- [ ] An unsigned hibernation image is rejected under lockdown (T-041) if hibernate is enabled.
- [ ] Crash dumps taken while locked do not contain disk keys.
- [ ] Resume without a valid unlock does not mount user data.

#### Verification
- Integration: `runtime:tests/sec/suspend_keys_*` on H-004 and H-005.
- Review: PWR lead confirms the path runs inside PWR-014.
- Manual: locked suspend then resume-unlock on H-004.

#### Evidence
- none

### SEC-049 · Provide passphrase-only unlock without a TPM
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-050, SEC-018
- Baseline: §51, §62
- Threats: T-008, T-010
- Invariants: I-074

Degraded path: machines without TPM 2.0 unlock with passphrase only and skip measured-boot seals.

#### Out of scope
Requirement Decision (SEC-050). PCR seal (SEC-052). Hardware enablement (HW).

#### Acceptance criteria
- [ ] On a QEMU profile without a TPM, passphrase unlock succeeds and measured-boot seal is reported as skipped in `os inspect`.
- [ ] The same volume still accepts a recovery key.
- [ ] The degraded path is the one named by SEC-050; a machine with a TPM does not take this path by default.

#### Verification
- Integration: `runtime:tests/sec/tpm_degraded_*` on `qemu-x86_64` without TPM and with TPM.
- Review: BOOT lead confirms measured-boot skip is visible to the bootloader.

#### Evidence
- none

### SEC-050 · Decide TPM 2.0 as requirement versus optional
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002, SEC-005
- Baseline: §51, §55, §62
- Decision: D-0276
- Threats: T-008
- Invariants: I-074

V2 measured boot and sealed keys need a hard-requirement versus optional Decision and degraded passphrase-only behavior on machines without a TPM.

<!-- covers: GAP-0190 -->

#### Out of scope
Degraded unlock implementation (SEC-049). Measured boot (BOOT-034). HCL publication (REL).

#### Acceptance criteria
- [ ] Options evaluated include at least: TPM 2.0 hard requirement for 1.0; optional with passphrase-only degraded path; required for Tier 1 only.
- [ ] The accepted option states degraded behavior on machines without a TPM and how the HCL records it.
- [ ] Each option cites T-008 and I-074.
- [ ] Review records BOOT, HW and REL lead sign-off on the pull request.

#### Verification
- Review: BOOT, HW and REL leads sign off on the pull request.
- Manual: decision file lists at least two options and names T-008.

#### Evidence
- none

### SEC-051 · Reseal TPM-bound keys across firmware updates
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-052, BOOT-034, HW-046
- Baseline: §30, §51
- Threats: T-021, T-022

Re-seal before and after firmware updates and Secure Boot key changes so PCR changes do not strand the user at recovery.

<!-- covers: GAP-0199 -->

#### Out of scope
Firmware payloads (HW-046). Capsule staging (BOOT). Recovery-key UX (SEC-025).

#### Acceptance criteria
- [ ] A firmware update that changes PCR values reseals the disk key before reboot; first boot after update unlocks without the recovery key, on H-002.
- [ ] A failed reseal leaves the previous seal intact and surfaces a typed error; the recovery key still unlocks.
- [ ] Secure Boot key change triggers the same reseal path.
- [ ] `os inspect` records reseal events with generation identity.

#### Verification
- Integration: `runtime:tests/sec/tpm_reseal_*` on H-002 with a simulated PCR change.
- Review: BOOT lead confirms extend order matches BOOT-034.
- Manual: firmware-update rehearsal on H-004.

#### Evidence
- none

### SEC-052 · Seal disk keys to TPM PCRs with a recovery key
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-053, SEC-025, SEC-017, BOOT-034
- Baseline: §51
- Threats: T-008, T-010
- Invariants: I-073

V2 FDE: optional PIN, PCR-bound unseal, and a recovery key the user already saved; no cloud escrow by default.

<!-- covers: GAP-0198 -->

#### Out of scope
Reseal across firmware (SEC-051). Pre-boot UI (BOOT). Cloud escrow (out).

#### Acceptance criteria
- [ ] On H-002, a PCR-bound unseal with optional PIN unlocks the volume without typing the disk passphrase.
- [ ] A PCR mismatch fails closed and the recovery key still unlocks.
- [ ] No network Capability is used during seal or unseal.
- [ ] `os inspect` shows seal policy and enrolled slots, not the volume key.

#### Verification
- Integration: `runtime:tests/sec/tpm_seal_*` on H-002 and H-004.
- Review: BOOT lead confirms PCR indices match BOOT-034.
- Manual: PIN-plus-TPM unlock on H-004.

#### Evidence
- none

### SEC-053 · Expose TPM only through a Capability-gated service
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-050, CAP-005
- Baseline: §9.1, §51
- Threats: T-016, T-020
- Invariants: I-074

No ambient TPM device node or shared TSS access. Userspace TPM use is a Capability-gated service.

<!-- covers: GAP-0192 -->

#### Out of scope
Seal policy (SEC-052). Event log inspect (SEC-065). Hardware enablement (HW).

#### Acceptance criteria
- [ ] A native Component without a TPM Capability cannot open the TPM; the Operation returns `Error::Rights` and allocates no handle.
- [ ] A holder of `Capability<Tpm>` can perform the sealed-key Operations the service exports and no others.
- [ ] Native software has no ambient TPM device node.
- [ ] Resource exhaustion of TPM sessions returns a typed exhaustion error (T-016).

#### Verification
- Unit: `runtime:tests/sec/tpm_service_*` on `qemu-x86_64` with swtpm and on `hw-h002`.
- Review: CAP lead confirms the object is typed in the registry.
- Fuzz: `runtime:fuzz/sec_tpm_service` one hour nightly without panic.

#### Evidence
- none

### SEC-054 · Enrol enterprise CAs into the trust store
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-016, SEC-015
- Baseline: §9.1, §63
- Threats: T-019
- Invariants: I-092

Enterprise-CA enrolment for public alpha, without directory login (a 1.0 non-goal). Pinning and personality mirrors apply to enrolled CAs.

#### Out of scope
Directory login (SEC-075). Native store (SEC-016). Personality mirrors (SEC-024).

#### Acceptance criteria
- [ ] An administrator enrolls an enterprise CA through Authorization; the CA appears in the typed store.
- [ ] A native TLS client with the store Capability trusts the enrolled CA for a test host.
- [ ] Enrolment requires elevation; a standard user receives `Error::Rights`.
- [ ] No LDAP, AD, Kerberos or SSO Interface is added.

#### Verification
- Integration: `runtime:tests/sec/enterprise_ca_*` on `qemu-x86_64`.
- Review: NET lead confirms handshake uses the enrolled CA.

#### Evidence
- none

### SEC-055 · Verify installer full-disk encryption and recovery key
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-017, SEC-025, INS-027, INS-007, SEC-052, SVC-023
- Baseline: §63
- Threats: T-008, T-010
- Invariants: I-073

V3 installer gate: FDE enabled on Tier 1, recovery key saved, no cloud escrow. INS owns the installer; SEC verifies the encryption contract.

#### Out of scope
Installer UI (INS-027). Encrypt-default Decision (INS-007). Dual-boot coexistence (INS).

#### Acceptance criteria
- [ ] A guided install on H-002 and H-004 produces an encrypted volume that unlocks with the saved recovery key.
- [ ] Opt-out of encryption is explicit when the INS Decision allows it; default-on is recorded in the install report.
- [ ] No cloud escrow endpoint is contacted during install.
- [ ] The recovery-key acknowledgment flag from SEC-025 is set before first login.

#### Verification
- Integration: INS installer QEMU matrix encrypted path plus H-002 and H-004 runs.
- Review: INS lead confirms the installer calls SEC unlock and recovery APIs.
- Demo: V3 encrypted install on a Tier 1 machine.

#### Evidence
- none

### SEC-056 · Support FIDO2 keys for disk unlock and login
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-014, SEC-018, SEC-020
- Baseline: §63
- Threats: T-010

V3 public alpha: FIDO2 hmac-secret as an additional disk-unlock and login factor on the pluggable authenticator Interface.

<!-- covers: GAP-0243 -->

#### Out of scope
WebAuthn platform authenticator (SEC-066). Greeter chrome (APP). Fingerprint plugin (SEC-057).

#### Acceptance criteria
- [ ] A FIDO2 authenticator plugin enrolls hmac-secret and unlocks a disk slot via SEC-018 on H-004.
- [ ] The same plugin authenticates a login Session.
- [ ] Unenroll requires Authorization; a standard user cannot unenroll another account's key.
- [ ] Absence of the key falls back to passphrase or recovery key without crashing the greeter.

#### Verification
- Integration: `runtime:tests/sec/fido2_*` on H-004 with a test token.
- Review: authenticator Interface remains the one defined by SEC-014.
- Manual: enroll, unlock, login, unenroll on H-004.

#### Evidence
- none

### SEC-057 · Add fingerprint login through pluggable authenticators
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-014, SEC-020, HW-063
- Baseline: §63
- Threats: T-010

GAP-0215 required fingerprint login at V3. Greeter UI is APP; SEC owns the authenticator plugin the greeter consumes.

#### Out of scope
Fingerprint hardware (HW-063). Greeter chrome (APP-030). FIDO2 (SEC-056).

#### Acceptance criteria
- [ ] The fingerprint plugin enrolls a template through the Authenticator Interface and authenticates a Session on a machine with a reader.
- [ ] Templates are stored as secrets, not as ambient files, and are not readable by other Components.
- [ ] A machine without a reader reports the plugin as unavailable in `os inspect` and does not fail login.
- [ ] Match never returns template bytes to the greeter.

#### Verification
- Integration: `runtime:tests/sec/fingerprint_*` on a Tier 1 laptop with a reader, skipped-as-unavailable otherwise.
- Review: HW lead confirms the Device object is Capability-gated.
- Manual: enroll and login on a reader-equipped H-ID.

#### Evidence
- none

### SEC-058 · Verify measured-boot Generation attestation
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-034, SEC-053, BOOT-043
- Baseline: §30, §51, §63
- Threats: T-008, T-022

V3 Secure Boot and measured-boot gate: generation identity is recorded in the TPM event log. BOOT measures; SEC verifies the security claim.

#### Out of scope
PCR extend implementation (BOOT-034). Event-log inspect API (SEC-065). Remote attestation (1.0 non-goal).

#### Acceptance criteria
- [ ] After boot on a Tier 1 machine, the TPM event log contains the booted SystemGeneration identity.
- [ ] A mismatched generation identity fails the verification Operation with a typed error.
- [ ] Remote attestation endpoints are not required for the check to pass.

#### Verification
- Integration: `runtime:tests/sec/generation_attest_*` on H-002 and H-004.
- Review: BOOT lead confirms the logged identity is the signed generation manifest.

#### Evidence
- none

### SEC-059 · Lock the Session with hardware-backed secrets
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-028, SEC-027, SEC-014, GFX-045
- Baseline: §32, §40, §63
- Threats: T-009
- Invariants: I-075

V3 scope: screen lock with hardware-backed secrets where available, using the secrets service and authenticators.

#### Out of scope
Lock chrome (APP-033). Compositor lock mode (GFX-045). FIDO2 enroll (SEC-056).

#### Acceptance criteria
- [ ] Locking the Session seals session secrets so they are not readable until unlock on H-004.
- [ ] Unlock through an enrolled hardware-backed authenticator restores secret reads.
- [ ] A compositor crash while locked restarts locked (I-075).
- [ ] Machines without hardware backing still lock using the password authenticator.

#### Verification
- Integration: `runtime:tests/sec/hw_lock_*` on H-004.
- Review: GFX lead confirms crash-while-locked still presents the lock Surface.

#### Evidence
- none

### SEC-060 · Implement multi-user login with isolated sessions
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: SEC-042, SEC-028, SEC-020, SEC-061, CAP-037
- Baseline: §9.1, §63
- Threats: T-026
- Invariants: I-021

V3 multi-user gate: two users with separate Sessions, Capability stores and encrypted data. Crosses identity, Session, secrets and storage. Split later if needed into session isolation, Capability-store isolation and data isolation matching SEC-061 and SEC-064.

#### Out of scope
Switcher chrome (APP-063). Supervision (SVC-039). State preservation (SEC-064). Enterprise directory (SEC-075).

#### Acceptance criteria
- [ ] Two local users can each create a Session; neither holds the other's Capability store.
- [ ] User A's secrets and per-user encrypted data Operations from User B's Session return `Error::Rights` and allocate no handle.
- [ ] Concurrent or switched sessions follow the option accepted by SEC-042.
- [ ] `os inspect` of User A's Session lists no objects owned by User B.
- [ ] Crash dumps from one Session do not contain the other's unlocked secrets (I-077).

#### Verification
- Integration: `runtime:tests/sec/multi_user_*` on H-002 and H-004.
- Review: CAP and STO leads confirm store and volume identity match the Session.
- Demo: V3 two-user isolated sessions on a Tier 1 machine.

#### Evidence
- none

### SEC-061 · Encrypt per-user data with keys unlocked at login
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-042, SEC-017, SEC-027, STO-039
- Baseline: §9.1, §26, §51
- Threats: T-010, T-026
- Invariants: I-073

Implements the multi-user-scope Decision: per-user home data encrypted under keys unlocked at login, required by the V3 two-user encrypted-data gate.

#### Out of scope
Session switching (SEC-060). Volume mechanism (SEC-017). ApplicationData layout (STO).

#### Acceptance criteria
- [ ] User A's home Collection is readable after A logs in and not readable from B's Session.
- [ ] Logging out A evicts A's home key; a subsequent read from a leftover handle returns a typed error.
- [ ] The wrapping scheme is the one named by SEC-042 and STO-039.
- [ ] Cryptographic erase of A's key does not destroy B's data.

#### Verification
- Integration: `runtime:tests/sec/home_keys_*` on H-002 and H-004.
- Review: STO lead confirms Collections map to the layered encryption Decision.

#### Evidence
- none

### SEC-062 · Add audit history and one-time grants to permissions UI
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-045, CAP-048, OBS-048, APP-029
- Baseline: §9.1, §49, §63
- Threats: T-001, T-025
- Invariants: I-021

V3 permissions UI v2: per-application audit history, one-time grants, and compatibility-application grants in the same UI as native apps. APP owns chrome.

<!-- covers: INV-1244, INV-0932 -->

#### Out of scope
One-time grant mechanics (CAP-048). Usage history store (OBS-048). Settings grant Surface (APP-029). Usability study (SEC-063).

#### Acceptance criteria
- [ ] Per-application history lists each grant and when it was last used.
- [ ] Converting a persistent grant to one-time is recorded; the next launch does not hold the grant after first use.
- [ ] A Linux-personality application's grants appear in the same list as a native application's.
- [ ] Revoke still takes effect on the next Operation.

#### Verification
- Integration: `runtime:tests/sec/permissions_ui_v2_*` on H-002 with a native app and a Linux-personality app.
- Review: LNX lead confirms personality grants are Component-scoped (T-025).

#### Evidence
- none

### SEC-063 · Run the permissions UI usability study
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-062
- Baseline: §58, §63

V3 gate: a user can see every grant and when it was used, revoke or convert to one-time. Study completion meets the threshold recorded on the V3 permissions gate.

#### Out of scope
UI implementation (SEC-062, APP). Research programme ownership (GOV, DOC).

#### Acceptance criteria
- [ ] A moderated study script asks participants to list grants, revoke one and convert one to one-time using only the shipping UI.
- [ ] The committed report records participant count, task completion against the V3 gate threshold, and every failed task.
- [ ] Findings that require product changes are filed as tasks with IDs; the report does not silently drop them.

#### Verification
- Review: APP and DOC leads sign off on the study report.
- Manual: report path is committed under `reports/` and cited by the V3 gate.

#### Evidence
- none

### SEC-064 · Preserve session state across user switches
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-060, SEC-028
- Baseline: §32, §63
- Threats: T-026

V3 multi-user gate: switching Sessions preserves state and does not leak Surfaces.

#### Out of scope
Switcher chrome (APP-063). Supervision (SVC). Compositor checkpoint (GFX).

#### Acceptance criteria
- [ ] Switching from User A to User B and back restores A's Session objects without requiring a new login.
- [ ] B cannot observe A's Surfaces, Capabilities or secrets during the switch; denied Operations return `Error::Rights`.
- [ ] A compositor restart during B's Session still restarts locked if A had locked, per I-075, without showing A's frames.

#### Verification
- Integration: `runtime:tests/sec/session_switch_*` on H-004.
- Review: GFX lead confirms Surfaces are not shared across Sessions.

#### Evidence
- none

### SEC-065 · Expose TPM event log and PCR state via os inspect
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-053, OBS-019, BOOT-034
- Baseline: §51, §64
- Threats: T-008
- Invariants: I-034

Local TPM event log and PCR inspection for understandability. Remote attestation is a documented 1.0 non-goal.

<!-- covers: GAP-0193 -->

#### Out of scope
Remote attestation (out). Generation attestation check (SEC-058). CLI rendering (SDK).

#### Acceptance criteria
- [ ] `os inspect` of the TPM service prints PCR values and event-log generation identity on H-002.
- [ ] A caller without Inspect or TPM Capability receives `Error::Rights` and no PCR bytes.
- [ ] The document attached to the inspect provider states that remote attestation is out of scope for 1.0.

#### Verification
- Integration: `runtime:tests/sec/tpm_inspect_*` on H-002 and H-004.
- Review: OBS lead confirms the provider registers through OBS-019.

#### Evidence
- none

### SEC-066 · Research a WebAuthn platform authenticator
- Type: spike
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-014, SEC-056, SEC-053
- Baseline: §63
- Threats: T-010

Spike whether the OS is a WebAuthn/passkey provider backed by TPM and biometrics, and what browser integration that needs, before a V4 provider.

<!-- covers: GAP-0226 -->

#### Out of scope
Provider implementation (SEC-073). Browser strategy (APP-019). FIDO2 disk unlock (SEC-056).

#### Acceptance criteria
- [ ] `reports/spikes/SEC-066.md` answers whether a platform authenticator ships, what TPM and biometric backing it needs, and what browser integration it requires.
- [ ] The report records a drop-the-provider option for SEC-073.
- [ ] The report does not freeze an ABI surface.

#### Verification
- Report: ship versus drop, TPM/biometric backing, browser integration, and 1.0 impact if dropped.
- Review: APP lead comments on browser integration.

#### Evidence
- none

### SEC-067 · Close High and Critical audit findings
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-070
- Baseline: §51, §63
- Risks: R-055

V4 gate: all High and Critical findings fixed. Medium findings are triaged separately.

#### Out of scope
Medium triage (SEC-068). Auditor re-verify (SEC-069). Component-isolation subset (CMP-053). Kernel subset (KRN).

#### Acceptance criteria
- [ ] Every High and Critical finding from SEC-070 has a fix with a regression test or a documented drop with `Dropped because`.
- [ ] Open High or Critical findings in the tracking list fail the V4 gate check.
- [ ] Fixes that change Capability enforcement run the isolation harness on `qemu-x86_64`.

#### Verification
- Integration: isolation harness plus finding-specific regression tests on `qemu-x86_64` and H-002.
- Review: SEC lead records each High and Critical ID as fixed or dropped.

#### Evidence
- none

### SEC-068 · Triage Medium audit findings with public tracking
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-070
- Baseline: §63
- Risks: R-055

V4 gate requires Medium findings triaged with public tracking, distinct from closing High and Critical code fixes.

#### Out of scope
Code fixes for High and Critical (SEC-067). Auditor letter (SEC-069).

#### Acceptance criteria
- [ ] Every Medium finding is listed publicly as fix, accept, or defer-past-1.0 with a named owner workstream.
- [ ] Deferred items become tasks with IDs; accepted items cite T-IDs they do not close.
- [ ] The list is linked from the published audit summary.

#### Verification
- Review: GOV and SEC leads sign off on the triage list.
- Manual: no Medium finding is absent from the list.

#### Evidence
- none

### SEC-069 · Re-verify High and Critical fixes with the auditor
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-067
- Baseline: §63
- Risks: R-055

V4 gate: High and Critical findings re-verified by the auditor, not only by the authors.

#### Out of scope
Fixes (SEC-067). Legal contract (SEC-070).

#### Acceptance criteria
- [ ] The auditor's re-verification letter names each High and Critical finding as closed or still open.
- [ ] Any still-open High or Critical finding fails this task's acceptance.
- [ ] The letter is stored as Evidence and linked from the published summary.

#### Verification
- Review: independent auditor sign-off recorded; Verified by is a human handle when the task is marked done.
- Manual: letter path or URL is valid Evidence grammar.

#### Evidence
- none

### SEC-070 · Commission an independent security audit
- Type: docs
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: SEC-072, SEC-002, GOV-024
- Baseline: §51, §63
- Risks: R-055

V4 gate: independent audit of Capability enforcement, native ABI and update pipeline; summary published. GOV GAP-0240 is the broader legal commissioning.

<!-- covers: GAP-0358, GAP-0240 -->

#### Out of scope
Legal-entity form (GOV-024). Closing findings (SEC-067). Personality-specific findings (LNX, WIN). Compositor findings (GFX).

#### Acceptance criteria
- [ ] The commissioned scope names Capability enforcement, native ABI and the update pipeline.
- [ ] A written summary is published; it does not claim certifications (I-091).
- [ ] Findings are ingested as tracked items with severity High, Critical or Medium.
- [ ] The audit cites the V0 threat model as the evaluation baseline.
- [ ] Review records GOV lead confirmation that the commissioning contract is recorded with the published summary.

#### Verification
- Review: GOV and SEC leads sign off on scope and published summary.
- Manual: published summary URL or path is Evidence.

#### Evidence
- none

### SEC-071 · Publish kernel-hardening and unsafe-isolation Evidence
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-034, KRN-056, SEC-010, KRN-029
- Baseline: §51
- Invariants: I-082

V4 gate: exploit mitigations enabled and measured, unsafe-code inventory published, unsafe authority minimized per §51. KRN and BLD produce the data; SEC publishes the security evidence pack.

#### Out of scope
Inventory generation (KRN-056). Hardening config (KRN-034). B-040 measurement (KRN-029).

#### Acceptance criteria
- [ ] The pack links the kernel unsafe inventory, the hardening config baseline and the B-040 reports for in-scope H-IDs.
- [ ] The pack states I-082 and that remaining `unsafe` blocks are justified.
- [ ] The pack contains no superiority claim and cites B-040 for cost.

#### Verification
- Review: KRN and BEN leads sign off that linked reports exist.
- Bench: B-040 reports cited are those produced by KRN-029.

#### Evidence
- none

### SEC-072 · Freeze the ambient-authority conformance suite
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-039, SEC-003, SEC-019, SEC-026
- Baseline: §9.1, §51
- Threats: T-001
- Invariants: I-021

V4 hardening gate: the §9.1 denial suite is complete, runs on every RC, and is part of the evidence pack. Required by V4-G04 (External security audit High and Critical closed).

#### Out of scope
Evidence pack publication (SEC-071). New Capability types after freeze (feature freeze is APP/ABI).

#### Acceptance criteria
- [ ] The suite includes filesystem, home, device, app-data, process enumeration, network, microphone, camera, hardware-id, secrets and screen-share denials.
- [ ] CI runs the suite on every RC image on `qemu-x86_64` and fails the RC if any case passes a denied Operation.
- [ ] The suite file list is an input to SEC-071.

#### Verification
- Integration: full suite on `qemu-x86_64` and H-002 as an RC job.
- Review: BLD lead confirms the RC gate wiring.

#### Evidence
- none

### SEC-073 · Ship a WebAuthn platform authenticator
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-066, SEC-014, SEC-053
- Baseline: §63
- Threats: T-010

A 1.0 desktop without a platform authenticator pushes users to third-party managers. Depends on the V3 spike; drop this task if the spike's accepted option is not to ship.

#### Out of scope
Spike (SEC-066). Browser embedding (APP, LNX). FIDO2 hmac-secret unlock (SEC-056).

#### Acceptance criteria
- [ ] The provider implements the option accepted by SEC-066 and registers as an Authenticator plugin.
- [ ] A passkey created on-device signs a WebAuthn challenge without exporting the private key to the calling Component.
- [ ] A Component without the authenticator Capability receives `Error::Rights` on create and get.
- [ ] If the spike report chose drop, this task is dropped in the same change with `Dropped because` citing the report.

#### Verification
- Integration: `runtime:tests/sec/webauthn_*` on H-004.
- Review: APP lead confirms browser integration matches the spike.
- Manual: spike report SEC-066 is cited on the pull request.

#### Evidence
- none

### SEC-074 · Declare formal certifications out of scope for 1.0
- Type: adr
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002, SEC-070
- Baseline: §51
- Decision: D-0271
- Invariants: I-091

Nongoal adr: do not pursue Common Criteria or FIPS 140 for 1.0. Certification cost would divert effort from the stability contract.

<!-- covers: GAP-0085 -->

#### Out of scope
External audit (SEC-070). FIPS-shaped crypto library choice (GOV).

#### Acceptance criteria
- [ ] Options evaluated include at least: pursue Common Criteria and FIPS 140 for 1.0; declare both out of scope for 1.0; pursue only one of the two.
- [ ] The accepted option is recorded as I-091 and listed in the published 1.0 non-promise list.
- [ ] Review records GOV lead sign-off on the pull request.

#### Verification
- Review: GOV lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options and names I-091.

#### Evidence
- none

### SEC-075 · Declare multi-seat, guest, kiosk, and enterprise directory out of scope
- Type: adr
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-014, SEC-012
- Baseline: §63
- Decision: D-0272
- Invariants: I-092

Nongoal adr: no multi-seat, guest, kiosk, LDAP/AD/Kerberos/SSO, Group Policy or fleet management for 1.0, while authenticators stay pluggable.

<!-- covers: GAP-0219, GAP-0441 -->

#### Out of scope
Authenticator plugins (SEC-014). Local multi-user (SEC-060). Fleet MDM (REL).

#### Acceptance criteria
- [ ] Options evaluated include at least: ship directory login and multi-seat for 1.0; declare multi-seat, guest, kiosk and directory out while keeping authenticators pluggable; ship kiosk only.
- [ ] The accepted option is recorded as I-092 and states that the authenticator Interface remains pluggable.
- [ ] Review records GOV and APP lead sign-off on the pull request.

#### Verification
- Review: GOV and APP leads sign off on the pull request.
- Manual: decision file lists at least two options and names I-092.

#### Evidence
- none

### SEC-076 · Verify 1.0 security-posture claims against the threat model
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-077, SEC-069, SEC-072, SEC-055, SEC-071, BLD-041, BLD-073
- Baseline: §9.1, §51, §63
- Invariants: I-021, I-073

1.0 definition security posture: no ambient authority, permissions UI, FDE, Secure Boot and measured boot on Tier 1, public threat model, external audit closed, continuous fuzzing, reproducible builds.

#### Out of scope
CVE SLA operation (REL). HCL publication (REL). Support window (GOV, REL).

#### Acceptance criteria
- [ ] A checklist maps each 1.0 security-posture claim to a done task ID and a T-ID.
- [ ] Ambient-authority, permissions UI, FDE, Secure Boot/measured boot, threat model, audit closure, fuzzing and reproducible builds each have a passing Evidence line on the cited task.
- [ ] The checklist is committed and linked from the 1.0 milestone review.

#### Verification
- Review: SEC, BOOT, BLD and REL leads sign off on the checklist.
- Manual: every claim cites a done task, not prose.

#### Evidence
- none

### SEC-077 · Refresh the public threat model for 1.0
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002, SEC-069, SEC-070
- Baseline: §9, §51, §63
- Risks: R-008

1.0 definition requires a public threat model. Refresh the V0 document against shipped surfaces and closed audit findings.

#### Out of scope
V0 chassis (SEC-002). New T-IDs that belong in `registers/threats.md` (add them there in the same change if needed).

#### Acceptance criteria
- [ ] The refreshed document lists shipped L2 surfaces that the V0 document did not name and maps each to T-IDs.
- [ ] Closed High and Critical findings are marked addressed; open Medium items cite SEC-068.
- [ ] The public copy matches the in-tree document besides formatting.

#### Verification
- Review: CAP, BOOT and ABI leads sign off on the refresh.
- Manual: every CAP/SEC/BOOT adr still cites T-IDs that exist.

#### Evidence
- none

### SEC-078 · Enable secure-enclave Components if the Spike recommends
- Type: build
- Milestone: LATER
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-035, SEC-037
- Baseline: §8
- Invariants: I-058

1.0 explicitly does not promise hardware confidentiality. Enablement waits on SEC-035 and stays off the 1.0 critical path. Drop this task if the spike's accepted option is not to enable. Required by the SEC scope: "hardware-enforcement hooks and enclave research".

#### Out of scope
Spike (SEC-035). CHERI (CAP). 1.0 gates (none).

#### Acceptance criteria
- [ ] If the spike recommends enablement, a Component can be started in the named enclave class and `os inspect` reports the enclave identity.
- [ ] Application-visible Capabilities do not change when the enclave path is enabled.
- [ ] If the spike recommends against enablement, this task is dropped in the same change with `Dropped because` citing the report.

#### Verification
- Integration: enclave Component start on hardware the spike named, or drop evidence.
- Review: CAP lead confirms no L1 break.
- Report: cites `reports/spikes/SEC-035.md`.

#### Evidence
- none

### SEC-079 · Decide remote-Interface Capability, identity, and encryption rules
- Type: adr
- Milestone: LATER
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-002, CAP-047
- Baseline: §43, §57
- Decision: D-0274
- Threats: T-019
- Invariants: I-047

Distribution is parked at LATER. When remote transports exist they must honor Capabilities, endpoint identity, encryption and explicit user policy, and must not become a kernel concern.

<!-- covers: INV-0809, INV-0810, INV-0811, INV-0814 -->

#### Out of scope
Transport implementation (IPC, NET). Cross-machine unforgeability encoding (CAP-047). Making the kernel distributed (forbidden, §57).

#### Acceptance criteria
- [ ] Options evaluated include at least: Capability-honoring encrypted transport with explicit user policy in userspace; keep remote out until a later major version; kernel-mediated remote (recorded as rejected under §57 if proposed).
- [ ] The accepted option states how Capabilities, endpoint identity, encryption and user policy are honored, and that the kernel is not a distributed system.
- [ ] Each option cites T-019 and I-047.
- [ ] Review records IPC and CAP lead sign-off on the pull request.

#### Verification
- Review: IPC and CAP leads sign off on the pull request.
- Manual: decision file lists at least two options and names I-047.

#### Evidence
- none
