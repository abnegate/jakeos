# NET · Networking
- Prefix: NET
- Lead: none
- Baseline: §2, §4, §5.1, §7, §9.1, §17, §23, §32, §43, §54, §57, §61, §62
- Baseline gap: No dedicated networking section; §4 places Network among user services, §7 names Object<NetworkConnection>, and §9.1 forbids ambient network access, but Wi-Fi, DHCP, DNS, VPN, firewall and remote shell are unspecified.

<!-- roadmap:generated:begin summary -->
Tasks: 40 live, 0 done, 0 in-progress, 40 todo, 0 dropped. Ready: 0. Blocked: 40. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

NET owns native networking on top of retained Linux TCP/IP, nftables, Ethernet and Wi-Fi drivers (§2, §5.1). Native software holds Capability-scoped `Object<NetworkConnection>` with Connect, Accept, Send and Receive Operations and never POSIX sockets (§7, §57). A userspace broker enforces per-Component network Capabilities and a default-deny inbound filter on retained nftables (§4, §9.1). A restartable network-management service holds connection profiles. Address assignment (DHCPv4, IPv6 SLAAC/DHCPv6 with privacy extensions), the system resolver (DNS-over-TLS, DNS-over-HTTPS, DNSSEC, later split DNS and mDNS/DNS-SD), native TLS consuming the SEC CA trust store, and a Capability-scoped network-namespace primitive for ENV and the Linux personality live here.

V1 daily-driving covers wired Ethernet and WPA2/WPA3 personal Wi-Fi with DHCP and DNS through Capabilities, plus sshd in the Linux personality behind an explicit listen right. Later rungs add VPN tunnels, enterprise Wi-Fi, airplane-mode, metered and MAC-randomisation flags, hotspot and tethering, wired 802.1X, system proxy, NIC DMA into a MemoryObject, Layer 2 interface lock, optional SMB folder sharing, HCL Wi-Fi verification, and the inherited-stack CVE runbook. `Object<NetworkConnection>` stays a Layer 1 prototype until ABI freeze at V4. NET-owned Layer 2 Interfaces (netmgr, resolver, mDNS, VPN, S-026) lock at V4.

## Out of scope

Kernel fork, networking kselftests and the retained-mechanism inventory (KRN). Capability rights encoding, mint, derive and revocation (CAP). Handle encoding, syscall entry and the error taxonomy (ABI). Connect and Accept Operation kinds (TSK). IDL, Channel transport and Layer 2 evolution rules (IPC). MemoryObject DMA properties, IOMMU mapping and NIC-to-decoder transfer (MEM). ResourceDomain network-policy field (SCH). `os inspect` command rendering (SDK) and the inspect data plane (OBS). Supervisor, hostname, time and service manifests (SVC). Shell picker, quick settings, Settings panels and consent UI (APP). NetworkManager emulation, Avahi and Bonjour APIs, sshd process hosting and personality netns (LNX). WinINet and Windows proxy (WIN). Bluetooth host power, USB gadget classes and HCL chipset rows (HW). Suspend-cycle harness (PWR). CA trust store, hardware-id denial and threat-model publication (SEC). `environment.yaml` NetworkNamespace consumption (ENV). SMB, NFS and WebDAV clients and Collections (STO). Fuzz fleet (BLD). Benchmark register ownership (BEN). HCL publication (REL). Lab access points and soak calendar (LAB). Codec Components (MED). Wi-Fi credential import during migration (INS). WWAN and eSIM (HW-090).

## Tasks

### NET-001 · Inventory retained TCP/IP, nftables and network drivers under native objects
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-017
- Baseline: §2, §5.1
- Invariants: I-009, I-010, I-054

Collapse the NET retain-Linux items into one inventory that lists TCP/IP, nftables, Ethernet and Wi-Fi drivers as the mechanism under native objects, feeding KRN's retained-mechanism list. V0 hardware is wired Ethernet only; Wi-Fi driver retention is recorded here and exercised at V1. KRN already runs networking kselftests, so this task does not add a second retain-regression suite.

<!-- covers: INV-0023, INV-0024, INV-0065, INV-0129 -->

#### Out of scope
Retained-subsystem kselftests (KRN-014). Native `Object<NetworkConnection>` (NET-014). Wi-Fi station bring-up (NET-021).

#### Acceptance criteria
- [ ] A committed inventory names TCP/IP, nftables, Ethernet drivers and Wi-Fi drivers as retained mechanisms under native objects and is cited from KRN-017.
- [ ] The inventory records that V0 hardware scope is wired Ethernet on H-001 and H-002, and that Wi-Fi driver retention is not exercised until V1 on H-004.
- [ ] No native crate, IDL or ABI entry in the inventory is a POSIX socket, `sockaddr` or Linux netlink handle (I-010).
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: KRN lead sign-off recorded on the pull request, confirming the inventory feeds the retained-mechanism list and does not duplicate kselftests.

#### Evidence
- none

### NET-002 · Prototype NetworkConnection wrap versus userspace broker with measurements
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-005, CAP-005, KRN-013
- Baseline: §4, §7, §54
- Explores: S-026
- Invariants: I-005, I-008, I-061

`Object<NetworkConnection>` is a §7 kernel object, so this V0 spike prototypes wrapping retained TCP/IP versus passing bytes through a userspace broker (and a hybrid kernel data path with userspace policy) and measures throughput and latency of each option. The report is the evidence for NET-008. The surface stays prototyped; this spike does not freeze it. Native software in the prototypes never sees a POSIX socket (I-005).

<!-- covers: GAP-0538 -->

#### Out of scope
The placement Decision (NET-008). Standing throughput harness (NET-004). Capability granularity (NET-006).

#### Acceptance criteria
- [ ] Prototypes exist for (A) wrapping retained TCP/IP, (B) a userspace byte broker, and (C) a hybrid kernel data path with userspace policy, each exposing Connect, Accept, Send and Receive as Operations.
- [ ] A report under `reports/spikes/NET-002.md` records throughput and latency of each option versus Linux TCP on H-001 and H-002 and states no superiority claim (I-061).
- [ ] The report names where Capability checks run on each option and whether native software observes a socket, `sockaddr` or netlink handle.
- [ ] The report does not freeze any ABI surface.

#### Verification
- Report: answers wrap versus broker versus hybrid; where Capability checks sit; what native software observes; measured throughput and latency versus Linux TCP on H-001 and H-002 with no superiority claim.
- Bench: publish-only comparison of the three prototypes versus Linux TCP on H-001 and H-002; numbers live only in the spike report.
- Review: ABI lead confirms the prototypes are not POSIX socket wrappers (I-005).

#### Evidence
- none

### NET-003 · Implement DHCPv4 and IPv6 SLAAC/DHCPv6 with privacy extensions
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-015, NET-012
- Baseline: §9.1, §61
- Threats: T-019
- Invariants: I-021

Address assignment is a userspace service over retained kernel autoconfig, gated by network Capabilities. DHCPv4 and IPv6 SLAAC/DHCPv6 with privacy extensions land here so Ethernet (and later Wi-Fi) hosts receive addresses without ambient netlink. Split DNS waits for VPN at V2. Wi-Fi DHCP on H-004 is verified by NET-020 once the station path exists.

