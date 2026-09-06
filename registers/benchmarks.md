# Benchmark register

This register holds every metric the project tracks: one entry per §54 metric plus every distinct benchmark gate in the milestone ladder. It is owned by BEN. Each entry defines the metric, the measurement method, the harness alias, the comparison baselines and the per-milestone target kind (`absolute`, `regression` or `publish`). The register holds definitions and targets only; measured values live in benchmark reports and in the generated `results` block, never in the entries. A milestone gate cites a B-ID and never restates a number. Every V0 target is `publish`: V0 measures and publishes, it does not promise. No performance claim may be made anywhere in the project without a report produced by the harness named here (I-061).

### B-001 · Component creation latency
- Metric: Wall-clock latency from a create-component request to the new component's first instruction executing, for a minimal component mapped from an immutable package.
- Method: p50 and p99 over 10 runs of 10,000 creations, warm, CPU frequency pinned, default mitigations, on every hardware entry in the milestone's scope.
- Harness: bench:component-create
- Baselines: Linux fork+exec of a static binary, clone(CLONE_VM), posix_spawn, podman run of a minimal OCI image
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 absolute p50 ≤ 100 µs; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 absolute p50 ≤ 100 µs; 1.0 absolute p50 ≤ 100 µs
- Status: defined
If the V0 published p50 exceeds 500 µs an accepted decision documenting root cause and remediation is required before V0 closes; the number itself is never a V0 gate.

### B-002 · Task creation latency
- Metric: Latency from a spawn request inside a component to the new task being runnable.
- Method: p50 and p99 over 10 runs of 1,000,000 spawns in one component, warm, frequency pinned, on every hardware entry in scope.
- Harness: bench:task-create
- Baselines: pthread_create on Linux, Tokio task spawn, Go goroutine spawn
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 absolute p50 ≤ 2 µs; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 absolute p50 ≤ 2 µs; 1.0 absolute p50 ≤ 2 µs
- Status: defined

### B-003 · Task switch and native handoff latency
- Metric: Latency of a native task-to-task handoff on the same core and of a component-to-component scheduler-aware handoff, compared with Linux thread and process switches.
- Method: p50 and p99 over 10 runs of 1,000,000 ping-pong handoffs, same core, frequency pinned.
- Harness: bench:task-switch
- Baselines: Linux futex ping-pong between two threads, pipe ping-pong between two processes
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined
This is the §59 "context-switch behavior" measurement.

### B-004 · IPC round trip, same core
- Metric: Round-trip latency of a small typed message (≤ 64 bytes payload) over a Channel<T> between two components pinned to the same core.
- Method: p50 and p99 over 10 runs of 1,000,000 round trips, warm, frequency pinned, tracing disabled unless stated.
- Harness: bench:ipc-roundtrip-same-core
- Baselines: Linux Unix-domain-socket ping-pong, pipe ping-pong, futex ping-pong, io_uring SQPOLL ping-pong; Fuchsia and seL4 published numbers cited as reference only
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 absolute p50 ≤ 2 µs; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 absolute p50 ≤ 2 µs; 1.0 absolute p50 ≤ 2 µs
- Status: defined

### B-005 · IPC round trip, cross core
- Metric: Round-trip latency of a small typed message over a Channel<T> between two components pinned to different cores, and separately to different NUMA nodes where the hardware has them.
- Method: p50 and p99 over 10 runs of 1,000,000 round trips per placement, warm, frequency pinned.
- Harness: bench:ipc-roundtrip-cross-core
- Baselines: Linux Unix-domain-socket ping-pong, pipe ping-pong, futex ping-pong with the same core placement
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 absolute p50 ≤ 5 µs; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 absolute p50 ≤ 5 µs; 1.0 absolute p50 ≤ 5 µs
- Status: defined

