# D-0264 · Decide that Rust std lives only as a Layer 3 crate
- Status: proposed
- Task: SDK-028
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §52, §65, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Rust `std` exposes filesystem, network and process APIs with POSIX shapes (paths, sockets, exit codes). If `std` on the native target were satisfied by adding Layer 1 entry points, POSIX would re-enter the kernel ABI by the back door (§3, §65, §66, I-013, I-026). This decision precedes the V1 `std` crate and fixes how much of `std` exists on the native target, how it is implemented, and that no Layer 1 entry point is ever added to make a `std` API work (§52).

## Options

### Option A · Layer 3 facade over the SDK
Summary: `std` is a Layer 3 crate implemented over the SDK: `std::fs` maps onto Capability-scoped storage objects the Component already holds, `std::net` onto granted network Capabilities, `std::process` is unsupported.
Consequences: Most of the Rust ecosystem compiles unchanged, which is decisive for developer adoption at V1. POSIX shapes leak through the facade (paths as strings, errno-like errors) and developers will design against them instead of the typed SDK; the facade must document exactly which calls fail with `Error::Rights` and why.
Evidence: none

### Option B · No std on the native target
Summary: The native target is `no_std` plus `alloc`; the SDK is the only API.
Consequences: The typed model is the only model and nothing POSIX-shaped exists to lean on. Nearly every third-party crate that touches I/O is unavailable, so V1 third-party applications are far fewer, and every developer writes against a new API from day one.
Evidence: none

### Option C · Allowlisted std modules with no new L1 entries
Summary: An allowlist of `std` modules (collections, strings, sync, time, formatting, `io::Read`/`Write` traits) exists on the native target; `fs`, `net`, `process` and `env` are absent and their functionality is reached only through the SDK.
Consequences: Ecosystem crates that do computation compile, crates that do I/O are ported to the SDK, and no POSIX shape appears anywhere. The allowlist is a maintained artifact that follows Rust releases, and `cargo` builds of unported crates fail with unfamiliar errors until the SDK ships porting guides.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
