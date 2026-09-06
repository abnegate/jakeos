# Threat register

This register names the adversaries, assets and vectors that CAP, SEC and BOOT decisions must cite. It is owned by SEC. Status starts `open`. `Addressed by` stays `none` until tasks exist. The V0 threat-model document enumerates these entries; it does not invent threats that are not here. Definitions live here; incident counts do not.

### T-001 · Ambient authority
- Actor: malicious native application
- Asset: user files, devices, network, other applications
- Vector: a component starts with more authority than it was handed, or inherits a POSIX-shaped default
- Status: open
- Addressed by: none
The founding threat of §9.1. A native application must not automatically receive filesystem, home, network, camera, microphone, process-enumeration, device, clipboard, location, screen or other-application access.

### T-002 · Confused deputy
- Actor: malicious component
- Asset: a privileged service's capabilities
- Vector: a service acts on a request using its own authority rather than the caller's attenuated capability
- Status: open
- Addressed by: none
File choosers, portals, the package manager and personality bridges are classic deputies.

### T-003 · Capability forgery
- Actor: malicious native application
- Asset: kernel object table
- Vector: userspace mints or guesses a handle that names an object it was never given
- Status: open
- Addressed by: none
Capabilities are unforgeable. Handle-table generation and type tags exist to make this fail with `Error::Rights`.

### T-004 · Rights amplification
- Actor: malicious native application
- Asset: derived capabilities
- Vector: `derive` with a mask that is not a subset, or a rights word that can be widened in place
- Status: open
- Addressed by: none
Attenuation is monotonic. S-003 is the encoding that a future hardware-tag path can check.

### T-005 · Revocation race
- Actor: malicious holder of a derived capability
- Asset: objects the user has revoked
- Vector: use-after-revoke of in-flight Operations, or a derivation tree that is not walked within one operation
- Status: open
- Addressed by: none
Q-004 records the open semantic questions. The V0 gate requires that every derived capability fails within one operation.

### T-006 · Malicious package
- Actor: malicious publisher
- Asset: the content-addressed store, user data, the signing root
- Vector: a package that requests excessive capabilities, ships a trojan component, or substitutes a dependency
- Status: open
- Addressed by: none
Install-time capability review, content addressing, signatures and revocation of a published package are the controls.

### T-007 · Supply chain
- Actor: compromised dependency or build worker
- Asset: system generations, SDK, kernel
- Vector: a GPLv2-incompatible or trojaned crate, a compromised CI runner, or an unsigned firmware blob
- Status: open
- Addressed by: none
Allowlists, reproducible builds, signed generations and vendor-firmware review address this.

### T-008 · Evil maid, device off
- Actor: attacker with physical access to a powered-off machine
- Asset: disk contents, firmware, ESP
- Vector: disk removal, ESP replacement, firmware implant, bootloader swap
- Status: open
- Addressed by: none
Full-disk encryption by default, UEFI Secure Boot, measured boot and a signed generation manifest are the controls.

### T-009 · Evil maid, device suspended
- Actor: attacker with physical access to a suspended or locked machine
- Asset: unlocked memory, session, disk keys
- Vector: DMA over Thunderbolt/USB4, cold-boot, lid-open while locked, compositor crash-to-unlock
- Status: open
- Addressed by: none
IOMMU plus device authorisation, lockdown, a compositor that restarts locked, and secrets that stay in the TPM or a sealed keyring.

### T-010 · Stolen laptop
- Actor: thief of a powered-off or locked laptop
- Asset: user data, credentials, keys
- Vector: offline disk attack, guessable login, leftover hibernation image
- Status: open
- Addressed by: none
FDE default, hardware-backed secrets, unsigned hibernation images rejected under lockdown.

### T-011 · Personality escape
- Actor: malicious Linux or Windows application
- Asset: native objects, other prefixes, the host capability set
- Vector: a personality process reaches a resource its enclosing component was not granted, via uid, file mode, `/proc`, a shared Wine prefix, or a raw syscall
- Status: open
- Addressed by: none
Personality authority is bounded by the enclosing component's capability set regardless of POSIX or Win32 checks.

