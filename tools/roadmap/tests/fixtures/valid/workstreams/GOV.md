# GOV · Governance
- Prefix: GOV
- Lead: none
- Baseline: §1, §67

<!-- roadmap:generated:begin summary -->
<!-- roadmap:generated:end -->

## Scope
Process, tooling and conventions for the roadmap.

## Out of scope
Architecture work owned by other prefixes.

## Tasks

### GOV-001 · Decide the roadmap repository process
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §1
- Decision: D-0001

Record how the roadmap is stored and validated.

#### Acceptance criteria
- [ ] Two or more options are evaluated in D-0001.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request.

#### Evidence
- none

### GOV-002 · Implement the roadmap validator
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: GOV-001
- Baseline: §1

Build the validator, formatter and generator for the roadmap grammar.

#### Acceptance criteria
- [ ] `roadmap check` validates the fixture repository.

#### Verification
- Unit: `tools/roadmap` tests on the local crate.

#### Evidence
- none
