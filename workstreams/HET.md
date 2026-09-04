# HET · Heterogeneous compute
- Prefix: HET
- Lead: none
- Baseline: §37, §38

<!-- roadmap:generated:begin summary -->
Tasks: 30 live, 0 done, 0 in-progress, 30 todo, 0 dropped. Ready: 1. Blocked: 29. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

HET owns heterogeneous compute: the ComputeDevice object, its ComputeQueue, GPUDispatch completion onto those queues, and the Layer 2 placement path that selects a device from workload preferences (§37, §38). Native software holds `Capability<ComputeDevice>` and `Capability<ComputeQueue>`; it never holds a DRM render node, a Vulkan instance, a CUDA context, or a POSIX device path as authority. CPU is the first enumerable class so the Layer 1 object exists before GPU, NPU, DSP, FPGA and generic accelerator classes are added. GPU dispatch uses retained DRM/Mesa or Vulkan compute per a V1 Decision; this workstream does not build a native GPU driver stack (§39, §56.1, I-045). Preferences name latency, throughput, energy, precision, memory and locality, and abstractions expose locality and cost rather than hiding them (§67 Principle 9).

## Out of scope

Handle encoding, syscall entry, error taxonomy, ComputeDevice type-id reservation and the Layer 1 freeze ADR (ABI). Capability rights words and the per-type rights registry (CAP). Operation ring, GPUDispatch as an Operation kind object, and the committed-work cancellation machine (TSK). MemoryObject placement attributes and device-local backing (MEM). Scheduling intent classes, GPU-queue priority and frequency governors (SCH, PWR). Inspect command rendering (SDK) and inspect/trace records (OBS). RenderQueue, compositor, Mesa-in-Component and DRM driver code (GFX, KRN). Reference-machine procurement and NVIDIA bring-up (HW, LAB). Benchmark runner and claim lint (BEN). Fuzz fleet (BLD). Generated reference site (DOC). Wasm host ABI (WASM). Codec Components (MED). AI broker (SEM). Linux and Windows personalities (LNX, WIN).

## Tasks

### HET-001 · Decide ComputeDevice enumeration ABI and open-ended class taxonomy
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-002, ABI-011
- Baseline: §1, §37, §38, §65, §67
- Decision: D-0117
- Risks: R-007
- Invariants: I-024, I-058, I-064, I-100

V0 scopes HET to this enumeration Decision so later classes do not require an ABI break. The record chooses how devices are named and listed, whether the CPU is a ComputeDevice, and whether placement cost is visible, without assuming topology stays CPU plus RAM plus GPU or that execution is CPU-only by design (§1, §37, §38). Nothing Layer 1 is frozen.

<!-- covers: INV-0045, INV-0678, INV-0686, INV-0705, INV-0715, INV-0716, INV-1300 -->

#### Out of scope
Layer 1 type-id reservation (ABI-032). GPU backend (HET-003). Queue derivation (HET-011). Implementation (HET-009).

#### Acceptance criteria
- [ ] Option A (closed class enum of CPU, GPU, NPU, DSP, FPGA, accelerator) and Option B (extensible class id) are evaluated against adding an NPU or a novel architecture without changing existing type ids.
- [ ] The accepted option states whether the CPU is a ComputeDevice or scheduler-only, and whether locality and cost are queryable or hidden (I-064).
- [ ] The accepted option does not assume a fixed CPU/GPU split or coherent memory between all devices (§38).
- [ ] No listed Layer 1 surface is recorded as frozen.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and HET reviewer sign-off recorded on the pull request.

#### Evidence
- none

### HET-002 · Survey heterogeneous dispatch models before the ComputeDevice API
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §37, §38, §58, §65
- Explores: S-028
- Risks: R-007
- Invariants: I-045, I-046

Layer 1 spikes belong in V0; committing ComputeDevice without this survey would freeze S-028 by accident (R-007). The report compares SYCL/oneAPI, CUDA graphs, Vulkan compute, Metal, Level Zero and ONNX Runtime execution providers against ComputeDevice, ComputeQueue, preference axes and Capability gating (§37, §58). Wasm as the native machine ABI is out of bounds (I-046). The spike does not freeze S-028.

<!-- covers: INV-0697 -->

#### Out of scope
Enumeration ADR (HET-001). GPU submit prototype (HET-010). Production ABI (HET-009).

#### Acceptance criteria
- [ ] The report covers SYCL/oneAPI, CUDA graphs, Vulkan compute, Metal, Level Zero and ONNX Runtime execution providers with citations.
- [ ] For each model the report states what maps onto ComputeDevice and ComputeQueue, what would leak DRM, CUDA or POSIX device nodes to native software, and what to reject.
- [ ] The report answers the Report questions and does not freeze S-028.

#### Verification
- Report: Which models expose locality and cost rather than hiding them? Which submission objects map onto ComputeQueue versus a hidden driver queue? Which models require ambient device access that `Capability<ComputeDevice>` would replace? Which portable IR, if any, is usable without making Wasm the native machine ABI? Path `reports/spikes/HET-002.md`.

#### Evidence
- none

