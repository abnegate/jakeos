# D-0201 · Decide the native TLS library and how it consumes the CA trust store
- Status: proposed
- Task: NET-005
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Native TLS must consume the SEC CA trust store with per-application pinning without POSIX sockets (§9.1, §51).

## Options

### Option A · rustls in the native runtime
Summary: rustls is the TLS library.
Consequences: Memory-safe and small; fewer legacy features.
Evidence: none

### Option B · OpenSSL linked into native Components
Summary: OpenSSL is linked.
Consequences: Complete; a large C attack surface.
Evidence: none

### Option C · Kernel TLS offload
Summary: kTLS in the kernel.
Consequences: Performance; kernel surface and handshake still in userspace.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
