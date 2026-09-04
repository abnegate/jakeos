use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::model::{Task, split_list};
use crate::repo::Repo;
use crate::util::is_none;
use std::collections::BTreeSet;

pub fn validate(repo: &Repo, diagnostics: &mut Diagnostics) {
    for task in &repo.tasks {
        validate_depends(repo, task, diagnostics);
        validate_id_fields(repo, task, diagnostics);
        validate_baseline(repo, task, diagnostics);
        validate_evidence(repo, task, diagnostics);
        validate_covers(repo, task, diagnostics);
    }
    for decision in &repo.decisions {
        for key in ["Task", "Surfaces", "Spikes", "Supersedes", "Superseded by"] {
            let Some(field) = decision.fields.get(key) else {
                continue;
            };
            for identifier in split_list(&field.value) {
                match key {
                    "Task" | "Spikes" => {
                        expect_task(repo, &decision.file, field.line, &identifier, diagnostics)
                    }
                    "Surfaces" => expect_family(
                        repo,
                        &decision.file,
                        field.line,
                        "S",
                        &identifier,
                        diagnostics,
                    ),
                    _ => expect_family(
                        repo,
                        &decision.file,
                        field.line,
                        "D",
                        &identifier,
                        diagnostics,
                    ),
                }
            }
        }
        if let Some(field) = decision.fields.get("Baseline") {
            validate_baseline_value(repo, &decision.file, field.line, &field.value, diagnostics);
        }
        if let Some(lines) = decision.body.get("Follow-ups") {
            for line in lines {
                for token in tokenize(line) {
                    if repo.family_of(&token).is_some() {
                        expect_existing(repo, &decision.file, decision.line, &token, diagnostics);
                    }
                }
            }
        }
    }
    for milestone in &repo.milestones {
        for key in ["Hardware scope", "Surfaces to freeze", "Risks to retire"] {
            let Some(field) = milestone.fields.get(key) else {
                continue;
            };
            let family = match key {
                "Hardware scope" => "H",
                "Surfaces to freeze" => "S",
                _ => "R",
            };
            for identifier in split_list(&field.value) {
                expect_family(
                    repo,
                    &milestone.file,
                    field.line,
                    family,
                    &identifier,
                    diagnostics,
                );
            }
        }
        if let Some(field) = milestone.fields.get("Baseline") {
            validate_baseline_value(repo, &milestone.file, field.line, &field.value, diagnostics);
        }
        for gate in &milestone.gates {
            for identifier in gate.verified_by() {
                expect_task(repo, &milestone.file, gate.line, &identifier, diagnostics);
            }
            if let Some(identifier) = gate.fields.value("Or")
                && !is_none(identifier)
            {
                expect_task(
                    repo,
                    &milestone.file,
                    gate.line,
                    identifier.trim(),
                    diagnostics,
                );
            }
            if let Some(identifier) = gate.fields.value("Benchmark")
                && !is_none(identifier)
            {
                expect_family(
                    repo,
                    &milestone.file,
                    gate.fields.line_of("Benchmark", gate.line),
                    "B",
                    identifier.trim(),
                    diagnostics,
                );
            }
            if let Some(identifier) = gate.fields.value("Corpus")
                && !is_none(identifier)
            {
                expect_family(
                    repo,
                    &milestone.file,
                    gate.fields.line_of("Corpus", gate.line),
                    "C",
                    identifier.trim(),
                    diagnostics,
                );
            }
        }
        for demo in &milestone.demos {
            for identifier in demo.verified_by() {
                expect_task(repo, &milestone.file, demo.line, &identifier, diagnostics);
            }
        }
    }
}

fn validate_depends(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    let line = task.fields.line_of("Depends on", task.line);
    let allowed: BTreeSet<&str> = repo
        .schema
        .task
        .depends_on_families
        .iter()
        .map(String::as_str)
        .collect();
    for identifier in task.depends_on() {
        if repo.is_example(&identifier) {
            continue;
        }
        if identifier == task.id {
            continue;
        }
        let Some(family) = repo.family_of(&identifier) else {
            diagnostics.push(Diagnostic::error(
                &task.file,
                line,
                code::INVALID_ID_TOKEN,
                format!(
                    "`{identifier}` on `{}` is not a task or question id",
                    task.id
                ),
                "Depends on accepts task ids and Q-ids only",
            ));
            continue;
        };
        if family == "DRAFT" {
            validate_draft_reference(repo, task, line, &identifier, diagnostics);
            continue;
        }
        if !allowed.contains(family) {
            diagnostics.push(Diagnostic::error(
                &task.file,
                line,
                code::INVALID_ID_TOKEN,
                format!("`{identifier}` is not allowed on `Depends on`"),
                "depend on the adr task, never the D-id; only task ids and Q-ids are allowed",
            ));
            continue;
        }
        expect_existing(repo, &task.file, line, &identifier, diagnostics);
    }
}