### HET-003 · Decide GPU ComputeDevice backend among Vulkan, DRM, or deferral
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-010, HET-002, HET-001
- Baseline: §37, §39, §56.1, §57
- Decision: D-0115
- Risks: R-016, R-050
- Invariants: I-009, I-045

GPU ComputeDevice dispatch needs an explicit backend so V2 does not ship a half-built API. Options are Vulkan compute, a lower-level DRM job interface, or deferral past 1.0. Deferral is a real option and must be scored against the V2 Throughput-on-GPU exit criterion. The accepted option retains mature DRM/Mesa mechanisms and does not rewrite a native GPU driver stack (§2, §39, I-045).

<!-- covers: GAP-0553 -->

#### Out of scope
Submit prototype (HET-010). GPU ComputeDevice implementation (HET-015). NVIDIA support stance (HW-018, GFX-064). RenderQueue (GFX).

#### Acceptance criteria
- [ ] Option A (Vulkan compute), Option B (DRM job interface) and Option C (defer GPU ComputeDevice past 1.0) are evaluated with consequences for the V2 Throughput-on-GPU criterion, Mesa/DRM file-descriptor leakage (R-016) and I-045.
- [ ] The accepted option names the retained mechanism and the native objects native software sees (ComputeDevice and ComputeQueue, not DRM).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead, GFX lead and HET reviewer sign-off recorded on the pull request.

#### Evidence
- none

### HET-004 · Decide portable workload representation for heterogeneous dispatch
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-002, HET-001, HET-010
- Baseline: §13, §37, §57, §65
- Decision: D-0116
- Invariants: I-046

`compute.dispatch` needs a decided workload bytes layout before V2 placement. Options are SPIR-V, Wasm, native kernels per device, or a multi-format envelope. Wasm as the only native machine ABI is rejected by I-046 and §57; Wasm may still appear as one envelope member if the Decision says so. The record does not freeze S-028.

<!-- covers: INV-0695 -->

#### Out of scope
Placement service (HET-017). User-facing dispatch (HET-016). Wasm host ABI (WASM).

#### Acceptance criteria
- [ ] Option A (SPIR-V only), Option B (Wasm only), Option C (native kernels per device) and Option D (multi-format envelope) are evaluated against CPU, GPU and a future NPU class.
- [ ] The accepted option does not make Wasm the native machine ABI (I-046).
- [ ] The accepted option names the typed error when a device cannot consume the supplied representation.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and HET reviewer sign-off recorded on the pull request.

#### Evidence
- none

### HET-005 · Dispatch a sample workload on the CPU ComputeDevice
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-008, HET-007, TSK-018
- Baseline: §18, §19, §37

The V1 ComputeDevice reservation is callable, not a type-only stub. A sample workload submitted to the CPU ComputeQueue completes when the CPU work signals, proving GPUDispatch exists before the V2 GPU-signal path (§18, §37). Native software never sees a thread-pool or POSIX spawn API as the dispatch surface.

#### Out of scope
GPU-signal completion (HET-019). Portable workload format (HET-004). Placement across devices (HET-017).

#### Acceptance criteria
- [ ] A sample Component with `Capability<ComputeDevice>` submits GPUDispatch to the CPU ComputeQueue on H-001 and H-002 and observes completion.
- [ ] `os inspect` on that Component names the CPU ComputeDevice and the ComputeQueue used.
- [ ] Submitting the sample without the Capability returns `Error::Rights` and allocates no Operation.

