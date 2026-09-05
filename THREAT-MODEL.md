# JakeOS threat model

This document is the narrative behind `registers/threats.md`. The register is the only place a threat is defined and given an identifier; this document explains who the adversaries are, what they want, where the trust boundaries lie, which safety layers stand between them and their targets, and how far a single compromise can reach. Every CAP, SEC and BOOT decision task cites the T-IDs it addresses in its `Threats:` field; a decision that addresses no registered threat is either redundant or the register is incomplete, and the register is fixed first.

Baseline sections that shape this model: §9 (no ambient authority), §9.1 (the PhotoEditor example), §11 (applications are component graphs, an exploit compromises the smallest useful unit), §51 (memory safety at several layers), §32 (failure and recovery are normal), §57 (security comes from authority design, not deny lists).

## 1. What we protect

| Asset | Where it lives | Who may touch it |
|---|---|---|
| User data | files, Collections, ApplicationData, encrypted volumes | only Components holding a Capability the user or the OS granted for that object |
| User attention and consent | the compositor's trusted surfaces, permission prompts, the chooser | the shell and the compositor alone; no application can draw over or synthesise input into them |
| Device access | camera, microphone, screen contents, location, storage devices | Components holding a device Capability, with a persistent indicator while in use |
| Kernel object table | Capabilities, Components, Channels, MemoryObjects, ResourceDomains | the kernel; user space names objects only through unforgeable handles |
| Disk keys and secrets | TPM-sealed disk keys, the secrets service, signing roots | the boot chain and the secrets service; never a general Component |
| The boot chain and system generations | firmware, boot manager, kernel, generation store | signed updates only; every generation is measured and roll-back below a watermark is refused |
| The update channel and package store | signing keys, repository, content-addressed store | release engineering with hardware-held keys; verification happens on every machine before activation |
| Other users on the machine | per-user capability stores, sessions, encrypted homes | the owning user only |

## 2. Adversaries

Each adversary is named here and enumerated as T-IDs in the register. The T-ID list is the authoritative mapping.

| Adversary | Position | Wants | Register entries |
|---|---|---|---|
| Malicious native application | a Component the user installed and ran | ambient authority it was never handed, other applications' data, devices, the screen | T-001, T-003, T-004, T-013, T-014, T-016 |
| Compromised Component | a legitimate Component that has been exploited (a decoder, a parser, a renderer) | to reach beyond its own capability set into its neighbours and the service that hosts it | T-002, T-005, T-015, T-016, T-038 |
| Compromised compatibility application | a Linux or Windows program running inside a personality, malicious or exploited | the native grants the personality holds on its behalf, other prefixes, the host | T-011, T-025, T-032 |
| Local unprivileged user | another login on the same machine | the first user's capability store, encrypted data, session surfaces, crash dumps | T-026, T-027, T-035 |
| Evil maid, device off | physical access to a powered-off machine | to alter what boots, read the disk, implant firmware | T-008, T-021, T-022, T-037, T-041 |
| Evil maid, device suspended or locked | physical access to a suspended or locked machine | unlocked memory, disk keys, the live session | T-009, T-010, T-024, T-041 |
| Malicious DMA peripheral | a Thunderbolt, USB4 or PCIe device the user plugs in | physical memory and MemoryObjects without going through the kernel | T-024, T-020 |
| Network attacker | on the path between the machine and the world | credentials, the update channel, user traffic, an unauthenticated listener | T-019, T-028, T-040 |
| Supply-chain attacker | inside a dependency, a build worker, a signing process or a firmware vendor | to ship code the project did not write to every machine | T-006, T-007, T-028, T-029 |
| Over-reaching automation or assistant | an AI broker or a background rule acting beyond its grants | user data and actions the user did not intend | T-017, T-018 |
| The project itself | telemetry intake, compatibility probes, recovery tooling | more data than the user consented to | T-023, T-042, T-035 |

Threats that no adversary above claims exclusively are still registered because a design must answer them: T-012 (compositor spoof), T-030 (shared writable atlas), T-031 (overlay and scanout leak), T-033 (grant survives publisher change), T-034 (mixed-version tree), T-036 (guest VM breakout), T-039 (screen-reader spoof). They are cited by the GFX, TXT, PKG, VIRT and ACC decisions that own them.

## 3. Trust boundaries

1. **Kernel and user space.** The native ABI is the only boundary. Everything above it holds authority solely as Capabilities; the kernel never consults a path, a user id or a process tree to decide access. This boundary is what makes T-001, T-003 and T-004 design questions rather than policy questions.
2. **Component and Component.** Two Components in different ResourceDomains share nothing unless a MemoryObject or Channel Capability was explicitly passed. Memory safety of the language (§51) is not part of this boundary; the boundary must hold when the code inside a Component is hostile.
3. **Personality and native.** A personality (Linux or Windows) is one or more Components. Everything its guests can reach is bounded by what the personality Component holds. The personality is a confused-deputy risk by construction (T-025): it holds chooser, clipboard, GPU and network grants on behalf of many guests and must attenuate them per guest.
4. **Trusted surfaces and applications.** The compositor draws permission prompts, the chooser, indicators and the lock screen in surfaces no application can cover, sample or inject input into (T-012, T-013, T-031, T-039).
5. **Boot chain and OS.** Firmware, the boot manager and the kernel are measured; generations below the security watermark do not boot (T-008, T-022, T-037); disk keys are sealed to the measured state (T-009, T-010, T-041).
6. **Machine and world.** Everything that leaves the machine is authenticated and encrypted; nothing listens without a Capability that names the listen right (T-019, T-040); packages and generations are verified before activation, never trusted because of where they came from (T-028, T-029).
7. **User and user.** Sessions, capability stores and encrypted homes are per user; nothing shared by default (T-026).

