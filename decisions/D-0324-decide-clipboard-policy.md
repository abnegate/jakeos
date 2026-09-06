# D-0324 · Decide clipboard authority policy: paste gesture or Capability, no ambient read
- Status: proposed
- Task: UIP-004
- Surfaces: S-032
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9, §9.1, §41
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Silent clipboard reading is a data-exfiltration vector (§9.1, T-001); the baseline names clipboard Capabilities (§9, §41) but not the policy. This decision fixes how reads are gated, that writes are permitted, where clipboard history lives, how typed content is negotiated with lazy MemoryObject transfer, and that X11 primary selection never enters S-032 (T-032, D-0181). Ambient read is forbidden by I-021.

## Options

### Option A · Reads only on a paste gesture plus Capability<ClipboardRead> for managers
Summary: A read happens only as the result of a user paste gesture into the focused Surface, delivered by the compositor; clipboard managers hold `Capability<ClipboardRead>` granted in Settings; writes are always allowed; history is a privileged shell feature.
Consequences: Ordinary applications never see a clipboard API that can be abused and users never see a prompt for paste, so security and usability both hold. Clipboard managers and accessibility tools need an explicit grant, the compositor is the only path for the paste gesture, and applications that read the clipboard on their own initiative (some IDEs, some terminals) need a redesign in the personality bridge.
Evidence: none

### Option B · Every read requires an explicit Capability
Summary: Every read, including a paste, requires an explicit `Capability<ClipboardRead>` granted by prompt.
Consequences: Strictly capability-only with no gesture inference in the compositor. Paste triggers a prompt the first time in each application, the prompt fatigue T-012 warns about arrives on the most common gesture, and users grant the persistent right to everything to make it stop.
Evidence: none

### Option C · Ambient clipboard read
Summary: Any Component may read the clipboard at any time.
Consequences: Matches X11 and most desktops, no policy to implement. Background exfiltration of passwords and tokens with no user signal, which §9.1 and I-021 forbid; recorded as rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
