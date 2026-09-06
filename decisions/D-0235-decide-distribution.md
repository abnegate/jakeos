# D-0235 · Decide origin, CDN and volunteer-mirror topology
- Status: proposed
- Task: REL-024
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Public alpha download volume exceeds what one origin serves (§27, §63). Because every object is content-addressed and repository metadata is signed (D-0245), an untrusted mirror can serve bytes without being trusted, which makes a project-owned global mirror network buy little. This decision fixes the topology and how mirror health is checked without giving any mirror signing authority or a way to tamper with signed objects (T-019); it sits on the repository model (D-0243) and the funding plan (GOV-041).

## Options

### Option A · Origin plus CDN plus verified volunteer mirrors
Summary: One origin behind a commercial CDN, plus volunteer mirrors that clients use only after verifying the signed index against the origin's; mirror health is measured by fetching known objects and comparing hashes.
Consequences: Cheap global scale and the community can help without being trusted. Volunteer mirrors go stale or vanish, so the client must fall back to the CDN transparently, and a freeze attack by a mirror is detectable only with the timestamp role of D-0245.
Evidence: none

### Option B · Origin plus CDN only
Summary: Origin plus CDN only.
Consequences: Simplest operation and one party to hold accountable. CDN egress cost scales with the user base with no offset, and regions the CDN serves poorly have no alternative.
Evidence: none

### Option C · Project-operated global mirror network
Summary: A project-operated global mirror network on rented hosts.
Consequences: Full control over freshness and availability. Standing infrastructure cost and operations burden for a one-person project, and it recreates what the CDN already sells; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