## 4. Safety layers

Security comes from several independent layers (§51). No single layer is assumed to hold.

| Layer | Stops | Fails when |
|---|---|---|
| Capability model (CAP) | ambient authority, forgery, amplification, confused deputies when services use the caller's capability | a service acts with its own authority (T-002), or revocation is not walked in one operation (T-005) |
| Component isolation (CMP, SCH) | a compromised Component reaching its neighbours' memory or exhausting shared resources | shared MemoryObjects are writable by both sides (T-030), or per-domain bounds are missing (T-016) |
| Language safety (Rust-first, unsafe inventory) | memory-corruption bugs in new code | inherited C, `unsafe` blocks, and hardware |
| Trusted surfaces (GFX, APP) | spoofed consent, clickjacking, capture | overlay planes or scanout leak pixels (T-031), synthesised input reaches a trusted surface (T-012) |
| Boot integrity (BOOT, SEC) | evil-maid modification, downgrade, unsigned kexec | firmware below the OS is compromised (T-021), or the watermark is not enforced by the boot manager (T-022) |
| Encryption at rest (SEC, STO) | data theft from a powered-off or stolen machine | keys survive in a hibernation image (T-041) or memory (T-009) |
| IOMMU (HW, KRN) | DMA from peripherals and user-space drivers | a device is not behind the IOMMU (T-024), or a driver is loaded without it (T-020) |
| Signing and verification (REL, PKG) | malicious or substituted packages and generations | a signing key leaks (T-029) or verification happens after activation (T-034) |
| Audit and revocation (OBS, CAP, SEM) | silent over-reach by assistants, rules and services | an action is not on the log (T-017), or grants outlive their justification (T-033) |
| Hardware enforcement (future, §8) | forged or widened capabilities even when kernel metadata is wrong | not available on the initial x86-64 targets; the ABI leaves room for it |

## 5. Blast radius: the ImageDecoder case (§11)

An ImageDecoder is the canonical smallest useful unit. It holds exactly: an input Channel carrying image bytes, an output Channel for bitmaps, a MemoryObject budget of the order named in §11, and a CPU share. It holds no filesystem, network, device, clipboard, screen or package Capability.

If a malicious image achieves code execution inside it (T-038), the attacker can:

- read and corrupt the bytes and bitmaps flowing through its two Channels;
- consume its own memory and CPU budget, and no more (T-016 is bounded by the ResourceDomain);
- crash it, which the supervisor observes as a typed exit cause and restarts (§32).

The attacker cannot:

- reach any file, including the image's source file, because the decoder never held a file Capability;
- reach the network, the clipboard, the screen or another application;
- forge a handle to anything else (T-003) or widen the rights it has (T-004);
- persist, because the Component has no writable storage and the package is immutable;
- attack a neighbour through shared memory, because the bitmap MemoryObject is transferred, not shared, and the glyph atlas is read-only (T-030);
- exceed the blast radius by exploiting the supervisor, because the supervisor uses the decoder's attenuated capability, not its own, when acting on the decoder's behalf (T-002).

Side channels (T-015) remain: a co-resident decoder can observe timing of shared caches and SMT siblings. The side-channel position statement (SEC) states which of these the project mitigates by default and which it accepts.

The same analysis must be written for every Component class that parses untrusted input: video decoders (MED), font shapers (TXT), the IDL wire decoder (IPC), the personality bridges (LNX, WIN) and the compositor's client protocol (GFX). Each is a task in its workstream and cites this section.

## 6. Requirements this model places on decisions

- Every CAP, SEC and BOOT adr task lists the T-IDs it addresses in `Threats:`. The validator checks that the IDs exist; review checks that the mapping is honest.
- A decision that changes a trust boundary (sections 3.1 to 3.7) names the boundary in its Consequences.
- A new adversary or vector is added to `registers/threats.md` first, then cited; this document is updated in the same change.
- Spike reports for capability encoding, handle representation, revocation and MemoryObject transfer record which T-IDs their measured design defeats and which it leaves to another layer.
- The Linux and Windows personalities are treated as untrusted Components holding many grants; every personality design decision cites T-011 and T-025.

## 7. Out of scope for this document

The compositor protocol section (GFX), the font and atlas section (TXT), and the installer and updater section (INS) are written by those workstreams against the same register. This document introduces no threat that the register does not already enumerate; if one is found missing while reading it, the register is the place to add it.