### B-006 · IPC throughput
- Metric: Sustained messages per second and bytes per second over one Channel<T> and over 64 concurrent channels for small messages, with the sender never blocking on backpressure.
- Method: Median of 10 runs of 30 s sustained transfer, same core and cross core.
- Harness: bench:ipc-throughput
- Baselines: Linux Unix-domain-socket and pipe throughput at the same message size, io_uring send/recv
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-007 · MemoryObject transfer cost
- Metric: Latency and copies incurred when transferring ownership of a MemoryObject of 4 KiB, 1 MiB and 1 GiB from one component to another, including any unmap and TLB shootdown on the sender.
- Method: p50 and p99 over 10 runs of 10,000 transfers per size (100 for 1 GiB), physical-page identity verified after each transfer, frequency pinned.
- Harness: bench:memobj-transfer
- Baselines: memcpy of the same size, Linux splice/vmsplice, memfd plus SCM_RIGHTS fd passing, dma-buf export/import
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 absolute 1 GiB p50 ≤ 1 ms; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-008 · Memory overhead per idle component and per idle task
- Metric: Resident kernel plus runtime memory attributable to one idle minimal component, and to one idle task, measured by creating N and dividing the delta.
- Method: Median of 10 runs creating 10,000 idle components and separately 1,000,000 idle tasks, kernel and user memory accounted per ResourceDomain.
- Harness: bench:memory-overhead
- Baselines: minimal static Linux process, Linux thread with default stack, Tokio task, Go goroutine, podman container
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 absolute ≤ 512 KiB resident per idle minimal component; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-009 · Operation submit-to-completion latency
- Metric: Latency from submitting a no-op Operation, a Timer Operation and a Wait Operation to observing its completion, including inline completion where the ABI allows it.
- Method: p50 and p99 over 10 runs of 1,000,000 operations per kind, frequency pinned.
- Harness: bench:operation-latency
- Baselines: io_uring NOP submit-to-completion, epoll timerfd wakeup, eventfd wakeup
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-010 · Scheduling wakeup-to-run latency per intent class
- Metric: Latency from a wakeup event to the woken task running, per scheduling intent class, under a scripted contention load.
- Method: p50, p99 and p99.9 over 10 runs of 100,000 wakeups per intent class with a background load of 2× core count busy tasks.
- Harness: bench:sched-wakeup
- Baselines: Linux default scheduler class, SCHED_FIFO, sched_ext sample scheduler under the same load
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-011 · ResourceDomain creation and teardown latency
- Metric: Latency to create a ResourceDomain with a memory budget and CPU share, attach one component, and tear both down with all accounting reclaimed.
- Method: p50 and p99 over 10 runs of 10,000 cycles, frequency pinned.
- Harness: bench:resourcedomain-lifecycle
- Baselines: cgroup v2 directory create plus attach plus remove, unshare of a full namespace set, podman run plus rm
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-012 · Tracing overhead
- Metric: Relative slowdown of B-004 and B-003 with tracing enabled versus disabled, and the cost of a disabled tracepoint on the hot path.
- Method: Ratio of medians over 10 runs each of the enabled and disabled configurations on the same hardware in the same session.
- Harness: bench:tracing-overhead
- Baselines: Linux ftrace and eBPF tracepoint overhead on the equivalent Unix-domain-socket ping-pong
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 absolute ≤ 3% on B-004 with tracing enabled; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 absolute ≤ 3% on B-004 with tracing enabled; 1.0 absolute ≤ 3% on B-004 with tracing enabled
- Status: defined

