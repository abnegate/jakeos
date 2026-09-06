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
Signed-repository fetch and native HTTPS must not go through POSIX sockets (I-005, I-049); they use `NetworkConnection` and a TLS library that consumes the SEC CA trust store with per-application pinning (§9.1, §51). This decision picks the library, how a Component holds the store Capability and how one without it fails, against T-019 (rogue CA) and R-030. Personality TLS stays with LNX and WIN. V1-G05's DNS-over-TLS path (NET-020) needs the choice.

## Options

### Option A · rustls in the native runtime
Summary: rustls in the native runtime, with a store provider that reads the CA set through `Capability<TrustStore>` and applies per-application pins.
Consequences: Memory-safe, small, no C dependency, and the store provider is ordinary typed code, so a Component without the Capability fails the handshake with `Error::Rights` before any network I/O. Fewer legacy ciphers and no client renegotiation, which some enterprise middleboxes and old servers still need; FIPS-style certification is not available.
Evidence: none

### Option B · OpenSSL linked into native Components
Summary: OpenSSL linked into native Components behind a Rust wrapper.
Consequences: Every protocol feature and every cipher, and the library the personalities already carry. A large C attack surface in every network-facing native Component (T-019 becomes a parsing bug away), and the trust store integration is a callback into C rather than a typed provider.
Evidence: none

### Option C · Kernel TLS offload
Summary: Kernel TLS offload (kTLS) for the record layer with the handshake in user space.
Consequences: Zero-copy sends and receives once the session is up, useful for the media and file paths. The handshake and certificate validation still need a user-space library (so this is an addition to A or B, not a replacement), and it puts TLS record processing in the kernel's threat surface.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
