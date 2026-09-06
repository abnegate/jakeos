# D-0278 · Decide Semantic Interface discovery and caller permissioning
- Status: proposed
- Task: SEM-004
- Surfaces: S-023
- Layer: L2
- Spikes: SEM-003
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §42, §44
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Semantic interfaces let automation rules and AI brokers invoke application behaviour (§42, §44). Shipping any of them without a permission model would give every caller de facto ambient authority (§9.1, T-001, I-021). This decision fixes how a caller discovers an interface on S-023, where the grant comes from, whether lookup is session-local or system-wide, and that a missing grant returns `Error::Rights` and allocates no handle. SEM-003's spike and the grant taxonomy (D-0269) are inputs.

## Options

### Option A · Per-interface Capability grants
Summary: Each exposed interface is a distinct Capability; a caller must be granted `Capability<Semantic<Interface>>` per application per interface through the chooser or prompt classes.
Consequences: Exact least authority: an automation rule can send mail but not read it. The grant count grows with interfaces times applications, so the rule editor and AI broker must batch grants into understandable prompts, and discovery of what could be granted needs its own unprivileged listing.
Evidence: `reports/spikes/SEM-003.md`

### Option B · Manifest-declared exposure with session grants
Summary: Applications declare exposed interfaces in the Package manifest; the session holds a registry of declared interfaces; a caller gets a session-scoped grant per application when the user enables it for automation.
Consequences: Practical granularity (per application, not per method), and the manifest declaration doubles as documentation and store metadata. A session grant covers every interface the application exposes, so a caller enabled for one purpose can invoke all of them, which the rejected-options record must acknowledge.
Evidence: `reports/spikes/SEM-003.md`

### Option C · User-consent bind flow
Summary: A consent flow at bind time: the first invocation of an interface by a caller shows what will be allowed and mints a persistent, revocable grant for that pair.
Consequences: Explicit and understandable at the moment of use, and persistent so it happens once. Every new pair interrupts the user, unattended automation cannot bind at all, and the flow reproduces prompt fatigue for AI callers that touch many applications.
Evidence: `reports/spikes/SEM-003.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