### T-012 · Compositor spoof
- Actor: malicious GUI application
- Asset: user consent, credentials, screen contents
- Vector: overlay, clickjacking, fake permission prompt, fake chooser, synthesised input into a trusted surface
- Status: open
- Addressed by: none
Permission prompts, file choosers and elevation render through a privileged trusted-UI component in compositor-protected surfaces.

### T-013 · Unauthorised screen capture
- Actor: malicious application
- Asset: on-screen contents of other applications and the shell
- Vector: a capture API that is ambient, or a shared GPU resource that leaks pixels
- Status: open
- Addressed by: none
Screen capture is S-034. Without the capability the application receives a denied or black surface, and sharing shows a persistent indicator.

### T-014 · Microphone and camera
- Actor: malicious application
- Asset: audio and video of the user
- Vector: ambient device nodes, a personality `/dev/video` or ALSA device, or a codec component that is over-privileged
- Status: open
- Addressed by: none
Camera and microphone are explicit capabilities with a persistent in-use indicator. Codec components receive only the buffers they decode.

### T-015 · Microarchitectural side channel
- Actor: malicious co-resident component
- Asset: secrets in another ResourceDomain
- Vector: shared glyph atlas, shared MemoryObjects, SMT siblings, GPU caches, timing of Channel operations
- Status: open
- Addressed by: none
The V1 side-channel position statement names what the capability model claims and which mitigations are on by default. The glyph atlas is a read-only object minted by a text service.

### T-016 · Resource exhaustion
- Actor: malicious or buggy component
- Asset: kernel memory, handles, CPU, GPU, storage
- Vector: unbounded handle, task, channel, MemoryObject or outstanding-Operation allocation; fork bombs in a personality
- Status: open
- Addressed by: none
ResourceDomain kernel-object limits return typed exhaustion errors. Personality forks count against the enclosing domain.

### T-017 · Malicious AI agent
- Actor: an AI-broker component acting beyond its grants
- Asset: user data, other applications, network
- Vector: omnipotent assistant, GUI automation, unscoped grants, actions not on the audit log
- Status: open
- Addressed by: none
The broker is a distinct principal, holds only granted capabilities, invokes typed interfaces, and every action is logged and revocable. No AI work precedes a done semantic-registry task.

### T-018 · Malicious automation rule
- Actor: a background rule or a rule the user did not intend
- Asset: files, environments, network
- Vector: GUI scraping, a rule that runs without the persistent-background capability, file-type confusion
- Status: open
- Addressed by: none
Automation uses semantic interfaces only. Background rules require the explicit persistent-background capability.

### T-019 · Network attacker
- Actor: off-machine adversary
- Asset: credentials, update channel, user traffic
- Vector: MITM of the package repository, rogue access point, unauthenticated listen, DNS hijack
- Status: open
- Addressed by: none
Signed repositories, TLS with a typed CA store, listen as an explicit capability, and no silent remote access.

### T-020 · Rogue driver
- Actor: compromised or malicious kernel or user-space driver
- Asset: physical memory, DMA, firmware
- Vector: unsigned module, user-space driver without IOMMU, Thunderbolt PCIe tunnel
- Status: open
- Addressed by: none
Modules signed under Secure Boot, user-space drivers behind IOMMU, Thunderbolt/USB4 authorisation before tunnelling.

### T-021 · Firmware implant
- Actor: attacker who can write SPI, EC or GPU firmware
- Asset: the boot chain below the OS
- Vector: unsigned firmware update, evil-maid firmware flash, vendor update without history
- Status: open
- Addressed by: none
Firmware updates are signed packages recorded as system-history events. Measured boot records firmware identity.

### T-022 · Generation downgrade
- Actor: attacker who can select an older generation
- Asset: patched vulnerabilities, measured-boot state
- Vector: boot-menu selection of a pre-fix generation, unsigned kexec, rollback of a security-fix watermark
- Status: open
- Addressed by: none
The anti-rollback decision records whether generations older than a security-fix watermark are blocked. kexec of unsigned images is forbidden under lockdown.

### T-023 · Telemetry leak
- Actor: the project, a compromised intake, or a curious operator
- Asset: user files, identifiers, crash dumps
- Vector: a telemetry agent with ambient authority, dumps that contain disk keys, unreviewed uploads
- Status: open
- Addressed by: none
Opt-in only. The user sees and can redact the report. The agent holds only the capabilities it needs. Dumps never contain disk keys or unlocked secrets.

