mod common;

use std::path::Path;

struct Case {
    code: &'static str,
    allow_drafts: bool,
    use_index: bool,
    patches: &'static [Patch],
}

enum Patch {
    Replace(&'static str, &'static str),
    File(&'static str, &'static str),
}

const CASES: &[Case] = &[
    Case {
        code: "E-001",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Type: build",
            "- Type: build\n- Flavour: extra",
        )],
    },
    Case {
        code: "E-002",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Type: build",
            "- Type: build\n- Type: build",
        )],
    },
    Case {
        code: "E-003",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Status: todo", "- Status: waiting")],
    },
    Case {
        code: "E-004",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### GOV-002 · Implement the roadmap validator",
            "### GOV-002 Implement the roadmap validator",
        )],
    },
    Case {
        code: "E-005",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### GOV-002 · Implement the roadmap validator",
            "### GOV-001 · Implement the roadmap validator",
        )],
    },
    Case {
        code: "E-006",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### GOV-002 · Implement the roadmap validator",
            "### ABI-002 · Implement the roadmap validator",
        )],
    },
    Case {
        code: "E-007",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::File(
            "workstreams/ZZZ.md",
            "# ZZZ · Unknown\n- Prefix: ZZZ\n",
        )],
    },
    Case {
        code: "E-008",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### GOV-001 · Decide the roadmap repository process",
            "### GOV-003 · Decide the roadmap repository process",
        )],
    },
    Case {
        code: "E-009",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Type: build\n", "")],
    },
    Case {
        code: "E-010",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Type: build", "- Type: benchmark")],
    },
    Case {
        code: "E-011",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Baseline: §1\n\nBuild the validator",
            "- Baseline: §1\n- Decision: D-0001\n\nBuild the validator",
        )],
    },
    Case {
        code: "E-012",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "#### Acceptance criteria\n- [ ] `roadmap check` validates the fixture repository.\n\n",
            "",
        )],
    },
    Case {
        code: "E-013",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("#### Evidence\n- none\n", "#### Evidence\n")],
    },
    Case {
        code: "E-014",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Unit: `tools/roadmap` tests on the local crate.",
            "- Foo: not a kind",
        )],
    },
    Case {
        code: "E-015",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- [ ] `roadmap check` validates the fixture repository.",
            "- this is not a checkbox",
        )],
    },
    Case {
        code: "E-016",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Owner: none", "- Owner: bob")],
    },
    Case {
        code: "E-017",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "#### Evidence\n- none",
            "#### Mystery\n- none\n\n#### Evidence\n- none",
        )],
    },
    Case {
        code: "E-018",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Decision: D-0001",
            "- Decision: D-0001, D-0002",
        )],
    },
    Case {
        code: "E-020",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Depends on: GOV-001",
            "- Depends on: GOV-999",
        )],
    },
    Case {
        code: "E-021",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Depends on: GOV-001",
            "- Depends on: D-0001",
        )],
    },
    Case {
        code: "E-022",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Baseline: §1\n\nBuild",
            "- Baseline: §999\n\nBuild",
        )],
    },
    Case {
        code: "E-023",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Baseline: §1\n\nBuild",
            "- Baseline: none\n\nBuild",
        )],
    },
    Case {
        code: "E-024",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Baseline: §1, §67",
            "- Baseline: §1, §67\n- Baseline gap: GOV should not declare a gap",
        )],
    },
    Case {
        code: "E-025",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Milestone: V0", "- Milestone: V9")],
    },
    Case {
        code: "E-026",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### GOV-002 · Implement the roadmap validator",
            "### GOV-@ring · Implement the roadmap validator",
        )],
    },
    Case {
        code: "E-027",
        allow_drafts: true,
        use_index: true,
        patches: &[Patch::Replace(
            "### GOV-002 · Implement the roadmap validator",
            "### GOV-@unknown · Implement the roadmap validator",
        )],
    },
    Case {
        code: "E-030",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Depends on: none",
            "- Depends on: GOV-002",
        )],
    },
    Case {
        code: "E-031",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Depends on: GOV-001",
            "- Depends on: GOV-002",
        )],
    },
    Case {
        code: "E-032",
        allow_drafts: false,
        use_index: false,
        patches: &[
            Patch::Replace("- Depends on: GOV-001", "- Depends on: GOV-003"),
            Patch::File("workstreams/GOV.md", ""),
        ],
    },
    Case {
        code: "E-033",
        allow_drafts: false,
        use_index: false,
        patches: &[
            Patch::Replace("- Status: todo\n- Size: M", "- Status: dropped\n- Size: M"),
            Patch::Replace(
                "- Decision: D-0001",
                "- Decision: D-0001\n- Dropped because: descoped: unused",
            ),
        ],
    },
    Case {
        code: "E-040",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### GOV-002 · Implement the roadmap validator\n- Type: build\n- Milestone: V0\n- Status: todo",
            "### GOV-002 · Implement the roadmap validator\n- Type: build\n- Milestone: V0\n- Status: done",
        )],
    },
    Case {
        code: "E-051",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### GOV-002 · Implement the roadmap validator\n- Type: build\n- Milestone: V0\n- Status: todo",
            "### GOV-002 · Implement the roadmap validator\n- Type: build\n- Milestone: V0\n- Status: in-progress",
        )],
    },
    Case {
        code: "E-052",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### GOV-002 · Implement the roadmap validator\n- Type: build\n- Milestone: V0\n- Status: todo\n- Size: L\n- Owner: none",
            "### GOV-002 · Implement the roadmap validator\n- Type: build\n- Milestone: V0\n- Status: in-progress\n- Size: XL\n- Owner: @human",
        )],
    },
    Case {
        code: "E-053",
        allow_drafts: false,
        use_index: false,
        patches: &[
            Patch::Replace("- Status: todo\n- Size: M", "- Status: dropped\n- Size: M"),
            Patch::Replace(
                "- Decision: D-0001",
                "- Decision: D-0001\n- Dropped because: because I felt like it",
            ),
        ],
    },
    Case {
        code: "E-060",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Type: build", "- Type: spike")],
    },
    Case {
        code: "E-061",
        allow_drafts: false,
        use_index: false,
        patches: &[
            Patch::Replace("- Type: build", "- Type: benchmark"),
            Patch::Replace(
                "- Baseline: §1\n\nBuild the validator",
                "- Baseline: §1\n- Benchmarks: B-001\n\nBuild the validator",
            ),
            Patch::File(B_REGISTER, B_ENTRY),
        ],
    },
    Case {
        code: "E-070",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Decision: D-0001", "- Decision: D-0099")],
    },
    Case {
        code: "E-071",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Task: GOV-001", "- Task: GOV-002")],
    },
    Case {
        code: "E-072",
        allow_drafts: false,
        use_index: false,
        patches: &[
            Patch::Replace("- Type: build", "- Type: adr"),
            Patch::Replace(
                "- Baseline: §1\n\nBuild the validator",
                "- Baseline: §1\n- Decision: D-0001\n\nBuild the validator",
            ),
        ],
    },
    Case {
        code: "E-073",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### Option B · Markdown plus a Rust tool\nSummary: Markdown source with a Rust validator.\nConsequences: the grammar is enforced.\nEvidence: none.\n",
            "",
        )],
    },
    Case {
        code: "E-074",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "## Context\nHow the roadmap is stored and validated.\n",
            "",
        )],
    },
    Case {
        code: "E-081",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### V0-G01 · Roadmap tool exists\n- Kind: process\n- Verified by: GOV-002\nThe validator crate compiles and checks the fixture.\n",
            "",
        )],
    },
    Case {
        code: "E-082",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Kind: process", "- Kind: benchmark")],
    },
    Case {
        code: "E-084",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Kind: process", "- Kind: compatibility")],
    },
    Case {
        code: "E-085",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::File(
            "milestones/V9.md",
            "# V9 · Nope\n- Sequence: 9\n",
        )],
    },
    Case {
        code: "E-090",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::File(
            "registers/risks.md",
            "### R-001 · Example risk\n- Likelihood: low\n- Impact: low\n- Status: open\n- Mitigated by: none\n- Retire by: V0\n- Colour: blue\n",
        )],
    },
    Case {
        code: "E-091",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::File(
            "registers/risks.md",
            "### R-001 · Example risk\n- Likelihood: extreme\n- Impact: low\n- Status: open\n- Mitigated by: none\n- Retire by: V0\n",
        )],
    },
    Case {
        code: "E-093",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::File(
            "registers/risks.md",
            "### B-001 · Wrong family\n- Likelihood: low\n- Impact: low\n- Status: open\n- Mitigated by: none\n- Retire by: V0\n",
        )],
    },
    Case {
        code: "E-094",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::File(
            "registers/benchmarks.md",
            "### B-001 · Latency\n- Metric: time\n- Method: bench\n- Harness: none\n- Baselines: none\n- Targets: banana\n- Status: defined\n",
        )],
    },
    Case {
        code: "E-100",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "Build the validator, formatter and generator for the roadmap grammar.",
            "Build the validator by 2026-04-01.",
        )],
    },
    Case {
        code: "W-001",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- [ ] `roadmap check` validates the fixture repository.",
            "- [x] `roadmap check` validates the fixture repository.",
        )],
    },
    Case {
        code: "W-003",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- Verified by: GOV-002",
            "- Verified by: EX-001",
        )],
    },
    Case {
        code: "W-005",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace("- Size: L", "- Size: XL")],
    },
    Case {
        code: "W-008",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "- [ ] `roadmap check` validates the fixture repository.",
            "- [ ] The validator should pass.",
        )],
    },
    Case {
        code: "W-009",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### GOV-002 · Implement the roadmap validator",
            "### GOV-002 · The capability derivation design",
        )],
    },
    Case {
        code: "W-013",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "Build the validator, formatter and generator for the roadmap grammar.",
            "Build the validator to 50% coverage.",
        )],
    },
    Case {
        code: "W-014",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "Build the validator, formatter and generator for the roadmap grammar.",
            "Build the validator with p50 ≤ 2 µs.",
        )],
    },
    Case {
        code: "W-015",
        allow_drafts: false,
        use_index: false,
        patches: &[Patch::Replace(
            "### GOV-002 · Implement the roadmap validator",
            "### GOV-002 · Implement the memoryobject path",
        )],
    },
];