<!-- covers: GAP-0292, INV-1200 -->

#### Out of scope
Wi-Fi association (NET-021). Split DNS (NET-026). Resolver (NET-019). Personality `dhclient` (LNX).

#### Acceptance criteria
- [ ] On H-001 and H-002 a Component holding a network Capability receives a DHCPv4 address on virtio-net or wired Ethernet and `os inspect` shows the address on the link.
- [ ] An IPv6-only QEMU netdev on H-001 assigns an address via SLAAC or DHCPv6 with privacy extensions enabled; the Interface identifier is not the burned-in MAC.
- [ ] A Component with no network Capability receives `Error::Rights` from address-assignment Operations and allocates no handle.
- [ ] Killing the address-assignment service under SVC re-establishes leases or fails clients with a typed disconnect; native software never opens a POSIX `AF_INET` socket for this path.

#### Verification
- Integration: `runtime:tests/net/address_assignment_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: SEC lead confirms privacy extensions are on by default for native SLAAC.

#### Evidence
- none

### NET-004 · Publish NetworkConnection throughput and latency against Linux sockets
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-014, BEN-007, BEN-005
- Baseline: §54
- Benchmarks: B-006
- Invariants: I-050, I-061

GAP-0538 requires measured throughput and latency for the placement Decision; the V0 spike publishes first-cut numbers and this harness is the standing comparison of native NetworkConnection against Linux TCP on the same hardware. BEN owns the register entry; NET owns the harness. Publication only; no superiority claim.

#### Out of scope
Register ownership and cross-OS publication (BEN). Placement Decision (NET-008). Zero-copy NIC-to-GPU copies (B-046, MEM-033).

#### Acceptance criteria
- [ ] Harness `bench:netconn-throughput` records NetworkConnection and Linux-TCP throughput and latency as separate series on H-001 and H-002.
- [ ] A report exists under `reports/benchmarks/` for H-001 and H-002 meeting the register target kind for V1 (publish).
- [ ] The report cites the series names and states no superiority claim (I-061).
- [ ] Native side of the harness uses NetworkConnection Operations only; it does not open a POSIX socket.

#### Verification
- Bench: NetworkConnection throughput and latency versus Linux TCP on H-001 and H-002; publish-only; target per the BEN register entry this harness feeds.
- Integration: `runtime:tests/net/bench_netconn_*` on `qemu-x86_64`.
- Review: BEN lead confirms the series names match the register method and that no number is restated in this task.

#### Evidence
- none

### NET-005 · Decide the native TLS library and how it consumes the CA trust store
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-007, SEC-002
- Baseline: §9.1, §51
- Decision: D-0201
- Risks: R-030
- Threats: T-019
- Invariants: I-005, I-049

Native TLS consumes the SEC CA trust store with per-application pinning so signed-repository fetch and native HTTPS do not go through POSIX sockets. This Decision picks the library and how it holds the store Capability. Personality TLS stays in LNX and WIN. Required by V1-G05 (Wi-Fi connects, roams and survives suspend): the DNS-over-TLS resolver path NET-020 exercises needs a chosen native TLS library.

#### Out of scope
CA trust-store object (SEC-016). TLS implementation (NET-011). Personality certificate stores (SEC-024).

#### Acceptance criteria
- [ ] Options evaluated include (A) rustls in the native runtime, (B) OpenSSL linked into native Components, and (C) kernel TLS offload.
- [ ] The accepted option names how a Component without the CA-store Capability fails TLS and that native TLS is not a POSIX socket API (I-005, I-049).
- [ ] Each option cites T-019 and R-030.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: SEC lead and NET reviewer sign-off recorded on the pull request, confirming at least two options and that personality TLS is out of scope.

#### Evidence
- none

### NET-006 · Decide per-application network Capability granularity and inbound firewall
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-007, NET-008, CAP-010, SEC-002
- Baseline: §9.1
- Decision: D-0202
- Threats: T-001, T-019
- Invariants: I-021, I-060

§9.1 says a native application must not automatically receive network access and never defines what a network Capability is. This Decision picks granularity and that default-deny inbound is retained nftables, not a rewritten filter. A Component without a network Capability cannot connect or listen. Surface S-026 stays prototyped until V4.

<!-- covers: GAP-0293, INV-0203 -->

#### Out of scope
Broker implementation (NET-012). ResourceDomain bandwidth field (SCH-033). eBPF as a native policy engine (KRN-024).

#### Acceptance criteria
- [ ] Options evaluated include (A) binary any/none, (B) the GAP-0293 set (any, internet-only, local-network, specific hosts/ports, listen), and (C) flow-level per connection.
- [ ] Every option records default-deny inbound on retained nftables and that listen is an explicit right, never ambient (T-001, I-021).
- [ ] The accepted option names the rights word CAP-036 must register for NetworkConnection.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: CAP lead and SEC lead sign-off recorded on the pull request, confirming at least two options, T-001 and retained nftables.

#### Evidence
- none

### NET-007 · Decide NET baseline-gap scope: preserved stack versus native objects
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-001, NET-002, KRN-001
- Baseline: none
- Decision: D-0203
- Invariants: I-005, I-009, I-010, I-049

BASELINE.md has no networking section. This first NET adr records the open scope: preserve Linux TCP/IP, nftables and inherited drivers with native NetworkConnection plus SVC-hosted management; reject a rewritten userspace TCP stack and reject POSIX sockets as the native API (§57). Every later V1 NET build depends on this Decision. The V0 cap forbids placing this adr in V0.

<!-- covers: INV-0065 -->

#### Out of scope
Data-path versus broker placement (NET-008). Capability granularity (NET-006). Supplicant choice (NET-009).

#### Acceptance criteria
- [ ] Options evaluated include (A) preserve Linux TCP/IP, nftables and inherited drivers with native NetworkConnection plus SVC-hosted management, (B) rewrite a userspace TCP stack, and (C) expose Linux sockets as the native API.
- [ ] Option C is rejected with I-005, I-049 and §57 cited; option B is evaluated against I-009 and I-010.
- [ ] The accepted option names TCP/IP, nftables and inherited drivers as the mechanism under native objects, and names that management is an SVC-hosted Component, not in-kernel.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: KRN lead and ABI lead sign-off recorded on the pull request, confirming at least two options and that POSIX sockets are not the native API.

#### Evidence
- none

### NET-008 · Decide whether NetworkConnection wraps the kernel TCP/IP stack
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-007, NET-002, ABI-013
- Baseline: §4, §7
- Decision: D-0204
- Invariants: I-008, I-040, I-055

INV-0110 places a network Capability broker in userspace while the TCP stack stays in the kernel. This Decision picks wrap, userspace-byte-broker or hybrid, using the spike measurements, and records that the surface stays prototyped until V4 (I-040). The choice shapes firewall and VPN design.

<!-- covers: GAP-0538, INV-0110 -->

#### Out of scope
Capability granularity (NET-006). Object implementation (NET-014). Standing throughput harness (NET-004).

#### Acceptance criteria
- [ ] Options evaluated include (A) wrap retained TCP/IP directly, (B) pass every byte through a userspace service, and (C) hybrid kernel data path with userspace policy.
- [ ] The accepted option cites the spike report path and names where Capability checks run relative to the TCP stack.
- [ ] The accepted option records that `Object<NetworkConnection>` remains prototyped until V4 and is not frozen here (I-040).
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request, confirming the spike report is cited and the surface is not frozen.

#### Evidence
- none

### NET-009 · Decide the Wi-Fi supplicant: iwd, wpa_supplicant or native Rust
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-007
- Baseline: §5.1, §61
- Decision: D-0206
- Risks: R-027
- Invariants: I-009, I-054

GAP-0291 names the supplicant ADR; the choice is the security and maintenance surface for V1 laptop daily-driving. Options are iwd, wpa_supplicant, or a native Rust supplicant. APP owns the shell picker UI. Retained cfg80211 and mac80211 stay the mechanism; this Decision does not rewrite a native MAC stack.

<!-- covers: GAP-0291 -->

#### Out of scope
Station implementation (NET-021). Shell picker (APP-041). Driver bring-up (HW-015).

#### Acceptance criteria
- [ ] Options evaluated include (A) iwd, (B) wpa_supplicant, and (C) a native Rust supplicant.
- [ ] The accepted option names the Component that hosts the supplicant, the retained kernel interfaces it consumes, and that cfg80211 is not rewritten (I-009, I-054).
- [ ] Each option records the WPA2/WPA3 personal, hidden-network and enterprise-EAP consequences for NET-021 and NET-028.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: SEC lead and HW lead sign-off recorded on the pull request, confirming at least two options and retained cfg80211.

#### Evidence
- none

### NET-010 · Write the native NetworkConnection programming model for SDK v1
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-014, NET-011, NET-019, NET-006
- Baseline: §7, §9.1, §52
- Invariants: I-005, I-049

V1 SDK v1 needs a native networking guide so developers never reach for sockets. This task authors Capability rights, Operations, resolver use and TLS for NetworkConnection. SDK packages the crate guide; NET owns the model text.

#### Out of scope
SDK crate publication (SDK-056). Docs site generation (DOC-014). Personality socket programming (LNX).

#### Acceptance criteria
- [ ] A committed guide names Connect, Accept, Send, Receive, resolver lookup and TLS as Operations on Capabilities, with `Error::Rights` for a missing network or CA-store Capability.
- [ ] The guide contains no POSIX socket, `sockaddr`, `getaddrinfo` or `SSL_read` as a native API (I-005, I-049).
- [ ] The guide is the NET chapter cited by SDK-056.
- [ ] Documentation review sign-off is recorded on the pull request.

#### Verification
- Review: SDK lead and DOC reviewer sign-off recorded on the pull request.

#### Evidence
- none

### NET-011 · Implement native TLS on NetworkConnection using the system CA store
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-005, NET-014, SEC-016, SEC-022
- Baseline: §9.1, §51
- Risks: R-030
- Threats: T-019
- Invariants: I-021

Implement the accepted TLS library on NetworkConnection so signed-repository fetch and native HTTPS consume SEC's CA trust store with per-application pinning. A Component without the store Capability cannot use default system CAs. Personality certificate stores stay in LNX and WIN.

<!-- covers: EXTRA-019 -->

#### Out of scope
CA store object (SEC-016). Personality mirroring (SEC-024). Resolver DoT/DoH configuration (NET-019).

#### Acceptance criteria
- [ ] A native Component holding NetworkConnection plus the CA-store Capability completes a TLS handshake to a test server whose chain is in the store, using only NetworkConnection Operations.
- [ ] A Component without the CA-store Capability receives `Error::Rights` and allocates no TLS session handle (I-021).
- [ ] Per-application pinning rejects a host whose pin does not match, with a typed error visible in `os inspect`.
- [ ] Native crates on this path contain no POSIX socket symbol.

#### Verification
- Integration: `runtime:tests/net/native_tls_*` on `qemu-x86_64` and `hw-h002`.
- Review: SEC lead confirms the store Capability is the only trust root for native TLS.

#### Evidence
- none

### NET-012 · Implement the network Capability broker and default-deny inbound firewall
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: NET-006, NET-014, CAP-005, SVC-015
- Baseline: §4, §9.1
- Threats: T-001, T-002, T-019
- Invariants: I-021, I-037, I-060

INV-0110 is a userspace broker above kernel TCP; INV-0203 is no ambient network access; GAP-0293 is default-deny inbound on retained nftables. A Component with no network Capability cannot open or listen; listen is an explicit right. SCH owns the ResourceDomain policy field this broker applies. The broker is an SVC-supervised Component, not in-kernel.

<!-- covers: INV-0110, INV-0203, GAP-0293 -->

#### Out of scope
ResourceDomain network-policy field (SCH-033). nftables rewrite (forbidden by NET-006). Personality default-cap bundle (LNX-013).

#### Acceptance criteria
- [ ] A Component with no network Capability that submits Connect or Accept receives `Error::Rights` and allocates no handle (I-021, T-001).
- [ ] Listen without an explicit listen right is denied the same way; an unsolicited inbound SYN to a host with default-deny nftables is dropped.
- [ ] Exercising a network Capability that is not a subset of the holder's rights returns `Error::Rights` and does not open a connection (T-002).
- [ ] Killing the broker under SVC restarts it; in-flight clients see a typed disconnect or a rebound Channel (I-037).
- [ ] `os inspect` lists each held network Capability with rights and target; no native crate opens a POSIX socket to implement this path.

#### Verification
- Unit: `kernel:tests/net/capability_broker_*` on `qemu-x86_64`.
- Integration: `runtime:tests/net/inbound_deny_*` on `qemu-x86_64` and `hw-h002`.
- Demo: V1 capability-denial demo; a Component without a network Capability cannot open a connection.
- Review: CAP lead confirms rights checks match NET-006.

#### Evidence
- none

### NET-013 · Export link, route and network Capability state to os inspect
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-014, NET-012, OBS-019
- Baseline: §24, §64
- Invariants: I-034

V1 daily-driving and the capability-denial demo need `os inspect` to show links, routes, NetworkConnection objects and held network Capabilities. OBS owns the inspect command; NET owns this data. Required by V1-G05 (Wi-Fi connects, roams and survives suspend): NET-020 reads the Capability denial and link state from this inspect data.

#### Out of scope
Inspect CLI rendering (SDK-007). Kernel inspect interface (OBS-006). Power and service inspect (PWR, SVC).

#### Acceptance criteria
- [ ] `os inspect` prints each link with operational state, each route, each live NetworkConnection, and each held network Capability with rights.
- [ ] A Component that is denied Connect appears in the inspect output with the typed `Error::Rights` denial, not as an open connection.
- [ ] The inspect provider is registered through OBS-019; NET does not add an `os` subcommand.
- [ ] Native inspect data contains no POSIX `ifconfig` or netlink dump as the ABI.

#### Verification
- Integration: `runtime:tests/net/inspect_export_*` on `qemu-x86_64`.
- Review: OBS lead confirms the provider schema matches other userspace inspect kinds.

#### Evidence
- none

### NET-014 · Implement Object<NetworkConnection> over the retained TCP/IP stack
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: NET-008, ABI-005, CAP-036, TSK-032, KRN-013
- Baseline: §2, §7
- Invariants: I-005, I-015, I-049

§7 `Object<NetworkConnection>` with Connect, Accept, Send and Receive Operations on the placement chosen by NET-008. Linux TCP/IP stays the mechanism; native software never sees sockets. Needed for V1 Ethernet and Wi-Fi through Capabilities. The object stays prototyped; freeze is ABI at V4.

<!-- covers: INV-0165, INV-0065 -->

#### Out of scope
Capability broker and inbound firewall (NET-012). TLS (NET-011). NIC DMA receive (NET-025). Personality sockets (LNX-001).

#### Acceptance criteria
- [ ] A pair of native Components on H-001 complete Connect, Accept, Send and Receive as Operations and transfer a payload without either Component holding a POSIX socket (I-005, I-049).
- [ ] CAP-036 lists NetworkConnection rights; an Operation outside the holder's rights returns `Error::Rights` and allocates no handle (I-015).
- [ ] `os inspect` shows each live NetworkConnection with endpoints and owner Component.
- [ ] The Linux personality still uses retained sockets; this object is not the personality ABI.
- [ ] No `unsafe` outside the NET kernel placement module named by the accepted placement Decision.

#### Verification
- Unit: `kernel:tests/net/netconn_ops_*` on `qemu-x86_64`.
- Integration: `runtime:tests/net/netconn_roundtrip_*` on `qemu-x86_64` and `hw-h002`.
- Review: ABI lead confirms the object type is registered and is not a socket wrapper.

#### Evidence
- none

### NET-015 · Build the restartable network management service with connection profiles
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: NET-007, NET-009, SVC-015, SVC-009, SVC-011, IPC-035
- Baseline: §32, §61
- Invariants: I-037

GAP-0291 connection profiles and INV-0595 restart/rebind: killing the network service re-establishes links or fails clients explicitly under SVC supervision. Hosted as an SVC service, not in-kernel. APP binds the shell picker to this Interface. V1 daily-driving on desktop Ethernet and laptop Wi-Fi.

<!-- covers: GAP-0291, INV-0595 -->

#### Out of scope
Shell picker UI (APP-041). Supplicant Decision (NET-009). Address assignment (NET-003). Personality NetworkManager (LNX-068).

#### Acceptance criteria
- [ ] Connection profiles for wired Ethernet exist as typed objects; applying a profile on H-001 and H-002 brings the link up or returns a typed error.
- [ ] Killing the netmgr Component under SVC restarts it; clients rebind by Interface identity or receive a typed disconnect (I-037).
- [ ] `os inspect service` shows netmgr restart count and remaining budget after each kill.
- [ ] The service manifest declares requested Capabilities, dependencies, restart policy and readiness; native software never talks a POSIX D-Bus NetworkManager API for this path.
- [ ] The netmgr Interface is registered in IPC-035 with a version identity.

#### Verification
- Integration: `runtime:tests/net/netmgr_restart_*` on `qemu-x86_64` and `hw-h002`.
- Review: SVC lead confirms supervision and rebind match §32.

#### Evidence
- none

### NET-016 · Expose network namespace isolation as a Capability-scoped primitive
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-006, NET-012, CMP-026
- Baseline: §9.1, §23, §35
- Threats: T-001
- Invariants: I-019, I-021

V1 `os env` creates a NetworkNamespace isolating service ports without ambient network authority. NET provides the primitive; ENV-003 consumes it. Retained kernel namespaces underneath, not a rewritten stack. Native software never configures namespace filesystems as a semantic step. Required by V1-G04 (Cached os env enter presents a working shell): declared services are reachable without ambient network authority only through the NetworkNamespace primitive ENV-003 consumes.

#### Out of scope
environment.yaml consumption (ENV-003, ENV-006). Personality netns (LNX-045). ResourceDomain network policy (SCH-033).

#### Acceptance criteria
- [ ] Creating a NetworkNamespace yields a Capability; a Component holding it can bind isolated listen rights without receiving ambient network access (I-021).
- [ ] A Component without that Capability cannot observe or bind the namespace's ports; attempts return `Error::Rights`.
- [ ] `os inspect` shows the namespace, its holder and its listen rights.
- [ ] Native crates do not call POSIX `setns` or write cgroup/netns paths as the ABI (I-019).

#### Verification
- Integration: `runtime:tests/net/network_namespace_*` on `qemu-x86_64`.
- Review: ENV lead confirms the primitive is sufficient for ENV-003 without ambient network.

#### Evidence
- none

### NET-017 · Provide sshd remote shell via the Linux Personality with listen Capability
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-012, LNX-030, SVC-026
- Baseline: §3, §9.1
- Threats: T-040
- Invariants: I-006, I-021

Developers need remote shell at V1. sshd runs in the Linux personality; the host listen right is an explicit network Capability, never ambient. A native typed remote shell is a later adr (NET-040).

<!-- covers: EXTRA-020 -->

#### Out of scope
Native typed remote shell Decision (NET-040). sshd process image (LNX). Secrets for SSH keys (SEC-030).

#### Acceptance criteria
- [ ] sshd listens only when a listen Capability is granted; without it the port is closed and `os inspect` shows no listen right (T-040, I-021).
- [ ] A developer session authenticates and opens a Linux-personality shell on H-001; native Components in the same boot still hold no ambient network Capability.
- [ ] Revoking the listen Capability closes the listen socket; new connections fail and the typed denial is visible in inspect.
- [ ] Native software has no POSIX `sshd` API; the daemon is personality-only (I-006).

#### Verification
- Integration: `runtime:tests/net/remote_sshd_listen_*` on `qemu-x86_64`.
- Manual: grant and revoke the listen Capability and confirm the port opens and closes.
- Review: SEC lead confirms listen is explicit (T-040).

#### Evidence
- none

### NET-018 · Investigate NIC-to-MemoryObject DMA on retained drivers via AF_XDP
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-007, MEM-028, MEM-037
- Baseline: §17
- Invariants: I-038, I-054, I-063

INV-0336: feasibility of NIC DMA into a MemoryObject on retained Linux drivers (AF_XDP/zero-copy receive) before the V2 deliverable. The report documents driver constraints for MEM's NIC-to-decoder path. This spike does not rewrite the NIC driver stack.

<!-- covers: INV-0336 -->

#### Out of scope
V2 DMA receive implementation (NET-025). MemoryObject DMA property (MEM-028). IOMMU policy (HW-017). Decoder transfer (MEM-039).

#### Acceptance criteria
- [ ] The report names which retained drivers on H-001 and H-002 support AF_XDP zero-copy receive into a DMA-compatible MemoryObject, and which require a copy.
- [ ] The report records IOMMU constraints (I-038) and the fallback when a driver cannot DMA into a MemoryObject.
- [ ] The report does not propose a native NIC driver rewrite (I-054).
- [ ] A committed report exists at `reports/spikes/NET-018.md`.

#### Verification
- Report: answers driver support, copy versus DMA, IOMMU constraints, and what MEM must provide for NET-025.
- Review: MEM lead confirms the constraints are usable by MEM-039.

#### Evidence
- none

### NET-019 · Implement the system resolver with DNS-over-TLS, DNS-over-HTTPS and DNSSEC
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-003, NET-011, SEC-016, SVC-024
- Baseline: §9.1, §61
- Threats: T-019
- Invariants: I-021

V1 gate: DNS works and must not leak plaintext when DoT or DoH is configured. DNSSEC validation is on by default for native resolution. mDNS/`.local` is stubbed until NET-024; split DNS per VPN is V2. The SEC CA store is the trust root.

<!-- covers: GAP-0292, INV-1200 -->

#### Out of scope
mDNS/DNS-SD (NET-024). Split DNS (NET-026). Personality `resolv.conf` (LNX-068). Hostname object (SVC-024).

#### Acceptance criteria
- [ ] A native Component holding a resolver Capability resolves a name over DoT or DoH using the system CA store; a packet capture on the test netdev shows no plaintext DNS for that lookup (T-019).
- [ ] DNSSEC validation is on by default for native resolution; a signed-bogus test zone returns a typed error.
- [ ] A Component without the resolver Capability receives `Error::Rights` and allocates no handle (I-021).
- [ ] `.local` names return a typed "not yet" result until NET-024; they are not leaked to a public resolver.

#### Verification
- Integration: `runtime:tests/net/resolver_dot_doh_dnssec_*` on `qemu-x86_64` and `hw-h002`.
- Review: SEC lead confirms the CA store is the DoT/DoH trust root.

#### Evidence
- none

### NET-020 · Verify the V1 network Gate: Wi-Fi, DHCP, DNS and Capability denial
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-021, NET-003, NET-019, NET-012, NET-013, PWR-014, LAB-009, BLD-044
- Baseline: §9.1, §61
- Risks: R-027
- Threats: T-001
- Invariants: I-021

Gate-verifying task for V1 Wi-Fi, DHCP, DNS and "a Component without a network Capability cannot open a connection", plus Wi-Fi up after the V1 suspend-cycle harness on the Intel laptop. Permanent CI scenarios on QEMU and H-004.

<!-- covers: INV-1200, INV-0203 -->

#### Out of scope
Suspend policy and cycle count (PWR-014). Lab fixtures (LAB-009). Enterprise Wi-Fi (NET-028).

#### Acceptance criteria
- [ ] On H-004 a WPA2 or WPA3 personal association, DHCPv4 or IPv6 assignment, and a DoT/DoH lookup all succeed; `os inspect` shows the link, address, route and held network Capability.
- [ ] A Component without a network Capability on H-001 and H-004 receives `Error::Rights` on Connect and allocates no handle (I-021).
- [ ] After every successful cycle of PWR-014 on H-004, Wi-Fi is associated and DHCP/DNS succeed again.
- [ ] The same Connect-denial and Ethernet DHCP/DNS scenarios pass on `qemu-x86_64`.

#### Verification
- Integration: `runtime:tests/net/v1_network_gate_*` on `qemu-x86_64` and `hw-h004`.
- Demo: V1 capability-denial and Intel-laptop Wi-Fi demo on H-004.
- Review: PWR lead confirms post-resume Wi-Fi checks are hooked into the suspend harness.

#### Evidence
- none

### NET-021 · Bring up WPA2/WPA3 personal Wi-Fi with hidden networks and roaming
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: NET-009, NET-015, NET-003, HW-015, LAB-011
- Baseline: §5.1, §61
- Risks: R-027
- Threats: T-019
- Invariants: I-054

V1 exit: WPA2/WPA3 personal connects, roams between the two lab access points, and survives suspend on the Intel laptop. Implements NET-009 with hidden-network and captive-portal detection. Enterprise EAP is NET-028 at V2. Retained Wi-Fi drivers, not a native MAC stack.

<!-- covers: GAP-0291, INV-1200, INV-0023 -->

#### Out of scope
Enterprise EAP, full roaming and captive-portal login (NET-028). Shell picker (APP). Driver SKU bring-up (HW-015). Suspend harness (PWR).

#### Acceptance criteria
- [ ] H-004 associates to a WPA2 personal and a WPA3 personal SSID from LAB-011 and receives an address via NET-003.
- [ ] H-004 roams between the two lab access points without dropping the netmgr profile; `os inspect` shows the new BSSID.
- [ ] A hidden SSID in a netmgr profile associates; a detected captive portal is flagged on the profile without granting ambient network to applications (T-019).
- [ ] Native software never talks nl80211 or wpa_supplicant sockets as its ABI; the supplicant is confined to the netmgr Component (I-054).

#### Verification
- Integration: `runtime:tests/net/wifi_station_personal_*` on `hw-h004`.
- Demo: V1 Intel-laptop Wi-Fi connect and roam on H-004.
- Review: HW lead confirms retained cfg80211 is the mechanism.

#### Evidence
- none

### NET-022 · Implement airplane-mode that drops radio Capabilities without tearing profiles
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-015, NET-021, HW-035
- Baseline: §9.1, §62
- Invariants: I-021

Radios off while connection profiles remain so Wi-Fi reconnects on restore. NET owns radio Capabilities; HW owns Bluetooth power. APP binds the quick-settings tile.

#### Out of scope
Quick-settings chrome (APP-036). Bluetooth controller power (HW-035). WWAN (HW-090).

#### Acceptance criteria
- [ ] Enabling airplane-mode drops Wi-Fi radio Capabilities; `os inspect` shows the profiles still present and the radio Capability absent (I-021).
- [ ] Disabling airplane-mode restores the radio Capability and H-004 reassociates using the preserved profile.
- [ ] Bluetooth radio power is requested through HW-035; NET does not talk HCI as a native API.
- [ ] Native software has no POSIX rfkill ABI.

#### Verification
- Integration: `runtime:tests/net/airplane_mode_*` on `hw-h004`.
- Review: HW lead confirms Bluetooth power is not duplicated in NET.

#### Evidence
- none

### NET-023 · Fuzz NetworkConnection Connect, Accept, Send and Receive without panic
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-014, BLD-035, IPC-044
- Baseline: §7, §51
- Invariants: I-015

NetworkConnection is a new L1 object and must have a harness before the V3 continuous-fuzzing gate. Runs in BLD nightly; NET owns the target. No known open panic on Connect, Accept, Send or Receive.

#### Out of scope
Fuzz fleet and crasher filing (BLD-035). Channel syscall fuzz (IPC-044). V3 open-crasher gate (BLD-063).

#### Acceptance criteria
- [ ] Harness `kernel:fuzz/netconn` is registered with BLD-035 and mutates Connect, Accept, Send and Receive including rights bits and handle slots.
- [ ] Nightly runs produce no panic or kernel BUG on this target; crashers file into the Markdown repository through BLD.
- [ ] Forged or wrong-type handles return `Error::Rights` without allocating a connection (I-015).
- [ ] The target is listed in the V3 IPC/kernel fuzz coverage report consumed by BLD-063.

#### Verification
- Fuzz: `kernel:fuzz/netconn` nightly without panic, feeding BLD-035.
- Review: BLD lead confirms the target is in the nightly set.

#### Evidence
- none

### NET-024 · Provide native mDNS/DNS-SD with Avahi and Bonjour Personality compatibility
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-019, SVC-024
- Baseline: §61, §62
- Threats: T-019
- Invariants: I-006

GAP-0435 plus the mDNS/DNS-SD clause of GAP-0292. Native `.local` and DNS-SD for printers, NAS and developer tooling; Avahi and Bonjour APIs exist only inside personalities. SVC hostname is an input.

<!-- covers: GAP-0435, GAP-0292 -->

#### Out of scope
Hostname object (SVC-024). Print discovery (HW-071). Avahi daemon hosting (LNX). Bonjour in Wine (WIN). SMB clients (STO-059).

#### Acceptance criteria
- [ ] A native Component holding a discovery Capability resolves a `.local` name published on the lab LAN and browses a DNS-SD type; results are typed records, not POSIX hostents.
- [ ] Without the discovery Capability, `.local` lookup returns `Error::Rights` and does not query a public resolver (T-019).
- [ ] Linux-personality Avahi and Windows-personality Bonjour APIs resolve the same records only inside those personalities (I-006).
- [ ] The native resolver no longer returns the V1 `.local` stub error for granted Components.

#### Verification
- Integration: `runtime:tests/net/mdns_dnssd_*` on `hw-h002` and `hw-h004`.
- Review: LNX lead confirms Avahi is personality-only.

#### Evidence
- none

### NET-025 · Deliver NIC DMA receive directly into a MemoryObject
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: NET-018, NET-014, MEM-028, MEM-037
- Baseline: §17
- Benchmarks: B-046
- Invariants: I-038, I-054, I-063

INV-0322 §17: NIC DMA lands in a MemoryObject for ownership transfer to decoder or GPU. Implements NET-018 on retained drivers. MEM owns DMA-compatible properties and the NIC-to-decoder transfer. No native NIC driver rewrite. B-046 counts copies on this path.

<!-- covers: INV-0322 -->

#### Out of scope
Decoder transfer (MEM-039). Copy-count publication (BEN-045, MEM-033). NIC driver rewrite (forbidden, I-054).

#### Acceptance criteria
- [ ] On hardware the spike named as DMA-capable, a received frame's payload bytes reside in a DMA-compatible MemoryObject whose physical pages match the NIC DMA mapping (I-063).
- [ ] The receive Operation completes with ownership of that MemoryObject; the sender mapping in the NIC ring is not writable by the receiving Component.
- [ ] Without IOMMU, the fallback recorded by the spike is taken and user-space DMA is not enabled (I-038).
- [ ] Drivers remain the retained Linux set; no native MAC or NIC driver is introduced (I-054).
- [ ] A B-046 report for this stage exists for H-002 meeting the register target kind for V2.

#### Verification
- Integration: `kernel:tests/net/nic_dma_recv_*` on `hw-h002`.
- Bench: B-046 on H-002; target per register; this task supplies the NIC-receive copy count.
- Review: MEM lead confirms physical-page identity checks match MEM-012.

#### Evidence
- none

### NET-026 · Implement split DNS per Interface and per VPN tunnel
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-027, NET-019
- Baseline: §9.1
- Threats: T-019

Remaining GAP-0292 clause: split DNS per interface or VPN so corporate resolvers do not leak to the public interface.

<!-- covers: GAP-0292 -->

#### Out of scope
VPN tunnels (NET-027). Resolver DoT/DoH (NET-019). Personality nsswitch (LNX-068).

#### Acceptance criteria
- [ ] A name in a VPN-only suffix resolves only through the tunnel's resolver; a packet capture on the public interface shows no query for that name (T-019).
- [ ] A public name still resolves through the public resolver while the tunnel is up.
- [ ] `os inspect` shows the per-interface and per-tunnel resolver set.
- [ ] Native software has no POSIX `resolv.conf` ABI for this policy.

#### Verification
- Integration: `runtime:tests/net/split_dns_*` on `qemu-x86_64`.
- Review: SEC lead confirms no suffix leak to the public interface.

#### Evidence
- none

### NET-027 · Implement WireGuard tunnels and Personality VPN clients as Capabilities
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: NET-006, NET-012, NET-015, NET-019
- Baseline: §9.1, §62
- Threats: T-019
- Invariants: I-006, I-021

GAP-0294: WireGuard is the native tunnel primitive; OpenVPN, IPsec and proprietary clients run in the Linux personality. Tunnels are Capabilities with per-app routing. APP owns the shell toggle. Always-on VPN must compose with NET-006.

<!-- covers: GAP-0294 -->

#### Out of scope
Shell toggle (APP-036). Split DNS (NET-026). Personality VPN GUIs (LNX). ResourceDomain bandwidth (SCH).

#### Acceptance criteria
- [ ] A native Component holding a WireGuard tunnel Capability sends traffic through the tunnel; a Component without it cannot use that tunnel and receives `Error::Rights` (I-021).
- [ ] Per-app routing policy sends only granted Components into the tunnel; other Components keep the default route.
- [ ] An OpenVPN or IPsec client runs in the Linux personality and appears as a Capability-scoped tunnel to native policy, not as a native POSIX interface (I-006).
- [ ] Always-on mode survives netmgr restart: the tunnel re-establishes or clients see a typed disconnect.
- [ ] `os inspect` lists each tunnel, its holders and its routing policy.

#### Verification
- Integration: `runtime:tests/net/vpn_wireguard_*` on `qemu-x86_64` and `hw-h002`.
- Review: CAP lead confirms tunnel rights match NET-006.

#### Evidence
- none

### NET-028 · Implement WPA2/WPA3 enterprise, captive portals and full roaming
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: NET-021, NET-015, HW-039, LAB-018, SEC-027
- Baseline: §62
- Risks: R-027
- Threats: T-019
- Invariants: I-054

INV-1218 and remaining GAP-0291: enterprise EAP, captive-portal login, and roaming on Intel and AMD laptops for the V2 laptop-day demo (open lid, Wi-Fi reconnects). Personal WPA2/WPA3 already landed at V1.

<!-- covers: INV-1218, GAP-0291 -->

#### Out of scope
Personal WPA2/WPA3 (NET-021). Lid and suspend policy (PWR-021). Captive-portal browser chrome (APP). Secrets storage (SEC-027).

#### Acceptance criteria
- [ ] H-004 and H-005 associate to a WPA2/WPA3-Enterprise AP using EAP credentials from SEC-027; credentials are not ambient files (T-019).
- [ ] A captive-portal network flags the profile and completes login through a Capability-scoped session; applications without network Capability remain denied.
- [ ] After lid-open resume on H-004 and H-005 the station reassociates using the existing profile.
- [ ] Roaming across the lab access points succeeds on both laptops; `os inspect` shows the new BSSID.
- [ ] Retained cfg80211 remains the mechanism (I-054).

#### Verification
- Integration: `runtime:tests/net/wifi_enterprise_*` on `hw-h004` and `hw-h005`.
- Demo: V2 laptop-day open-lid Wi-Fi reconnect on H-004 and H-005.
- Review: SEC lead confirms EAP credentials live in the secrets service.

#### Evidence
- none

### NET-029 · Decide whether 1.0 ships an SMB server and where it is hosted
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-007, NET-012, STO-059
- Baseline: none
- Decision: D-0205
- Invariants: I-006, I-021

GAP-0434 pulled from V4 to V3 so a V4 implementation can land before feature freeze. Options: do not ship a server; Samba in the Linux personality; a native sharing service. STO owns SMB clients. Listen Capability is explicit if a server ships.

<!-- covers: GAP-0434 -->

#### Out of scope
SMB/NFS/WebDAV clients (STO-059). Folder-share implementation (NET-036). Samba packaging (LNX).

#### Acceptance criteria
- [ ] Options evaluated include (A) do not ship a server in 1.0, (B) Samba in the Linux personality, and (C) a native sharing service.
- [ ] The accepted option names whether NET-036 is in scope or dropped, and that any listen path is an explicit network Capability (I-021).
- [ ] Option B records that Samba is personality-only (I-006).
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: STO lead and LNX lead sign-off recorded on the pull request, confirming client/server split and at least two options.

#### Evidence
- none

### NET-030 · Implement Wi-Fi hotspot and USB/Bluetooth tethering
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-021, NET-012, HW-035, HW-075
- Baseline: none
- Threats: T-040
- Invariants: I-021, I-093

GAP-0296 hotspot and USB/Bluetooth tethering for public alpha. Bluetooth PAN depends on HW's V2 Bluetooth stack; USB tethering uses retained gadget/CDC. Listen and forward rights are network Capabilities. WWAN remains parked (I-093).

<!-- covers: GAP-0296 -->

#### Out of scope
Bluetooth host (HW-035). USB CDC gadget class (HW-075). WWAN/eSIM (HW-090). Shell toggle (APP).

#### Acceptance criteria
- [ ] A Wi-Fi hotspot profile on H-004 grants listen and forward Capabilities; a second lab station associates and receives an address.
- [ ] USB tethering using retained gadget/CDC and Bluetooth PAN via HW-035 each require the same listen/forward rights (I-021).
- [ ] Without those rights, hotspot start returns `Error::Rights` and no SSID is advertised (T-040).
- [ ] No WWAN or eSIM Interface is added (I-093).

#### Verification
- Integration: `runtime:tests/net/hotspot_tethering_*` on `hw-h004`.
- Review: HW lead confirms USB gadget and Bluetooth PAN are not reimplemented in NET.

#### Evidence
- none

### NET-031 · Implement metered-connection flagging and MAC address randomisation
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-015, NET-021, SEC-019
- Baseline: §9.1
- Threats: T-019
- Invariants: I-078

GAP-0296 public-alpha laptop checks: metered flag for update and backup policy, and per-network MAC randomisation. Native only; personalities observe the same flags via LNX-068.

<!-- covers: GAP-0296 -->

#### Out of scope
Updater metered policy (INS-009). Hardware-id Capability (SEC-019). Personality NM flags (LNX-068). Backup policy (STO-071).

#### Acceptance criteria
- [ ] A netmgr profile can be marked metered; `os inspect` shows the flag and PKG/INS consumers read it through the typed Interface.
- [ ] Per-network MAC randomisation is on by default for Wi-Fi profiles; the burned-in MAC is not used as the station address without a hardware-id Capability (I-078).
- [ ] A Component without that Capability cannot read the burned-in MAC; attempts return `Error::Rights`.
- [ ] Personality software sees the same metered and MAC flags only through LNX-068.

#### Verification
- Integration: `runtime:tests/net/metered_mac_random_*` on `hw-h004`.
- Review: SEC lead confirms I-078 for MAC addresses.

#### Evidence
- none

### NET-032 · Implement system proxy configuration applied to native and Personality traffic
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-011, NET-019, LNX-068
- Baseline: none
- Threats: T-019
- Invariants: I-006, I-021

GAP-0296 system proxy. Native TLS and resolver honour the proxy Capability; LNX-068 and WIN propagate the same settings. APP owns the settings panel.

<!-- covers: GAP-0296 -->

#### Out of scope
Settings panel (APP-041). Personality NM/proxy APIs (LNX-068, WIN). Resolver implementation (NET-019).

#### Acceptance criteria
- [ ] A Component holding a proxy Capability has native TLS and resolver traffic sent through the configured proxy; a packet capture shows no direct origin connect for those lookups (T-019).
- [ ] A Component without the proxy Capability does not inherit the system proxy and cannot read proxy credentials (`Error::Rights`, I-021).
- [ ] Linux-personality traffic honours the same settings via LNX-068 (I-006).
- [ ] `os inspect` shows the proxy configuration and holders.

#### Verification
- Integration: `runtime:tests/net/system_proxy_*` on `qemu-x86_64`.
- Review: LNX lead confirms personality propagation without a native POSIX proxy API.

#### Evidence
- none

### NET-033 · Verify Wi-Fi on community hardware via inherited drivers and the HCL probe
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-028, HW-079, REL-035, REL-048
- Baseline: §55, §62
- Invariants: I-054, I-095

V3 scope: Wi-Fi hardware breadth beyond target machines via inherited Linux drivers and the compatibility database. NET supplies connect, scan and DHCP scenarios the REL probe and HCL publish; HW owns chipset listing. Universal PC compatibility is not promised (I-095). Required by 1.0-G07 (Installer with FDE and Secure Boot on every Tier 1 machine): the Tier 2 community HCL rows that gate publishes carry the Wi-Fi verdicts these scenarios produce.

#### Out of scope
Chipset enablement notes (HW-079). Probe binary and HCL publication (REL-035, REL-048). Tier 1 suite (NET-037).

#### Acceptance criteria
- [ ] Connect, scan and DHCP scenarios are packaged for REL-035 and produce pass/fail records without serial numbers or burned-in MACs.
- [ ] Each scenario uses retained drivers only (I-054).
- [ ] HCL rows consumed by REL-048 name the NET scenario identifiers for Wi-Fi.
- [ ] No criterion requires every community adapter to pass (I-095).

#### Verification
- Integration: `runtime:tests/net/hcl_wifi_probe_scenarios_*` on `hw-h004` as the lab oracle.
- Review: REL and HW leads confirm scenario identifiers match HCL schema.

#### Evidence
- none

### NET-034 · Implement wired 802.1X authentication on Ethernet profiles
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-028, NET-015, SEC-027
- Baseline: none
- Threats: T-019

GAP-0296 wired 802.1X for office Ethernet at public alpha. Shares EAP machinery with NET-028 but is a distinct link type in netmgr profiles.

<!-- covers: GAP-0296 -->

#### Out of scope
Wi-Fi enterprise EAP (NET-028). Secrets storage (SEC-027). Shell UI (APP).

#### Acceptance criteria
- [ ] An Ethernet netmgr profile on H-002 completes 802.1X using EAP credentials from SEC-027 and then runs NET-003.
- [ ] Failed 802.1X leaves the link without an address and records a typed error in `os inspect`; credentials are not written to a POSIX file (T-019).
- [ ] The profile kind is distinct from Wi-Fi enterprise in the netmgr schema.
- [ ] Native software has no POSIX `wpa_supplicant` socket ABI for this path.

#### Verification
- Integration: `runtime:tests/net/wired_8021x_*` on `hw-h002`.
- Review: SEC lead confirms EAP credentials are secrets-service objects.

#### Evidence
- none

### NET-035 · Lock Layer 2 network management and resolver Interface versions for 1.x
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-015, NET-019, NET-024, NET-027, NET-006, NET-002, IPC-042, IPC-062, NET-005
- Baseline: §66
- Freezes: S-026
- Invariants: I-040

V4 exit: Layer 2 interface versions for 1.x are enumerated and locked with old-client/new-service tests. NET-owned L2 surfaces are netmgr, resolver, mDNS, VPN and S-026. NetworkConnection remains L1 and freezes with ABI, not here (I-040).

#### Out of scope
Layer 1 NetworkConnection freeze (ABI-049). Umbrella L2 version lock (IPC-068, IPC-068). Service-manifest lock (SVC-041).

#### Acceptance criteria
- [ ] S-026 and the netmgr, resolver, mDNS and VPN Interfaces have enumerated 1.x versions recorded in the surfaces register with this task as Frozen by.
- [ ] Old-client/new-service and new-client/old-service tests pass for each of those Interfaces in IPC-062.
- [ ] No Layer 1 NetworkConnection entry is marked frozen by this task (I-040).
- [ ] CI rejects an unversioned change to a locked NET L2 Interface.

#### Verification
- Integration: `runtime:tests/net/l2_evolution_*` on `qemu-x86_64`.
- Review: ABI lead confirms S-026 Frozen by this task and that L1 NetworkConnection is untouched.

#### Evidence
- none

### NET-036 · Ship folder sharing to Windows and macOS clients per the SMB-server Decision
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: NET-029, NET-012, STO-036
- Baseline: none
- Threats: T-040
- Invariants: I-006, I-021

Implements NET-029 if 1.0 ships sharing. Feature-freeze at V4 RC1 requires the server, if accepted, to exist here. Listen Capability is explicit. This task is dropped in the same change as the Decision if the accepted option is not to ship.

<!-- covers: GAP-0434 -->

#### Out of scope
SMB client Collections (STO-076). Personality Samba packaging (LNX). Listen Capability broker (NET-012).

#### Acceptance criteria
- [ ] If the accepted option ships a server, a listen-Capability-gated share is reachable from a Windows or macOS client on the lab LAN and `os inspect` shows the listen right (T-040, I-021).
- [ ] If the accepted option is Samba in the Linux personality, native software has no SMB server ABI (I-006).
- [ ] If the accepted option is do not ship, this task is dropped with `Dropped because: descoped` in the same change as the Decision.
- [ ] Revoking the listen Capability closes the share; new client connects fail.
- [ ] Shared content is a Capability-scoped Collection from STO, not an ambient home directory.

#### Verification
- Integration: `runtime:tests/net/smb_folder_share_*` on `hw-h002` when the Decision ships a server.
- Review: STO lead confirms the Collection grant; GOV maintainer confirms drop versus ship matches the Decision.

#### Evidence
- none

### NET-037 · Run the Tier 1 Wi-Fi hardware suite on every V4 release candidate
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-028, NET-033, NET-020, HW-086, LAB-023
- Baseline: §62
- Risks: R-027
- Invariants: I-095

V4 hardware gate: every Tier 1 machine passes the full suite including Wi-Fi each RC. Turns V1/V2 connect, roam, suspend-restore and enterprise scenarios into a published per-machine matrix. LAB hosts the runs; NET owns the cases. Desktops without Wi-Fi record an explicit not-applicable row rather than a silent pass.

#### Out of scope
Combined hardware suite (HW-086). Lab fleet (LAB-023). HCL publication (REL, HW-088).

#### Acceptance criteria
- [ ] Connect, roam, suspend-restore and enterprise cases from V1/V2 run on every Tier 1 laptop in the hardware register each RC.
- [ ] Tier 1 desktops without Wi-Fi record not-applicable with the Network field from `registers/hardware.md`; they do not count as Wi-Fi passes.
- [ ] Per-machine results are attached to HW-086 reports.
- [ ] Universal PC compatibility is not claimed (I-095).

#### Verification
- Integration: `runtime:tests/net/tier1_wifi_suite_*` on every Tier 1 laptop CI entry.
- Review: HW lead confirms NET cases are included in the combined suite.

#### Evidence
- none

### NET-038 · Verify published HCL Wi-Fi claims on every Tier 1 machine for 1.0
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-037, REL-048, HW-088, REL-066
- Baseline: §63
- Invariants: I-095

1.0 definition: Tier 1 HCL lists Wi-Fi as working on the machines whose Network field includes Wi-Fi. Re-runs NET-037 on the soak candidate and attaches results to the published HCL. REL publishes; NET owns pass/fail.

#### Out of scope
HCL publication (REL-048, HW-088). Soak calendar (REL-066, LAB-025). Combined feature sign-off (HW-089).

#### Acceptance criteria
- [ ] Every Tier 1 machine whose `registers/hardware.md` Network field includes Wi-Fi has a pass from NET-037 on the 1.0 soak candidate.
- [ ] Those passes are attached to the published HCL rows REL-048 serves.
- [ ] Machines without Wi-Fi stay not-applicable and are not advertised as Wi-Fi working (I-095).
- [ ] A mismatch between HCL text and suite results fails this task.

#### Verification
- Integration: re-run of `runtime:tests/net/tier1_wifi_suite_*` on the soak candidate.
- Review: REL lead confirms HCL rows match NET pass/fail.

#### Evidence
- none

### NET-039 · Write the retained TCP/IP and Wi-Fi CVE triage runbook for 1.x support
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-001, KRN-031, REL-006
- Baseline: §56.4, §63
- Invariants: I-054

1.0 support: inherited High/Critical CVEs ship within the CVE SLA. This runbook maps tcp/ip, nftables, cfg80211 and supplicant CVEs onto NetworkConnection and netmgr exposure and names the regression tests to run.

#### Out of scope
Kernel CVE pipeline (KRN-030, REL-006). SLA target register (REL-060). MemoryObject mm CVEs (MEM-055).

#### Acceptance criteria
- [ ] A committed runbook maps tcp/ip, nftables, cfg80211 and supplicant CVEs onto NetworkConnection, netmgr, resolver and Wi-Fi station exposure.
- [ ] Each mapping names the NET regression tests to run after a backport.
- [ ] The runbook is cited from KRN-031 as the NET chapter.
- [ ] Documentation review sign-off is recorded on the pull request.

#### Verification
- Review: KRN lead and REL lead sign-off recorded on the pull request.

#### Evidence
- none

### NET-040 · Decide whether to ship a native typed remote shell after 1.0
- Type: adr
- Milestone: LATER
- Status: todo
- Size: S
- Owner: none
- Depends on: NET-017
- Baseline: §43, §57
- Decision: D-0200
- Threats: T-040
- Invariants: I-047

EXTRA-020 later half: sshd via the personality is V1; a native typed remote shell is post-1.0. Options: keep sshd only; a Channel-based remote shell honouring Capabilities; an SSH-compatible native daemon. §43 forbids making distribution a kernel concern; this adr stays in userspace (I-047).

<!-- covers: EXTRA-020 -->

#### Out of scope
V1 sshd (NET-017). Remote-interface Capability rules (SEC-079). Kernel distribution (forbidden, I-047).

#### Acceptance criteria
- [ ] Options evaluated include (A) keep sshd in the Linux personality only, (B) a Channel-based remote shell honouring Capabilities, and (C) an SSH-compatible native daemon.
- [ ] Every option records that the implementation is userspace (I-047) and that listen remains an explicit Capability (T-040).
- [ ] The accepted option names whether a post-1.0 NET build task is required.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and SEC lead sign-off recorded on the pull request, confirming userspace-only and at least two options.

#### Evidence
- none