### B-013 · V0 demo pipeline latency, copies and memory
- Metric: End-to-end latency from Component A submitting Channel<Request> to A reading the result from the transferred MemoryObject, the number of payload copies per stage, and memory overhead per component and per channel in the pipeline.
- Method: p50 and p99 over 10 runs of 100,000 requests for 4 KiB and 1 MiB results, copy count verified by physical-page identity, memory by ResourceDomain accounting.
- Harness: bench:demo-pipeline
- Baselines: Two Linux processes exchanging a request over a Unix-domain socket and a result over memfd plus fd passing
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-014 · Concurrent task scale
- Metric: Memory per task and total creation time for 500,000 live tasks in one component without a kernel thread per task.
- Method: Median of 5 runs creating 500,000 tasks that each block on a Wait Operation; peak resident memory and creation wall time recorded.
- Harness: bench:task-scale
- Baselines: 500,000 Tokio tasks, 500,000 goroutines, 500,000 Linux threads where the machine allows it
- Targets: V0 publish; V0.5 regression 10% vs V0; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-015 · Native isolation cost versus OCI containers
- Metric: Creation latency and resident memory of ResourceDomain plus Component plus capability attachment, compared with starting an equivalent OCI container.
- Method: Median of 10 runs of 1,000 creations each side on the same machine, same payload binary.
- Harness: bench:isolation-vs-oci
- Baselines: podman run and crun/runc direct for a minimal image, systemd-nspawn
- Targets: V1 publish; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-016 · Native application warm startup
- Metric: Latency from the launch request (click or `os run`) to the first presented non-blank frame for each shipped native application, with content already in the page cache.
- Method: p50 and p99 over 10 runs of 50 launches per application; the boundary is the compositor's first presentation of a non-blank frame (Q-029 fixes the definition).
- Harness: bench:app-startup-warm
- Baselines: GTK and Qt terminal and editor on a Linux Wayland desktop, Windows Terminal and Notepad on Windows, Terminal and TextEdit on macOS, each on comparable hardware
- Targets: V0.5 publish; V1 absolute p50 ≤ 20 ms for Terminal and Editor; V2 absolute p50 ≤ 20 ms for Terminal and Editor; V3 regression 10% vs V2; V4 absolute p50 ≤ 20 ms for Terminal and Editor; 1.0 absolute p50 ≤ 20 ms for Terminal and Editor
- Status: defined
The 20 ms figure is a measurement target and never a guarantee in public material (I-042).

### B-017 · Native application cold startup
- Metric: Latency from launch request to first presented non-blank frame with the application's package content evicted from the page cache.
- Method: p50 and p99 over 10 runs of 20 launches per application after a cache drop.
- Harness: bench:app-startup-cold
- Baselines: The B-016 baselines after an equivalent cache drop
- Targets: V0.5 publish; V1 publish; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-018 · Compositor frame latency
- Metric: Latency from a client's surface commit to the frame containing it reaching scanout.
- Method: p50 and p99 over 10 runs of 3,600 frames at the display's fixed refresh with a scripted animating client, timestamps from the display controller's vblank.
- Harness: bench:compositor-frame-latency
- Baselines: A wlroots-based compositor, KWin and Mutter on the same hardware with the same client
- Targets: V0.5 publish; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-019 · Compositor frame deadline misses
- Metric: Fraction of frames that miss their presentation deadline under a scripted mixed desktop workload (video playback, animating window, text input, background build).
- Method: Median over 10 runs of 10 minutes at 60 Hz and at the display's maximum refresh; misses counted from vblank timestamps.
- Harness: bench:compositor-deadline-misses
- Baselines: The B-018 compositors under the same workload
- Targets: V2 absolute < 0.1% of frames at 60 Hz; V3 regression 10% vs V2; V4 absolute < 0.1% of frames at 60 Hz and at maximum refresh; 1.0 absolute < 0.1% of frames at 60 Hz and at maximum refresh
- Status: defined