const B_REGISTER: &str = "registers/benchmarks.md";
const B_ENTRY: &str = "### B-001 · Latency\n- Metric: time\n- Method: bench\n- Harness: none\n- Baselines: none\n- Targets: V0 publish\n- Status: defined\n";

const EXTRA_TASK: &str = r#"
### GOV-003 · Implement later work
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §1

Later work.

#### Acceptance criteria
- [ ] The later work exists.

#### Verification
- Review: named reviewer.

#### Evidence
- none
"#;

#[test]
fn each_rule_emits_its_code() {
    for case in CASES {
        if case.code == "E-032" {
            continue;
        }
        let repo = common::materialize_valid();
        apply(&repo.path, case);
        let index = if case.use_index {
            Some(repo.path.join("tools/coverage/slugs.tsv"))
        } else {
            None
        };
        let entries = common::check_entries(&repo.path, case.allow_drafts, index);
        assert!(
            common::has_code(&entries, case.code),
            "{} not found in {:?} ({})",
            case.code,
            common::codes(&entries),
            messages(&entries)
        );
    }
}

#[test]
fn monotonicity_and_unanchored_and_registers() {
    let repo = common::materialize_valid();
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov = gov.replace("- Depends on: GOV-001", "- Depends on: GOV-003");
    gov.push_str(EXTRA_TASK);
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "E-032"),
        "{:?}",
        common::codes(&entries)
    );
    assert!(
        common::has_code(&entries, "W-002"),
        "{:?}",
        common::codes(&entries)
    );
}

