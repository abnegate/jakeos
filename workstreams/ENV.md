# ENV · Native development environments
- Prefix: ENV
- Lead: none
- Baseline: §35, §36

<!-- roadmap:generated:begin summary -->
Tasks: 36 live, 0 done, 0 in-progress, 36 todo, 0 dropped. Ready: 1. Blocked: 35. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

ENV owns development isolation as a native OS primitive (§35): the environment.yaml project definition (runtime, services and resources), the DevelopmentEnvironment object composed from a ResourceDomain, StorageSnapshot, CapabilityNamespace, NetworkNamespace and Components, and the object API that creates, rebuilds, destroys, caches and restores those environments. Named runtimes resolve to immutable Packages from the content-addressed store and map directly, with no image extraction. Declared services start as isolated Components inside the environment, including Linux-personality software hosted by LNX. Service endpoints are granted without ambient network authority. Secrets enter as Capabilities. Environment changes emit history events. The native Terminal and Editor attach through those Capabilities.

A V0 lint forbids native software from depending on an OCI runtime, a Docker daemon, a Linux VM or Docker Desktop; OCI remains a Linux-personality concern (§36). V1 ships the php-postgres-redis reference stack and the self-host environment.yaml so developers build the OS inside `os env`. Later rungs add compose and devcontainer converters, a service catalogue, multi-user isolation, a conformance suite, Layer 2 schema lock and workspace restore.

## Out of scope

`os env` list, enter, leave, destroy and rebuild CLI verbs (SDK). StorageSnapshot primitive, copy-on-write clones and the hash-keyed cache substrate (STO). ResourceDomain budgets and exhaustion policy (SCH). Capability discovery namespace (CAP). NetworkConnection objects and the network-namespace primitive (NET). Linux-personality runtime and OCI host (LNX). Package store, lock identity, history log and `os history` (PKG). Secrets service (SEC). Inspect transport and CLI rendering (OBS, SDK). Terminal and Editor applications (APP). Semantic registry, automation rules and AI broker (SEM). B-025 harness and publication (BEN). Toolchain, image build and hermetic CI (BLD). Docs site (DOC). Repository signing and third-party review (REL). Virtual machines as a fallback (VIRT). Wasm component hosting (WASM). Windows personality (WIN).

## Tasks

### ENV-001 · Forbid OCI, Docker and VM dependencies in native software
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-011
- Baseline: §35, §36
- Invariants: I-019, I-043

Native isolation is cheaper and more fundamental than containers (§36). This standing pre-merge lint collapses the §35 nongoals so no native crate or service depends on an OCI runtime, a Docker daemon, a Linux VM or Docker Desktop before any DevelopmentEnvironment ships. Linux-personality OCI support stays in LNX.

<!-- covers: INV-0672, INV-0645, INV-0646, INV-0649 -->

#### Out of scope
OCI runtime inside the Linux personality (LNX). Docker socket compatibility question (Q-032). DevelopmentEnvironment object (ENV-013).

#### Acceptance criteria
- [ ] Pre-merge CI fails a native workspace crate whose `cargo metadata` lists docker, containerd, runc, crun, podman or bollard as a dependency.
- [ ] Pre-merge CI fails a native crate manifest or source tree that references Docker Desktop or a Linux VM manager as a required host.
- [ ] The LNX personality crate may depend on an OCI runtime and is named on the lint allowlist; native crates are not.
- [ ] The lint job is a required check of BLD-011 on CI matrix entry `qemu-x86_64`.

#### Verification
- Unit: `env:tests/lint_native_oci_*` on `qemu-x86_64`.
- Integration: a fixture native crate with a docker dependency fails the pre-merge lint; the same fixture under the LNX allowlist path does not.
- Review: LNX lead confirms the allowlist names only personality crates.

#### Evidence
- none

### ENV-002 · Create a CapabilityNamespace for each development environment
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-023, ENV-007, ENV-010
- Baseline: §9.1, §35
- Threats: T-001
- Invariants: I-021

Environment creation scopes which Capabilities Components inside the environment may hold (§35). CAP owns the discovery namespace; ENV instantiates one per DevelopmentEnvironment at create so a service Component cannot obtain a grant the environment did not name.

<!-- covers: INV-0658 -->

#### Out of scope
Discovery mechanism (CAP-023). Endpoint grants (ENV-006). Secret injection (ENV-015).

#### Acceptance criteria
- [ ] Creating an environment creates exactly one CapabilityNamespace owned by that environment, visible in `os inspect`.
- [ ] A Component inside the environment that looks up a Capability not named by the namespace receives `Error::Rights` and allocates no handle.
- [ ] Destroying the environment drops the namespace; a stale handle then fails within one Operation.

#### Verification
- Unit: `env:tests/capability_namespace_*` on `qemu-x86_64`.
- Integration: php-postgres-redis create path on `qemu-x86_64` and `hw-h002`.
- Review: CAP lead confirms the namespace is the CAP-023 mechanism, not a second broker.

#### Evidence
- none

### ENV-003 · Create a NetworkNamespace for each development environment
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-002, ENV-006, ENV-010, NET-012, NET-016, SCH-033
- Baseline: §9.1, §23, §35
- Threats: T-001, T-002
- Invariants: I-021

Environment creation isolates service ports and network policy (§35). NET owns NetworkConnection and the namespace primitive; ENV applies ENV-006 so declared services are reachable from the developer's shell, IDE and browser without ambient network authority.

<!-- covers: INV-0659 -->

#### Out of scope
NetworkConnection object and retained TCP/IP (NET). ResourceDomain network policy field (SCH-033). Terminal and Editor attach (ENV-024).

#### Acceptance criteria
- [ ] Creating an environment creates one NetworkNamespace; Components inside it cannot open a connection that the endpoint grant did not name (`Error::Rights`, no handle).
- [ ] Each declared service is reachable from an entered shell through the Capability shape accepted by ENV-006.
- [ ] A Component outside the environment that holds no endpoint Capability cannot connect to those service ports (`Error::Rights`).
- [ ] `os inspect` lists the namespace, declared ports and held endpoint Capabilities.