### B-020 · Input-to-photon latency
- Metric: Latency from a physical input event to the resulting pixel change on the display, measured externally.
- Method: p50 and p99 over 10 runs of 200 actuations with the LAB photodiode or high-speed camera rig at 60 Hz and at the display's maximum refresh, for a native application and for a Linux-personality application.
- Harness: bench:input-to-photon
- Baselines: A Linux Wayland desktop on the same machine; Windows on the same machine where dual boot exists
- Targets: V0.5 publish; V1 regression 10% vs V0.5; V2 publish at 60 Hz and maximum refresh per target machine; V3 regression 10% vs V2; V4 absolute p99 ≤ one frame interval plus 4 ms at 60 Hz and at maximum refresh; 1.0 absolute p99 ≤ one frame interval plus 4 ms at 60 Hz and at maximum refresh
- Status: defined

### B-021 · Package install time and store deduplication ratio
- Metric: Wall time to install a package into the content-addressed store (make-available, no activation), and the ratio of unique stored bytes to total package bytes across a corpus of 20 packages sharing dependencies.
- Method: Median of 10 runs over the fixed 20-package corpus from a local source; store size measured before and after.
- Harness: bench:package-install
- Baselines: dnf/apt install of equivalent packages, flatpak install, nix-env with a shared store
- Targets: V0.5 publish; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-022 · Generation creation, switch and rollback time
- Metric: Wall time to build a new system generation from an updated package set, to switch the boot default to it, and to roll back to the previous generation, plus disk overhead per generation.
- Method: Median of 10 runs on a system with 10 retained generations; overhead measured as store growth per generation.
- Harness: bench:generation-switch
- Baselines: ostree deploy and rollback, nixos-rebuild switch and rollback, Windows restore point creation and restore
- Targets: V0.5 publish; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 publish per Tier 1 machine; V4 publish per Tier 1 machine; 1.0 publish per Tier 1 machine
- Status: defined

### B-023 · Compositor restart-to-rebound time
- Metric: Latency from killing the compositor to every open window being presented again by the restarted compositor with input working.
- Method: p50 and p99 over 100 kills on QEMU and 20 on hardware with 10 open windows.
- Harness: bench:compositor-rebound
- Baselines: None comparable; Linux compositors terminate their clients. Published alone.
- Targets: V0.5 publish; V1 absolute p50 ≤ 500 ms; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-024 · Service restart-to-rebind latency
- Metric: Latency from a supervised service (audio server, storage service, network service) exiting to its clients receiving a rebound channel and completing a request.
- Method: p50 and p99 over 100 restarts per service with 10 clients attached.
- Harness: bench:service-rebind
- Baselines: PipeWire restart with attached PulseAudio clients, systemd service restart with socket activation
- Targets: V0.5 publish; V1 publish; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-025 · Environment startup, cached and cold
- Metric: Latency of `os env enter` on a previously built environment to a working shell with declared services reachable, and separately from a fresh environment.yaml with no cached objects.
- Method: p50 and p99 over 10 runs of 50 cached entries and 10 cold entries for the reference Postgres plus Redis stack.
- Harness: bench:env-startup
- Baselines: docker compose up for the equivalent stack (cached images and cold pull), devcontainer open
- Targets: V1 absolute cached p50 ≤ 50 ms with cold published; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 absolute cached p50 ≤ 50 ms; 1.0 absolute cached p50 ≤ 50 ms
- Status: defined

### B-026 · Linux compatibility overhead
- Metric: Syscall latency and workload throughput for the current Linux corpus workloads running under the Linux personality, relative to upstream Linux of the same version on the same hardware.
- Method: Median of 10 runs of the corpus's non-graphics workload scripts and a syscall microbenchmark set; ratio reported per workload.
- Harness: bench:linux-overhead
- Baselines: Upstream Linux kernel of the same base version with a mainstream distribution userspace on the same machine
- Targets: V0 publish (fork syscall microbenchmarks only); V1 publish on L2 workloads; V2 publish on L3 workloads; V3 publish on L4 workloads; V4 publish on L5 workloads; 1.0 publish on L5 workloads
- Status: defined
The V1 aspiration is within 5% for non-graphics workloads; the number is published regardless and no milestone gates on it.