### T-024 · Malicious DMA peripheral
- Actor: a Thunderbolt, USB4 or PCIe device
- Asset: physical memory, MemoryObjects
- Vector: DMA from a device not behind the IOMMU
- Status: open
- Addressed by: none
Target hardware requires IOMMU. Ownership-transfer of DMA-suitable MemoryObjects is unsafe without it.

### T-025 · Compromised compatibility application
- Actor: a Linux or Windows application that is malicious or exploited
- Asset: native grants the personality holds
- Vector: the personality is a confused deputy for chooser, clipboard, network or GPU capabilities
- Status: open
- Addressed by: none
Each personality application is its own component with the compatibility-default capability set. A Linux-personality browser download grants a native editor access to exactly that file, not a directory.

### T-026 · Local unprivileged user
- Actor: another login on a multi-user machine
- Asset: the first user's capability store, encrypted data, session
- Vector: shared tmp, shared Wine prefix, readable crash dumps, session switch that leaks surfaces
- Status: open
- Addressed by: none
V3 multi-user: separate sessions, separate capability stores, separate encrypted data. Session switch preserves state and does not leak surfaces.

### T-027 · Debugger attach
- Actor: malicious application or a local user
- Asset: another component's memory and capabilities
- Vector: ptrace-style same-uid attach, personality gdb, Crashpad
- Status: open
- Addressed by: none
Debugger attachment, tracing of another component and reading its memory are explicit debug capabilities, not same-user checks.

### T-028 · Update channel compromise
- Actor: attacker who can publish on an update channel
- Asset: every machine that takes the channel
- Vector: stolen signing key, malicious delta, a generation that boots and then exfiltrates
- Status: open
- Addressed by: none
Key hierarchy with offline root, automatic rollback on failed boot, boot counting, and a rehearsed key-rotation drill.

### T-029 · Key compromise
- Actor: attacker who obtains a release or package signing key
- Asset: the entire installed base
- Vector: HSM mishandling, a leaked CI secret, a compromised maintainer
- Status: open
- Addressed by: none
Custody, quorum, rotation and a compromise-response runbook are REL and GOV work before V3 public signing.

### T-030 · Shared writable atlas
- Actor: malicious text or GUI component
- Asset: pixels and font-parsing attack surface of other components
- Vector: a shared writable GPU glyph atlas as a cross-component MemoryObject
- Status: open
- Addressed by: none
The atlas is a read-only object minted by a text service. A writable shared atlas is a cross-domain channel.

### T-031 · Overlay and scanout leak
- Actor: malicious compositor client
- Asset: other clients' frames, HDR metadata, cursor
- Vector: overlay planes, direct scanout, or a hardware cursor that samples another surface
- Status: open
- Addressed by: none
Plane assignment does not grant a client the contents of another client's buffer.

### T-032 · Personality primary-selection leak
- Actor: Linux GUI application
- Asset: native clipboard
- Vector: X11 primary selection crossing the Wayland bridge into the native clipboard
- Status: open
- Addressed by: none
Primary selection stays inside the bridge.

### T-033 · Persistent grant after publisher change
- Actor: a replacement package with a different publisher
- Asset: the user's prior grants
- Vector: grants keyed on content hash or package name rather than publisher identity
- Status: open
- Addressed by: none
Persistent grants key on package identity plus publisher and revoke on publisher change.

### T-034 · Running mixed-version tree
- Actor: a partially applied generation
- Asset: ABI assumptions of running components
- Vector: a running component observes a mixed-version package tree after an update
- Status: open
- Addressed by: none
Old objects stay mapped; activation of the new generation is deferred until restart. No running component observes a mixed-version tree.

### T-035 · Recovery-environment escape
- Actor: attacker who can boot recovery
- Asset: unlocked user volumes, generation store
- Vector: recovery that mounts user data without authentication, or a signed recovery that is older than the watermark
- Status: open
- Addressed by: none
Recovery authenticates, and the anti-rollback policy applies to recovery images.

