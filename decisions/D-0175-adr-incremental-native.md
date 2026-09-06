# D-0175 · Decide incremental native-Interface adoption
- Status: proposed
- Task: LNX-062
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §42, §46
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
If the only way for a Linux application to use the native file chooser, semantic interfaces or Capabilities were a complete rewrite, no existing application would ever become native (§46). At the same time the §3 firewall says native code sees no POSIX (I-025). This decision fixes whether a personality application may adopt native interfaces incrementally, which interfaces it may call and the limits that keep the firewall; it sits on the opt-in decision (D-0176), the personality's native bindings (LNX-036) and portals (D-0179).

## Options

### Option A · Explicit incremental bridge
Summary: An explicit bridge: a Linux application may call a listed set of native interfaces (chooser, semantic registry, notifications, clipboard, typed storage objects) through a personality-provided library that translates each call over a Channel; the native side sees a personality Component, never POSIX.
Consequences: Applications migrate one feature at a time and the list is a published, growing contract. The bridge is a second binding of each interface to maintain, and every listed interface needs a translation of its object types into things a Linux process can hold.
Evidence: none

### Option B · All-or-nothing rewrite
Summary: An application is either fully native or fully in the personality; there is no bridge.
Consequences: The firewall is trivially intact. The migration path is a rewrite, so the native application ecosystem starts from zero and stays there, which §46's compatibility strategy is designed to avoid.
Evidence: none

### Option C · Silent mixing of POSIX and native APIs
Summary: A Component may link both the SDK and libc and call whichever it likes.
Consequences: The easiest porting story. POSIX and native authority mix inside one Component, the firewall (§3, I-025) is gone, and every such Component is a confused deputy waiting to happen; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
