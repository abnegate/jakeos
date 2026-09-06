# Review 02 · How the plan compounds, and what a swarm of agents would trip on

Scope: review 01 read every line as a unit. This pass asks what happens when the plan is executed in dependency order by agents who see one task at a time: which early choices become load-bearing, where two tasks pull the same object in different directions, where a gate or task points at a quantity nobody wrote down, and where an agent would have to invent something the roadmap should have fixed. Method: topological walk of V0 through V2 following the critical path; keyword and graph sweeps over all 2,281 tasks; cross-checks between tasks, gates, registers and decisions.

Severity as in review 01: **blocker**, **defect**, **gap**, **note**.

## 1. Compounding chains

### 1.1 Inherited C stacks inside native Components (blocker)

The plan makes native user space `no_std` plus `alloc` with no libc (SDK-003, ABI-003). It then asks native Components to host Mesa (GFX-036 at V0.5, GFX-056 at V1), codecs (MED-007 at V1), Bluetooth and Wi-Fi supplicants (HW-029, NET-009 at V1) and audio (AUD-002 at V1). Each of those is a large C code base that assumes a libc, file descriptors and `dlopen`. The roadmap answered the question five separate times, once per workstream, and never as a rule. Executed as written, GFX would pick whatever GFX-036 found expedient for Mesa, MED would pick something else for FFmpeg or GStreamer, and HW a third thing for BlueZ; by V2 the system would carry two or three hosting models that V4's Layer 2 lock would then have to reconcile. That is the late refactor the plan must not contain.

Action: SDK-097 / D-0351 (V0.5, before the compositor is built) decides one rule for all of them: a Layer 3 libc-compatible library over native Objects (the Fuchsia fdio-plus-musl shape), personality-hosted helper Components, or Rust ports per stack. GFX-016, GFX-056, MED-007, AUD-002, NET-009, HW-029 and LNX-016 now depend on it. SDK-098 (V1) builds the library when the Decision names it and is dropped otherwise. ABI-003 now says how that library differs from Linux `libc`.

### 1.2 Kernel evolution phase decided after the work it governs (defect)

LNX-003 (V0.5) decides personality depth per rung; LNX-090 (V2) translates Linux syscalls onto native primitives; KRN-050 decided at V3 which phase (C, D or E) 1.0 requires. Executed as written, the V2 translation work would be built and then judged necessary or unnecessary a rung later. KRN-050 is now V1, depends on LNX-003 instead of KRN-042, and is a V1-G20 decision.

### 1.3 Wrapper replacement versus the freeze (defect)

Component creation is a wrapper over `task_struct`, `mm_struct`, namespaces and cgroups at V0 (CMP-005); the native AddressSpace and membership replacements land at V2 (CMP-045, CMP-046). The V4 conformance suite that freezes S-007 (CMP-052) depended on the wrapper strategy decision but not on the replacements, so a slip of CMP-045 or CMP-046 would have frozen the ABI against the wrapper's observable behaviour. CMP-052 now depends on both.

### 1.4 Hardware that does not exist yet (defect)

GFX-072 and HW-048 (hybrid graphics: render offload and mux switching) were V2 tasks verified on H-011 and H-012, machines whose first milestone is V4; R-044 asked V2 to retire a risk that V2 hardware cannot exercise. Both tasks, the risk and the gate citations move to V4. VIRT-001 (V1) publishes guest images tested on the nested-virtualisation profile H-015, which was first scheduled for V2; H-015 is now a V1 profile. Fifty-three V4 and 1.0 tasks verify on `qemu-x86_64` while those rungs' hardware scope excluded H-001; the CI profiles are now in scope for every rung (the QEMU benchmark exemption from review 01 keeps them out of the numbers).

### 1.5 Benchmark tasks with no target to meet (defect)

Five benchmark tasks cited a B-ID whose register had no target clause for the task's rung (BEN-003 on B-026 at V0, SVC-001 on B-024 at V0.5, KRN-029 on B-040, MEM-033 on B-046 and SCH-029 on B-045 at V1). Rule 7.4.7 would have refused to close any of them. Publish clauses added.

### 1.6 The V0 critical path (note)

After review 01 the path is CAP-013 → CAP-008 → ABI-010 → ABI-012 → ABI-002 → ABI-005 → CMP-014 → TSK-023 → TSK-013 → TSK-018 → TSK-020 → SDK-004 → SDK-001 → SDK-002 → CMP-011 → BEN-002. It is the intended shape: prototype, decide, build the kernel object, build the Operation path, build the runtime, build the sample, run the demo, publish. Nothing in it is redundant. It does mean V0 has one lane of design work for its first quarter and every other workstream waits on the handle word; the honest schedule consequence is written into the milestone notes rather than hidden by fan-out.

## 2. What a swarm would have had to invent

