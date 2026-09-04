use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::repo::Repo;
use crate::util::{apply_glossary, is_imperative_title, strip_generated_text};

pub fn validate(repo: &Repo, diagnostics: &mut Diagnostics) {
    for task in &repo.tasks {
        if task.line_count() > repo.config.policy.task_lines_warning {
            diagnostics.push(Diagnostic::warning(
                &task.file,
                task.line,
                code::TASK_TOO_LONG,
                format!(
                    "task `{}` is {} lines (warning at {})",
                    task.id,
                    task.line_count(),
                    repo.config.policy.task_lines_warning
                ),
                "split the task",
            ));
        }
        if task.size() == "XL" {
            let text = task.description_text().to_ascii_lowercase();
            if !text
                .split(|character: char| !character.is_ascii_alphabetic())
                .any(|word| word == "split")
            {
                diagnostics.push(Diagnostic::warning(
                    &task.file,
                    task.line,
                    code::XL_WITHOUT_SPLIT,
                    format!("XL task `{}` has no split plan in its Description", task.id),
                    "name the pieces the later split will produce",
                ));
            }
        }
        if !is_imperative_title(&task.title) {
            diagnostics.push(Diagnostic::warning(
                &task.file,
                task.line,
                code::NON_IMPERATIVE_TITLE,
                format!("title of `{}` is not imperative: {}", task.id, task.title),
                "start with a verb naming the deliverable",
            ));
        }
        let canonical = apply_glossary(&task.title, &repo.glossary);
        if canonical != task.title {
            diagnostics.push(Diagnostic::warning(
                &task.file,
                task.line,
                code::GLOSSARY_CASING,
                format!(
                    "title of `{}` should use glossary casing: {canonical}",
                    task.id
                ),
                "run roadmap fmt, or spell the term as in GLOSSARY.md",
            ));
        }
        for criterion in &task.criteria {
            for word in &repo.schema.task.banned_criteria_words {
                if criterion
                    .text
                    .split(|character: char| !character.is_ascii_alphabetic())
                    .any(|token| token.eq_ignore_ascii_case(word))
                {
                    diagnostics.push(Diagnostic::warning(
                        &task.file,
                        criterion.line,
                        code::BANNED_CRITERIA_WORD,
                        format!(
                            "acceptance criterion on `{}` contains banned word `{word}`",
                            task.id
                        ),
                        "write an observable statement without 'should'",
                    ));
                }
            }
        }
        for line in task.description.iter().chain(task.out_of_scope.iter()) {
            warn_prose_numbers(repo, &task.file, task.line, line, diagnostics);
        }
        warn_prose_numbers(repo, &task.file, task.line, &task.title, diagnostics);
    }
    for workstream in &repo.workstreams {
        if workstream.line_count > repo.config.policy.workstream_lines_warning {
            diagnostics.push(Diagnostic::warning(
                &workstream.file,
                1,
                code::WORKSTREAM_TOO_LONG,
                format!(
                    "`{}` is {} lines (warning at {})",
                    workstream.file,
                    workstream.line_count,
                    repo.config.policy.workstream_lines_warning
                ),
                "open a GOV adr task proposing a split",
            ));
        }
        if let Some(lead) = workstream.fields.get("Lead")
            && !repo.patterns.owner.is_match(&lead.value)
        {
            diagnostics.push(Diagnostic::error(
                &workstream.file,
                lead.line,
                code::INVALID_OWNER,
                format!("`Lead: {}` is not a valid owner", lead.value),
                "use `none`, `@handle` or `@agent/<name>`",
            ));
        }
    }
    scan_source_files(repo, diagnostics);
}

fn warn_prose_numbers(
    repo: &Repo,
    file: &str,
    line: usize,
    text: &str,
    diagnostics: &mut Diagnostics,
) {
    if repo.patterns.percentage.is_match(text) {
        diagnostics.push(Diagnostic::warning(
            file,
            line,
            code::HAND_TYPED_PERCENT,
            format!("hand-typed percentage in `{file}`: {}", text.trim()),
            "cite a B-id or C-id; let generated views print percents",
        ));
    }
    if repo.patterns.performance_number.is_match(text) {
        diagnostics.push(Diagnostic::warning(
            file,
            line,
            code::PERFORMANCE_NUMBER,
            format!("performance number in prose in `{file}`"),
            "cite a B-id; put the number in registers/benchmarks.md",
        ));
    }
}

fn scan_source_files(repo: &Repo, diagnostics: &mut Diagnostics) {
    let mut files = Vec::new();
    for workstream in &repo.workstreams {
        files.push(workstream.file.clone());
    }
    for milestone in &repo.milestones {
        files.push(milestone.file.clone());
    }
    for decision in &repo.decisions {
        files.push(decision.file.clone());
    }
    for file in files {
        let Ok(content) = std::fs::read_to_string(repo.absolute(&file)) else {
            continue;
        };
        let stripped = strip_generated_text(&content);
        for (index, line) in stripped.lines().enumerate() {
            if repo.patterns.date.is_match(line) {
                diagnostics.push(Diagnostic::error(
                    &file,
                    index + 1,
                    code::CALENDAR_DATE,
                    format!("calendar date in `{file}`: {}", line.trim()),
                    "express sequence with Milestone and Depends on, never a date",
                ));
            }
        }
    }
    for milestone in &repo.milestones {
        for gate in &milestone.gates {
            for line in &gate.prose {
                warn_prose_numbers(repo, &milestone.file, gate.line, line, diagnostics);
            }
        }
        for demo in &milestone.demos {
            for line in &demo.prose {
                warn_prose_numbers(repo, &milestone.file, demo.line, line, diagnostics);
            }
        }
    }
}