fn validate_id_fields(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    for (key, conditional) in &repo.schema.task.conditional {
        let Some(family) = &conditional.id_family else {
            continue;
        };
        let Some(field) = task.fields.get(key) else {
            continue;
        };
        for identifier in task.list(key) {
            if repo.is_example(&identifier) {
                continue;
            }
            if family == "TASK" && repo.family_of(&identifier) == Some("DRAFT") {
                validate_draft_reference(repo, task, field.line, &identifier, diagnostics);
                continue;
            }
            expect_family(
                repo,
                &task.file,
                field.line,
                family,
                &identifier,
                diagnostics,
            );
        }
    }
}

fn validate_draft_reference(
    repo: &Repo,
    task: &Task,
    line: usize,
    identifier: &str,
    diagnostics: &mut Diagnostics,
) {
    if !repo.options.allow_drafts {
        diagnostics.push(Diagnostic::error(
            &task.file,
            line,
            code::DRAFT_NOT_ALLOWED,
            format!("draft id `{identifier}` is not allowed without --allow-drafts"),
            "pass --allow-drafts --index tools/coverage/slugs.tsv",
        ));
        return;
    }
    if repo.task(identifier).is_none()
        && repo
            .slugs
            .as_ref()
            .is_none_or(|index| index.get(identifier).is_none())
    {
        diagnostics.push(Diagnostic::error(
            &task.file,
            line,
            code::UNKNOWN_DRAFT,
            format!("draft `{identifier}` is not a known task or slug-index entry"),
            "add the draft task or list it in the slug index",
        ));
        return;
    }
    if let Some(index) = &repo.slugs
        && repo.task(identifier).is_none()
        && index.get(identifier).is_none()
    {
        diagnostics.push(Diagnostic::error(
            &task.file,
            line,
            code::UNKNOWN_DRAFT,
            format!("draft `{identifier}` is not listed in the slug index"),
            "add the slug to tools/coverage/slugs.tsv",
        ));
    }
}

fn validate_baseline(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    let Some(field) = task.fields.get("Baseline") else {
        return;
    };
    if field.value == "none" {
        let workstream = repo.workstreams.get(task.workstream);
        let allowed = workstream.is_some_and(|entry| entry.has_baseline_gap());
        if !allowed {
            diagnostics.push(Diagnostic::error(
                &task.file,
                field.line,
                code::BASELINE_NONE_FORBIDDEN,
                format!(
                    "task `{}` may not use `Baseline: none` without a workstream `Baseline gap:`",
                    task.id
                ),
                "cite a § heading from BASELINE.md, or move the task to a baseline-gap workstream",
            ));
        }
        return;
    }
    validate_baseline_value(repo, &task.file, field.line, &field.value, diagnostics);
}

fn validate_baseline_value(
    repo: &Repo,
    file: &str,
    line: usize,
    value: &str,
    diagnostics: &mut Diagnostics,
) {
    if is_none(value) {
        return;
    }
    for token in split_list(value) {
        if !repo.patterns.baseline_reference.is_match(&token) {
            diagnostics.push(Diagnostic::error(
                file,
                line,
                code::BASELINE_UNRESOLVED,
                format!("`{token}` is not a §N or §N.M baseline citation"),
                "cite headings that exist in BASELINE.md",
            ));
            continue;
        }
        if !repo.baseline.resolves(&token) {
            diagnostics.push(Diagnostic::error(
                file,
                line,
                code::BASELINE_UNRESOLVED,
                format!("`{token}` does not resolve to a BASELINE.md heading"),
                "cite an existing §N or §N.M heading",
            ));
        }
    }
}

fn validate_evidence(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    let mut saw_none = false;
    let mut saw_other = false;
    for entry in &task.evidence {
        let text = entry.text.trim();
        if text == "none" {
            saw_none = true;
            continue;
        }
        saw_other = true;
        if !repo.patterns.evidence.is_match(text) {
            diagnostics.push(Diagnostic::error(
                &task.file,
                entry.line,
                code::MALFORMED_BLOCK,
                format!("evidence `{text}` is not valid grammar"),
                "use none, alias@sha, alias#n, https://, report:<path>, or decision:D-NNNN",
            ));
            continue;
        }
        if let Some(path) = text.strip_prefix("report:")
            && !repo.has_report(path)
        {
            diagnostics.push(Diagnostic::error(
                &task.file,
                entry.line,
                code::DANGLING_REFERENCE,
                format!("report `{path}` does not exist"),
                "commit the report under reports/ or drop the evidence line",
            ));
        }
        if let Some(id) = text.strip_prefix("decision:") {
            expect_family(repo, &task.file, entry.line, "D", id, diagnostics);
        }
        if let Some((alias, rest)) = text.split_once('@')
            && !text.starts_with("report:")
            && !text.starts_with("http")
            && rest.chars().all(|character| character.is_ascii_hexdigit())
            && !repo.alias_exists(alias)
        {
            diagnostics.push(Diagnostic::error(
                &task.file,
                entry.line,
                code::DANGLING_REFERENCE,
                format!("repository alias `{alias}` is not in registers/repos.md"),
                "add the alias to registers/repos.md",
            ));
        }
        if let Some((alias, rest)) = text.split_once('#')
            && rest.chars().all(|character| character.is_ascii_digit())
            && !repo.alias_exists(alias)
        {
            diagnostics.push(Diagnostic::error(
                &task.file,
                entry.line,
                code::DANGLING_REFERENCE,
                format!("repository alias `{alias}` is not in registers/repos.md"),
                "add the alias to registers/repos.md",
            ));
        }
    }
    if saw_none && saw_other {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::MALFORMED_BLOCK,
            format!("task `{}` mixes `none` with other evidence lines", task.id),
            "replace `- none` once real evidence exists",
        ));
    }
}

