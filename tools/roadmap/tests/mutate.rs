mod common;

use std::path::Path;
use std::process::Command;

fn ok(root: &Path, args: &[&str]) -> String {
    let output = common::run(root, args);
    assert!(
        output.status.success(),
        "expected success for {args:?}\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn fails(root: &Path, args: &[&str]) -> String {
    let output = common::run(root, args);
    assert!(
        !output.status.success(),
        "expected failure for {args:?}\nstdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn block(root: &Path, id: &str) -> String {
    let content = common::read(root, "workstreams/GOV.md");
    let start = content
        .find(&format!("### {id} ·"))
        .unwrap_or_else(|| panic!("{id} missing"));
    let rest = &content[start + 4..];
    let end = rest
        .find("\n### ")
        .map(|index| index + 4)
        .unwrap_or(content.len() - start);
    content[start..start + end].to_string()
}

fn accept_decision(root: &Path) {
    let decision = common::read(root, "decisions/D-0001-process.md")
        .replace("- Status: proposed", "- Status: accepted");
    common::write(root, "decisions/D-0001-process.md", &decision);
}

#[test]
fn claim_and_unclaim_follow_the_status_model() {
    let repo = common::materialize_valid();
    let root = &repo.path;
    ok(root, &["claim", "GOV-002", "@alice"]);
    let claimed = block(root, "GOV-002");
    assert!(claimed.contains("- Status: in-progress"));
    assert!(claimed.contains("- Owner: @alice"));
    let message = fails(root, &["claim", "GOV-002", "@bob"]);
    assert!(message.contains("only todo tasks can be claimed"));
    fails(root, &["claim", "GOV-001", "none"]);
    ok(root, &["unclaim", "GOV-002"]);
    let released = block(root, "GOV-002");
    assert!(released.contains("- Status: todo"));
    assert!(released.contains("- Owner: none"));
}

#[test]
fn claim_rejects_xl_tasks() {
    let repo = common::materialize_valid();
    let root = &repo.path;
    let content = common::read(root, "workstreams/GOV.md").replace("- Size: L", "- Size: XL");
    common::write(root, "workstreams/GOV.md", &content);
    let message = fails(root, &["claim", "GOV-002", "@alice"]);
    assert!(message.contains("must be split"));
}

#[test]
fn done_enforces_evidence_ticks_and_verifier_policy() {
    let repo = common::materialize_valid();
    let root = &repo.path;
    let unticked = fails(root, &["done", "GOV-001", "--evidence", "decision:D-0001"]);
    assert!(unticked.contains("unticked"));
    let no_verifier = fails(
        root,
        &["done", "GOV-001", "--tick", "--evidence", "decision:D-0001"],
    );
    assert!(no_verifier.contains("--verified-by"));
    let agent = fails(
        root,
        &[
            "done",
            "GOV-001",
            "--tick",
            "--evidence",
            "decision:D-0001",
            "--verified-by",
            "@agent/claude",
        ],
    );
    assert!(agent.contains("@agent/"));
    let rejected = fails(
        root,
        &[
            "done",
            "GOV-001",
            "--tick",
            "--evidence",
            "decision:D-0001",
            "--verified-by",
            "@jake",
        ],
    );
    assert!(
        rejected.contains("E-047"),
        "decision still proposed must be rejected and reverted"
    );
    assert!(block(root, "GOV-001").contains("- Status: todo"));
    accept_decision(root);
    ok(
        root,
        &[
            "done",
            "GOV-001",
            "--tick",
            "--evidence",
            "decision:D-0001",
            "--verified-by",
            "@jake",
        ],
    );
    let done = block(root, "GOV-001");
    assert!(done.contains("- Status: done"));
    assert!(done.contains("- Verified by: @jake"));
    assert!(done.contains("- decision:D-0001"));
    assert!(!done.contains("- [ ]"));
    let dependent = fails(
        root,
        &["done", "GOV-002", "--evidence", "https://example.org/pr/1"],
    );
    assert!(dependent.contains("unticked"));
    ok(
        root,
        &[
            "done",
            "GOV-002",
            "--tick",
            "--evidence",
            "https://example.org/pr/1",
        ],
    );
    assert!(block(root, "GOV-002").contains("- https://example.org/pr/1"));
    let again = fails(
        root,
        &[
            "done",
            "GOV-002",
            "--tick",
            "--evidence",
            "https://example.org/pr/2",
        ],
    );
    assert!(again.contains("cannot be marked done"));
}

#[test]
fn drop_split_move_and_renumber_keep_history() {
    let repo = common::materialize_valid();
    let root = &repo.path;
    ok(
        root,
        &[
            "new",
            "task",
            "GOV",
            "Write the contributor guide",
            "--milestone",
            "V0",
            "--size",
            "S",
            "--baseline",
            "§1",
        ],
    );
    ok(
        root,
        &[
            "new",
            "task",
            "GOV",
            "Publish the guide",
            "--milestone",
            "V0",
            "--size",
            "S",
            "--depends",
            "GOV-003",
            "--baseline",
            "§1",
        ],
    );
    let bad_reason = fails(root, &["drop", "GOV-004", "--because", "not needed"]);
    assert!(bad_reason.contains("must start with"));
    let needs_superseder = fails(root, &["drop", "GOV-003", "--because", "duplicate GOV-002"]);
    assert!(needs_superseder.contains("--superseded-by"));
    ok(
        root,
        &[
            "drop",
            "GOV-003",
            "--because",
            "duplicate GOV-002",
            "--superseded-by",
            "GOV-002",
        ],
    );
    let dropped = block(root, "GOV-003");
    assert!(dropped.contains("- Status: dropped"));
    assert!(dropped.contains("- Superseded by: GOV-002"));
    assert!(block(root, "GOV-004").contains("- Depends on: GOV-002"));
    ok(
        root,
        &[
            "split",
            "GOV-004",
            "--into",
            "Draft the guide",
            "--into",
            "Review the guide",
            "--size",
            "S",
        ],
    );
    let parent = block(root, "GOV-004");
    assert!(parent.contains("- Status: dropped"));
    assert!(parent.contains("- Superseded by: GOV-005, GOV-006"));
    assert!(block(root, "GOV-005").contains("- Depends on: GOV-002"));
    assert!(block(root, "GOV-006").contains("### GOV-006 · Review the guide"));
    ok(root, &["move", "GOV-005", "--milestone", "V1"]);
    assert!(block(root, "GOV-005").contains("- Milestone: V1"));
    fails(root, &["move", "GOV-005", "--milestone", "V9"]);
    ok(root, &["renumber", "GOV-006", "GOV-040", "--base", "HEAD"]);
    let content = common::read(root, "workstreams/GOV.md");
    assert!(content.contains("### GOV-040 · Review the guide"));
    assert!(!content.contains("GOV-006"));
    fails(root, &["renumber", "GOV-040", "ABI-001", "--base", "HEAD"]);
}

#[test]
fn drop_never_erases_done_work() {
    let repo = common::materialize_valid();
    let root = &repo.path;
    accept_decision(root);
    ok(
        root,
        &[
            "done",
            "GOV-001",
            "--tick",
            "--evidence",
            "decision:D-0001",
            "--verified-by",
            "@jake",
        ],
    );
    let message = fails(root, &["drop", "GOV-001", "--because", "descoped"]);
    assert!(message.contains("history"));
}

#[test]
fn block_mints_a_question_and_binds_it() {
    let repo = common::materialize_valid();
    let root = &repo.path;
    common::write(
        root,
        "registers/questions.md",
        "# Questions\n\nOpen questions.\n\n### Q-001 · Which forge hosts the code\n- Workstream: GOV\n- Status: open\n- Answered by: none\nGitHub or a self-hosted forge.\n\n<!-- roadmap:generated:begin status -->\n<!-- roadmap:generated:end -->\n",
    );
    ok(
        root,
        &["block", "GOV-002", "Reference laptop is not available yet"],
    );
    let questions = common::read(root, "registers/questions.md");
    assert!(questions.contains("### Q-002 · Reference laptop is not available yet"));
    assert!(questions.contains("- Workstream: GOV"));
    assert!(block(root, "GOV-002").contains("- Depends on: GOV-001, Q-002"));
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("git");
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn committed_repo() -> common::TempRepo {
    let repo = common::materialize_valid();
    let root = &repo.path;
    git(root, &["init", "-q", "-b", "main"]);
    git(
        root,
        &["-c", "user.email=t@t", "-c", "user.name=t", "add", "-A"],
    );
    git(
        root,
        &[
            "-c",
            "user.email=t@t",
            "-c",
            "user.name=t",
            "commit",
            "-q",
            "-m",
            "base",
        ],
    );
    repo
}

fn base_codes(root: &Path) -> String {
    let output = common::run(root, &["check", "--base", "HEAD"]);
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn base_check_rejects_deleted_renumbered_and_moved_ids() {
    let repo = committed_repo();
    let root = &repo.path;
    let original = common::read(root, "workstreams/GOV.md");
    let start = original.find("### GOV-002").expect("GOV-002");
    common::write(root, "workstreams/GOV.md", &original[..start]);
    assert!(
        base_codes(root).contains("E-110"),
        "deleting an id must fail against the base"
    );
    common::write(
        root,
        "workstreams/GOV.md",
        &original.replace("GOV-002", "GOV-009"),
    );
    assert!(
        base_codes(root).contains("E-110"),
        "renumbering must fail against the base"
    );
    let moved_block = &original[start..];
    common::write(root, "workstreams/GOV.md", &original[..start]);
    common::write(
        root,
        "workstreams/ABI.md",
        &format!(
            "# ABI · Native kernel ABI\n- Prefix: ABI\n- Lead: none\n- Baseline: none\n\n## Scope\n\n## Out of scope\n\n## Tasks\n\n{moved_block}"
        ),
    );
    assert!(
        base_codes(root).contains("E-111"),
        "moving an id between files must fail"
    );
    common::write(root, "workstreams/GOV.md", &original);
    std::fs::remove_file(root.join("workstreams/ABI.md")).expect("remove");
    assert!(
        !base_codes(root).contains("E-11"),
        "the unchanged tree is clean against its base"
    );
}

#[test]
fn base_check_freezes_done_tasks() {
    let repo = committed_repo();
    let root = &repo.path;
    accept_decision(root);
    ok(
        root,
        &[
            "done",
            "GOV-001",
            "--tick",
            "--evidence",
            "decision:D-0001",
            "--verified-by",
            "@jake",
        ],
    );
    git(
        root,
        &["-c", "user.email=t@t", "-c", "user.name=t", "add", "-A"],
    );
    git(
        root,
        &[
            "-c",
            "user.email=t@t",
            "-c",
            "user.name=t",
            "commit",
            "-q",
            "-m",
            "done",
        ],
    );
    let done = common::read(root, "workstreams/GOV.md");
    common::write(
        root,
        "workstreams/GOV.md",
        &done.replace("- Size: M", "- Size: L"),
    );
    assert!(base_codes(root).contains("E-112"));
    common::write(
        root,
        "workstreams/GOV.md",
        &done.replacen(
            "- Status: done",
            "- Status: dropped\n- Dropped because: descoped",
            1,
        ),
    );
    assert!(base_codes(root).contains("E-113"));
    common::write(
        root,
        "workstreams/GOV.md",
        &done.replacen("- Status: done", "- Status: in-progress", 1),
    );
    assert!(base_codes(root).contains("E-114"));
    common::write(root, "workstreams/GOV.md", &done);
    assert!(!base_codes(root).contains("E-11"));
}
