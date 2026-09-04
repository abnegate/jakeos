use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::model::{Status, Task, TaskType};
use crate::repo::Repo;
use crate::schema::{Conditional, SECTION_ORDER};
use crate::validate::policy_flag;
use std::collections::BTreeSet;

pub fn validate(repo: &Repo, diagnostics: &mut Diagnostics) {
    for task in &repo.tasks {
        validate_keys(repo, task, diagnostics);
        validate_enums(repo, task, diagnostics);
        validate_required(repo, task, diagnostics);
        validate_conditionals(repo, task, diagnostics);
        validate_owner(repo, task, diagnostics);
        validate_sections(repo, task, diagnostics);
        validate_type_rules(repo, task, diagnostics);
        validate_dropped_reason(repo, task, diagnostics);
    }
}

fn validate_keys(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for field in &task.fields.items {
        if !repo.schema.task.field_order.contains(&field.key) {
            diagnostics.push(Diagnostic::error(
                &task.file,
                field.line,
                code::UNKNOWN_FIELD,
                format!("unknown field `{}` in task `{}`", field.key, task.id),
                format!(
                    "allowed fields: {}",
                    repo.schema.task.field_order.join(", ")
                ),
            ));
        }
        if !seen.insert(field.key.as_str()) {
            diagnostics.push(Diagnostic::error(
                &task.file,
                field.line,
                code::DUPLICATE_FIELD,
                format!(
                    "field `{}` appears more than once in `{}`",
                    field.key, task.id
                ),
                "keep exactly one line per field",
            ));
        }
    }
}

fn validate_enums(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    for (key, allowed) in &repo.schema.task.enums {
        let Some(field) = task.fields.get(key) else {
            continue;
        };
        if !allowed.contains(&field.value) {
            diagnostics.push(Diagnostic::error(
                &task.file,
                field.line,
                code::INVALID_ENUM,
                format!(
                    "`{}: {}` is not a valid value for task `{}`",
                    key, field.value, task.id
                ),
                format!("allowed values: {}", allowed.join(" | ")),
            ));
        }
    }
    if let Some(field) = task.fields.get("Milestone")
        && !repo.schema.milestones.contains(&field.value)
    {
        diagnostics.push(Diagnostic::error(
            &task.file,
            field.line,
            code::UNKNOWN_MILESTONE,
            format!("`{}` is not a milestone token", field.value),
            format!("allowed tokens: {}", repo.schema.milestones.join(" | ")),
        ));
    }
}

fn validate_required(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    for key in &repo.schema.task.required {
        if task.fields.value(key).map(str::is_empty).unwrap_or(true) {
            diagnostics.push(Diagnostic::error(
                &task.file,
                task.line,
                code::MISSING_FIELD,
                format!("task `{}` is missing required field `{key}`", task.id),
                format!("add `- {key}: …` under the heading"),
            ));
        }
    }
}

fn condition_matches(
    repo: &Repo,
    task: &Task,
    condition: &std::collections::BTreeMap<String, String>,
) -> bool {
    condition.iter().all(|(key, expected)| match key.as_str() {
        "policy" => policy_flag(repo, expected),
        other => task.fields.value_or_empty(other) == expected,
    })
}

fn field_is_present(task: &Task, key: &str, null_token: &str) -> bool {
    task.fields
        .value(key)
        .map(|value| !value.is_empty() && value != null_token)
        .unwrap_or(false)
}

fn validate_conditionals(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    let null_token = repo.schema.task.null_token.as_str();
    for (key, conditional) in &repo.schema.task.conditional {
        let present = field_is_present(task, key, null_token);
        let line = task.fields.line_of(key, task.line);
        if let Some(condition) = &conditional.required_when
            && condition_matches(repo, task, condition)
            && !present
        {
            diagnostics.push(Diagnostic::error(
                &task.file,
                line,
                code::CONDITIONAL_REQUIRED,
                format!(
                    "task `{}` requires field `{key}` because {}",
                    task.id,
                    describe(condition)
                ),
                format!("add `- {key}: …`"),
            ));
        }
        if let Some(condition) = &conditional.forbidden_unless
            && !condition_matches(repo, task, condition)
            && present
        {
            diagnostics.push(Diagnostic::error(
                &task.file,
                line,
                code::CONDITIONAL_FORBIDDEN,
                format!(
                    "task `{}` may not carry `{key}` unless {}",
                    task.id,
                    describe(condition)
                ),
                format!("remove the `{key}` line"),
            ));
        }
        if let Some(condition) = &conditional.forbidden_when
            && condition_matches(repo, task, condition)
            && present
        {
            diagnostics.push(Diagnostic::error(
                &task.file,
                line,
                code::CONDITIONAL_FORBIDDEN,
                format!(
                    "task `{}` may not carry `{key}` when {}",
                    task.id,
                    describe(condition)
                ),
                format!("remove the `{key}` line"),
            ));
        }
        validate_single(task, key, conditional, present, line, diagnostics);
    }

    if repo.config.policy.verify_freezes_and_adr_always
        && task.status() == Status::Done
        && (task.task_type() == TaskType::Adr || !task.list("Freezes").is_empty())
        && !field_is_present(task, "Verified by", null_token)
    {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.fields.line_of("Verified by", task.line),
            code::DONE_WITHOUT_VERIFIER,
            format!(
                "done task `{}` freezes a surface or records a decision and needs independent verification",
                task.id
            ),
            "add `- Verified by: @handle` naming someone other than the owner",
        ));
    }
}

