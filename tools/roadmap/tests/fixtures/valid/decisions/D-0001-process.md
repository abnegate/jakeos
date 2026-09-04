# D-0001 · Roadmap repository process
- Status: proposed
- Task: GOV-001
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §1, §67
- Revisit when: the validator cannot express a rule the project needs

## Context
How the roadmap is stored and validated.

## Options
### Option A · Markdown only
Summary: conventions enforced by review.
Consequences: consistency depends on reviewers.
Evidence: none.

### Option B · Markdown plus a Rust tool
Summary: Markdown source with a Rust validator.
Consequences: the grammar is enforced.
Evidence: none.

## Decision
Option B is the working proposal.

## Consequences
A Rust crate owns validation and generation.

## Rejected options and why
Markdown only cannot keep two thousand tasks consistent.

## Follow-ups
none