### B-027 · Windows compatibility overhead
- Metric: Frames per second and frame-time p99 for each Gold-rated title in the current Windows corpus, and CPU overhead of the personality, relative to Linux plus Proton and to Windows on the same hardware.
- Method: Median of 5 runs of each title's scripted benchmark scene at 1440p with identical settings, dual boot where available.
- Harness: bench:windows-overhead
- Baselines: Linux plus Proton on the same machine, Windows on the same machine where dual boot exists
- Targets: V2 publish on W1 Gold titles; V3 publish on W2 Gold titles; V4 publish on W3 Gold titles; 1.0 publish on W3 Gold titles
- Status: defined

### B-028 · Audio round-trip latency
- Metric: Capture-to-playback round trip through the native low-latency audio path at the smallest stable buffer size, and glitch count over a ten-minute run.
- Method: Loopback cable measurement, p50 and p99 over 10 runs of 1,000 round trips, glitch count from the loopback signal.
- Harness: bench:audio-roundtrip
- Baselines: PipeWire on Linux, WASAPI exclusive mode on Windows, CoreAudio on macOS, each on comparable hardware
- Targets: V1 publish; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-029 · Bluetooth audio latency and pairing time
- Metric: End-to-end Bluetooth audio latency for the negotiated codec, time from pairing request to first audio, and time from power-on to reconnect for a known device.
- Method: p50 and p99 over 100 automated pairing and reconnect cycles per device class (headset A2DP and HFP, mouse, keyboard); audio latency by loopback.
- Harness: bench:bluetooth
- Baselines: BlueZ plus PipeWire on Linux, Windows on the same machine
- Targets: V2 publish; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-030 · Suspend and resume latency
- Metric: Time from suspend request to platform suspend, and from wake event to an interactive unlocked desktop with Wi-Fi, display and audio functional.
- Method: p50 and p99 over the automated suspend cycles run for the milestone's functional gate.
- Harness: bench:suspend-resume
- Baselines: A mainline Linux distribution and Windows on the same laptop
- Targets: V2 publish; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-031 · Idle power draw and battery runtime
- Metric: Average power draw of the idle desktop and battery runtime under a scripted mixed workload, on every laptop in scope.
- Method: External power meter for idle draw (median of 5 runs of 30 minutes); battery discharge from full to shutdown under the LAB energy methodology for runtime (median of 3 runs).
- Harness: bench:energy
- Baselines: A mainline Linux distribution on the same laptop, Windows on the same laptop where dual boot exists
- Targets: V1 publish on the Intel laptop; V2 publish per laptop; V3 publish per Tier 1 laptop; V4 publish per Tier 1 laptop; 1.0 publish per Tier 1 laptop
- Status: defined
No superiority claim is made from this metric at any milestone.

### B-032 · Boot time to login
- Metric: Time from firmware handoff to the bootloader to the greeter accepting input, and separately to the first presented frame.
- Method: p50 and p99 over 20 cold boots per machine, timestamps from the bootloader and compositor.
- Harness: bench:boot-time
- Baselines: A mainline Linux distribution and Windows on the same machine where dual boot exists
- Targets: V0.5 publish; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 publish per Tier 1 machine; V4 publish per Tier 1 machine; 1.0 publish per Tier 1 machine
- Status: defined

### B-033 · Unlock-to-interactive-desktop latency
- Metric: Time from successful authentication at the lock screen or greeter to the desktop accepting input.
- Method: p50 and p99 over 100 unlock cycles per machine.
- Harness: bench:unlock-latency
- Baselines: GNOME and KDE lock screens on Linux, Windows lock screen on the same machine
- Targets: V2 publish; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-034 · Install, first-boot and update-apply time
- Metric: Wall time from installer media boot to first login, time for the first-boot setup to reach the desktop, and time to download and apply a typical update as a new generation.
- Method: Median of 10 automated runs per Tier 1 machine from USB 3 media and from the update channel.
- Harness: bench:install-update-time
- Baselines: A mainline Linux distribution installer and Windows setup on the same machine
- Targets: V3 publish per Tier 1 machine; V4 publish per Tier 1 machine; 1.0 publish per Tier 1 machine
- Status: defined