fn validate_single(
    task: &Task,
    key: &str,
    conditional: &Conditional,
    present: bool,
    line: usize,
    diagnostics: &mut Diagnostics,
) {
    if conditional.single && present && task.list(key).len() > 1 {
        diagnostics.push(Diagnostic::error(
            &task.file,
            line,
            code::SINGLE_VALUE_FIELD,
            format!("field `{key}` on `{}` accepts exactly one value", task.id),
            format!("keep one identifier on the `{key}` line"),
        ));
    }
}

fn describe(condition: &std::collections::BTreeMap<String, String>) -> String {
    condition
        .iter()
        .map(|(key, value)| {
            if key == "policy" {
                format!("policy {value} is enabled")
            } else {
                format!("{key} is {value}")
            }
        })
        .collect::<Vec<String>>()
        .join(" and ")
}

fn validate_owner(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    let Some(field) = task.fields.get("Owner") else {
        return;
    };
    if !repo.patterns.owner.is_match(&field.value) {
        diagnostics.push(Diagnostic::error(
            &task.file,
            field.line,
            code::INVALID_OWNER,
            format!("`Owner: {}` is not a valid owner", field.value),
            "use `none`, `@handle` or `@agent/<name>`",
        ));
    }
}

fn validate_sections(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    for name in SECTION_ORDER {
        let Some(section) = repo.schema.task.sections.get(name) else {
            continue;
        };
        let present = task.present_sections.iter().any(|entry| entry == name);
        if section.required && !present {
            diagnostics.push(Diagnostic::error(
                &task.file,
                task.line,
                code::MISSING_SECTION,
                format!("task `{}` is missing the `#### {name}` section", task.id),
                format!("add a `#### {name}` section to the block"),
            ));
            continue;
        }
        if !present {
            continue;
        }
        let count = match name {
            "Acceptance criteria" => task.criteria.len(),
            "Verification" => task.verification.len(),
            "Evidence" => task.evidence.len(),
            _ => usize::MAX,
        };
        if count < section.min_items {
            diagnostics.push(Diagnostic::error(
                &task.file,
                task.line,
                code::SECTION_TOO_SHORT,
                format!(
                    "`#### {name}` on `{}` needs at least {} entr{}",
                    task.id,
                    section.min_items,
                    if section.min_items == 1 { "y" } else { "ies" }
                ),
                format!("add at least {} entry to `#### {name}`", section.min_items),
            ));
        }
    }

    let allowed = repo
        .schema
        .task
        .sections
        .get("Verification")
        .map(|section| section.line_kinds.clone())
        .unwrap_or_default();
    for line in &task.verification {
        if !allowed.contains(&line.kind) {
            diagnostics.push(Diagnostic::error(
                &task.file,
                line.line,
                code::INVALID_VERIFICATION_KIND,
                format!("`{}` is not a verification kind", line.kind),
                format!("allowed kinds: {}", allowed.join(", ")),
            ));
        }
    }
}

fn validate_type_rules(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    let Some(required_kind) = repo
        .schema
        .task
        .verification_required_kind
        .get(task.task_type().label())
    else {
        return;
    };
    if task
        .verification
        .iter()
        .any(|line| &line.kind == required_kind)
    {
        return;
    }
    let (rule, hint) = match required_kind.as_str() {
        "Report" => (
            code::SPIKE_WITHOUT_REPORT_LINE,
            "add `- Report: <what the spike report must answer>`",
        ),
        _ => (
            code::BENCHMARK_WITHOUT_BENCH_LINE,
            "add `- Bench: <B-ID> <target restated, H-IDs>`",
        ),
    };
    diagnostics.push(Diagnostic::error(
        &task.file,
        task.line,
        rule,
        format!(
            "task `{}` is Type {} and needs a `{required_kind}:` verification line",
            task.id,
            task.task_type()
        ),
        hint,
    ));
}

fn validate_dropped_reason(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    if task.status() != Status::Dropped {
        return;
    }
    let line = task.fields.line_of("Dropped because", task.line);
    let Some(value) = task.fields.value("Dropped because") else {
        return;
    };
    let reasons = &repo.schema.task.dropped_reasons;
    let starts_with_reason = reasons.iter().any(|reason| {
        value == reason.as_str()
            || value.starts_with(&format!("{reason}:"))
            || value.starts_with(&format!("{reason} "))
    });
    if !starts_with_reason {
        diagnostics.push(Diagnostic::error(
            &task.file,
            line,
            code::DROPPED_REASON_ENUM,
            format!("`Dropped because: {value}` does not start with a drop reason"),
            format!("start the line with one of: {}", reasons.join(" | ")),
        ));
    }
}