#### Verification
- Unit: `kernel:tests/het/cpu_dispatch_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: sample Component submit/complete on H-001 and H-002.

#### Evidence
- none

### HET-006 · Expose ComputeDevice locality and cost through inspect data
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-009, HET-001, OBS-006
- Baseline: §24, §37, §64, §67
- Invariants: I-034, I-064

Principle 9 requires abstractions to expose locality and cost rather than hiding them. HET emits per-device records (class, locality relative to MemoryObjects, and the cost fields named by HET-001) onto the OBS inspect Interface (§24, §64). SDK owns `os inspect` rendering.

#### Out of scope
Inspect command (SDK). Trace format (OBS). Placement service (HET-017). MemoryObject placement attributes (MEM-041).

#### Acceptance criteria
- [ ] Inspect of the CPU ComputeDevice on H-001 and H-002 returns class, locality and the cost fields named by HET-001.
- [ ] A caller without inspect rights receives `Error::Rights` and no device records.
- [ ] Cost fields are present for every enumerated ComputeDevice; omitting them fails the unit tests.

#### Verification
- Unit: `kernel:tests/het/device_cost_inspect_*` on `qemu-x86_64` and `hw-h002`.
- Integration: OBS inspect Interface dump of the CPU device on H-002.

#### Evidence
- none

### HET-007 · Gate ComputeDevice access behind Capability with no ambient path
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-009, CAP-010, ABI-009
- Baseline: §7, §9.1, §37
- Threats: T-001
- Invariants: I-021

No-ambient-authority applies as soon as a CPU ComputeDevice exists. Enumeration, queue derivation and dispatch require `Capability<ComputeDevice>` with the rights named by CAP-010 (§7, §9.1). There is no default device namespace and no ambient DRM node.

<!-- covers: INV-0698 -->

#### Out of scope
Per-type rights registry expansion (CAP-036). GPU device (HET-015). Personality device nodes (LNX).

#### Acceptance criteria
- [ ] A freshly created Component holds no `Capability<ComputeDevice>`; enumerate, derive-queue and dispatch each return `Error::Rights` and allocate no handle.
- [ ] A holder of `Capability<ComputeDevice>` with dispatch rights submits GPUDispatch; a derived Capability without dispatch rights returns `Error::Rights` and allocates no Operation.
- [ ] Native crates have no API that opens a DRM node, `/dev/dri` path or Vulkan instance as authority.

#### Verification
- Unit: `kernel:tests/het/device_capability_*` on `qemu-x86_64` and `hw-h002`.
- Integration: ambient-denial case in the isolation suite on H-002.

#### Evidence
- none

### HET-008 · Implement ComputeQueue as a typed submission Object
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-011, HET-009, HET-007
- Baseline: §7, §37, §39

`Object<ComputeQueue>` is the submission queue of a ComputeDevice (§7, §37). CPU queues land at V1 with the device ABI; GPU queues bind at V2. How a queue is obtained from a device is HET-011. GFX RenderQueue remains GFX.

<!-- covers: INV-0164 -->

#### Out of scope
GFX ComputeQueue for Mesa (GFX-046). RenderQueue (GFX). GPU bind (HET-015). GPU-queue priority (SCH-039).

#### Acceptance criteria
- [ ] A CPU ComputeQueue is obtained only by the mint or derive path named by HET-011; any other mint returns `Error::Rights` and allocates no handle.
- [ ] Submit and complete of GPUDispatch on that CPU queue succeed on H-001 and H-002.
- [ ] Inspect names the queue, its ComputeDevice and outstanding submissions without a DRM render-node path.
- [ ] Native software never holds a DRM or Vulkan queue handle as the queue object.

#### Verification
- Unit: `kernel:tests/het/computequeue_*` on `qemu-x86_64` and `hw-h002`.
- Integration: CPU queue submit/complete with inspect on H-002.

#### Evidence
- none

### HET-009 · Implement ComputeDevice kernel ABI with CPU as first class
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: HET-001, HET-011, ABI-032, ABI-014
- Baseline: §7, §37, §38, §65, §69
- Invariants: I-024, I-058

V1 reserves the Layer 1 ComputeDevice object so later GPU and NPU classes do not break the ABI (§65 rule 8, §69). CPU is the first enumerable class; the class taxonomy stays open-ended per HET-001. ABI owns the type-id slot; HET implements enumeration, inspect records and CPU dispatch entry points.

<!-- covers: INV-0057, INV-0679, INV-0680, INV-1320 -->

#### Out of scope
Type-id reservation (ABI-032). GPU class (HET-015). NPU class (HET-020). Placement service (HET-017).

#### Acceptance criteria
- [ ] The CPU enumerates as a ComputeDevice on H-001 and H-002; inspect reports the class id named by HET-001.
- [ ] Adding a second class id in tests does not change the CPU type id; the ABI snapshot check remains green.
- [ ] GPUDispatch is a reserved Operation kind; invoking it on the CPU device completes or returns the typed unsupported error named by ABI-009, never an untyped errno.
- [ ] Native software holds `Capability<ComputeDevice>`, not a Linux device node.

#### Verification
- Unit: `kernel:tests/het/device_abi_*` on `qemu-x86_64` and `hw-h002`.
- Integration: ABI snapshot includes the ComputeDevice type id and GPUDispatch kind used by this implementation.

#### Evidence
- none

### HET-010 · Prototype Vulkan compute versus DRM job submission on AMD
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-002, HET-001
- Baseline: §37, §39, §56.1, §57
- Explores: S-028
- Risks: R-016
- Invariants: I-045

GAP-0553 and the V2 GPU ComputeDevice gate need a measured prototype on H-002 before HET-003 is accepted. The spike submits the same workload through Vulkan compute and through a DRM job interface on the reference AMD GPU, and records what leaks to userspace (file descriptors, device nodes, ioctls). It does not build a native GPU driver stack and does not freeze S-028.

#### Out of scope
Backend Decision (HET-003). Production GPU ComputeDevice (HET-015). Native GPU driver stack (forbidden before 1.0). B-048 standing harness (HET-018).

#### Acceptance criteria
- [ ] Both prototypes submit and complete the same workload on H-002.
- [ ] The report records submit-to-complete observations, copies, and which DRM or Vulkan objects were visible to the calling Component, for each path.
- [ ] The report answers the Report questions and does not freeze S-028.

#### Verification
- Report: Which path can sit behind `Capability<ComputeDevice>` without exposing DRM ioctls to native software? Which path reuses Mesa without a native driver rewrite? What submit-to-complete cost difference appears on H-002, cited only in the report? Path `reports/spikes/HET-010.md`.
- Manual: both prototypes run on H-002; procedure recorded in the report.

#### Evidence
- none

### HET-011 · Decide how ComputeDevice relates to ComputeQueue
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-001, HET-002, Q-033
- Baseline: §7, §37, §39
- Decision: D-0118

Answers whether a ComputeQueue is derived from a ComputeDevice Capability and who owns scheduling between them, before `Object<ComputeQueue>` and GPU backends land (§7, §37, Q-033). GFX ComputeQueue is a candidate GPU queue, not an automatic answer.

<!-- covers: INV-0696 -->

#### Out of scope
CPU queue implementation (HET-008). GFX Mesa ComputeQueue (GFX-046). GPU-queue priority (SCH-039).

#### Acceptance criteria
- [ ] Option A (ComputeQueue derived from `Capability<ComputeDevice>`), Option B (independent queue objects minted by the kernel) and Option C (GFX ComputeQueue as the GPU queue, CPU has no queue) are evaluated against CPU-only V1 and GPU V2.
- [ ] The accepted option states who owns scheduling among queues of one device.
- [ ] The accepted option names the typed error when a caller holds a device Capability but not a queue, or the reverse.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead, GFX lead and HET reviewer sign-off recorded on the pull request.

#### Evidence
- none

### HET-012 · Define GPUDispatch cancellation after work is submitted to the GPU
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-019, TSK-003, TSK-017, TSK-010
- Baseline: §18, §19, §37

Operation cancellation stays uniform when GPU work is already submitted. Hardware abort does not always succeed; the caller-visible result follows the committed-work contract from TSK-017 as specialised here (§19). Native software never sees a DRM or Vulkan abort ioctl.

#### Out of scope
Operation kind object (TSK-049). Hardware cancel matrix across Wi-Fi and NVMe (TSK-048). RenderQueue cancel (GFX).

#### Acceptance criteria
- [ ] Cancel after GPU submit on H-002 returns the committed-work result named by TSK-003 (wait, fail, or best-effort) and never delivers a successful completion for that Operation.
- [ ] Inspect of the Operation distinguishes cancelled-after-submit from never-submitted.
- [ ] Native crates have no DRM or Vulkan abort entry point for this path.

#### Verification
- Unit: `kernel:tests/het/gpudispatch_cancel_*` on `hw-h002`.
- Integration: cancel-after-submit on H-002; inspect shows cancelled-after-submit and no successful completion.

#### Evidence
- none

### HET-013 · Route Throughput to GPU and LowLatency to CPU in the V2 Demo
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-016, HET-017, HET-014, HET-015
- Baseline: §22, §37, §62

V2 exit: a sample workload with preference Throughput runs on the GPU and LowLatency on the CPU, and inspect shows the ComputeQueue (§37, §62). HET emits the objects and placement; OBS and SDK render them.

#### Out of scope
Inspect CLI (SDK-065). Trace records (OBS-045). Intent-to-frequency bridge (HET-025). B-048 publication (HET-018).

#### Acceptance criteria
- [ ] The sample with preference Throughput completes on the GPU ComputeDevice on H-002; inspect names that device and its ComputeQueue.
- [ ] The sample with preference LowLatency completes on the CPU ComputeDevice on H-002; inspect names that device and its ComputeQueue.
- [ ] The same Component holds only the ComputeDevice Capabilities it was granted; it does not open a DRM node to force placement.

#### Verification
- Integration: V2 ComputeDevice demo on H-002.
- Demo: Throughput-on-GPU and LowLatency-on-CPU with inspect showing each ComputeQueue on H-002.

#### Evidence
- none

### HET-014 · Encode ComputeDevice dispatch preferences for workload placement
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-001, HET-009, MEM-034, MEM-041, IPC-042
- Baseline: §17, §37, §38
- Invariants: I-064

Collapses the six §37 preference axes (latency, throughput, energy, precision, memory, locality) into one schema consumed by `compute.dispatch` and the placement service. Locality is expressed relative to MemoryObject placement attributes owned by MEM; HET does not invent a second locality field.

<!-- covers: INV-0687, INV-0688, INV-0689, INV-0690, INV-0691, INV-0692 -->

#### Out of scope
Placement policy (HET-017). MemoryObject attribute implementation (MEM-041). SCH intent classes (SCH). Layer 2 version lock (HET-027).

#### Acceptance criteria
- [ ] A preference record can set latency, throughput, energy, precision (including fp32, fp16 and int8), memory and locality relative to a named MemoryObject.
- [ ] Decoding an unknown field follows the Layer 2 evolution rules from IPC-042 and does not panic.
- [ ] A malformed preference blob returns a typed error and allocates no Operation.
- [ ] Native software never passes a POSIX nice value or a DRM priority ioctl as a preference.

#### Verification
- Unit: `kernel:tests/het/preference_schema_*` on `qemu-x86_64` and `hw-h002`.
- Fuzz: `kernel:fuzz/het_preference_decode` one hour nightly without panic.

#### Evidence
- none

### HET-015 · Expose the GPU as a ComputeDevice on the chosen backend
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: HET-003, HET-010, HET-009, HET-008, HET-007, MEM-046, GFX-046
- Baseline: §37, §39, §56.1, §57
- Risks: R-016
- Invariants: I-045

V2 requires GPU dispatch. The GPU enumerates as a ComputeDevice on the backend named by HET-003, using retained DRM/Mesa or Vulkan compute. Native software sees ComputeDevice and ComputeQueue, not DRM. No native GPU driver stack ships (§39, I-045).

<!-- covers: INV-0681 -->

#### Out of scope
Backend Decision (HET-003). GPU-signal completion (HET-019). NVIDIA desktop (HET-023). Compositor RenderQueue (GFX). Device-local MemoryObject providers (MEM-046).

#### Acceptance criteria
- [ ] The discrete GPU on H-002 enumerates as a ComputeDevice; inspect reports the GPU class id without a DRM node path.
- [ ] A Component with `Capability<ComputeDevice>` for that GPU obtains a ComputeQueue by the path named by HET-011.
- [ ] A Component without that Capability receives `Error::Rights` on enumerate-GPU, derive-queue and dispatch, and allocates no handle.
- [ ] Native crates have no public DRM ioctl, render-node or Vulkan instance type for this path.

#### Verification
- Unit: `kernel:tests/het/gpu_device_*` on `hw-h002`.
- Integration: GPU ComputeDevice enumeration and queue mint on H-002; CPU-only refusal on H-001.

#### Evidence
- none

### HET-016 · Implement compute.dispatch as an asynchronous Operation
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-004, HET-014, HET-017, HET-019, HET-005, TSK-018
- Baseline: §18, §19, §37

User-facing §37 `compute.dispatch(workload, preference)` submits an Operation that placement routes to a ComputeQueue. Kernel GPUDispatch is the submit/complete primitive; TSK owns the Operation kind object. Completion is asynchronous (§18, §19).

<!-- covers: INV-0693 -->

#### Out of scope
Operation kind and committed-work cancel (TSK-049, HET-012). Placement policy (HET-017). B-048 harness (HET-018).

#### Acceptance criteria
- [ ] `compute.dispatch` with a CPU-runnable workload and LowLatency preference completes on the CPU ComputeQueue on H-002.
- [ ] `compute.dispatch` with a GPU-runnable workload and Throughput preference completes on the GPU ComputeQueue on H-002.
- [ ] Dispatch without `Capability<ComputeDevice>` returns `Error::Rights` and allocates no Operation.
- [ ] The call returns an Operation the caller can cancel, deadline and await; it does not block a kernel thread for the whole GPU run.

#### Verification
- Unit: `kernel:tests/het/compute_dispatch_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V2 demo dispatch on H-002.

