# Compatibility corpora

This register defines the Linux and Windows compatibility corpora that milestone gates cite. It is owned by LNX for C-001 through C-006 and C-010, and by WIN for C-007 through C-009 and C-011. Each entry records the personality, the entry count, the scenario-harness alias, the rating scale and the per-milestone threshold. Thresholds are definitions, never measured pass rates; results live in `reports/compat/`. A compatibility gate cites a C-ID and never restates a number.

### C-001 · Linux L0 syscall and busybox corpus
- Personality: linux
- Size: 1000 Linux Test Project syscall tests plus a busybox root filesystem, bash, coreutils, python3 and a static Go binary
- Scenario: compat:linux-L0
- Scale: pass, fail
- Thresholds: V0 zero regressions versus the unforked baseline kernel of the same version on the same hardware; V0.5 zero regressions versus V0; V1 zero regressions versus V0.5; V2 zero regressions versus V1; V3 zero regressions versus V2; V4 zero regressions versus V3; 1.0 zero regressions versus V4
- Status: defined
The V0 compatibility firewall: the unmodified Linux syscall ABI still works on the fork.

### C-002 · Linux L1 developer-tool and Wayland corpus
- Personality: linux
- Size: 20 CLI and developer tools (git, gcc, clang, python3, node, rustc/cargo, bash, tmux, vim, curl, ssh, make, tar and peers) each with a scripted scenario, plus 5 Wayland-native GUI applications
- Scenario: compat:linux-L1
- Scale: pass, fail, with integration scoring for window, input and clipboard on GUI entries
- Thresholds: V0.5 CLI pass 100%, GUI pass ≥ 80%; V1 zero regressions versus V0.5; V2 zero regressions versus V1; V3 zero regressions versus V2; V4 zero regressions versus V3; 1.0 zero regressions versus V4
- Status: defined

### C-003 · Linux L2 daily-driver corpus
- Personality: linux
- Size: 50 applications with scripted scenarios covering a Chromium-based browser, a Firefox-based browser, a mainstream IDE, a GIMP-class image editor, a LibreOffice-class suite, a Qt application, a GTK application, an Electron application, an XWayland-only application, an OCI container runtime, Flatpak and 30 CLI tools
- Scenario: compat:linux-L2
- Scale: pass, fail, with integration scoring for launcher, task switcher, clipboard, file chooser via portal, notifications and scaling
- Thresholds: V1 pass ≥ 90%, with 100% for the browser, IDE, container runtime and Git entries; V2 zero regressions versus V1; V3 zero regressions versus V2; V4 zero regressions versus V3; 1.0 zero regressions versus V4
- Status: defined
Seccomp-bpf, user namespaces, overlayfs, ptrace-equivalent and file-watch Operations are in place before this corpus gates.

### C-004 · Linux L3 desktop corpus
- Personality: linux
- Size: 150 applications including distribution packages, games via Steam on Linux, media players, creative tools and video conferencing
- Scenario: compat:linux-L3
- Scale: pass, fail, with integration scoring for launcher, clipboard, notifications, file chooser, audio and scaling
- Thresholds: V2 pass ≥ 85%; V3 zero regressions on Gold entries versus V2; V4 zero regressions on Gold entries versus V3; 1.0 zero regressions on Gold entries versus V4
- Status: defined
C-010 is a named sub-corpus scored inside this corpus.

### C-005 · Linux L4 popularity corpus
- Personality: linux
- Size: 300 applications drawn from Flathub and distribution popularity data plus the Steam-on-Linux top list without kernel anti-cheat
- Scenario: compat:linux-L4
- Scale: pass, fail, with integration scoring as for C-004
- Thresholds: V3 pass ≥ 85%; V4 zero regressions on Gold entries versus V3; 1.0 zero regressions on Gold entries versus V4
- Status: defined

### C-006 · Linux L5 1.0 corpus
- Personality: linux
- Size: 500 applications
- Scenario: compat:linux-L5
- Scale: pass, fail, with integration scoring as for C-004
- Thresholds: V4 pass ≥ 90%, with 100% for the browser, IDE, container runtime, Steam client and office-suite entries; 1.0 the V4 thresholds with zero regressions on Gold entries versus V4
- Status: defined

### C-007 · Windows W1 proof corpus
- Personality: windows
- Size: 50 titles (35 games without kernel-level anti-cheat drawn from widely played lists, 15 productivity applications)
- Scenario: compat:windows-W1
- Scale: Platinum, Gold, Silver, Bronze, Broken
- Thresholds: V2 Gold ≥ 50%, Silver ≥ 70%, at least 10 Gold with HDR or VRR exercised; V3 zero Gold-to-lower regressions versus V2; V4 zero Gold-to-lower regressions versus V3; 1.0 zero Gold-to-lower regressions versus V4
- Status: defined
Integration scoring covers taskbar, launcher, notifications, clipboard, audio, file chooser and gamepad input.

### C-008 · Windows W2 alpha corpus
- Personality: windows
- Size: 150 titles (110 games without kernel-level anti-cheat, 40 applications)
- Scenario: compat:windows-W2
- Scale: Platinum, Gold, Silver, Bronze, Broken
- Thresholds: V3 Gold ≥ 60%, Silver ≥ 80%; V4 zero Gold-to-lower regressions versus V3; 1.0 zero Gold-to-lower regressions versus V4
- Status: defined
A public per-title report with the rating scale is published with each V3 and later gate run.

### C-009 · Windows W3 1.0 corpus
- Personality: windows
- Size: 300 titles (220 games without kernel-level anti-cheat, 80 applications)
- Scenario: compat:windows-W3
- Scale: Platinum, Gold, Silver, Bronze, Broken
- Thresholds: V4 Gold ≥ 70%, Silver ≥ 85%; 1.0 the V4 thresholds with zero Gold-to-lower regressions versus V4
- Status: defined
Per-title public reports and scenario scripts are reproducible by third parties.

### C-010 · Steam runtime sub-corpus
- Personality: linux
- Size: Steam client plus pressure-vessel, gamescope, 32-bit multilib, /dev/input via udev and SDL game-controller evdev titles drawn from the Steam-on-Linux list without kernel anti-cheat
- Scenario: compat:steam-runtime
- Scale: pass, fail, with integration scoring for input, overlay, audio and GPU acceleration
- Thresholds: V2 scripted scenarios exist and are scored as part of C-004; V3 scored as part of C-005; V4 scored as part of C-006 with the Steam client at 100%; 1.0 the V4 thresholds
- Status: defined
This is a named LNX sub-corpus, not an independent pass-rate gate. The 32-bit decision is made at V1 so syscall pruning cannot delete ia32 emulation before Steam is in scope.

### C-011 · Wine test-suite corpus
- Personality: windows
- Size: upstream Wine test suite as run by the Wine project, executed under the Linux personality
- Scenario: compat:wine-tests
- Scale: pass, fail, skip, per upstream Wine test classification
- Thresholds: V1 publish, no pass-rate gate; V2 used as a bring-up signal for C-007, no independent threshold; V3 publish; V4 publish; 1.0 publish
- Status: defined
Non-gated at V1: the suite runs in nightly CI so V2 is not a research programme. A feasibility report accompanies the V1 publish.

<!-- roadmap:generated:begin status -->
<!-- roadmap:generated:end -->