#### Verification
- Unit: `env:tests/network_namespace_*` on `qemu-x86_64`.
- Integration: postgres and redis reachability from an entered shell on `qemu-x86_64` and `hw-h002`.
- Review: NET lead confirms no ambient listen or connect right is minted.

#### Evidence
- none

### ENV-004 · Map environment resources onto a ResourceDomain
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-007, ENV-010, SCH-014, SCH-006, SCH-019, SCH-032, SCH-007, SCH-008, SCH-034
- Baseline: §23, §35
- Threats: T-016
- Invariants: I-033

Environment creation creates a ResourceDomain and maps environment.yaml `resources` (memory, cpu) onto SCH budgets (§23, §35). Exceeding a budget returns a typed error and does not start further Components.

<!-- covers: INV-0653, INV-0656 -->

#### Out of scope
Exhaustion policy choice (SCH-016). GPU budget (SCH-031). Service Component spawn (ENV-017).

#### Acceptance criteria
- [ ] Creating an environment from a definition with memory and cpu resource fields creates one ResourceDomain whose budgets match those fields, visible in `os inspect`.
- [ ] Every Component started inside the environment is a member of that ResourceDomain and of no other.
- [ ] A start that would exceed the domain memory budget returns the SCH typed exhaustion error and starts no further Component.
- [ ] Destroying the environment tears the domain down and reclaims its accounting.

#### Verification
- Unit: `env:tests/resource_domain_*` on `qemu-x86_64`.
- Integration: budget-hit on `qemu-x86_64` and `hw-h002` leaves no extra live Component in `os inspect`.
- Review: SCH lead confirms budgets are SCH ResourceDomain fields, not a parallel ENV counter.

#### Evidence
- none

### ENV-005 · Create a StorageSnapshot for each development environment
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-007, ENV-010, STO-011, STO-043
- Baseline: §26, §35
- Invariants: I-043, I-044

Cached enter needs a copy-on-write view of project and dependency storage (§26, §35). ENV creates the snapshot; STO owns the primitive. The enter path uses that snapshot, not an overlay filesystem stack.

<!-- covers: INV-0657, INV-0647 -->

#### Out of scope
StorageSnapshot primitive (STO-043). Overlayfs inside the Linux personality (LNX). Environment cache of warm snapshots (ENV-014).

#### Acceptance criteria
- [ ] Creating an environment creates one StorageSnapshot of the project and resolved dependency storage that does not mutate the source Directory.
- [ ] The create path uses STO snapshot and clone Operations; a filesystem-diff test finds no overlay mount or overlayfs module load for that path.
- [ ] Writes inside the snapshot are visible to Components in the environment and invisible in the source.
- [ ] Destroying the environment drops the snapshot and leaves the source intact.

#### Verification
- Unit: `env:tests/storage_snapshot_*` on `qemu-x86_64`.
- Integration: create-write-destroy on `qemu-x86_64` and `hw-h002` with source checksum unchanged.
- Review: STO lead confirms no overlay stack is used.

#### Evidence
- none

### ENV-006 · Decide how environment endpoints are granted without ambient network
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ENV-007, NET-006, SEC-002, Q-031
- Baseline: §9, §9.1, §35
- Decision: D-0072
- Threats: T-001, T-002
- Invariants: I-021

Answers how the environment NetworkNamespace exposes postgres and redis (and later catalogue services) to the developer's shell, IDE and browser without granting ambient network authority (§9.1, §35). NET owns NetworkConnection rights; this adr names the grant shape ENV mints per declared service.

<!-- covers: INV-0665 -->

#### Out of scope
NetworkConnection implementation (NET-014). Namespace create (ENV-003). Docker socket compatibility (Q-032).

#### Acceptance criteria
- [ ] Options evaluated include at least: (A) per-service loopback Endpoint Capabilities; (B) named `Capability<NetworkConnection>` filtered to declared ports; (C) a single shared environment network grant.
- [ ] The accepted option states that a Component without the named grant cannot connect or listen, and that the grant is not ambient network authority (T-001, I-021).
- [ ] Each rejected option records whether it reintroduces a confused deputy (T-002) or a wildcard connect right.
- [ ] Review records NET and SEC lead sign-off on the pull request.

#### Verification
- Review: NET lead and SEC lead sign-off recorded on the pull request that accepts the decision file.
- Manual: decision file lists at least two options with consequences and names T-001, T-002 and I-021.

#### Evidence
- none

### ENV-007 · Decide whether DevelopmentEnvironment is kernel or userspace
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: CMP-005, SCH-007
- Baseline: §35, §65
- Decision: D-0073
- Invariants: I-055

V1 composes ResourceDomain, StorageSnapshot, CapabilityNamespace, NetworkNamespace and Components into development isolation (§35). This adr decides whether that composition is a userspace supervisor Component, a Package-profile instantiation with no extra object, or a new Layer 1 kernel object. §65 prefers high-level semantics in userspace and a minimal kernel ABI; a new L1 surface would need a spike and could not freeze before V4 (I-040).

#### Out of scope
Schema of environment.yaml (ENV-008). Service hosting path (ENV-009). Implementation of the chosen object (ENV-013). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] Options evaluated include at least: (A) userspace supervisor Component; (B) Package-profile instantiation with no extra object; (C) new Layer 1 kernel object.
- [ ] The accepted option names the object that `os inspect` will list, which existing primitives it composes, and whether any new L1 surface is introduced.
- [ ] If a new L1 surface is accepted, the decision names that it stays prototyped through V1 and records the missing spike as a follow-up; otherwise it names that no new L1 surface exists.
- [ ] Review records ABI and CMP lead sign-off on the pull request.

#### Verification
- Review: ABI lead and CMP lead sign-off recorded on the pull request that accepts the decision file.
- Manual: decision file lists at least two options, cites §65 rules 1 and 2, and names I-055.

#### Evidence
- none

### ENV-008 · Decide environment.yaml schema versus Package manifest profile
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-006, PKG-011, Q-030, ENV-021
- Baseline: §28, §29, §35
- Decision: D-0074
- Invariants: I-036

V1 `os env` needs a canonical project definition (§35). This single adr chooses whether environment.yaml is a distinct schema or a profile of the Package manifest (§28), and for the accepted shape records the source format, lockfile location, version-specifier locking (`php: 8.6`) and how service Packages are discovered. The Decision lists ABI surface S-021 in prototyped state and does not freeze it.