#### Evidence
- none

### HET-017 · Place workloads on a ComputeDevice from requirements and preferences
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-014, HET-015, HET-005, HET-006, MEM-041
- Baseline: §17, §37, §66
- Invariants: I-064

Layer 2 placement selects CPU versus GPU from encoded preferences and MemoryObject locality. The kernel ABI stays device and queue objects; unmatched workloads fail closed with a typed error. Cost and locality records from HET-006 are inputs, not hidden.

<!-- covers: INV-0694 -->

#### Out of scope
Preference schema (HET-014). User-facing `compute.dispatch` (HET-016). SCH intent bias (SCH-049). Device-local backing (MEM).

#### Acceptance criteria
- [ ] Throughput preference with a GPU-local MemoryObject on H-002 selects the GPU ComputeDevice.
- [ ] LowLatency preference with a host MemoryObject on H-002 selects the CPU ComputeDevice.
- [ ] A preference no enumerated device can satisfy returns a typed error, allocates no queue handle, and does not silently run on a different class.
- [ ] Placement reads MEM placement attributes; it does not invent a second locality vocabulary.

#### Verification
- Unit: `kernel:tests/het/placement_*` on `qemu-x86_64` and `hw-h002`.
- Integration: Throughput-to-GPU and LowLatency-to-CPU selection on H-002; unmatched-class case on H-001.

