use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::model::Task;
use crate::repo::Repo;
use std::collections::{BTreeMap, BTreeSet};

pub fn validate(repo: &Repo, diagnostics: &mut Diagnostics) {
    let mut seen: BTreeMap<String, (String, usize)> = BTreeMap::new();
    for task in &repo.tasks {
        record(diagnostics, &mut seen, &task.id, &task.file, task.line);
        if task.is_draft() {
            validate_draft(repo, task, diagnostics);
        } else if task.number.is_none() {
            diagnostics.push(Diagnostic::error(
                &task.file,
                task.line,
                code::INVALID_ID_SYNTAX,
                format!("task `{}` is not a valid task id", task.id),
                "use `PREFIX-NNN` or `PREFIX-@slug` with --allow-drafts",
            ));
        }
    }
    for workstream in &repo.workstreams {
        let (start, end) = workstream.task_range;
        let mut previous: Option<&Task> = None;
        for task in &repo.tasks[start..end] {
            if let (Some(last), Some(number)) = (previous, task.number)
                && let Some(last_number) = last.number
                && number <= last_number
            {
                diagnostics.push(Diagnostic::error(
                    &task.file,
                    task.line,
                    code::NON_ASCENDING_ID,
                    format!(
                        "task `{}` is not in ascending numeric order after `{}`",
                        task.id, last.id
                    ),
                    "keep tasks sorted by numeric id inside the workstream file",
                ));
            }
            if task.number.is_some() {
                previous = Some(task);
            }
        }
    }
    for decision in &repo.decisions {
        if !decision.id.is_empty() {
            record(
                diagnostics,
                &mut seen,
                &decision.id,
                &decision.file,
                decision.line,
            );
        }
        if !decision.id.is_empty() && !repo.patterns.matches_family("D", &decision.id) {
            diagnostics.push(Diagnostic::error(
                &decision.file,
                decision.line,
                code::INVALID_ID_SYNTAX,
                format!("`{}` is not a valid decision id", decision.id),
                "use `D-NNNN`",
            ));
        }
    }
    for register in repo.registers.values() {
        for entry in &register.entries {
            record(
                diagnostics,
                &mut seen,
                &entry.id,
                &register.file,
                entry.line,
            );
        }
    }
    for milestone in &repo.milestones {
        let mut local: BTreeSet<String> = BTreeSet::new();
        for gate in &milestone.gates {
            if !local.insert(gate.id.clone()) {
                diagnostics.push(Diagnostic::error(
                    &milestone.file,
                    gate.line,
                    code::DUPLICATE_ID,
                    format!("duplicate gate id `{}`", gate.id),
                    "gate ids must be unique",
                ));
            }
            record(diagnostics, &mut seen, &gate.id, &milestone.file, gate.line);
        }
        for demo in &milestone.demos {
            if !local.insert(demo.id.clone()) {
                diagnostics.push(Diagnostic::error(
                    &milestone.file,
                    demo.line,
                    code::DUPLICATE_ID,
                    format!("duplicate demo id `{}`", demo.id),
                    "demo ids must be unique",
                ));
            }
            record(diagnostics, &mut seen, &demo.id, &milestone.file, demo.line);
        }
    }
}

fn record(
    diagnostics: &mut Diagnostics,
    seen: &mut BTreeMap<String, (String, usize)>,
    id: &str,
    file: &str,
    line: usize,
) {
    if let Some((first_file, first_line)) = seen.get(id) {
        diagnostics.push(Diagnostic::error(
            file,
            line,
            code::DUPLICATE_ID,
            format!("duplicate id `{id}` (also at {first_file}:{first_line})"),
            "never reuse an id; allocate the next unused number",
        ));
        return;
    }
    seen.insert(id.to_string(), (file.to_string(), line));
}

fn validate_draft(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    if !repo.options.allow_drafts {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::DRAFT_NOT_ALLOWED,
            format!(
                "draft id `{}` is not allowed without --allow-drafts",
                task.id
            ),
            "pass --allow-drafts --index tools/coverage/slugs.tsv or run assign-ids",
        ));
        return;
    }
    if !repo.patterns.matches_family("DRAFT", &task.id) {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::INVALID_ID_SYNTAX,
            format!("`{}` is not a valid draft id", task.id),
            "use `PREFIX-@slug` with a lowercase slug",
        ));
        return;
    }
    if let Some(index) = &repo.slugs
        && index.get(&task.id).is_none()
    {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::UNKNOWN_DRAFT,
            format!("draft `{}` is not listed in the slug index", task.id),
            "add the slug to tools/coverage/slugs.tsv before using it",
        ));
    }
}