### B-035 · Delta update size and rollback time
- Metric: Bytes downloaded for a typical release-to-release update relative to the full generation size, and wall time for automatic rollback after a fault-injected failing generation.
- Method: Median over the milestone's shipped releases; rollback timed from boot failure detection to the previous generation's greeter.
- Harness: bench:delta-rollback
- Baselines: ostree static deltas, Windows cumulative update size, full image size
- Targets: V3 publish; V4 publish per Tier 1 machine; 1.0 publish per Tier 1 machine
- Status: defined

### B-036 · Snapshot creation and restore time
- Metric: Wall time to create a storage snapshot of user data and to restore a previous snapshot or package set from the settings UI.
- Method: Median of 10 runs on a 100 GiB user data set.
- Harness: bench:snapshot
- Baselines: btrfs or bcachefs snapshot and restore on Linux, Windows restore point
- Targets: V2 publish; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-037 · Filesystem and object access
- Metric: Latency and throughput of File read and write Operations, content-store object open and map, and directory listing through Capability<Directory>, for small and large objects.
- Method: p50 and p99 over 10 runs of 100,000 operations per shape on NVMe with a warm and a cold cache.
- Harness: bench:storage-access
- Baselines: Linux read/pread/pwrite, mmap of the same file, io_uring reads on the same filesystem
- Targets: V0.5 publish; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-038 · Disk encryption overhead
- Metric: Sequential and random NVMe throughput and latency with full-disk encryption enabled versus disabled on the same volume.
- Method: Median of 10 runs of a fixed fio profile per configuration with AES-NI available.
- Harness: bench:fde-overhead
- Baselines: LUKS2 dm-crypt on Linux, BitLocker on Windows on the same drive
- Targets: V2 publish; V3 publish per Tier 1 machine; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-039 · Build time of the full system image
- Metric: Wall time for a clean build of the full system image on the OS inside `os env` versus on Linux on the same hardware, plus incremental kernel rebuild and incremental native userspace rebuild times.
- Method: Median of 3 clean builds and 10 incremental builds per host configuration.
- Harness: bench:build-time
- Baselines: The same build on a mainline Linux distribution on the same machine
- Targets: V1 publish; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-040 · Security mitigation overhead
- Metric: Relative cost of the shipped kernel and runtime exploit mitigations (mitigations on versus off) on B-001, B-004, B-016 and B-026 workloads.
- Method: Ratio of medians over 10 runs per configuration in the same session.
- Harness: bench:mitigation-overhead
- Baselines: The same mitigations toggled on upstream Linux on the same machine
- Targets: V1 publish; V4 publish; 1.0 publish
- Status: defined

### B-041 · Crash-free session rate
- Metric: Fraction of opt-in telemetry sessions with no component crash, compositor restart or kernel panic, over the measurement window.
- Method: Computed from opt-in telemetry over a 60-day window (30 days for the 1.0 soak) across all reporting machines; published with the machine count.
- Harness: bench:crash-free-rate
- Baselines: None comparable; published alone
- Targets: V3 absolute ≥ 97% over 60 days on ≥ 500 machines; V4 absolute ≥ 99.5% over 60 days on ≥ 2,000 machines; 1.0 absolute ≥ 99.5% over the final 30-day soak
- Status: defined

### B-042 · Kernel panic rate
- Metric: Kernel panics per 1,000 machine-days from opt-in telemetry.
- Method: Computed monthly from opt-in telemetry; published with the machine-day denominator.
- Harness: bench:panic-rate
- Baselines: None comparable; published alone
- Targets: V3 publish monthly; V4 absolute < 1 per 1,000 machine-days; 1.0 absolute < 1 per 1,000 machine-days over the final 30-day soak
- Status: defined