### T-036 · Guest VM breakout
- Actor: malicious guest
- Asset: host capabilities, other VMs
- Vector: a VM manager that grants folders, devices, network or clipboard ambiently
- Status: open
- Addressed by: none
Each VM is a Component in a ResourceDomain. Host access is granted through capabilities visible in `os inspect`.

### T-037 · Unsigned kexec and command line
- Actor: local attacker with a boot-menu or kexec path
- Asset: Secure Boot and measured-boot state
- Vector: `init=/bin/sh`, `lockdown=off`, unsigned kexec
- Status: open
- Addressed by: none
The kernel command line is part of the signed generation. Unsigned kexec is forbidden under lockdown. Developer mode is a persistent on-screen indicator.

### T-038 · Codec sandbox escape
- Actor: malicious media file
- Asset: the decoder's neighbours
- Vector: a parser bug in an over-privileged ImageDecoder or video decoder
- Status: open
- Addressed by: none
Codecs are isolated components with no network, no arbitrary filesystem, no microphone and no process enumeration. Hardware decode uses MemoryObject transfer.

### T-039 · Screen-reader spoof
- Actor: malicious application
- Asset: the user's assistive-technology view of trusted UI
- Vector: a fake a11y tree, or injecting nodes into the trusted chooser
- Status: open
- Addressed by: none
The accessibility tree of trusted UI is produced by the trusted-UI component, not by the requesting application.

### T-040 · Remote shell overreach
- Actor: remote attacker or a developer tool left listening
- Asset: the development machine
- Vector: an sshd or native remote shell with ambient authority
- Status: open
- Addressed by: none
Listen is an explicit network capability. V1 remote shell is via the Linux personality with that capability visible.

### T-041 · Hibernation image
- Actor: evil maid or thief
- Asset: RAM contents including keys
- Vector: an unsigned or plaintext hibernation image
- Status: open
- Addressed by: none
Unsigned hibernation images are rejected under lockdown. Confidentiality lockdown disables them.

### T-042 · Community probe over-collection
- Actor: the project or a third party reading HCL submissions
- Asset: serial numbers, MAC addresses, user identity
- Vector: a hardware probe that uploads identifiers
- Status: open
- Addressed by: none
Hardware and compatibility submissions are opt-in, pseudonymous, reviewable before upload, and contain no serial numbers or network identifiers.

### T-043 · Rogue HID device
- Actor: a USB or Bluetooth device that presents itself as a keyboard or pointer
- Asset: trusted surfaces, the locked session, the user's consent
- Vector: a newly attached device injects keystrokes into the unlock prompt, a permission prompt or the chooser, or enumerates while the session is locked
- Status: open
- Addressed by: none
Input from a device is not consent. New USB devices are not enumerated while the session is locked, and a new keyboard is confirmed by the user before its input reaches a trusted surface.

### T-044 · Hostile filesystem image
- Actor: a removable drive or foreign partition carrying a crafted filesystem image
- Asset: the kernel, the mounting Component, user data
- Vector: a malformed ext4, exFAT, NTFS or LUKS image exploits an inherited C filesystem parser when auto-mounted or probed
- Status: open
- Addressed by: none
Inherited filesystem drivers parse attacker-controlled bytes in kernel mode. The mounting policy decides which types are parsed in an isolated Component, which stay in the kernel with restrictions, and what auto-mount does.

### T-045 · Wasm runtime escape
- Actor: a malicious or exploited Wasm Component
- Asset: the host Component, Capabilities in the import table, MemoryObjects it was lent
- Vector: a bug in the JIT, the host ABI or the Capability import table lets guest code reach beyond its declared imports
- Status: open
- Addressed by: none
The Wasm runtime is a Component like any other; an escape gains only that Component's Capabilities. The host ABI and import table are fuzzed continuously and the runtime never runs in the kernel.

### T-046 · Assistive-technology client over-reach
- Actor: an application holding an accessibility-tree or automation Capability
- Asset: the contents and actions of every other application's UI
- Vector: an assistive-technology bridge that reads any window or synthesises any action becomes a universal sandbox bypass
- Status: open
- Addressed by: none
Accessibility access is `Capability<AccessibilityTree>` with redaction, granted per client and visible in the permissions UI; secret fields are redacted and trusted-UI trees come from the trusted-UI Component.
