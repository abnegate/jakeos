use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::model::parse_target_clauses;
use crate::repo::Repo;
use crate::util::is_none;

pub fn validate(repo: &Repo, diagnostics: &mut Diagnostics) {
    let mut seen_files: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for milestone in &repo.milestones {
        if !seen_files.insert(milestone.token.clone()) {
            diagnostics.push(Diagnostic::error(
                &milestone.file,
                milestone.line,
                code::DUPLICATE_ID,
                format!("duplicate milestone file for `{}`", milestone.token),
                "keep one file per milestone token",
            ));
        }
        if milestone.token != "LATER" && milestone.gates.is_empty() {
            diagnostics.push(Diagnostic::error(
                &milestone.file,
                milestone.line,
                code::MILESTONE_WITHOUT_GATES,
                format!("milestone `{}` has no gates", milestone.token),
                "add at least one ### TOKEN-GNN gate",
            ));
        }
        if let Some(sequence) = milestone.fields.value("Sequence")
            && let Ok(parsed) = sequence.parse::<u32>()
            && let Some(rank) = repo.schema.rank(&milestone.token)
            && parsed != rank
        {
            diagnostics.push(Diagnostic::error(
                &milestone.file,
                milestone.fields.line_of("Sequence", milestone.line),
                code::MALFORMED_BLOCK,
                format!(
                    "milestone `{}` Sequence {parsed} does not match rank {rank}",
                    milestone.token
                ),
                "set Sequence to the schema rank (LATER is 99)",
            ));
        }
        for key in &repo.schema.milestone_file.field_order {
            if key.as_str() == "Title" {
                continue;
            }
            if milestone
                .fields
                .value(key)
                .map(str::is_empty)
                .unwrap_or(true)
            {
                diagnostics.push(Diagnostic::error(
                    &milestone.file,
                    milestone.line,
                    code::MISSING_FIELD,
                    format!("milestone `{}` is missing `{key}`", milestone.token),
                    format!("add `- {key}: …`"),
                ));
            }
        }
        for section in &repo.schema.milestone_file.sections {
            if !milestone.sections.iter().any(|name| name == section) {
                diagnostics.push(Diagnostic::error(
                    &milestone.file,
                    milestone.line,
                    code::MISSING_SECTION,
                    format!("milestone `{}` is missing `## {section}`", milestone.token),
                    "use the milestone file skeleton from CONVENTIONS.md",
                ));
            }
        }
        for gate in &milestone.gates {
            validate_gate(repo, milestone, gate, diagnostics);
        }
        for demo in &milestone.demos {
            if demo.verified_by().is_empty() {
                diagnostics.push(Diagnostic::error(
                    &milestone.file,
                    demo.line,
                    code::MISSING_FIELD,
                    format!("demo `{}` is missing `Verified by`", demo.id),
                    "list the tasks that produce the demo",
                ));
            }
        }
    }
}

fn validate_gate(
    repo: &Repo,
    milestone: &crate::model::Milestone,
    gate: &crate::model::Gate,
    diagnostics: &mut Diagnostics,
) {
    for key in &repo.schema.milestone_file.gate.required {
        if is_none(gate.fields.value_or_empty(key)) {
            diagnostics.push(Diagnostic::error(
                &milestone.file,
                gate.line,
                code::MISSING_FIELD,
                format!("gate `{}` is missing `{key}`", gate.id),
                format!("add `- {key}: …`"),
            ));
        }
    }
    if let Some(kind) = gate.fields.get("Kind") {
        let allowed = repo
            .schema
            .milestone_file
            .gate
            .enums
            .get("Kind")
            .cloned()
            .unwrap_or_default();
        if !allowed.is_empty() && !allowed.contains(&kind.value) {
            diagnostics.push(Diagnostic::error(
                &milestone.file,
                kind.line,
                code::INVALID_ENUM,
                format!("`Kind: {}` is not valid on `{}`", kind.value, gate.id),
                format!("allowed values: {}", allowed.join(" | ")),
            ));
        }
    }
    let rank = repo.rank(&milestone.token);
    for identifier in gate.verified_by() {
        if repo.is_example(&identifier) {
            continue;
        }
        if let Some(task) = repo.task(&identifier) {
            let task_rank = repo.rank(task.milestone());
            if task_rank > rank {
                diagnostics.push(Diagnostic::error(
                    &milestone.file,
                    gate.line,
                    code::GATE_RANK,
                    format!(
                        "gate `{}` is verified by `{}` at {} (later than {})",
                        gate.id,
                        task.id,
                        task.milestone(),
                        milestone.token
                    ),
                    "move the verifying task to this rung or earlier",
                ));
            }
        }
    }
    match gate.kind() {
        "benchmark" => {
            let benchmark = gate.fields.value("Benchmark").map(str::trim).unwrap_or("");
            if is_none(benchmark) {
                diagnostics.push(Diagnostic::error(
                    &milestone.file,
                    gate.line,
                    code::GATE_WITHOUT_BENCHMARK,
                    format!("benchmark gate `{}` has no `Benchmark:` field", gate.id),
                    "add `- Benchmark: B-NNN`",
                ));
            } else if let Some(entry) = repo.register_entry("B", benchmark) {
                let targets = parse_target_clauses(entry.fields.value_or_empty("Targets"))
                    .unwrap_or_default();
                if !targets
                    .iter()
                    .any(|target| target.milestone == milestone.token)
                {
                    diagnostics.push(Diagnostic::error(
                        &milestone.file,
                        gate.fields.line_of("Benchmark", gate.line),
                        code::BENCHMARK_WITHOUT_TARGET,
                        format!(
                            "`{benchmark}` has no target clause for {}",
                            milestone.token
                        ),
                        "add a `<TOKEN> publish|absolute|regression …` clause on the register entry",
                    ));
                }
            }
        }
        "compatibility" => {
            let corpus = gate.fields.value("Corpus").map(str::trim).unwrap_or("");
            if is_none(corpus) {
                diagnostics.push(Diagnostic::error(
                    &milestone.file,
                    gate.line,
                    code::GATE_WITHOUT_CORPUS,
                    format!("compatibility gate `{}` has no `Corpus:` field", gate.id),
                    "add `- Corpus: C-NNN`",
                ));
            } else if let Some(entry) = repo.register_entry("C", corpus) {
                let thresholds = parse_target_clauses(entry.fields.value_or_empty("Thresholds"))
                    .unwrap_or_default();
                if !thresholds
                    .iter()
                    .any(|target| target.milestone == milestone.token)
                    && !entry
                        .fields
                        .value_or_empty("Thresholds")
                        .contains(&milestone.token)
                {
                    diagnostics.push(Diagnostic::error(
                        &milestone.file,
                        gate.fields.line_of("Corpus", gate.line),
                        code::BENCHMARK_WITHOUT_TARGET,
                        format!("`{corpus}` has no threshold clause for {}", milestone.token),
                        "add a per-milestone threshold on the corpus entry",
                    ));
                }
            }
        }
        _ => {}
    }
}