### B-043 · Update success rate
- Metric: Fraction of update attempts that boot the new generation successfully without automatic rollback, on Tier 1 hardware and across the opt-in fleet.
- Method: Computed from updater reports and telemetry over the milestone; published with attempt counts.
- Harness: bench:update-success
- Baselines: None comparable; published alone
- Targets: V3 publish; V4 absolute ≥ 99% for V3 to V4 in-place upgrades; 1.0 absolute ≥ 99.9% across the beta fleet
- Status: defined

### B-044 · Kernel and system image size
- Metric: Size of the kernel image, of a minimal system generation and of the installer image.
- Method: Measured on every CI build of main; published per release.
- Harness: bench:image-size
- Baselines: A mainline Linux distribution kernel and minimal install image
- Targets: V0.5 publish; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-045 · Desktop essentials latencies
- Metric: Keystroke-to-glyph through the input-method path, notification post-to-display, lock-to-unlock, Wi-Fi reconnect after resume, and camera cold start.
- Method: p50 and p99 over 100 trials per item using the LAB rig where a photon boundary applies.
- Harness: bench:desktop-essentials
- Baselines: GNOME and KDE on Linux, Windows on the same machine
- Targets: V1 publish; V2 publish; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-046 · Zero-copy media path
- Metric: Copies per stage and end-to-end latency for a frame travelling from NIC or storage through a decoder component to the GPU and to scanout, on hardware that permits shared physical memory.
- Method: Copy count by physical-page identity at each stage, latency p50 and p99 over 10 runs of 1,000 frames.
- Harness: bench:zero-copy-media
- Baselines: GStreamer with dma-buf on Linux, the same pipeline with software copies
- Targets: V1 publish; V2 publish; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-047 · Wasm component instantiation and channel round trip
- Metric: Instantiation latency of a Wasm component and round-trip latency of a typed message between a Wasm component and a machine-code component over a Channel<T>.
- Method: p50 and p99 over 10 runs of 10,000 instantiations and 1,000,000 round trips.
- Harness: bench:wasm-component
- Baselines: B-001 and B-004 for machine-code components, Wasmtime component instantiation on Linux
- Targets: V1 publish; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-048 · ComputeDevice dispatch overhead
- Metric: Submit-to-completion latency and throughput of a fixed compute workload dispatched through ComputeDevice to the GPU and to the CPU, relative to direct Vulkan compute and a native thread pool.
- Method: p50 and p99 over 10 runs of 10,000 dispatches per target device.
- Harness: bench:compute-dispatch
- Baselines: Direct Vulkan compute queue submission, Rayon thread pool on the same workload
- Targets: V2 publish; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-049 · Interoperability paths
- Metric: NTFS and exFAT throughput, SMB client throughput, VM-guest application launch time and GPU fraction, and migration import time per 100 GB with verification.
- Method: Median of 5 runs per path on Tier 1 hardware.
- Harness: bench:interop
- Baselines: Linux and Windows on the same hardware for each path
- Targets: V3 publish; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-050 · Developer onboarding time
- Metric: Time for a new developer to go from SDK download to a running hello component, and from a fresh clone to a booted QEMU image running the CI test harness.
- Method: Timed onboarding sessions with at least 5 participants per release; median reported.
- Harness: bench:onboarding
- Baselines: Equivalent onboarding for Android SDK and for a Linux kernel development setup
- Targets: V1 publish; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-051 · All tracked benchmarks regression check
- Metric: Aggregate check that no benchmark in this register regresses beyond the allowed band against the previous milestone's published report on any hardware entry in scope.
- Method: For every B entry with a report at the previous milestone, compare the current milestone's p50 (or primary statistic) on the same hardware entry; any exceedance requires an accepted decision explaining it.
- Harness: bench:regression-check
- Baselines: The previous milestone's committed benchmark reports
- Targets: V0.5 regression 10% vs V0; V1 regression 10% vs V0.5; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined

