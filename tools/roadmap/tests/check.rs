mod common;

use roadmap::diagnostic::count_errors;

#[test]
fn valid_fixture_has_no_errors_after_gen() {
    let repo = common::materialize_valid();
    assert!(common::run(&repo.path, &["fmt"]).status.success());
    assert!(common::run(&repo.path, &["gen"]).status.success());
    let output = common::run(&repo.path, &["check"]);
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let entries = common::check_entries(&repo.path, false, None);
    assert_eq!(count_errors(&entries), 0, "{entries:?}");
}

#[test]
fn check_json_contains_file_line_code() {
    let repo = common::materialize_valid();
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov = gov.replace("- Type: build", "- Type: nope");
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let output = common::run(&repo.path, &["check", "--json"]);
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"code\": \"E-003\"") || stdout.contains("\"code\":\"E-003\""));
    assert!(stdout.contains("\"file\""));
    assert!(stdout.contains("\"line\""));
    assert!(stdout.contains("\"hint\""));
}

#[test]
fn milestone_token_one_point_zero_is_valid() {
    let repo = common::materialize_valid();
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov = gov.replace("- Milestone: V0", "- Milestone: 1.0");
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let entries = common::check_entries(&repo.path, false, None);
    assert!(
        !common::has_code(&entries, "E-025"),
        "1.0 should be a valid milestone token: {entries:?}"
    );
}

#[test]
fn draft_ids_require_allow_drafts() {
    let repo = common::materialize_valid();
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov = gov.replace(
        "### GOV-002 · Implement the roadmap validator",
        "### GOV-@ring · Implement the roadmap validator",
    );
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let denied = common::check_entries(&repo.path, false, None);
    assert!(common::has_code(&denied, "E-026"), "{denied:?}");
    let allowed = common::check_entries(&repo.path, true, None);
    assert!(!common::has_code(&allowed, "E-026"), "{allowed:?}");
}

#[test]
fn assign_ids_converts_drafts() {
    let repo = common::materialize_valid();
    let mut gov = common::read(&repo.path, "workstreams/GOV.md");
    gov.push_str(
        r#"
### GOV-@extra-docs · Document the validator commands
- Type: docs
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-002
- Baseline: §1

Write the command guide.

#### Acceptance criteria
- [ ] Every command is documented.

#### Verification
- Review: GOV maintainer reads the guide.

#### Evidence
- none
"#,
    );
    common::write(&repo.path, "workstreams/GOV.md", &gov);
    let output = common::run(
        &repo.path,
        &[
            "assign-ids",
            "--index",
            "tools/coverage/slugs.tsv",
            "--dry-run",
        ],
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("GOV-@extra-docs -> GOV-003"), "{stdout}");
    let applied = common::run(
        &repo.path,
        &["assign-ids", "--index", "tools/coverage/slugs.tsv"],
    );
    assert!(
        applied.status.success(),
        "{}",
        String::from_utf8_lossy(&applied.stderr)
    );
    let after = common::read(&repo.path, "workstreams/GOV.md");
    assert!(after.contains("### GOV-003 · Document the validator commands"));
    assert!(!after.contains("GOV-@extra-docs"));
}

#[test]
fn new_task_allocates_next_id() {
    let repo = common::materialize_valid();
    let output = common::run(
        &repo.path,
        &[
            "new",
            "task",
            "GOV",
            "Write the contributor guide",
            "--milestone",
            "V0",
            "--size",
            "S",
            "--type",
            "docs",
        ],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("GOV-003"), "{stdout}");
    let gov = common::read(&repo.path, "workstreams/GOV.md");
    assert!(gov.contains("### GOV-003 · Write the contributor guide"));
}

#[test]
fn commands_show_ready_progress_export() {
    let repo = common::materialize_valid();
    let generated = common::run(&repo.path, &["gen"]);
    assert!(
        generated.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&generated.stdout),
        String::from_utf8_lossy(&generated.stderr)
    );
    let show = common::run(&repo.path, &["show", "GOV-001"]);
    assert!(show.status.success());
    let text = String::from_utf8_lossy(&show.stdout);
    assert!(text.contains("GOV-001"));
    assert!(text.contains("Derived:"));
    let ready = common::run(&repo.path, &["ready"]);
    assert!(ready.status.success());
    assert!(String::from_utf8_lossy(&ready.stdout).contains("GOV-001"));
    let blocked = common::run(&repo.path, &["blocked"]);
    assert!(blocked.status.success());
    let progress = common::run(&repo.path, &["progress", "--json"]);
    assert!(progress.status.success());
    let payload = String::from_utf8_lossy(&progress.stdout);
    assert!(payload.contains("totals"));
    let export = common::run(&repo.path, &["export", "--json"]);
    assert!(export.status.success());
    assert!(String::from_utf8_lossy(&export.stdout).contains("GOV-001"));
    let gate = common::run(&repo.path, &["gate", "V0"]);
    assert!(gate.status.success());
    let impact = common::run(&repo.path, &["impact", "GOV-001", "--summary"]);
    assert!(impact.status.success());
    let coverage = common::run(&repo.path, &["coverage", "--json"]);
    assert!(coverage.status.success());
}

#[test]
fn later_rank_is_last() {
    let repo = common::materialize_valid();
    let loaded = common::load(&repo.path, false, None);
    assert_eq!(loaded.rank("LATER"), 99);
    assert!(loaded.rank("LATER") > loaded.rank("1.0"));
}