<!-- covers: INV-0663, INV-0664 -->

#### Out of scope
Parser and validation (ENV-010). Package manifest schema (PKG-011). Service hosting (ENV-009). Layer 2 version lock (ENV-034).

#### Acceptance criteria
- [ ] Options evaluated include at least: (A) distinct YAML environment.yaml plus a sibling lockfile with repository name discovery; (B) a profile of the Package manifest in the typed manifest format, lock recorded as Package Dependencies; (C) a distinct typed (non-YAML) environment manifest plus lockfile.
- [ ] The accepted option names the canonical source format, the lockfile path relative to the project, how `php: 8.6` is pinned to a content-addressed Package, and how postgres and redis Packages are discovered.
- [ ] The accepted option states that identical definition plus lock yields an identical immutable Component Package set (I-036) and lists S-021 as prototyped, not frozen.
- [ ] Review records PKG and SDK lead sign-off on the pull request.

#### Verification
- Review: PKG lead and SDK lead sign-off recorded on the pull request that accepts the decision file.
- Manual: decision file lists at least two complete options covering schema, format, lock location and discovery, and names S-021.

#### Evidence
- none

### ENV-009 · Decide how environment services are hosted and packaged
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ENV-022, PKG-047
- Baseline: §3, §35, §36
- Decision: D-0075
- Risks: R-026
- Invariants: I-019, I-043

V1 `os env` must reach Postgres and Redis. Most developer services exist only as Linux software, so the native environment story depends on this compatibility choice (§35, §36). Options are taken from the ENV-022 report. Native software still never sees POSIX; Linux-personality software stays inside LNX.

<!-- covers: GAP-0552 -->

#### Out of scope
Measurement (ENV-022). Instantiation (ENV-017). Personality host (LNX-032). OCI layer import (PKG-058).

#### Acceptance criteria
- [ ] Options evaluated include at least: (A) Linux-personality OCI images inside native isolation; (B) native Packages; (C) both, with a documented default for postgres and redis.
- [ ] The accepted option cites the spike report's B-025 cached and cold columns per path and names the default for the V1 reference stack.
- [ ] The accepted option states that the chosen path uses no Docker daemon, Linux VM or overlay filesystem on the native enter path (I-043) and that no native subsystem depends on an OCI runtime (I-019).
- [ ] Review records LNX and PKG lead sign-off on the pull request.

#### Verification
- Review: LNX lead and PKG lead sign-off recorded on the pull request that accepts the decision file.
- Report: `reports/spikes/ENV-022.md` is cited as evidence in the decision options.

#### Evidence
- none

### ENV-010 · Implement the environment.yaml schema with lock and validation
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-008
- Baseline: §27, §29, §35
- Invariants: I-036

Parser and schema for the runtime, services and resources sections of the project definition accepted by ENV-008 (§35). Identical definition plus lock yields identical immutable Component Package identity sets. Parser fuzz of untrusted input stays BLD-042.

<!-- covers: INV-0650, INV-0668 -->

#### Out of scope
Parser fuzz harness (BLD-042). Runtime resolution (ENV-019). Package manifest schema (PKG-031). Layer 2 lock (ENV-034).

#### Acceptance criteria
- [ ] A valid definition with runtime, services and resources sections plus lock parses into a typed value whose Package identity set is a function of definition plus lock only.
- [ ] Two parses of identical definition plus lock on `qemu-x86_64` produce byte-identical Package identity sets.
- [ ] A missing required section, an unknown field or a lock that disagrees with the definition returns a typed error naming the field and allocates no environment.
- [ ] The schema implementation matches the format and lock location accepted by ENV-008.

#### Verification
- Unit: `env:tests/schema_parse_*` on `qemu-x86_64`.
- Integration: php-postgres-redis definition plus lock round-trip on `qemu-x86_64` and `hw-h002`.
- Review: PKG lead confirms lock identity uses the store content hash, not a mutable name.

#### Evidence
- none

### ENV-011 · Emit development environment changes as history events
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ENV-012, PKG-022
- Baseline: §31, §35

V1 `os history` lists environment events such as Changed project environment (§31). ENV emits typed payloads on create, rebuild and destroy; PKG owns the log. Restore of environments is V2.

<!-- covers: INV-0575 -->

#### Out of scope
History log and `os history` CLI (PKG-022, PKG-059). Event type slots in the log schema (PKG-053). Restore (ENV-028).

#### Acceptance criteria
- [ ] Create, rebuild and destroy each append one typed history event with environment identity and definition hash.
- [ ] The PKG log API returns those events in append order for that environment identity.
- [ ] A caller without the history-append Capability receives `Error::Rights` and the log is unchanged.

#### Verification
- Unit: `env:tests/history_events_*` on `qemu-x86_64`.
- Integration: create-rebuild-destroy sequence appears in the PKG log on `qemu-x86_64`.
- Review: PKG lead confirms payload fields match the V1 event type slots.

#### Evidence
- none

### ENV-012 · Implement development environment create, rebuild and destroy
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-013, ENV-014, ENV-017
- Baseline: §35
- Invariants: I-043

Object operations for create, rebuild and destroy: snapshot drop, cache drop and Component teardown (§35). SDK-042 owns list, enter, leave, destroy and rebuild CLI verbs; this is the ENV object API those verbs call.

#### Out of scope
CLI verbs (SDK-041, SDK-042). Rebuild trigger on file change (ENV-027). History emit (ENV-011).

#### Acceptance criteria
- [ ] Create from a valid definition plus lock returns a DevelopmentEnvironment that `os inspect` lists with domain, snapshot, namespaces and Components.
- [ ] Rebuild tears down service Components, recreates the snapshot and cache entries, and starts the resolved set; the previous snapshot remains until destroy or restore.
- [ ] Destroy drops snapshot, cache entries and Components; `os inspect` no longer lists the environment and a subsequent enter returns a typed not-found error.
- [ ] Create and rebuild use no Docker daemon, Linux VM or overlay filesystem (I-043).

#### Verification
- Unit: `env:tests/lifecycle_*` on `qemu-x86_64`.
- Integration: create-rebuild-destroy on `qemu-x86_64` and `hw-h002` with leak check via `os inspect`.
- Review: SDK lead confirms CLI verbs call only this object API.