#### Evidence
- none

### HET-018 · Measure ComputeDevice dispatch overhead against Vulkan and Rayon
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-016, HET-013, BEN-005, BEN-007, OBS-045
- Baseline: §37, §54
- Benchmarks: B-048
- Invariants: I-061

Harness `bench:compute-dispatch` for B-048. V2 target kind is publish. Later rungs reuse this harness for regression bands. Numbers live in reports; this task does not restate them.

#### Out of scope
Register ownership and cross-OS publication dashboards (BEN-036, BEN-043). GPU profiler UI (SDK-071).

#### Acceptance criteria
- [ ] B-048 reports exist for H-002 covering GPU and CPU dispatch of the fixed workload, with direct Vulkan compute and a Rayon thread pool as baselines on the same machine.
- [ ] A CPU-only report exists for H-001 using the Rayon baseline.
- [ ] No documentation or task prose states a dispatch speedup except by citing these reports and B-048.

#### Verification
- Bench: B-048 on H-001 and H-002; target per register (V2 publish).
- Review: BEN reviewer confirms reports follow the B-048 method.

#### Evidence
- none

### HET-019 · Complete GPUDispatch when the GPU signals queue completion
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-015, HET-008, ABI-014, TSK-018, TSK-010
- Baseline: §18, §19, §37, §39