#[test]
fn freeze_discipline() {
    let repo = common::materialize_valid();
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov = gov.replace(
        "- Baseline: §1\n\nBuild the validator",
        "- Baseline: §1\n- Freezes: S-001\n\nBuild the validator",
    );
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    common::write(
        &repo.path,
        "registers/surfaces.md",
        "### S-001 · Native ABI\n- Layer: L1\n- Owner: ABI\n- State: open\n- Explored by: none\n- Decided by: none\n- Frozen by: none\n",
    );
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "E-049"),
        "{:?}",
        common::codes(&entries)
    );
}

#[test]
fn gate_rank() {
    let repo = common::materialize_valid();
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov.push_str(EXTRA_TASK);
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let mut milestone = common::read(&repo.path, "milestones/V0.md");
    milestone = milestone.replace("- Verified by: GOV-002", "- Verified by: GOV-003");
    common::write(&repo.path, "milestones/V0.md", &milestone);
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "E-080"),
        "{:?}",
        common::codes(&entries)
    );
}

#[test]
fn benchmark_gate_without_target() {
    let repo = common::materialize_valid();
    let mut milestone = common::read(&repo.path, "milestones/V0.md");
    milestone = milestone.replace("- Kind: process", "- Kind: benchmark");
    milestone = milestone.replace(
        "- Verified by: GOV-002",
        "- Verified by: GOV-002\n- Benchmark: B-001",
    );
    common::write(&repo.path, "milestones/V0.md", &milestone);
    common::write(
        &repo.path,
        "registers/benchmarks.md",
        "### B-001 · Latency\n- Metric: time\n- Method: bench\n- Harness: none\n- Baselines: none\n- Targets: V1 publish\n- Status: defined\n",
    );
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "E-083"),
        "{:?}",
        common::codes(&entries)
    );
}