#### Evidence
- none

### ENV-013 · Implement DevelopmentEnvironment as composed native primitives
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-005, ENV-002, ENV-003, ENV-004, ENV-005, ENV-007
- Baseline: §10, §35
- Invariants: I-019, I-043

Development isolation is a native OS object, not a userspace container tool (§35). This task implements the object chosen by ENV-007 as a composition of ResourceDomain, StorageSnapshot, CapabilityNamespace, NetworkNamespace and Components. CLI verbs stay SDK.

<!-- covers: INV-0644 -->

#### Out of scope
CLI verbs (SDK-041, SDK-042). Service instantiation (ENV-017). Inspect records (ENV-016).

#### Acceptance criteria
- [ ] A DevelopmentEnvironment is enumerable through `os inspect` as the object kind named by ENV-007.
- [ ] The object holds Capabilities to one ResourceDomain, one StorageSnapshot, one CapabilityNamespace and one NetworkNamespace created for it.
- [ ] Creating the object does not start a Docker daemon, Linux VM or overlay filesystem, and native crates on its path still pass ENV-001.
- [ ] Destroying the object tears down those four primitives and any member Components, with kernel and userspace object counts returned to the pre-create baseline in `os inspect`.

#### Verification
- Unit: `env:tests/dev_environment_object_*` on `qemu-x86_64`.
- Integration: create-inspect-destroy on `qemu-x86_64` and `hw-h002`.
- Review: ABI lead confirms the placement matches ENV-007 and introduces no unfrozen L1 surface.

#### Evidence
- none

### ENV-014 · Implement the development environment cache for warm enter
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-040, ENV-005, ENV-013, ENV-019, STO-044
- Baseline: §27, §34, §35
- Benchmarks: B-025
- Invariants: I-039

Pre-resolves Packages, pre-creates StorageSnapshots and keeps warm Component images so a second enter reuses them (§34, §35). Measurement and the docker-compose baseline stay BEN-025. This cache is the ENV mechanism B-025 cached enter uses.

<!-- covers: INV-0662 -->

#### Out of scope
B-025 harness and publication (BEN-025). Hash-keyed store API (STO-044). Build-output cache (ENV-023).

#### Acceptance criteria
- [ ] After a first successful create, a second enter of the same definition plus lock reuses the resolved Package set, StorageSnapshot and warm Component images, recorded as cache hits in `os inspect`.
- [ ] Changing the lock invalidates those hits and forces resolve and snapshot create; the miss is visible in `os inspect`.
- [ ] Cache objects are content-addressed through STO-044 and are not overlay layers or extracted images.
- [ ] Destroy drops the environment's pins; unreferenced cache objects become eligible for STO GC.

#### Verification
- Unit: `env:tests/env_cache_*` on `qemu-x86_64`.
- Integration: two enters of php-postgres-redis on `qemu-x86_64` and `hw-h002` with hit/miss counters.
- Bench: B-025 cached path on H-001 and H-002 consumes this cache; target per register.

#### Evidence
- none

### ENV-015 · Inject service secrets as Capabilities into environment Components
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ENV-002, ENV-007, SEC-027
- Baseline: §9, §9.1, §35
- Threats: T-001
- Invariants: I-021

Postgres and Redis (and later catalogue services) need credentials without ambient authority (§9.1). ENV wires SEC secrets into service Components as Capabilities. Secret files are not written into the project snapshot by default.

#### Out of scope
Secrets service (SEC-027). Isolation suite across applications (SEC-026). Instantiation (ENV-017).

#### Acceptance criteria
- [ ] Starting a declared service Component grants it a secret Capability from SEC-027 and no filesystem Capability to the project snapshot that contains the secret bytes.
- [ ] A neighboring Component in the same environment that was not granted that secret receives `Error::Rights` and allocates no handle.
- [ ] `os inspect` lists the secret Capability on the service Component and does not print secret bytes.

#### Verification
- Unit: `env:tests/service_secrets_*` on `qemu-x86_64`.
- Integration: postgres start on `qemu-x86_64` authenticates with the injected Capability; a sibling Component cannot read it.
- Review: SEC lead confirms no secret file is created in the snapshot by default.

#### Evidence
- none

### ENV-016 · Expose DevelopmentEnvironment state through os inspect
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ENV-013, OBS-019, SDK-007
- Baseline: §24, §35, §64
- Invariants: I-034

Every primitive ships with inspect data in the same change (§24, §64). ENV owns DevelopmentEnvironment records (domain, snapshot, namespaces, Components, endpoints); SDK owns the `os inspect` CLI.

#### Out of scope
Inspect CLI rendering (SDK-007). Provider registry (OBS-019). Audit log (OBS).

#### Acceptance criteria
- [ ] `os inspect` on a live environment prints ResourceDomain, StorageSnapshot, CapabilityNamespace, NetworkNamespace, member Components and granted endpoints.
- [ ] After destroy, that identity is absent from enumeration.
- [ ] A caller without inspect rights receives `Error::Rights` and sees no environment records.

#### Verification
- Unit: `env:tests/inspect_environment_*` on `qemu-x86_64`.
- Integration: inspect during php-postgres-redis enter on `qemu-x86_64` and `hw-h002`.
- Review: OBS lead confirms the provider registers through OBS-019.

#### Evidence
- none

### ENV-017 · Instantiate environment services as isolated Components
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: CMP-017, ENV-009, ENV-013, ENV-015, ENV-019, LNX-032, PKG-058
- Baseline: §3, §10, §35, §36
- Threats: T-011
- Invariants: I-019, I-025, I-043

V1 enter with Postgres and Redis reachable. Instantiates runtime and services as Components inside the environment ResourceDomain and namespaces, including Linux-personality software per ENV-009 (§35, §36). LNX-032 is the personality host; this task is the ENV consumer. Native software still never sees POSIX.

<!-- covers: INV-0652, INV-0660, INV-0667 -->

#### Out of scope
Personality host (LNX-032). OCI import (PKG-058). Endpoint grant shape (ENV-003). Reference definition file (ENV-018).

