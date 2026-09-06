# D-0304 · Decide whether to retain chrony or build a native NTP/NTS client
- Status: proposed
- Task: SVC-018
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
TLS validity, Package signature verification and Operation deadlines all fail with a wrong clock, so the V1 time service (SVC-032) must synchronise it and set the RTC. This decision picks the client the service hosts: retained chrony under supervision, a native Rust NTS client, or a minimal SNTP client; names NTS versus NTP; and records that native software never calls a POSIX `ntp_adjtime`. It sits on the boot clock (BOOT-021) and the time service scaffold (SVC-015).

## Options

### Option A · chrony hosted as a supervised personality service
Summary: chrony runs as a supervised personality-hosted helper; the native time service owns the network Capability it uses and the RTC, and exposes a typed Interface.
Consequences: The most accurate and battle-tested client, with NTS support and good behaviour on laptops that sleep. A C daemon on a native path under the C-library strategy (SDK-097), configured through chrony's own files that the settings model wraps, and its clock-stepping behaviour must be mapped onto the step notification of D-0298.
Evidence: none

### Option B · Native Rust NTS client Component
Summary: A native Rust NTS client Component that speaks NTS-KE and NTP over `NetworkConnection` and adjusts the clock through a typed kernel Operation.
Consequences: Authenticated time by default and no personality dependency on the boot path. NTP's discipline loop (drift, slew, leap handling) is subtle to reimplement, and correctness bugs show up as expired certificates weeks later.
Evidence: none

### Option C · systemd-timesyncd-class minimal client
Summary: A minimal SNTP client in the style of systemd-timesyncd.
Consequences: Small and quick to write. No NTS, so time is unauthenticated and an on-path attacker can move the clock to defeat certificate and signature checks (T-019 adjacent); acceptable only behind option A or B as a fallback.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
