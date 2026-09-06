# D-0250 · Decide hierarchical versus flat ResourceDomains and budget delegation via Capability
- Status: proposed
- Task: SCH-002
- Surfaces: S-009
- Layer: L1
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §8, §9.1, §23
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
ResourceDomains carry memory, CPU-share and kernel-object budgets (§23) and every Component runs in exactly one (§10). `Capability<ResourceDomain>` is how a launcher, a session or `os env` grants budget (§8, §9.1). Nesting versus a flat set decides the shape of the V0 kernel object, whether every charge walks a tree, and how delegation is expressed. cgroups stay the retained mechanism underneath and are never configured by native software (§57).

## Options

### Option A · Hierarchical nested budgets with parent enforcement
Summary: Domains form a tree; a child's budget is carved from its parent's and the parent enforces the sum, including its children, on every charge.
Consequences: Matches launcher, session, application and helper nesting and `os env` directly, and a parent can always reclaim its children. Every charge walks up the tree, so accounting cost grows with depth and the tree shape is part of S-009. A child whose budget exceeds its parent's remaining budget is rejected at create or attach with `Error::Exhausted`, and parent counters include children. Maps one-to-one onto the retained cgroup v2 hierarchy.
Evidence: none

### Option B · Flat set structured only by Capability attenuation
Summary: All domains are siblings; a holder derives a `Capability<ResourceDomain>` with attenuated budget rights and hands it on, and the kernel enforces each domain on its own.
Consequences: Constant-time accounting and a simpler kernel object; delegation reuses CAP derivation (CAP-014) with no new structure. No parent can reclaim a child, and over-commit is possible unless derivation deducts from the grantor's budget, which reintroduces a ledger in the kernel. Session and application aggregates for `os inspect` are computed in user space from the capability graph.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
