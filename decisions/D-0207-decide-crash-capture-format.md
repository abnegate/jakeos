# D-0207 · Decide the Component crash capture format
- Status: proposed
- Task: OBS-029
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §10, §24, §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A Component that panics, aborts or violates a Capability terminates with a typed exit cause (D-0066) and the kernel captures state for two consumers: the SDK debugger at V1, which must show asynchronous Task stacks (§24, §64), and the INS crash-report client at V3, which uploads with consent (§61). The record must contain no disk keys or unlocked secrets (I-077) and must not become an exfiltration path (T-023). This decision names the format, how Task stacks are represented and where the flight-recorder trace window attaches.

## Options

### Option A · Minidump-compatible
Summary: Minidump-compatible files with a JakeOS extension stream for Task identities and the trace window.
Consequences: Breakpad, Crashpad, Sentry and every symbolication pipeline read them today, so REL's crash pipeline is mostly configuration. Minidumps model OS threads, not Tasks, so asynchronous Task stacks live only in the extension stream that stock tools ignore, and redaction (I-077) happens after capture in a format designed to dump memory.
Evidence: none

### Option B · Native typed record with async Task stacks
Summary: A native typed record: exit cause, Component and Task identities, each live Task's logical stack captured from the runtime's Task frames, register state, the flight-recorder window, and a memory allowlist; secrets are excluded at capture.
Consequences: The debugger shows what a Task was awaiting rather than which worker thread it sat on, redaction is structural, and the record reuses the S-035 trace schema. New tooling for symbolication and upload, and a converter to minidump for third-party services.
Evidence: none

### Option C · Core-file plus sidecar
Summary: A full core file plus a sidecar with typed metadata.
Consequences: Complete state for post-mortem debugging with gdb-class tools. Cores are large, contain every secret in memory so I-077 requires scrubbing before storage, and are unsuitable for consented upload; acceptable as a local developer option, not as the format.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