Inventory placed GPUDispatch at V0.5, but V0.5 has no HET gate and compositor GPU work is GFX RenderQueue. Earliest gate is V2 ComputeDevice dispatch. This is the GPU-signal completion path onto ComputeQueue: submit returns an Operation that completes when the GPU signals (§18, §19). TSK owns the Operation kind object.

<!-- covers: INV-0354 -->

#### Out of scope
Operation kind wrapper and rights check on the TSK object (TSK-049). Cancel-after-submit specialisation (HET-012). RenderQueue (GFX).

#### Acceptance criteria
- [ ] GPUDispatch submitted to the GPU ComputeQueue on H-002 completes when the GPU signals, not when the CPU merely enqueues.
- [ ] The completing Operation is visible to the submitting Task and carries the result named by ABI-009 on device error.
- [ ] H-001 (no GPU) returns a typed unsupported error and allocates no GPU queue handle.
- [ ] Native software awaits the Operation; it does not poll a DRM fence fd.

#### Verification
- Unit: `kernel:tests/het/gpudispatch_complete_*` on `hw-h002`; unsupported path on `qemu-x86_64`.
- Integration: GPU signal-to-complete on H-002 with OBS queue-latency events.

#### Evidence
- none

### HET-020 · Define NPU ComputeDevice class with one reference hardware backend
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: HET-009, HET-001, HET-015, HET-016
- Baseline: §37, §38
- Invariants: I-024, I-058

V3 expands ComputeDevice beyond CPU/GPU with at least one NPU backend on target hardware, proving the open-ended class id does not require an ABI break (§37, §38). The backend uses a retained vendor driver; this is not a native NPU driver rewrite (§57).

<!-- covers: INV-0682 -->

#### Out of scope
DSP, FPGA and generic accelerator class ids without backends (HET-021). AI broker (SEM). Scheduler-intent bridge (HET-025).

#### Acceptance criteria
- [ ] The NPU class id is distinct from CPU and GPU; the ABI snapshot for those ids does not change.
- [ ] H-004 or H-007 enumerates an NPU ComputeDevice in inspect.
- [ ] Dispatch with `Capability<ComputeDevice>` for that NPU completes on the reference backend; missing Capability returns `Error::Rights` and allocates no handle.
- [ ] Native software does not open a vendor character device as authority.

#### Verification
- Unit: `kernel:tests/het/npu_class_*` on `qemu-x86_64` (class-id registry) and `hw-h004`.
- Integration: enumerate and dispatch on H-004 or H-007; class-id snapshot check in CI.

#### Evidence
- none

### HET-021 · Define DSP FPGA and accelerator ComputeDevice classes without backends
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-001, HET-009
- Baseline: §37, §38
- Invariants: I-024

§37 requires DSP, FPGA and generic accelerator classes as interface definitions with backends optional before 1.0. Shipping unused drivers is out; the taxonomy stays open. Real backends are HET-030 on LATER.

<!-- covers: INV-0683, INV-0684, INV-0685 -->

#### Out of scope
Reference NPU backend (HET-020). DSP and FPGA backends (HET-030). Vendor driver bring-up (HW).

#### Acceptance criteria
- [ ] DSP, FPGA and generic accelerator class ids exist in the ComputeDevice taxonomy and in inspect's class registry on H-001.
- [ ] Enumerate returns an empty set for those classes on V3 Tier 1 machines that do not have the hardware.
- [ ] Dispatch targeting a class with no enumerated device returns a typed error and allocates no handle.
- [ ] CPU and GPU type ids are unchanged in the ABI snapshot.

#### Verification
- Unit: `kernel:tests/het/optional_classes_*` on `qemu-x86_64` and `hw-h002`.
- Integration: class-registry inspect on H-002; unmatched-class dispatch fail-closed.

#### Evidence
- none

### HET-022 · Publish Layer 1 reference pages for ComputeDevice and ComputeQueue
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-009, HET-008, HET-019, HET-007
- Baseline: §7, §37, §56.5, §65

V3 documentation requires a reference page for every Layer 1 entry point. HET authors ComputeDevice, ComputeQueue and GPUDispatch semantics; DOC generates and publishes pages (DOC-023). ABI owns the rest of the Layer 1 reference.

#### Out of scope
Page generation and the docs site (DOC). ABI-wide reference (ABI-046). Layer 2 preference schema guide (HET-027).

#### Acceptance criteria
- [ ] Authored semantics exist for every ComputeDevice, ComputeQueue and GPUDispatch Layer 1 entry point in the machine-readable ABI definition.
- [ ] A generator coverage check fails CI when one of those entry points lacks HET-authored prose.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and HET reviewer sign-off recorded on the pull request.
- Integration: coverage check in CI against ComputeDevice, ComputeQueue and GPUDispatch entry points.

#### Evidence
- none

### HET-023 · Enumerate NVIDIA GPU as a ComputeDevice on the V3 desktop
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-015, HET-003, HET-016, HW-018, HW-070
- Baseline: §37, §39, §56.1, §57
- Risks: R-037, R-050
- Invariants: I-045