#### Acceptance criteria
- [ ] A definition that names php, postgres and redis starts one Component per runtime or service entry inside the environment ResourceDomain and namespaces.
- [ ] Postgres and Redis accept connections through the granted endpoint Capabilities from an entered shell on `qemu-x86_64` and `hw-h002`.
- [ ] A Linux-personality service cannot reach a resource its enclosing Component was not granted (T-011); the denial is `Error::Rights`.
- [ ] Instantiation follows ENV-009 and uses no Docker daemon, Linux VM or overlay filesystem on the native path.
- [ ] Native crates on this path still pass ENV-001; OCI libraries appear only in allowlisted LNX crates.
- [ ] Failure to start one service returns a typed error, starts no later service, and leaves no leaked Component in `os inspect`.

#### Verification
- Unit: `env:tests/instantiate_services_*` on `qemu-x86_64`.
- Integration: php-postgres-redis start and endpoint ping on `qemu-x86_64` and `hw-h002`.
- Review: LNX lead confirms personality services are enclosed by the environment Component and cannot escape it.

#### Evidence
- none

### ENV-018 · Ship the php-postgres-redis reference environment.yaml
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-011, ENV-012, ENV-016, ENV-017
- Baseline: §35, §61
- Benchmarks: B-025
- Invariants: I-043

V1 exit and demo: `os env enter` on a cached environment presents a working shell with Postgres and Redis reachable (§35, §61). This is the B-025 workload and the permanent V1 acceptance test. Latency is gated by B-025, not here.

<!-- covers: INV-0669, INV-1202 -->

#### Out of scope
B-025 harness (BEN-025). CLI enter implementation (SDK-041). Self-host environment.yaml (ENV-020).

#### Acceptance criteria
- [ ] The tree ships a php-postgres-redis environment.yaml plus lock that ENV-010 accepts.
- [ ] `os env enter` on a cached instance of that environment presents a shell from which Postgres and Redis answer a ping through granted endpoint Capabilities, on `qemu-x86_64` and `hw-h002`.
- [ ] `os inspect` lists the environment's ResourceDomain, snapshot, namespaces, Components and endpoints during that enter.
- [ ] The enter path uses no Docker daemon, Linux VM or overlay filesystem.

#### Verification
- Integration: `env:tests/reference_php_stack_*` on `qemu-x86_64` and `hw-h002`.
- Demo: V1-G04 cached enter with postgres and redis reachable on H-002.
- Bench: B-025 on H-001 and H-002 uses this stack; target per register.

#### Evidence
- none

### ENV-019 · Resolve environment.yaml runtimes to immutable Packages
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-017, ENV-010, PKG-037, PKG-038
- Baseline: §27, §29, §34, §35
- Invariants: I-036, I-039, I-043

The runtime section (`php: 8.6`) resolves named versions to content-addressed Packages (§27, §29) and maps them directly (§34). No image extraction. PKG owns the store; ENV owns the resolver.

<!-- covers: INV-0651, INV-0648 -->

#### Out of scope
Content store (PKG-038). Address-space mapping (CMP-017). Service instantiation (ENV-017). Catalogue index (ENV-026).

#### Acceptance criteria
- [ ] Resolving `php: 8.6` against the lock returns a content-addressed Package identity that PKG can map; no tar or OCI layer is extracted on disk.
- [ ] A second resolve of the same lock returns the same identity without fetching.
- [ ] An unknown runtime name or a lock pin missing from the store returns a typed error and allocates no environment.
- [ ] Mapped runtime objects are immutable Package objects (I-039); a write through the mapping returns `Error::Rights`.

#### Verification
- Unit: `env:tests/resolve_runtime_*` on `qemu-x86_64`.
- Integration: php runtime resolve-and-map on `qemu-x86_64` and `hw-h002` with store GC roots pinned.
- Review: PKG lead confirms identity is the store content hash.

#### Evidence
- none

### ENV-020 · Provide the self-host environment.yaml for building the OS
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-043, ENV-012, ENV-001, ENV-019, SDK-041
- Baseline: §35, §50, §61
- Risks: R-026

V1 exit: a clean checkout builds the full system image on the OS inside `os env` (§61). ENV ships the tree's environment.yaml and enter path; BLD owns the toolchain and image build (B-039).

#### Out of scope
Toolchain and image build (BLD-043). Bit-for-bit rebuild CI (BLD-041). B-039 publication (BEN). php-postgres-redis reference (ENV-018).

#### Acceptance criteria
- [ ] The OS repository contains an environment.yaml plus lock that creates and enters on H-002.
- [ ] A clean checkout inside that environment invokes the BLD image-build path and produces an image that boots on H-001 and H-002.
- [ ] Native crates used in that environment still pass ENV-001.
- [ ] `os inspect` shows the self-host environment's ResourceDomain and member Components during the build.

#### Verification
- Integration: clean-checkout enter-and-build on `hw-h002`; produced image boots on `qemu-x86_64`.
- Demo: V1-D01 clone, edit, build inside `os env` on H-002.
- Review: BLD lead confirms this definition is the sandbox BLD-043 uses.

#### Evidence
- none

### ENV-021 · Prototype environment.yaml versus Package-manifest profiles
- Type: spike
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §35, §36
- Explores: S-021

Prototype environment.yaml as a development-environment declaration (packages, services, ResourceDomain, storage snapshot, capability and network namespaces) so ENV-008 is not a paper Decision (§35, §36). Surface S-021 remains open.

#### Out of scope
The Decision (ENV-008). Freeze of S-021 (ENV-034). `os env` CLI (SDK-041).

#### Acceptance criteria
- [ ] A prototype environment.yaml enters a ResourceDomain with a storage snapshot on `qemu-x86_64`.
- [ ] A second prototype expresses the same environment as a Package manifest profile for comparison.
- [ ] Surface S-021 remains `open` or `prototyped`, never `frozen`.

#### Verification
- Report: which fields cannot live in a Package manifest, how capability and network namespaces are named, and which options ENV-008 must evaluate.
- Integration: the prototype enters on `qemu-x86_64`.

#### Evidence
- none

### ENV-022 · Measure environment service hosting paths against B-025
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-005, PKG-058, Q-001, SCH-007, STO-043
- Baseline: §35, §36, §54
- Benchmarks: B-025
- Invariants: I-043, I-061

