# D-0138 · Decide client update orchestration, metered links and deferral
- Status: proposed
- Task: INS-009
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V3 updater client needs a policy before it is built (§30, §63): how updates download in the background, whether they apply only at reboot or activate user space live, what happens on a metered link, and how a user defers without ever seeing a mixed-version tree (T-034). The update model is SystemGenerations plus reboot (I-086) unless this decision records a user-space exception; PKG-070 (reboot-free switching) and the REL channels (D-0248) are inputs, and the accepted option names the Settings and updater surfaces and the PKG and BOOT operations they call.

## Options

### Option A · Reboot-only apply
Summary: Generations download and compose in the background; the switch happens only at the next reboot, which the shell offers and the user schedules.
Consequences: One activation path, one integrity story, and never a mixed tree. Every update, including a single user-space fix, waits for a reboot, so time-to-fix on a laptop that is never rebooted is unbounded unless a deadline is enforced.
Evidence: none

### Option B · Live userspace activation with kernel reboot for kernel changes
Summary: User-space Packages activate live under the D-0226 rule (running Components keep the old objects until exit); a kernel or boot-chain change still requires a reboot.
Consequences: Most updates need no reboot and security fixes to services take effect at their next restart. Two activation states coexist ("user space at N+1, kernel at N"), which `os inspect`, rollback and the integrity attestation must all represent, and T-034 must be argued per Component rather than per generation.
Evidence: none

### Option C · Defer-until-idle with a deadline
Summary: Downloads happen in the background; activation waits until the machine is idle or a deadline (per channel) passes, then applies as in A or B.
Consequences: Unobtrusive for users and bounded for security: nothing is deferred forever. A deadline that forces a reboot is the behaviour users hate most on other platforms, so the deadline must be visible and long, and idle detection needs PWR's input.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
