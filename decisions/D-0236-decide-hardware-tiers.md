# D-0236 · Define hardware support tiers as the HCL unit
- Status: proposed
- Task: REL-011
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §62, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
HW-043 decided the two-tier HCL model; V2's three machines seed Tier 1 and REL publishes the list (§62, §63). This decision fixes the published unit so the V3 community database and the installer's verdict share one schema, names the promotion path into Tier 1, and records that unlisted hardware is unsupported (I-095). It sits on the reference machine list (HW-003), the tier decision (HW-043) and the probe data model (HW-047).

## Options

### Option A · Tier 1 lab-gated, Tier 2 community-reported, unsupported otherwise
Summary: Tier 1 (in the lab, full suite every release), Tier 2 (community-reported with probe data and a verification procedure), unsupported otherwise; the published unit is one machine SKU with its probe signature.
Consequences: Matches D-0128 and gives the installer three verdicts it can state honestly. Tier 2 quality depends on community reports the project cannot verify, so the schema must carry the report's evidence and date-free freshness (a release identifier), and promotion to Tier 1 requires a machine in the lab.
Evidence: none

### Option B · Lab-only until 1.0
Summary: Only lab machines are listed until 1.0.
Consequences: Every listed entry is verified. The list is tiny, most users' machines are "unsupported" though they work, and the V3 community database has nothing to hold.
Evidence: none

### Option C · Three tiers including a periodic-CI middle tier
Summary: Three tiers, with a middle tier of machines exercised periodically by CI (remote or partner labs).
Consequences: Finer-grained confidence. A tier the project cannot staff without partners it does not have yet; the maintenance of a third procedure outweighs the nuance before 1.0.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