GAP-0552 requires measurement before the hosting adr. Prototype Postgres and Redis as Linux-personality OCI images inside native isolation, as native Packages, and as a hybrid, and report cached and cold enter against B-025 for each path. The report does not claim superiority outside the harness (I-061).

<!-- covers: GAP-0552 -->

#### Out of scope
Hosting Decision (ENV-009). B-025 register ownership (BEN). Production instantiate path (ENV-017).

#### Acceptance criteria
- [ ] The report records B-025 cached and cold p50 and p99 for OCI-in-personality, native Package, and hybrid paths on H-002, using Q-001 methodology.
- [ ] Each path's write-up names whether it used a Docker daemon, Linux VM or overlay filesystem, and records I-043 compliance.
- [ ] The report names a default candidate for ENV-009 and the reasons, without a public superiority claim.
- [ ] Evidence includes `report:reports/spikes/ENV-022.md`.

#### Verification
- Report: answers, for each of the three paths, cached and cold B-025 on H-002; which path can meet the B-025 V1 cached target without overlayfs, Docker daemon or a Linux VM; what LNX surface the OCI path requires; what Package contract a native postgres or redis Package needs that the OCI path does not; and which path is the default candidate and why.
- Bench: B-025 method on H-001 and H-002 for each path; target per register, publish-only for the spike.
- Review: BEN lead confirms series names match the B-025 harness.

#### Evidence
- none

### ENV-023 · Store environment build outputs as content-addressed objects
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-014, STO-044
- Baseline: §27, §35
- Invariants: I-036

Cached build outputs from development environments are content-addressed objects (§27). ENV writes through STO's hash-keyed cache; hits are reused across enters and rebuilds of the same inputs.

<!-- covers: INV-0523 -->

#### Out of scope
Hash-keyed cache API and GC (STO-044). Compiler producers (BLD, SDK). Warm Package and snapshot cache (ENV-014).

#### Acceptance criteria
- [ ] A build output is stored under the content identifier STO-044 defines; a second build with identical inputs is a cache hit and does not rewrite bytes.
- [ ] A changed input produces a different identity and does not return the previous object.
- [ ] Hits are reused across enter and rebuild of the same environment; destroy unpins objects so STO GC may reclaim them.

#### Verification
- Unit: `env:tests/build_cache_*` on `qemu-x86_64`.
- Integration: two builds of the self-host kernel object on `hw-h002` with hit on the second.
- Review: STO lead confirms keys are content hashes, not paths.

#### Evidence
- none

### ENV-024 · Attach the native Terminal and Editor to a development environment
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-011, APP-016, ENV-003, ENV-005, ENV-012
- Baseline: §35, §41
- Threats: T-002
- Invariants: I-021

Grants the native Terminal and Editor the environment's endpoint and snapshot Capabilities so they attach without a Docker plugin. APP owns the apps; ENV owns the attach API.

#### Out of scope
Terminal and Editor applications (APP). IDE strategy (APP-020). Semantic startEnvironment (ENV-029). Linux-personality IDEs (LNX).

#### Acceptance criteria
- [ ] Attach returns attenuated endpoint and snapshot Capabilities to the Terminal and Editor Components; those apps hold no ambient network or home-directory Capability from this path.
- [ ] The Terminal's entered shell reaches postgres and redis through the granted endpoints on `qemu-x86_64` and `hw-h002`.
- [ ] The Editor opens a file in the environment snapshot and cannot open a path outside that snapshot (`Error::Rights`).
- [ ] A third application that was not attached receives `Error::Rights` when it requests those Capabilities.

#### Verification
- Integration: `env:tests/attach_editor_terminal_*` on `qemu-x86_64` and `hw-h002`.
- Demo: native Terminal and Editor attached to php-postgres-redis on H-002 with no Docker plugin.
- Review: APP lead confirms the apps consume the ENV attach API only.

#### Evidence
- none

### ENV-025 · Convert docker-compose.yml and devcontainer.json to environment.yaml
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-008, ENV-009, ENV-010
- Baseline: §35

V2 adoption: converters report unsupported constructs explicitly so developers are not forced to hand-rewrite project environments. The converter reads those files as documents and emits environment.yaml; it does not start a Docker daemon or make POSIX the native API.

<!-- covers: GAP-0423 -->

#### Out of scope
Schema (ENV-010). Service catalogue names (ENV-026). Docker socket compatibility (Q-032). Linux-personality compose runtime (LNX).

#### Acceptance criteria
- [ ] A fixture docker-compose.yml that declares postgres and redis emits an environment.yaml whose services section names those versions and that ENV-010 accepts.
- [ ] A fixture devcontainer.json that declares a runtime image emits a runtime section or a typed unsupported-construct diagnostic naming the field.
- [ ] Unsupported constructs (bind mounts outside the project, privileged mode, host network) are listed in the diagnostic and do not appear as ambient grants in the emitted definition.
- [ ] The converter crate still passes ENV-001.

#### Verification
- Unit: `env:tests/convert_compose_*` and `env:tests/convert_devcontainer_*` on `qemu-x86_64`.
- Integration: converted php-postgres-redis compose file enters on `qemu-x86_64`.
- Review: SEC lead confirms privileged and host-network constructs never become ambient grants.

#### Evidence
- none

### ENV-026 · Publish the environment service catalogue and version index
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-018, ENV-019, PKG-064
- Baseline: §28, §29, §35
- Invariants: I-036

Indexed named runtimes and services with lockable versions for environment.yaml, backed by the Package repository. First-party php, postgres and redis are already in the V1 reference stack.

#### Out of scope
Repository server and signing (REL). Third-party runtime publication (ENV-032). Resolver (ENV-019).

#### Acceptance criteria
- [ ] The catalogue lists php, postgres and redis with lockable versions that resolve to the same Package identities as the V1 reference lock.
- [ ] A catalogue lookup for an unknown name returns a typed not-found error and allocates no Package.
- [ ] Adding a first-party runtime version is a Package publish; environment.yaml pins the new identity through the lockfile.
- [ ] `os inspect` or the catalogue query API shows name, version and Package identity for each entry.

