# D-0245 · Decide Package and SystemGeneration signing scheme
- Status: proposed
- Task: REL-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27, §28, §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 daily driving requires a signed remote repository whose clients reject tampering before a Package becomes available (§27, §28, §30). The scheme is fixed before the first signed artifact so that V3 roles, if any, extend the same trust root rather than replacing it. It sits on the key hierarchy (D-0244) and the repository format (PKG-029), and it states what a client verifies before activation and how mix-and-match and replay of an index are detected, or that detection waits for REL-037.

## Options

### Option A · Signed content-addressed index at V1 with TUF roles at V3
Summary: V1 publishes a signed content-addressed index per channel; TUF snapshot and timestamp roles are added at V3 with the same root.
Consequences: Small to build for V1 and every object is still verified by identity. Until V3 a mirror can serve an old index (freeze attack) or mix indices, so REL-037 must land before the public repository, and the V1 client must be written to accept the added roles without a trust-root change.
Evidence: none

### Option B · TUF root, targets, snapshot and timestamp from the first channel
Summary: TUF from the first channel: root, targets, snapshot and timestamp roles with the D-0244 keys.
Consequences: Replay, freeze and mix-and-match are detected from day one, and the V3 public repository changes nothing in the client. More metadata and key operations before the first internal channel exists, and timestamp signing requires an online key with its own custody rule.
Evidence: none

### Option C · Per-Package signatures only
Summary: Each Package is signed by its publisher; the repository has no signed metadata.
Consequences: Simplest publishing. A mirror can withhold updates, serve old versions or combine Packages arbitrarily without detection, so tampering with the set is invisible; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
