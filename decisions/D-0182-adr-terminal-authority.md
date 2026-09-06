# D-0182 · Decide terminal-session authority for Linux programs
- Status: proposed
- Task: LNX-022
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §35, §46
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V1 developer terminal is the one place users expect ambient Linux authority: a shell, a compiler, a package manager (§35, §46). Without a decision, the terminal would become the hole through which §9 erodes. This decision records what authority a terminal session confers on the Linux programs it launches, how a developer escalates or attenuates it, how `os inspect` shows the grant, and rejects an unbounded uid-0 shell (I-021). It sits on the personality's capability acquisition (LNX-013), the native opt-in (LNX-016) and the Terminal application (APP-004).

## Options

### Option A · Ambient Linux environment scoped by the terminal's own Capabilities
Summary: Programs launched from a terminal inherit the terminal Component's Capabilities as an ambient Linux environment: the folders it was granted appear as paths, its network right is the network, its ResourceDomain is the budget; `os grant` inside the session widens or narrows the set through the normal chooser and prompt classes.
Consequences: Developers get a normal shell with normal tools, bounded by what the terminal holds, and the bound is visible in `os inspect` as one Component's grants. A terminal granted the home folder is as powerful as the home folder, so the default grant set is a security decision APP-004 must make deliberately.
Evidence: none

### Option B · Per-command grant prompt
Summary: Every command that touches a new object or right triggers a grant prompt.
Consequences: Precise least authority per command. Shell pipelines, build systems and package managers issue thousands of opens, so the prompt rate makes the terminal unusable and users grant everything to stop it (T-012).
Evidence: none

### Option C · Unbounded uid-0 shell
Summary: The terminal is a uid-0 Linux shell with the whole machine visible.
Consequences: Exactly what developers expect from Linux. It is ambient root over the native system through the personality, which I-021 forbids; recorded as rejected, with administrative operations reached through typed `os` commands instead.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