#[test]
fn question_unbound_and_benchmark_unreferenced() {
    let repo = common::materialize_valid();
    common::write(
        &repo.path,
        "registers/questions.md",
        "### Q-001 · Open question\n- Workstream: GOV\n- Status: open\n- Answered by: none\n",
    );
    common::write(
        &repo.path,
        "registers/benchmarks.md",
        "### B-001 · Latency\n- Metric: time\n- Method: bench\n- Harness: none\n- Baselines: none\n- Targets: V0 publish\n- Status: defined\n",
    );
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "W-007"),
        "{:?}",
        common::codes(&entries)
    );
    assert!(
        common::has_code(&entries, "W-006"),
        "{:?}",
        common::codes(&entries)
    );
}

#[test]
fn generated_stale_warns_before_gen() {
    let repo = common::materialize_valid();
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "W-004"),
        "{:?}",
        common::codes(&entries)
    );
}

#[test]
fn length_and_fan_in_warnings() {
    let repo = common::materialize_valid();
    common::write(
        &repo.path,
        "roadmap.toml",
        "[weights]\nS = 1\nM = 3\nL = 8\nXL = 20\n\n[policy]\nrequire_independent_verification = false\nverify_freezes_and_adr_always = true\nfan_in_warning = 1\nworkstream_lines_warning = 5\ntask_lines_warning = 5\n",
    );
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov.push_str(
        EXTRA_TASK
            .replace("Depends on: none", "Depends on: GOV-001")
            .as_str(),
    );
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "W-010"),
        "{:?}",
        common::codes(&entries)
    );
    assert!(
        common::has_code(&entries, "W-011"),
        "{:?}",
        common::codes(&entries)
    );
    assert!(
        common::has_code(&entries, "W-012"),
        "{:?}",
        common::codes(&entries)
    );
}

