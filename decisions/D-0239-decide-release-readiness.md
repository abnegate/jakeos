# D-0239 · Define release-readiness gates and freeze policy
- Status: proposed
- Task: REL-027
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V3 public releases need a blocker definition, go and no-go roles and per-channel freeze rules so qualification is not renegotiated under time pressure (§63). REL-017 implements the mechanical half; this decision fixes the policy: what a blocker bug is, who may override a red qualification job, and how each channel freezes before promotion (D-0248).

## Options

### Option A · Mechanical CI checklist with named go/no-go roles and per-channel freeze
Summary: A mechanical CI checklist (gate-run results, corpus thresholds, open blockers) gates every promotion; named go and no-go roles per channel may override a red job only with a recorded reason and a follow-up task.
Consequences: Predictable and auditable; a promotion never depends on who was in the room. Process overhead for every release and the roles must exist as people, which for a one-person project means the same person in each role until V4.
Evidence: none

### Option B · Human-run checklist only
Summary: A human-run checklist with no mechanical gate.
Consequences: Flexible and cheap to set up. Inconsistent between releases and unenforceable against a tired maintainer; rejected.
Evidence: none

### Option C · Automatic promote when CI is green with no freeze
Summary: Automatic promotion whenever CI is green, with no freeze window.
Consequences: Fast and hands-off. No human judgement on known-but-unlinted regressions, and a soak period cannot exist, so a bad nightly reaches stable; rejected for stable and LTS and acceptable for nightly only.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
