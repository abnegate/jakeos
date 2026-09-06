# Review 03 · Walking back from 1.0

Scope: the previous two reviews read forward. This one starts at each 1.0 gate, follows `Verified by` into the dependency closure, and asks at every step whether the thing the gate needs is produced by a task in that closure, whether the task that produces it is detailed enough for an agent to build the right thing, and whether any task's own criteria quietly assume work outside its closure. Method: closure computation for all 19 gates and 5 demos of 1.0 and the 28 gates of V4; a sweep of every criterion for task IDs outside the closure; a sweep for build tasks with no executable verification; a re-reading of the 54 tasks whose criteria contained no code span, identifier, number or failure verb.

Every closure reaches V0 roots. The graph is connected end to end: 2,265 of 2,278 non-LATER tasks are reachable from a gate or demo, and the 13 unreachable ones are LATER by design. What follows is what the closures were missing or assuming.

## 1. Things 1.0 needs that nothing in the chain produced

| # | Severity | Finding | Decision and why |
|---|---|---|---|
| 1.1 | blocker | 1.0-G13 and V4-G17 require every §54 metric published against macOS "where a comparable class exists". BEN-046 pins a macOS baseline at V2 but no task buys an Apple machine, so every macOS column would have been vacuously `class: none` and the baseline's §54 comparison would never have happened. | LAB-027 (V2) procures one current Apple laptop as comparison equipment (not a Reference machine), mounts the photon fixture and external meter on it, and BEN-046 depends on it. Exact hardware parity is impossible; the comparison is class-level on input-to-photon, startup and energy, which is what BEN-046 already scoped. Declaring macOS out of scope was the alternative and it contradicts §54. |
| 1.2 | blocker | The corpora are 50 to 300 Windows titles and 50 to 500 Linux applications run under automation hundreds of times. No task acquires them, holds the store accounts, or checks that their licences allow automated lab execution and public per-title reporting. C-007 could not run at V2. | LAB-026 (V1) owns the corpus software library: accounts under the project entity, purchased licences, pinned installers, a licence register with lab-use and reporting terms, and a purchase ledger feeding GOV-041. WIN-006, WIN-070, WIN-079, LNX-056, LNX-084, LNX-100 and LNX-107 depend on it. Put in LAB because it is inventory and custody, not compatibility engineering. |
| 1.3 | blocker | 1.0-G16 demands the input-to-photon target on every Tier 1 machine. LAB-001 builds one rig on H-002; the fleet racking tasks LAB-018, LAB-021 and LAB-023 only said fixtures are "documented as attachable". Twelve machines, one rig, no measurement. | Those three tasks now require the fixture mounted and calibrated on every racked Tier 1 machine with B-020 running as a scheduler job. Photodiode fixtures are cheap; the alternative was discovering at V4 that the 1.0 gate cannot be run. |
| 1.4 | defect | HET-020 (V3) defines the NPU ComputeDevice class with one hardware backend and names H-004 or H-007 as the machine, but no register entry requires an NPU and H-004 was chosen at V0.5 without one. 1.0-G02's HET-029 review would rest on a class with no backend. | H-007's selection criteria now require an NPU with an upstream Linux driver; HET-020 depends on HW-062, the V3 laptop bring-up. |
| 1.5 | defect | Two 1.0 and V1 gates list a task whose existence is conditional on a decision (SDK-093 on PKG-069, SDK-098 on D-0351). A dropped task without a superseder makes a gate unsatisfiable. | Gate text and milestone notes now state the drop-and-remove rule for the negative outcome, citing the decision. The `Or:` field was considered and rejected: it satisfies the gate on acceptance regardless of which way the decision went. |

## 2. Criteria that assumed work outside their closure

Sixty-nine acceptance criteria named another task's lint, artifact or report that was not in the task's dependency closure. An agent executing such a task cannot satisfy the criterion. Each was resolved one of two ways, chosen by asking which direction the artifact flows:

