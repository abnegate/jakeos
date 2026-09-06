# Review 04 · Dry run of the swarm's first day

Reviews 01 to 03 read the roadmap. This review executes it: it asks what an agent that takes a task from `roadmap ready` today actually hits, and what the human on the other side of the pull request has to do. Every probe is a script over `generated/index.json` or a grep over `workstreams/`, so it can be rerun; the findings that could be made permanent became validator rules in the same commit.

## 1. What the first wave looks like

| Measure | Value |
|---|---|
| V0 tasks still todo | 190 |
| Tasks ready at the start | 25 |
| Dependency waves to the last V0 task | 18 |
| Size-weighted critical path (S 2, M 5, L 10 agent-days) | 130 agent-days |
| Total V0 work at the same weights | 955 agent-days |
| V0 tasks whose done needs a human `Verified by` | 38 (every adr) |
| V0.5 tasks whose done needs a human `Verified by` | 86 (every adr) |

The critical chain runs CAP-013 → CAP-008 → ABI-010 → ABI-012 → ABI-002 → ABI-005 → CMP-014 → TSK-023 → TSK-013 → TSK-018 → TSK-010 → CAP-004 → STO-001 → TSK-011 → IPC-013 → IPC-019 → IPC-002 → IPC-021. Eighteen hand-offs means V0 cannot finish faster than eighteen review latencies however many agents run; that latency, not agent count, is the V0 schedule.

## 2. Findings

### 2.1 Thirty questions could never be answered (fixed)

Review 01 removed the 25 dependencies where a task's text said it answered the question it depended on, and W-017 now catches that phrasing. The same deadlock survived in a second shape: 30 open questions had `Answered by: none` and exactly one dependent, and that dependent was the adr or spike whose title is the question (CAP-009 "Decide revocation semantics" depended on Q-004 "What are revocation semantics"). W-007 treated a question with a consumer as bound, so nothing fired. Three of the thirty were V0 tasks on the critical region: CAP-009, TSK-003 and TSK-017 were permanently blocked.

Fix: every one of the 30 questions now names its answering task in `Answered by`, the answering task no longer depends on the question, Q-029 is marked answered (BEN-016 and D-0032 already decided the visible-UI boundary), and SDK-093 depends on APP-056, the spike that answers Q-056. W-007 now fires for any open question with no `Answered by` task, whatever its consumers, with a test for the consumer-but-no-answerer case.

Decision weight: a question is a promise that some task will produce an answer. A consumer is evidence that the answer is needed, never that it will arrive. The rule had the two confused.

### 2.2 No task created the platform monorepo (fixed)

D-0037 decided two code repositories. KRN-010 created the kernel fork. Nothing created github.com/abnegate/jakeos-platform, which is where every Verification path outside the kernel points (`sdk:`, `idl:`, `runtime:`, `bench:` and 33 more aliases), and BLD-081, the layout document, had zero dependents. Thirty-one V0 tasks would each have created the repository ad hoc, starting with SDK-006 in the first wave.

Fix: BLD-082 creates the monorepo from the BLD-081 layout with the D-0102 licences, the D-0033 workspace and CI on GitHub-hosted runners; the 31 V0 tasks that verify inside the platform tree depend on it; V0-G18 cites it.

### 2.3 Verification environments were assumed, not provided (rule added)

94 V0 tasks verify on `hw-h002` and 45 on `qemu-x86_64`. Neither entry exists until LAB-003 racks the desktop and BLD-012 defines the QEMU matrix, and neither task was in those closures. Adding 139 dependency edges was rejected: it would have made LAB-003 a hub of 94 dependents, emptied the ready set behind procurement, and been wrong in kind, because a task can start before its verification environment exists; it just cannot finish.

Fix: hardware entries carry `Matrix entry` (the canonical CI name) and `Provided by` (the tasks that make the entry real: LAB-003 for `hw-h002`, BLD-012 for every QEMU profile plus BLD-028 for virtio-gpu and KRN-036 for nested, the LAB racking tasks for later machines). Three rules enforce it: E-116, a done task that verifies on an entry whose providers are not done; W-018, an entry with no provider or a provider later than the machine's first milestone; W-019, a Verification line naming an entry no hardware declares. The only existing violation was KRN-010, whose Verification claimed a build "on `qemu-x86_64`" that ran on a GitHub-hosted runner; the line now says what ran.

### 2.4 Two tasks with one title (fixed)

IPC-042 (adr) and IPC-046 (build) were both titled "Freeze the Layer 2 Interface-evolution rules for SDK v1". They are the decision and the freeze record, a legitimate pair, but an agent scanning titles sees a duplicate. IPC-042 and D-0144 now say "Decide whether …".

### 2.5 Criteria still cite tasks with no edge (left, rule proposed)

81 acceptance criteria name another task ("returns the typed error named by ABI-009") with no dependency in either direction. Review 03 wired the 58 where an artifact flows; the rest are references to a definition that is earlier in every case checked. They are left because a lint would need to tell "defined by X" from "that is X's job" (APP-041 names APP-064 to exclude it), and the honest fix is a lint that exempts Out-of-scope phrasing, which is a tooling task, not a review edit.

### 2.6 Criteria wording is clean

A scan of all 8,138 acceptance criteria for hedge words (appropriate, reasonable, as needed, etc., roughly, robust, TBD and forty more) found hits in 95 tasks, every one a false positive on inspection (generation N+1, "fast user switching", "clean checkout", `/etc`). No change.

## 3. Facts the operator must decide before the swarm starts

These are not roadmap defects; they are choices the roadmap cannot make and that change how the first week goes.

1. **The swarm's git identity.** All 19 commits so far are by the maintainer's own account, and every done task names the maintainer as `Verified by`. CI accepts a done transition when the verifier is the pull-request author, so a swarm pushing under the maintainer's account verifies itself. Running agents under a bot identity makes every `Status: done` require a real approval from the maintainer, which is what the rule intends.
2. **Human verification throughput.** With the current policy only adr and `Freezes:` tasks need a verifier (38 in V0, 86 in V0.5). CONVENTIONS section 12 switches `require_independent_verification` on at V1, which makes every one of the 567 V1 tasks need a human verifier. Either recruit verifiers before V1 or decide now that the V1 switch stays off for build tasks.
3. **Forty-one decisions are already accepted.** They fix the kernel base (mainline v7.2, merge not rebase), the toolchain (Clang only, pinned Rust), the licences (GPLv2 kernel, MIT above), UEFI-only boot, the component and spawn model, no PIDs, the repository topology, the lab site and the Windows scoping. `roadmap show` on any of the 46 done tasks lists them. They should be read once as a set, because every V0 task inherits them.
4. **154 proposed decisions have at least one option described in under 120 characters** (V0: 10, V0.5: 46, V1: 50). The option list is the space the executing agent chooses from; a thin option is an invitation to invent. The ten V0 decisions are the ones to fatten before the first adr task is claimed.
5. **Six product questions only the owner can answer** are currently scheduled as V1 or later decisions: 32-bit Windows at 1.0 (Q-040), the anti-cheat policy (Q-043), the browser strategy (Q-044), the IDE strategy (Q-045), the legal jurisdiction (Q-049) and whether the maintainer's own machines are the lab (implicit in D-0169). Answering them now costs nothing and removes the chance that an agent decides them by default.

## 4. Rerun

```
python3 reviews/scripts/waves.py        # wave structure, critical chain, effort per milestone
roadmap check --strict                  # W-007, W-018, W-019, E-116 are now permanent
```