V3 hardware scope adds gated H-006. ComputeDevice enumeration and dispatch must work on that GPU using the V1 backend Decision, not a native NVIDIA driver rewrite (§56.1, I-045). Driver residency and Secure Boot module load stay with HW.

#### Out of scope
NVIDIA kernel-module and Secure Boot Decision (HW-018). Compositor bring-up on H-006 (HW-052). Native NVIDIA driver stack (forbidden before 1.0).

#### Acceptance criteria
- [ ] The NVIDIA GPU on H-006 enumerates as a ComputeDevice; inspect reports the GPU class without a vendor character-device path as authority.
- [ ] `compute.dispatch` with Throughput preference completes on that device for a Component that holds the Capability.
- [ ] Dispatch without the Capability returns `Error::Rights` and allocates no handle.
- [ ] The implementation uses the backend named by HET-003; it does not add a project-owned NVIDIA kernel driver.

#### Verification
- Integration: enumerate and Throughput dispatch on H-006.
- Unit: `kernel:tests/het/nvidia_device_*` on `hw-h006`.

#### Evidence
- none

### HET-024 · Fuzz GPUDispatch submission and preference decoding without panic
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-016, HET-014, HET-019, BLD-016, BLD-035
- Baseline: §18, §37, §51
- Invariants: I-040

V3 continuous fuzzing of the native syscall surface includes GPUDispatch submit and preference blobs so a public alpha does not ship an unfuzzed Layer 1 object. BLD owns the fleet; HET owns the grammar and oracles (rights denial allocates no handle, decode never panics).

#### Out of scope
Fuzz infrastructure and crasher-age gate (BLD-035, BLD-063). Operation-ring grammar (TSK-051).

#### Acceptance criteria
- [ ] A syzkaller (or successor) grammar covers GPUDispatch submit, preference-blob decode, cancel-after-submit and Capability-missing paths.
- [ ] Oracle: missing `Capability<ComputeDevice>` returns `Error::Rights` and allocates no handle; malformed preference blobs return a typed error and do not panic.
- [ ] The target runs on BLD's continuous fleet; a panic in this grammar is a crasher filed against HET.

#### Verification
- Fuzz: `kernel:fuzz/het_gpudispatch` and preference-blob grammar on BLD continuous fleet.
- Unit: oracle tests `kernel:tests/het/fuzz_oracle_*` on `qemu-x86_64`.

#### Evidence
- none

### HET-025 · Integrate dispatch preferences with scheduler intent and frequency
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-014, HET-017, SCH-026, SCH-037, SCH-039
- Baseline: §22, §37, §54
- Invariants: I-032

Throughput, LowLatency and EnergyEfficient dispatch influence device choice and frequency selection via SCH intent (§22, §37). SCH owns GPU-queue priority and the intent classes; PWR owns governors; HET maps dispatch preferences onto those intents.

<!-- covers: INV-0700 -->

#### Out of scope
Intent class implementation (SCH). Frequency governors (PWR). GPU-queue priority mapping (SCH-039). Energy measurement (BEN, PWR).

#### Acceptance criteria
- [ ] Throughput dispatch on H-002 sets SCH Throughput intent on the submitting Task or ResourceDomain, visible in inspect, and placement still selects the GPU when a GPU ComputeDevice exists.
- [ ] LowLatency dispatch on H-002 sets SCH LowLatency intent and placement selects the CPU.
- [ ] EnergyEfficient dispatch sets SCH EnergyEfficient intent; HET does not program a governor ioctl itself.
- [ ] Native software does not call CPU-frequency or DRM-priority ioctls to achieve these mappings.