### B-053 · Capability operation cost
- Metric: Latency of Capability mint, attenuating derive, transfer over a Channel and revocation, measured per operation on the kernel fast path.
- Method: p50 and p99 over 10 runs of 1,000,000 operations per kind, warm, frequency pinned, on every hardware entry in the milestone's scope; revocation measured at derivation depth 1 and 8.
- Harness: bench:capability-ops
- Baselines: Linux dup/close and SCM_RIGHTS fd passing, seL4 capability invoke figures as published
- Targets: V0.5 publish; V1 publish; V2 regression 10% vs V1; V3 regression 10% vs V2; V4 regression 5% vs V3; 1.0 regression 5% vs V4
- Status: defined
Publish-only until the capability encoding is frozen at V4; the harness exists so CAP-009's revocation bounds are measured rather than claimed.

<!-- roadmap:generated:begin results -->
| ID | Title | Status |
| --- | --- | --- |
| B-001 | Component creation latency | defined |
| B-002 | Task creation latency | defined |
| B-003 | Task switch and native handoff latency | defined |
| B-004 | IPC round trip, same core | defined |
| B-005 | IPC round trip, cross core | defined |
| B-006 | IPC throughput | defined |
| B-007 | MemoryObject transfer cost | defined |
| B-008 | Memory overhead per idle component and per idle task | defined |
| B-009 | Operation submit-to-completion latency | defined |
| B-010 | Scheduling wakeup-to-run latency per intent class | defined |
| B-011 | ResourceDomain creation and teardown latency | defined |
| B-012 | Tracing overhead | defined |
| B-013 | V0 demo pipeline latency, copies and memory | defined |
| B-014 | Concurrent task scale | defined |
| B-015 | Native isolation cost versus OCI containers | defined |
| B-016 | Native application warm startup | defined |
| B-017 | Native application cold startup | defined |
| B-018 | Compositor frame latency | defined |
| B-019 | Compositor frame deadline misses | defined |
| B-020 | Input-to-photon latency | defined |
| B-021 | Package install time and store deduplication ratio | defined |
| B-022 | Generation creation, switch and rollback time | defined |
| B-023 | Compositor restart-to-rebound time | defined |
| B-024 | Service restart-to-rebind latency | defined |
| B-025 | Environment startup, cached and cold | defined |
| B-026 | Linux compatibility overhead | defined |
| B-027 | Windows compatibility overhead | defined |
| B-028 | Audio round-trip latency | defined |
| B-029 | Bluetooth audio latency and pairing time | defined |
| B-030 | Suspend and resume latency | defined |
| B-031 | Idle power draw and battery runtime | defined |
| B-032 | Boot time to login | defined |
| B-033 | Unlock-to-interactive-desktop latency | defined |
| B-034 | Install, first-boot and update-apply time | defined |
| B-035 | Delta update size and rollback time | defined |
| B-036 | Snapshot creation and restore time | defined |
| B-037 | Filesystem and object access | defined |
| B-038 | Disk encryption overhead | defined |
| B-039 | Build time of the full system image | defined |
| B-040 | Security mitigation overhead | defined |
| B-041 | Crash-free session rate | defined |
| B-042 | Kernel panic rate | defined |
| B-043 | Update success rate | defined |
| B-044 | Kernel and system image size | defined |
| B-045 | Desktop essentials latencies | defined |
| B-046 | Zero-copy media path | defined |
| B-047 | Wasm component instantiation and channel round trip | defined |
| B-048 | ComputeDevice dispatch overhead | defined |
| B-049 | Interoperability paths | defined |
| B-050 | Developer onboarding time | defined |
| B-051 | All tracked benchmarks regression check | defined |
| B-053 | Capability operation cost | defined |
<!-- roadmap:generated:end -->
