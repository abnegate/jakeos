use crate::model::{Task, Workstream};
use crate::repo::Repo;
use crate::schema::{SECTION_ORDER, Schema};
use crate::util::{apply_glossary, display_list, ensure_trailing_newline, is_none};
use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::path::Path;

pub fn format_task(task: &Task, glossary: &[String], schema: &Schema) -> String {
    let title = apply_glossary(&task.title, glossary);
    let mut lines = Vec::new();
    lines.push(format!("### {} · {title}", task.id));
    let mut emitted: BTreeSet<String> = BTreeSet::new();
    for key in &schema.task.field_order {
        let required = schema.task.required.iter().any(|item| item == key);
        let value = task.fields.value(key).map(str::trim);
        let present = value.is_some_and(|item| !item.is_empty());
        if !present {
            if required {
                lines.push(format!("- {key}: none"));
                emitted.insert(key.clone());
            }
            continue;
        }
        let value = value.unwrap_or("none");
        if !required && is_none(value) {
            continue;
        }
        lines.push(format!("- {key}: {value}"));
        emitted.insert(key.clone());
    }
    for field in &task.fields.items {
        if emitted.contains(&field.key) {
            continue;
        }
        if is_none(&field.value) {
            continue;
        }
        lines.push(format!("- {}: {}", field.key, field.value.trim()));
    }
    lines.push(String::new());
    let mut description: Vec<String> = task
        .description
        .iter()
        .filter(|line| !line.trim().starts_with("<!-- covers:"))
        .cloned()
        .collect();
    while description
        .first()
        .is_some_and(|line| line.trim().is_empty())
    {
        description.remove(0);
    }
    while description
        .last()
        .is_some_and(|line| line.trim().is_empty())
    {
        description.pop();
    }
    if !description.is_empty() {
        lines.extend(description);
        lines.push(String::new());
    }
    if !task.covers.is_empty() {
        lines.push(format!("<!-- covers: {} -->", task.covers.join(", ")));
        lines.push(String::new());
    }
    for name in SECTION_ORDER {
        let include = match name {
            "Out of scope" => !task.out_of_scope.is_empty(),
            "Acceptance criteria" | "Verification" | "Evidence" => {
                task.present_sections.iter().any(|item| item == name) || name != "Out of scope"
            }
            _ => false,
        };
        if !include && name == "Out of scope" {
            continue;
        }
        if name != "Out of scope"
            && !task.present_sections.iter().any(|item| item == name)
            && match name {
                "Acceptance criteria" => task.criteria.is_empty(),
                "Verification" => task.verification.is_empty(),
                "Evidence" => task.evidence.is_empty(),
                _ => true,
            }
        {
            continue;
        }
        lines.push(format!("#### {name}"));
        match name {
            "Out of scope" => lines.extend(task.out_of_scope.clone()),
            "Acceptance criteria" => {
                for criterion in &task.criteria {
                    let mark = if criterion.ticked { "x" } else { " " };
                    lines.push(format!("- [{mark}] {}", criterion.text));
                }
            }
            "Verification" => {
                for line in &task.verification {
                    lines.push(format!("- {}: {}", line.kind, line.text));
                }
            }
            "Evidence" => {
                if task.evidence.is_empty() {
                    lines.push("- none".to_string());
                } else {
                    for entry in &task.evidence {
                        lines.push(format!("- {}", entry.text));
                    }
                }
            }
            _ => {}
        }
        lines.push(String::new());
    }
    while lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }
    lines.push(String::new());
    lines.join("\n")
}

pub fn format_workstream(
    content: &str,
    workstream: &Workstream,
    tasks: &[Task],
    glossary: &[String],
    schema: &Schema,
) -> String {
    let _ = workstream;
    let lines: Vec<&str> = content.lines().collect();
    let Some(first_line) = tasks.iter().map(|task| task.line).min() else {
        return ensure_trailing_newline(content);
    };
    let preamble_end = first_line.saturating_sub(1);
    let mut preamble = if preamble_end == 0 {
        String::new()
    } else {
        lines[..preamble_end.min(lines.len())].join("\n")
    };
    preamble = preamble.trim_end().to_string();
    let mut output = preamble;
    if !output.is_empty() {
        output.push('\n');
        output.push('\n');
    }
    for (index, task) in tasks.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }
        output.push_str(&format_task(task, glossary, schema));
        if !output.ends_with('\n') {
            output.push('\n');
        }
    }
    ensure_trailing_newline(&output)
}

pub fn format_repo(repo: &Repo) -> Result<Vec<(String, String, bool)>> {
    let mut changes = Vec::new();
    for workstream in &repo.workstreams {
        let relative = workstream.file.clone();
        let path = repo.absolute(&relative);
        let original =
            std::fs::read_to_string(&path).with_context(|| format!("reading {relative}"))?;
        let (start, end) = workstream.task_range;
        let formatted = format_workstream(
            &original,
            workstream,
            &repo.tasks[start..end],
            &repo.glossary,
            &repo.schema,
        );
        let changed = original != formatted;
        changes.push((relative, formatted, changed));
    }
    Ok(changes)
}

pub fn apply(repo: &Repo, check: bool) -> Result<Vec<String>> {
    let mut dirty = Vec::new();
    for (relative, formatted, changed) in format_repo(repo)? {
        if !changed {
            continue;
        }
        dirty.push(relative.clone());
        if !check {
            std::fs::write(repo.absolute(&relative), formatted)
                .with_context(|| format!("writing {relative}"))?;
        }
    }
    Ok(dirty)
}

pub fn write_workstream_tasks(
    path: &Path,
    workstream: &Workstream,
    tasks: &[Task],
    glossary: &[String],
    schema: &Schema,
) -> Result<()> {
    let original = std::fs::read_to_string(path).unwrap_or_default();
    let formatted = format_workstream(&original, workstream, tasks, glossary, schema);
    std::fs::write(path, formatted)?;
    Ok(())
}

pub fn stub_task(
    id: &str,
    title: &str,
    milestone: &str,
    size: &str,
    task_type: &str,
    depends: &[String],
    baseline: &str,
) -> String {
    let depends_value = display_list(depends);
    let mut fields = vec![
        format!("- Type: {task_type}"),
        format!("- Milestone: {milestone}"),
        "- Status: todo".to_string(),
        format!("- Size: {size}"),
        "- Owner: none".to_string(),
        format!("- Depends on: {depends_value}"),
        format!("- Baseline: {baseline}"),
    ];
    if task_type == "adr" {
        fields.push("- Decision: none".to_string());
    }
    if task_type == "benchmark" {
        fields.push("- Benchmarks: none".to_string());
    }
    format!(
        "### {id} · {title}\n{}\n\nReplace this description.\n\n#### Acceptance criteria\n- [ ] Replace this criterion.\n\n#### Verification\n- Review: Replace this verification line.\n\n#### Evidence\n- none\n",
        fields.join("\n")
    )
}