#[test]
fn done_invariants() {
    let repo = common::materialize_valid();
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov = gov.replace(
        "### GOV-001 · Decide the roadmap repository process\n- Type: adr\n- Milestone: V0\n- Status: todo\n- Size: M\n- Owner: none",
        "### GOV-001 · Decide the roadmap repository process\n- Type: adr\n- Milestone: V0\n- Status: done\n- Size: M\n- Owner: @alice",
    );
    gov = gov.replace(
        "- [ ] Two or more options are evaluated in D-0001.\n- [ ] A Review line names who accepts the decision.",
        "- [x] Two or more options are evaluated in D-0001.\n- [x] A Review line names who accepts the decision.",
    );
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "E-044"),
        "verifier {:?}",
        common::codes(&entries)
    );
    assert!(
        common::has_code(&entries, "E-047"),
        "closed {:?}",
        common::codes(&entries)
    );
    assert!(
        common::has_code(&entries, "E-042"),
        "evidence {:?}",
        common::codes(&entries)
    );
}

#[test]
fn verifier_rules() {
    let repo = common::materialize_valid();
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov = gov.replace(
        "### GOV-001 · Decide the roadmap repository process\n- Type: adr\n- Milestone: V0\n- Status: todo\n- Size: M\n- Owner: none",
        "### GOV-001 · Decide the roadmap repository process\n- Type: adr\n- Milestone: V0\n- Status: done\n- Size: M\n- Owner: @alice",
    );
    gov = gov.replace(
        "- Decision: D-0001",
        "- Decision: D-0001\n- Verified by: @alice",
    );
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "E-045"),
        "{:?}",
        common::codes(&entries)
    );
    gov = common::read(&repo.path, "workstreams/GOV.md");
    gov = gov.replace("- Verified by: @alice", "- Verified by: @agent/bot");
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "E-046"),
        "{:?}",
        common::codes(&entries)
    );
}

#[test]
fn spike_report_missing() {
    let repo = common::materialize_valid();
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov = gov.replace("- Type: build", "- Type: spike");
    gov = gov.replace(
        "- Unit: `tools/roadmap` tests on the local crate.",
        "- Report: what the spike must answer",
    );
    gov = gov.replace(
        "### GOV-002 · Implement the roadmap validator\n- Type: spike\n- Milestone: V0\n- Status: todo",
        "### GOV-002 · Implement the roadmap validator\n- Type: spike\n- Milestone: V0\n- Status: done",
    );
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "E-048"),
        "{:?}",
        common::codes(&entries)
    );
}

#[test]
fn register_invalid_id_list() {
    let repo = common::materialize_valid();
    common::write(
        &repo.path,
        "registers/risks.md",
        "### R-001 · Example risk\n- Likelihood: low\n- Impact: low\n- Status: open\n- Mitigated by: NOPE-001\n- Retire by: V0\n",
    );
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        common::has_code(&entries, "E-092"),
        "{:?}",
        common::codes(&entries)
    );
}

fn apply(root: &Path, case: &Case) {
    let mut gov = common::read(root, "workstreams/GOV.md");
    let mut decision = common::read(root, "decisions/D-0001-process.md");
    let mut milestone = common::read(root, "milestones/V0.md");
    for patch in case.patches {
        match patch {
            Patch::Replace(from, to) => {
                if from.is_empty() {
                    continue;
                }
                if gov.contains(from) {
                    gov = gov.replacen(from, to, 1);
                } else if decision.contains(from) {
                    decision = decision.replacen(from, to, 1);
                } else if milestone.contains(from) {
                    milestone = milestone.replacen(from, to, 1);
                }
            }
            Patch::File(path, content) => {
                if !content.is_empty() {
                    common::write(root, path, content);
                }
            }
        }
    }
    common::write(root, "workstreams/GOV.md", &gov);
    common::write(root, "decisions/D-0001-process.md", &decision);
    common::write(root, "milestones/V0.md", &milestone);
}

fn messages(entries: &[roadmap::diagnostic::Diagnostic]) -> String {
    entries
        .iter()
        .map(|entry| format!("{} {}", entry.code, entry.message))
        .collect::<Vec<_>>()
        .join("; ")
}