#### Verification
- Unit: `env:tests/service_catalogue_*` on `qemu-x86_64`.
- Integration: resolve php, postgres and redis from the catalogue on `qemu-x86_64` and `hw-h002`.
- Review: PKG lead confirms identities are store content hashes.

#### Evidence
- none

### ENV-027 · Rebuild a development environment when environment.yaml changes
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: ENV-011, ENV-012, STO-035
- Baseline: §18, §31, §35

V2 daily-driving: an environment.yaml edit rebuilds via ENV-012, emits a history event, and leaves the previous snapshot restorable. STO owns file-change Operations; ENV owns the rebuild trigger.

#### Out of scope
Change-notification Operation (STO-035). Lifecycle object API (ENV-012). Restore (ENV-028).

#### Acceptance criteria
- [ ] After an entered environment, a write to environment.yaml that changes a service version triggers one rebuild and one history event.
- [ ] The previous StorageSnapshot remains restorable until destroy or an explicit restore.
- [ ] A write that does not change definition plus lock identity triggers no rebuild.
- [ ] A caller without the rebuild Capability receives `Error::Rights` and the environment is unchanged.

#### Verification
- Unit: `env:tests/rebuild_on_change_*` on `qemu-x86_64`.
- Integration: edit-and-rebuild of php-postgres-redis on `qemu-x86_64` and `hw-h002` with previous snapshot listed in `os inspect`.
- Review: STO lead confirms the trigger uses the typed change-notification Operation.

#### Evidence
- none

### ENV-028 · Restore a development environment from system history
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-011, ENV-012, PKG-048
- Baseline: §31, §35
- Risks: R-084

V2 snapshots and rollback UI; §31 `os restore` covers environments. Re-materialises the environment recorded by ENV-011. Full workspace restore (open projects) waits for ENV-035.

<!-- covers: INV-0666 -->

#### Out of scope
History log (PKG). Settings rollback UI (INS-014). Workspace and window restore (ENV-035, APP-039). Generation restore (PKG-060).

#### Acceptance criteria
- [ ] Restoring a Changed project environment history event re-creates a DevelopmentEnvironment whose definition plus lock identity matches the event payload.
- [ ] The restored environment enters and its declared services are reachable through granted endpoints on `qemu-x86_64`.
- [ ] Restoring does not mutate other live environments; `os inspect` lists both until destroy.
- [ ] A caller without the restore Capability receives `Error::Rights` and no environment is created.

#### Verification
- Unit: `env:tests/restore_environment_*` on `qemu-x86_64`.
- Integration: create, rebuild, restore-previous on `qemu-x86_64` and `hw-h002`.
- Demo: V2-G08 restore of a project environment from history on H-002.

#### Evidence
- none

### ENV-029 · Expose a semantic startEnvironment action for automation
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: ENV-012, SEM-006, SEM-029, SEM-007
- Baseline: §42, §45, §35
- Threats: T-018
- Invariants: I-023, I-051

V2 exit: example rule project opened then start environment. ENV exports a typed semantic action over the SEM registry. No AI broker here. SEM owns discovery and the rules engine.

#### Out of scope
Registry and rules engine (SEM-029, SEM-031). Demo rule wiring (SEM-021). AI broker (SEM-010). Attach API (ENV-024).

#### Acceptance criteria
- [ ] ENV registers a `startEnvironment` Semantic interface method that creates or enters the environment for a Project Capability.
- [ ] Invoking the method without the interface Capability returns `Error::Rights` and allocates no environment.
- [ ] The method does not synthesise input or scrape UI; it calls ENV-012 only.
- [ ] SEM-021 can bind this method; this task's test invokes it without GUI input on `qemu-x86_64`.

#### Verification
- Unit: `env:tests/semantic_start_env_*` on `qemu-x86_64`.
- Integration: startEnvironment from a Project Capability on `qemu-x86_64` with the environment listed in `os inspect`.
- Review: SEM lead confirms the method is catalogue-registered and is not an AI-broker path.

#### Evidence
- none

### ENV-030 · Publish the development environment authoring guide
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-032, ENV-025, ENV-010, ENV-026, ENV-018, ENV-020
- Baseline: §35, §52, §56.5, §63

V3 exit: SDK guide published for strangers. ENV owns the environment.yaml, `os env`, converters and catalogue chapter; DOC owns the docs site.

#### Out of scope
Docs site and IDL-to-docs generator (DOC). CLI reference pages (DOC-013). Schema implementation (ENV-010).

#### Acceptance criteria
- [ ] The guide contains worked examples for authoring environment.yaml, locking versions, `os env enter`, converting compose and devcontainer files, and looking up catalogue services.
- [ ] Native examples do not tell the reader to install Docker Desktop, start a Linux VM or call POSIX container APIs.
- [ ] Every example definition in the guide parses with ENV-010 in docs CI.
- [ ] DOC-032 links these chapters as the normative ENV text.

#### Verification
- Review: SDK and DOC leads sign off on the pull request.
- Integration: example definitions parse in docs CI on `qemu-x86_64`.
- Manual: broken-link check includes the new chapters.

#### Evidence
- none

### ENV-031 · Isolate development environments across user sessions
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-049, CMP-051, ENV-012, ENV-016, SEC-060
- Baseline: §9.1, §35, §63
- Threats: T-026
- Invariants: I-021

V3 multi-user gate: two users with separate sessions and Capability stores. Environments, snapshots, caches and endpoints of one user are not enterable or inspectable by the other.

#### Out of scope
Session objects (SEC-060). Per-user grant stores (CAP-049). Component isolation tests (CMP-051). Session switcher UI (APP-063).

#### Acceptance criteria
- [ ] User B's `os env enter` of user A's environment identity returns `Error::Rights` and allocates no handle, on `qemu-x86_64`.
- [ ] User B's `os inspect` enumeration does not list user A's environments, snapshots, caches or endpoints.
- [ ] User B cannot connect to user A's granted service endpoints (`Error::Rights`).
- [ ] Destroy by user A does not affect user B's environments.

#### Verification
- Integration: `env:tests/multiuser_isolation_*` on `qemu-x86_64` and `hw-h002` with two sessions from SEC-060.
- Review: SEC lead confirms Capability stores are per session and no shared tmp path leaks snapshot bytes.