| # | Severity | Finding | Action |
|---|---|---|---|
| 2.1 | blocker | Thirty-one gate texts said a count, rate, window or bound is "named in the verifying task" and the named task said "the gate names it" or said nothing. Nobody had written the number. Examples: V0-G08's cancellation bound, V0.5-G08's dogfooding sessions, V1-G02's daily-driving days, V3-G01's install success rate, V3-G03's consecutive updates, V4-G05's clean-fuzzing window, 1.0-G01's soak length. | Each quantity now lives in exactly one verifying task as an acceptance criterion (TSK-022, APP-003 to APP-006, GOV-032, AUD-010, WIN-029, INS-027, INS-028, INS-029, INS-032, INS-043, INS-054, REL-033, REL-038, REL-064, SEC-063, SEC-076, BLD-063, BLD-073, BLD-076, LAB-025, GOV-064, GOV-073, PWR-023). The values are defaults chosen in this review and are listed in section 4 for the maintainer to tune. |
| 2.2 | blocker | Verification lines name test paths with 37 different `alias:` prefixes; only 12 were registered aliases, so `gfx:`, `pkg:`, `personality:`, `hw:`, `storage:`, `idl:` and twenty more resolved to nothing. No task fixed the monorepo layout or crate naming, so every agent would have invented its own. | 26 aliases registered in `registers/repos.md` as directories of jakeos-platform. BLD-081 (V0, a V0-G21 document) fixes the layout, the crate naming rule, the test-path grammar and a lint that rejects unknown aliases or matrix entries. |
| 2.3 | defect | Sixteen tasks still referred to other tasks by generation-time draft slugs (`decide-operation-transport`, `decide-task-mapping`, `operation-cancel-deadline`, `decide-repo-retention` and more) that no agent could resolve. | All replaced by task IDs. |
| 2.4 | defect | MEM-002 and MEM-011 measured "the sizes named in GAP-0497" (4 KB and 64 MB) while B-007's register method names 4 KiB, 1 MiB and 1 GiB: two size sets for the same measurement, one of them only readable from a JSONL file. | Both tasks cite the B-007 sizes. |
| 2.5 | gap | 130 tasks cite inventory items (`GAP-`, `INV-`, `EXTRA-`) in prose as if the reader could see them; the text lives only in `tools/coverage/*.jsonl`. | `roadmap show` now prints the text of every item a task covers or cites, and `generated/coverage-items.md` lists them all. |
| 2.6 | defect | The error vocabulary drifted: V0-G07 and IPC-011 said `Timeout`, TSK-010 and SDK-005 said `DeadlineExceeded`, for the same result. | One provisional vocabulary in GLOSSARY.md (`Error::Rights`, `Exhausted`, `Cancelled`, `DeadlineExceeded`, `Revoked`, `Disconnected`, `Integrity`) until ABI-009 fixes the model; the `Timeout` uses renamed. |
| 2.7 | defect | CI matrix entries were consistent except one `hw-h003` (H-003 is the `qemu-virtio-gpu` profile) and two singleton spellings. | Fixed; every hardware register entry now states its matrix entry and BLD-012 lists the canonical set. |
| 2.8 | defect | V2-G20 gated "the shell is translated into the language count named in the verifying task" with no translation task at V2 (five languages is TXT-041 at V3, ten is V4-G11). | V2-G20 gates the localization pipeline (TXT-032) and defers counts to V3 and V4. |

## 3. Things checked and found sound

- Storage before the installer: filesystem choice, partition layout, encryption layer and generation materialisation are all V0.5 decisions and the image builder depends on them; V1 developer installs and V3 public installs share one layout.
- Signing fields reserved at V0.5, signing at V1, public repository at V3: no manifest break.
- SDK v1 at V1 over an unfrozen Layer 1: covered by R-028 and the compatibility suite; the SDK absorbs Layer 1 change.
- Two service managers (native init plus Linux-personality daemons): R-072, SVC-026 and LNX-029 keep them distinct.
- Freeze order: every Layer 1 surface now has spike, decision and freezing task in one closure after review 01.
- Absolute benchmark targets (B-001 100 µs Component creation, B-002 2 µs Task creation, B-004 2 µs same-core round trip, B-007 1 ms for 1 GiB transfer, B-016 20 ms warm start) are within what comparable systems have demonstrated on similar hardware; none requires a mechanism the plan does not build.

## 4. Defaults chosen in this review

These numbers were absent and are now pinned in the named task. Change the task, never the gate.

| Task | Quantity |
|---|---|
| TSK-022 | 3-level tree, 1,000 Tasks, 1,000 Timer Operations, cancelled within 50 ms on H-002 and 200 ms on QEMU |
| APP-003 to APP-006 | two core team members, ten one-hour sessions each over ten working days, per application |
| GOV-032 | 20 consecutive working days of daily-driving per core team member |
| AUD-010 | 60 minutes of simultaneous native and PipeWire playback with zero underruns |
| WIN-029 | 2560 by 1440 for the V2-D03 demo title |
| PWR-023 | battery estimate within 10 percent of measured runtime |
| INS-032 | 98 of 100 automated encrypted installs per Tier 1 machine |
| INS-028 | verdict correct on 20 Tier 2 sample machines |
| INS-029 | eight participants, seven unaided successes |
| INS-043 | six consecutive alpha updates |
| INS-027 | 30 minutes from media boot to desktop |
| REL-038 | crash report in the tracker within 24 hours |
| REL-033 | ten packages from outside the core team |
| SEC-063 | ten participants, 90 percent task completion |
| BLD-063 | no open crasher older than 14 days at V3 |
| BLD-073 | 30 consecutive clean days before the V4 gate |
| BLD-076 | 14-day soak per release candidate |
| GOV-064 | five external RFCs decided |
| GOV-073 | 50 external native packages, 100 external contributors |
| INS-054 | 100 community V3 installs upgraded |
| LAB-025 | 30-day final soak |
| REL-064 | 95 percent of applicable High and Critical CVEs within the SLA |
| SEC-076 | no fuzzer crasher open more than 30 days at declaration |

## 5. Left alone

- The wrapper-then-native evolution of §6 Phase C is a designed refactor with equivalence tests (CMP-042, CMP-048), not an accidental one; it stays.
- The dual Linux syscall path and native ABI coexist through 1.0 by design; translation (LNX-090) is gated on zero corpus regressions.
- Gate kind `functional` on publish-only benchmark series (review 01, 1.15).