fn validate_covers(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    if task.covers.is_empty() {
        return;
    }
    let known: BTreeSet<&str> = repo
        .coverage_items()
        .into_iter()
        .map(|item| item.id.as_str())
        .collect();
    if known.is_empty() {
        return;
    }
    for cover in &task.covers {
        if !known.contains(cover.as_str()) {
            diagnostics.push(Diagnostic::error(
                &task.file,
                task.line,
                code::DANGLING_REFERENCE,
                format!("covers id `{cover}` is not in inventory, gaps, or extra"),
                "use an INV-, GAP-, or EXTRA- id from tools/coverage",
            ));
        }
    }
}

fn expect_task(
    repo: &Repo,
    file: &str,
    line: usize,
    identifier: &str,
    diagnostics: &mut Diagnostics,
) {
    if repo.is_example(identifier) {
        return;
    }
    if repo.family_of(identifier) == Some("DRAFT") {
        if !repo.options.allow_drafts {
            diagnostics.push(Diagnostic::error(
                file,
                line,
                code::DRAFT_NOT_ALLOWED,
                format!("draft id `{identifier}` is not allowed without --allow-drafts"),
                "pass --allow-drafts",
            ));
            return;
        }
        if repo.task(identifier).is_none()
            && repo
                .slugs
                .as_ref()
                .is_none_or(|index| index.get(identifier).is_none())
        {
            diagnostics.push(Diagnostic::error(
                file,
                line,
                code::UNKNOWN_DRAFT,
                format!("draft `{identifier}` does not exist"),
                "add the draft task or list it in the slug index",
            ));
        }
        return;
    }
    expect_family(repo, file, line, "TASK", identifier, diagnostics);
}

fn expect_family(
    repo: &Repo,
    file: &str,
    line: usize,
    family: &str,
    identifier: &str,
    diagnostics: &mut Diagnostics,
) {
    if repo.is_example(identifier) {
        return;
    }
    if !repo.patterns.matches_family(family, identifier)
        && !(family == "TASK" && repo.patterns.matches_family("DRAFT", identifier))
    {
        diagnostics.push(Diagnostic::error(
            file,
            line,
            code::INVALID_ID_TOKEN,
            format!("`{identifier}` is not a valid {family} id"),
            format!("use the {family} id family from fields.json"),
        ));
        return;
    }
    expect_existing(repo, file, line, identifier, diagnostics);
}

fn expect_existing(
    repo: &Repo,
    file: &str,
    line: usize,
    identifier: &str,
    diagnostics: &mut Diagnostics,
) {
    if repo.is_example(identifier) {
        return;
    }
    if exists(repo, identifier) {
        return;
    }
    diagnostics.push(Diagnostic::error(
        file,
        line,
        code::DANGLING_REFERENCE,
        format!("`{identifier}` does not exist"),
        "create the referenced entry or remove the citation",
    ));
}

fn exists(repo: &Repo, identifier: &str) -> bool {
    if repo.task(identifier).is_some() {
        return true;
    }
    if repo.decision(identifier).is_some() {
        return true;
    }
    if repo.milestones.iter().any(|milestone| {
        milestone.gates.iter().any(|gate| gate.id == identifier)
            || milestone.demos.iter().any(|demo| demo.id == identifier)
    }) {
        return true;
    }
    if let Some(family) = repo.family_of(identifier)
        && let Some(register) = repo.register(family)
    {
        return register.get(identifier).is_some();
    }
    false
}

fn tokenize(line: &str) -> Vec<String> {
    line.split(|character: char| {
        !(character.is_ascii_alphanumeric()
            || character == '-'
            || character == '@'
            || character == '.')
    })
    .filter(|token| !token.is_empty())
    .map(ToString::to_string)
    .collect()
}