#### Evidence
- none

### ENV-032 · Publish environment runtimes as third-party Packages
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-010, ENV-026, PKG-084, REL-021, REL-025
- Baseline: §28, §35, §63
- Threats: T-006
- Invariants: I-036

V3 public package repository: third parties publish additional environment runtimes and services into the catalogue. ENV defines the runtime Package contract; REL and PKG own signing and review.

#### Out of scope
Publisher pipeline (REL-021). `os package publish` (PKG-084). Catalogue index (ENV-026). License allowlist (GOV).

#### Acceptance criteria
- [ ] A third-party runtime Package that meets the ENV contract is accepted by catalogue ingest and resolves from environment.yaml on `qemu-x86_64`.
- [ ] A runtime Package that requests ambient network or home-directory authority is rejected at review with a typed Capability lint error.
- [ ] Pinning the published version in a lockfile yields the signed content hash; a tampered blob is rejected by PKG signature verification.
- [ ] The contract document names required Interfaces, ResourceDomain profile and the prohibition on Docker daemon or VM dependencies.

#### Verification
- Integration: third-party fixture runtime published and resolved on `qemu-x86_64`.
- Review: REL and PKG leads sign off that signing and review are theirs; ENV owns only the contract.

#### Evidence
- none

### ENV-033 · Build the environment.yaml conformance suite
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-010, ENV-013, ENV-001, ENV-018
- Baseline: §35, §36, §66
- Benchmarks: B-025
- Invariants: I-043, I-036

V4 feature freeze: a permanent suite that identical yaml plus lock yields identical Component sets, that the §35 nongoals still hold, and that the php-postgres-redis reference still enters. B-025 remains BEN.

#### Out of scope
B-025 publication (BEN). Schema version lock (ENV-034). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] Two creates of identical definition plus lock on `qemu-x86_64` produce identical Component Package identity sets.
- [ ] The suite asserts the enter path loads no overlay filesystem, starts no Docker daemon and starts no Linux VM.
- [ ] The php-postgres-redis reference still enters with postgres and redis reachable on `qemu-x86_64` and `hw-h002`.
- [ ] A mutated lock that disagrees with the definition is rejected with a typed error and creates no environment.
- [ ] The suite is a required CI job on every V4 RC.

#### Verification
- Integration: `env:tests/conformance_*` on `qemu-x86_64` and `hw-h002`, run each RC.
- Bench: B-025 cached enter on H-002 still recorded beside the suite; target per register.
- Review: ABI lead confirms the suite freezes no L1 surface.

#### Evidence
- none

### ENV-034 · Lock the environment.yaml schema for the 1.x line
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: ENV-008, ENV-010, ENV-033, IPC-062, ENV-021
- Baseline: §12, §35, §66
- Freezes: S-021

V4 feature freeze and Layer 2 version lock. Pins the environment.yaml and lockfile schema versions decided at V1, with old-definition/new-supervisor evolution tests. Not a Layer 1 freeze. S-021 stays a Layer 2 surface.

#### Out of scope
Interface-evolution matrix harness (IPC-062). Conformance suite content (ENV-033). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] The 1.x environment.yaml and lockfile schema versions are enumerated in the lock document.
- [ ] An old-definition/new-supervisor pair from the V1 schema still enters on `qemu-x86_64`.
- [ ] A new-definition/old-supervisor pair returns a typed version error naming both versions and creates no environment.
- [ ] The lock document lists S-021 as a Layer 2 surface and does not mark any L1 surface frozen.

#### Verification
- Integration: `env:tests/schema_evolution_*` on `qemu-x86_64`.
- Review: IPC lead confirms the pair sits on IPC-062.
- Manual: lock document names 1.x versions and S-021 only.

#### Evidence
- none

### ENV-035 · Restore workspaces of open projects and environments
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-039, APP-056, ENV-028, PKG-069
- Baseline: §31
- Risks: R-084

1.0 `os restore` ambition (§31): revert open projects and environments together. Builds on ENV-028; APP owns window and session restore of the Editor.

<!-- covers: INV-0584 -->

#### Out of scope
Environment-only restore (ENV-028). Editor window restore (APP-039). Application-state restore scope (PKG-069). Generation restore (PKG-060).

#### Acceptance criteria
- [ ] Restoring a history event that recorded open projects and environments re-materialises those environments with matching definition plus lock identity.
- [ ] After restore, ENV reports the environment identities to APP; this task's test asserts those identities are enterable without creating a second copy.
- [ ] Restoring environments without recorded projects leaves no extra environment beyond the payload list.
- [ ] A caller without the restore Capability receives `Error::Rights` and no environment is created.

#### Verification
- Integration: `env:tests/restore_workspaces_*` on `qemu-x86_64` and `hw-h002`.
- Demo: 1.0 working-day restore of open projects and environments on H-002.
- Review: APP lead confirms window restore is APP; ENV supplies only environment objects.

#### Evidence
- none

### ENV-036 · Verify cached environment enter on every Tier 1 machine
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-025, ENV-033, ENV-014, ENV-018
- Baseline: §35, §54, §61
- Benchmarks: B-025
- Invariants: I-061

1.0 benchmark gate B-025 and the working-day demo in `os env`. Re-runs the reference stack enter on every Tier 1 machine; BEN publishes the number. No superiority claim without the harness report (I-061).

#### Out of scope
Harness and report publication (BEN-025). Conformance suite (ENV-033). Reference definition (ENV-018).

#### Acceptance criteria
- [ ] Cached enter of php-postgres-redis presents a shell with postgres and redis reachable on every machine in the 1.0 hardware scope.
- [ ] A B-025 report exists under `reports/benchmarks/B-025/` for each of those machines meeting the register target kind for 1.0.
- [ ] Cold enter is published on the same machines in the same session.
- [ ] The reports state no superiority claim.

#### Verification
- Bench: B-025 on H-002, H-004, H-005, H-006, H-007, H-008, H-009, H-010, H-011, H-012, H-013 and H-014; target per register.
- Integration: reference enter ping on each listed machine.
- Demo: 1.0 working-day native development in `os env` on H-002.
- Review: BEN lead confirms the reports are the 1.0-G15 B-025 evidence.

#### Evidence
- none