- **Consumer depends on producer** (58 edges added). Examples: INS-001 needs SVC-007's native init to boot its image; SEC-069's auditor re-verification happens after every audit-closing task (KRN-054, IPC-066, CAP-050, CMP-053, GFX-093, STO-081, BOOT-046, LNX-103, WIN-073) so SEC-069 depends on all nine; BLD-063's fuzz gate reads IPC-061's inventory; BEN-044 measures the titles WIN-029 runs; KRN-013's native subsystem scaffold uses KRN-014's regression matrix to prove a disabled build matches upstream, which is the right order for a fork that must not break hardware.
- **Reword when the dependency would put a lint in front of the foundation** (5 criteria). ABI-020's spike ships the errno negative fixture rather than depending on the ABI-003 lint; IPC-016 (V0) counts payload copies in a unit test rather than citing the V0.5 lint IPC-034; SDK-001 proves Task ownership through `os inspect` rather than citing SDK-016; WIN-027 exposes the typed error through inspect rather than through the V3 task WIN-064.

Thirteen criteria reached into a later rung. Eleven were descriptive and left alone; two were fixed as above, and one was a real ordering error: UIP-014 (V0.5) enforces that X11 primary selection never enters the native clipboard while the decision saying so, LNX-020, sat at V1. LNX-020 is now V0.5 and UIP-014 depends on it.

Late-rung tasks with no dependencies at all were checked: MED-010 (media Interface prototypes) now depends on the IDL compiler, MemoryObject transfer and the SDK sample it needs; ENV-021 (environment.yaml prototype) on the Package manifest decision. The remaining roots are pure research or legal tasks.

## 3. Detail sufficient for an agent

- **The retained-mechanism inventory** (KRN-017) is consumed as a machine input by the config-fragment check (KRN-011) and the regression matrix (KRN-014) and was specified only as "the report". It is now a named TOML file in the kernel tree with a fixed field set; both consumers cite it. Three agents would otherwise have written three formats.
- **Fifty-four tasks** had no code span, identifier, number or failure verb in any criterion. On reading, fifty are decision or documentation tasks whose criteria are the options to evaluate or the chapters to write, which is the right shape for that type. Four wording fixes landed (APP-010, APP-050, UIP-048, MEM-052).
- **Vague qualifiers**: twenty criteria matched; sixteen were legitimate uses (`/etc`, "sufficient" inside a decision option). The four real ones are fixed above.
- **Build tasks verified only by review**: counted in this pass and listed in section 5.

## 4. Re-weighing earlier decisions

Review 02 pinned twenty-three gate quantities as defaults. Re-examined here with the question "would a maintainer accept this number on day one":

- INS-032 was 98 of 100 automated installs on Tier 1 lab hardware. Two percent install failure on machines the project owns and lists as fully supported is not a supported claim. Raised to 99 of 100 with every failure classified as lab infrastructure with logs attached; a failure attributable to the installer fails the gate regardless of count.
- TSK-022 (V0-G08 bound): 1,000 Tasks with 1,000 Timer Operations cancelled within 50 ms on H-002 is 50 µs per Task including the scheduling round trip to observe cancellation, well inside what the B-002 and B-003 targets imply. Kept.
- KRN-050 to V1: it depends only on the V0 divergence policy and the V0.5 personality-depth decision, and it must precede the V2 translation work it governs. Kept.
- SDK-097 at V0.5 before the compositor: the compositor is the first native Component that must host a C stack, so deciding after it is built is deciding after the fact. Kept.
- The remaining defaults stand as proposed; the rationale for each is that it is the smallest number at which the gate's sentence is true rather than aspirational, and every one lives in a single task where changing it is a one-line edit.

## 5. Left as findings

- Only three build tasks had no executable verification line. IPC-061 (fuzz-coverage inventory) gained an Integration line naming its job. APP-065 (the V4 feature-freeze declaration) and ACC-037 (a LATER prototype) are review-shaped by nature and were left.
- 1.0-G17 states "the team is staffed for the published support window". That is a fact about people, not a deliverable; GOV-081 records the commitment.
- V4-G20 is verified by HW-082 and INS-051, two unrelated tasks that happen to have no other gate. Harmless.
