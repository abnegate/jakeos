use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::model::{Status, Task};
use crate::parser;
use crate::repo::Repo;
use anyhow::{Context, Result, bail};
use std::collections::BTreeMap;
use std::process::Command;

pub fn validate(repo: &Repo, base_ref: &str, diagnostics: &mut Diagnostics) -> Result<()> {
    let base = base_tasks(repo, base_ref)?;
    let current: BTreeMap<&str, &Task> = repo
        .tasks
        .iter()
        .map(|task| (task.id.as_str(), task))
        .collect();
    for (id, before) in &base {
        if before.number.is_none() {
            continue;
        }
        let Some(after) = current.get(id.as_str()) else {
            diagnostics.push(Diagnostic::error(
                &before.file,
                1,
                code::BASE_ID_REMOVED,
                format!("`{id}` exists on {base_ref} but not in the working tree"),
                "ids are never deleted or renumbered; set Status dropped instead",
            ));
            continue;
        };
        if after.file != before.file {
            diagnostics.push(Diagnostic::error(
                &after.file,
                after.line,
                code::BASE_ID_MOVED,
                format!("`{id}` moved from {} to {}", before.file, after.file),
                "tasks stay in the file that allocated them",
            ));
        }
        check_transition(base_ref, before, after, diagnostics);
    }
    Ok(())
}

fn check_transition(base_ref: &str, before: &Task, after: &Task, diagnostics: &mut Diagnostics) {
    let from = before.status();
    let to = after.status();
    match (from, to) {
        (Status::Done, Status::Dropped) => diagnostics.push(Diagnostic::error(
            &after.file,
            after.line,
            code::BASE_DONE_TO_DROPPED,
            format!("`{}` was done on {base_ref} and is now dropped", after.id),
            "done work is history; add a new task instead",
        )),
        (Status::Done, Status::InProgress | Status::Todo) => {
            if after.criteria.iter().all(|criterion| criterion.ticked) {
                diagnostics.push(Diagnostic::error(
                    &after.file,
                    after.line,
                    code::BASE_REOPENED_WITHOUT_UNTICK,
                    format!(
                        "`{}` was reopened from done without unticking an acceptance criterion",
                        after.id
                    ),
                    "untick the criterion that no longer holds in the same change",
                ));
            }
        }
        (Status::Dropped, Status::Done | Status::InProgress) => {
            diagnostics.push(Diagnostic::error(
                &after.file,
                after.line,
                code::BASE_ILLEGAL_TRANSITION,
                format!(
                    "`{}` moves from dropped to {} without passing through todo",
                    after.id,
                    to.label()
                ),
                "revive a dropped task to todo first",
            ));
        }
        _ => {}
    }
    if from == Status::Done {
        let frozen = ["Milestone", "Type", "Size"];
        for key in frozen {
            if before.fields.value_or_empty(key) != after.fields.value_or_empty(key) {
                diagnostics.push(Diagnostic::error(
                    &after.file,
                    after.fields.line_of(key, after.line),
                    code::BASE_DONE_MUTATED,
                    format!(
                        "`{}` is done on {base_ref}; `{key}` may not change",
                        after.id
                    ),
                    "done tasks are frozen except for description clarifications and evidence",
                ));
            }
        }
        let before_criteria: Vec<&str> = before
            .criteria
            .iter()
            .map(|criterion| criterion.text.as_str())
            .collect();
        let after_criteria: Vec<&str> = after
            .criteria
            .iter()
            .map(|criterion| criterion.text.as_str())
            .collect();
        if before_criteria != after_criteria {
            diagnostics.push(Diagnostic::error(
                &after.file,
                after.line,
                code::BASE_DONE_MUTATED,
                format!(
                    "`{}` is done on {base_ref}; its acceptance criteria may not change",
                    after.id
                ),
                "done tasks are frozen except for description clarifications and evidence",
            ));
        }
    }
}

fn base_tasks(repo: &Repo, base_ref: &str) -> Result<BTreeMap<String, Task>> {
    let mut tasks = BTreeMap::new();
    for (index, prefix) in repo.schema.workstreams.iter().enumerate() {
        let relative = format!("workstreams/{prefix}.md");
        let Some(content) = git_show(repo, base_ref, &relative)? else {
            continue;
        };
        let parsed = parser::workstream::parse(
            &relative,
            prefix,
            &content,
            &repo.schema,
            &repo.patterns,
            index,
        );
        for task in parsed.tasks {
            tasks.insert(task.id.clone(), task);
        }
    }
    Ok(tasks)
}

fn git_show(repo: &Repo, base_ref: &str, relative: &str) -> Result<Option<String>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(&repo.root)
        .arg("show")
        .arg(format!("{base_ref}:{relative}"))
        .output()
        .context("running git show for the base ref")?;
    if output.status.success() {
        return Ok(Some(String::from_utf8_lossy(&output.stdout).into_owned()));
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("does not exist") || stderr.contains("exists on disk, but not in") {
        return Ok(None);
    }
    bail!("git show {base_ref}:{relative} failed: {}", stderr.trim());
}
