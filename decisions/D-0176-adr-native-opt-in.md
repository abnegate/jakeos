# D-0176 · Decide how native applications opt into a Personality
- Status: proposed
- Task: LNX-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §46
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Native software never sees POSIX (§3) unless it explicitly opts into a compatibility environment (§46). This answers Q-002: what the opt-in is, what authority it carries, and how it appears on S-030. It sits on the personality depth decision (LNX-003), the Layer 1 handshake (ABI-025), the grant model (CAP-007) and the C-library strategy (SDK-097): a native application that hosts an inherited C stack is the main consumer.

## Options

### Option A · Capability to the Personality
Summary: The application holds a `Capability<Personality>` granted by its manifest; with it the runtime can spawn personality processes and exchange objects with them over typed Channels.
Consequences: Explicit, revocable and visible in `os inspect`; the application itself stays native and only its helpers live in the personality. The grant is coarse (a whole personality), so finer control over what the helper may do is the personality's job, and every crossing is a Channel round trip.
Evidence: none

### Option B · Embedded Linux Component
Summary: The application graph embeds a Linux Component (a personality instance private to the application) declared in its Package manifest.
Consequences: Per-application isolation: the helper shares nothing with other personality instances and dies with the application. Each such application carries a personality runtime and its memory, and startup pays for it.
Evidence: none

### Option C · SDK shim
Summary: The SDK offers a shim crate that exposes POSIX-shaped calls implemented over the personality behind the application's back.
Consequences: The most convenient port path for existing code. POSIX shapes appear in native source, developers stop noticing the boundary, and the shim becomes the de facto API, which §3 and I-013 forbid; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