#### Verification
- Integration: preference-to-intent mapping on H-002 and H-004; inspect shows intent and selected ComputeDevice.
- Unit: `kernel:tests/het/intent_bridge_*` on `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### HET-026 · Ship ComputeDevice Layer 1 conformance tests for the ABI freeze
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-009, HET-008, HET-019, HET-007, HET-024, HET-005, HET-022
- Baseline: §37, §65, §66
- Risks: R-054
- Invariants: I-040

V4 freezes Layer 1. Every ComputeDevice, ComputeQueue and GPUDispatch entry point needs a conformance test. No Layer 1 freeze is scheduled before this rung (I-040). ABI owns the freeze ADR; HET owns these cases.

#### Out of scope
Accepting the freeze ADR (ABI-049). Golden binary suite across all Layer 1 (ABI-047). Layer 2 preference lock (HET-027).

#### Acceptance criteria
- [ ] Every ComputeDevice, ComputeQueue and GPUDispatch Layer 1 entry point has a conformance test that passes on `qemu-x86_64` and H-002.
- [ ] A binary built against the freeze candidate runs those tests on a later V4 build without rebuild.
- [ ] Rights-denial and unmatched-class cases remain in the suite.
- [ ] No Layer 1 ComputeDevice surface is recorded as frozen by this task; freeze state is ABI-049.

#### Verification
- Unit: `kernel:tests/het/conformance_v4_*` on `qemu-x86_64` and `hw-h002`.
- Integration: freeze-candidate binary replay on a subsequent V4 image.
- Review: ABI lead sign-off that the ComputeDevice Layer 1 cases are in the V4 suite.

#### Evidence
- none

### HET-027 · Lock the Layer 2 dispatch preference schema for the 1.x line
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-002, HET-001, HET-004, HET-014, HET-017, IPC-042, HET-010
- Baseline: §12, §37, §66
- Freezes: S-028
- Risks: R-054
- Invariants: I-041

V4 locks Layer 2 interface versions for 1.x. Dispatch preference records and the placement-service Interface are Layer 2 (S-028) and must pass old-client/new-service negotiation. Spike HET-002 and adr HET-001 sit in the dependency closure; no earlier task freezes S-028. Layer 1 is not frozen here.

#### Out of scope
Layer 1 freeze (ABI-049, HET-026). Global Layer 2 version enumeration (IPC-068).

#### Acceptance criteria
- [ ] Preference record and placement-service Interface version identities are listed for 1.x.
- [ ] An old client against a new service accepts unknown newer preference fields; a new client against an old service receives the typed negotiation error named by IPC evolution rules.
- [ ] Changing the locked schema without a version bump fails CI.
- [ ] S-028 is listed `frozen` only when this task is done and the exploring spike plus enumeration adr are done.

#### Verification
- Integration: old-client/new-service and new-client/old-service cases on `qemu-x86_64`.
- Review: ABI lead records S-028 freeze against this lock, HET-002 and HET-001.

#### Evidence
- none

### HET-028 · Publish B-048 ComputeDevice dispatch results on every Tier 1 machine
- Type: benchmark
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-018, HET-023, HET-026, BEN-060
- Baseline: §37, §54
- Benchmarks: B-048
- Invariants: I-061

1.0 re-runs every tracked benchmark on Tier 1. B-048 target kind is regression versus V4 using the V2 harness. No superiority claim without the published table.

#### Out of scope
Cross-OS dashboard publication (BEN-060). Claim audit of the announcement (BEN-062). Methodology pack (BEN-063).

#### Acceptance criteria
- [ ] A B-048 report exists for every in-scope 1.0 Tier 1 H-ID, using harness `bench:compute-dispatch`.
- [ ] Each report names the V4 baseline and the register regression band; exceptions cite an accepted Decision.
- [ ] No 1.0 announcement cites a dispatch number except by B-048.

#### Verification
- Bench: B-048 on every in-scope 1.0 Tier 1 H-ID; target per register (1.0 regression versus V4).
- Review: BEN reviewer confirms every in-scope H-ID has a report.

#### Evidence
- none

### HET-029 · Review ComputeDevice against CXL unified memory NPU and CHERI
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: HET-001, HET-009, HET-020, HET-021, HET-027
- Baseline: §8, §37, §38, §65, §70
- Invariants: I-058, I-100

1.0 fossilization review of ComputeDevice against x86-64, coherent-memory and discrete-GPU assumptions (CXL, unified CPU/GPU memory, NPUs, CHERI) without claiming hardware capability enforcement (§8, §38, I-100). ABI owns the umbrella review; HET authors the ComputeDevice chapter. No LATER backend is required.

#### Out of scope
Umbrella ABI/MemoryObject/Capability review (ABI-054). CHERI enforcement (CAP). CXL backing (MEM). DSP/FPGA backends (HET-030).

#### Acceptance criteria
- [ ] A published review walks ComputeDevice, ComputeQueue, GPUDispatch and S-028 against CXL, unified CPU/GPU memory, NPU, CHERI and novel-architecture scenarios.
- [ ] Each scenario records whether the frozen or locked surface can accommodate it without a major-version break, or names a 2.0 RFC item.
- [ ] The review does not claim hardware capability enforcement and does not introduce a calendar date.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead, MEM lead and HET reviewer sign-off recorded on the pull request.

#### Evidence
- none

### HET-030 · Implement optional DSP and FPGA ComputeDevice backends
- Type: build
- Milestone: LATER
- Status: todo
- Size: L
- Owner: none
- Depends on: HET-021, HET-016, HET-007
- Baseline: §37, §38
- Invariants: I-024, I-058

V3 ships DSP and FPGA as interface-only classes with backends optional before 1.0. Real backends live here so they can return without being fake 1.0 work. Nothing outside LATER depends on this task.

#### Out of scope
Class ids (HET-021). NPU reference backend (HET-020). 1.0 gates.

#### Acceptance criteria
- [ ] A DSP ComputeDevice enumerates on a machine that has the hardware and accepts GPUDispatch through `Capability<ComputeDevice>`.
- [ ] An FPGA ComputeDevice enumerates on a machine that has the hardware and accepts GPUDispatch through `Capability<ComputeDevice>`.
- [ ] Machines without those devices still return empty enumerate sets and typed unmatched-class errors.
- [ ] CPU, GPU and NPU type ids are unchanged in the ABI snapshot.

#### Verification
- Integration: DSP and FPGA enumerate-and-dispatch on the lab machines named when those SKUs are procured; unmatched-class tests remain green on H-001 and H-002.
- Unit: `kernel:tests/het/optional_backends_*`.
- Review: ABI snapshot check for existing class ids.

#### Evidence
- none
