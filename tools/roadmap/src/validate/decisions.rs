use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::graph::Graph;
use crate::model::{Status, TaskType, split_list};
use crate::repo::Repo;
use crate::util::is_none;
use std::collections::BTreeMap;

pub fn validate(repo: &Repo, diagnostics: &mut Diagnostics) {
    let mut by_decision: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (position, task) in repo.tasks.iter().enumerate() {
        if task.task_type() != TaskType::Adr {
            continue;
        }
        let Some(id) = task.fields.value("Decision").map(str::trim) else {
            continue;
        };
        if is_none(id) {
            continue;
        }
        by_decision
            .entry(id.to_string())
            .or_default()
            .push(position);
        if repo.decision(id).is_none() {
            diagnostics.push(Diagnostic::error(
                &task.file,
                task.fields.line_of("Decision", task.line),
                code::DECISION_FILE_MISSING,
                format!("adr task `{}` references missing decision `{id}`", task.id),
                format!("create decisions/{id}-<slug>.md from TEMPLATE.md"),
            ));
        }
    }

    for decision in &repo.decisions {
        if decision.id.is_empty() {
            continue;
        }
        let linked = by_decision.get(&decision.id).cloned().unwrap_or_default();
        if linked.len() > 1 {
            let ids: Vec<String> = linked
                .iter()
                .map(|index| repo.tasks[*index].id.clone())
                .collect();
            diagnostics.push(Diagnostic::error(
                &decision.file,
                decision.line,
                code::DECISION_TASK_NOT_UNIQUE,
                format!(
                    "decision `{}` is linked from multiple adr tasks: {}",
                    decision.id,
                    ids.join(", ")
                ),
                "keep exactly one adr task per decision file",
            ));
        }
        let declared = decision.task().trim();
        if !is_none(declared) {
            match repo.task(declared) {
                Some(task) => {
                    if task.task_type() != TaskType::Adr
                        || task.fields.value("Decision").map(str::trim)
                            != Some(decision.id.as_str())
                    {
                        diagnostics.push(Diagnostic::error(
                            &decision.file,
                            decision.fields.line_of("Task", decision.line),
                            code::DECISION_TASK_MISMATCH,
                            format!(
                                "decision `{}` lists Task `{declared}` which is not its adr task",
                                decision.id
                            ),
                            "set Task: to the adr task that carries this Decision field",
                        ));
                    } else {
                        validate_status_coupling(task, decision, diagnostics);
                    }
                }
                None => {
                    if !repo.is_example(declared) {
                        diagnostics.push(Diagnostic::error(
                            &decision.file,
                            decision.fields.line_of("Task", decision.line),
                            code::DECISION_TASK_MISMATCH,
                            format!("decision `{}` lists unknown Task `{declared}`", decision.id),
                            "point Task: at the adr task id",
                        ));
                    }
                }
            }
        }
        if linked.is_empty() && !repo.is_example(declared) {
            diagnostics.push(Diagnostic::error(
                &decision.file,
                decision.line,
                code::DECISION_TASK_MISMATCH,
                format!(
                    "decision `{}` has no adr task carrying `Decision: {}`",
                    decision.id, decision.id
                ),
                "add Type: adr with this Decision field, or drop the file",
            ));
        }
        if decision.options.len() < repo.schema.decision.min_options {
            diagnostics.push(Diagnostic::error(
                &decision.file,
                decision.line,
                code::DECISION_TOO_FEW_OPTIONS,
                format!(
                    "decision `{}` has {} option(s); {} are required",
                    decision.id,
                    decision.options.len(),
                    repo.schema.decision.min_options
                ),
                "add at least two `### Option` headings under ## Options",
            ));
        }
        for section in &repo.schema.decision.sections {
            if !decision.sections.iter().any(|name| name == section) {
                diagnostics.push(Diagnostic::error(
                    &decision.file,
                    decision.line,
                    code::DECISION_MISSING_SECTION,
                    format!("decision `{}` is missing `## {section}`", decision.id),
                    "use decisions/TEMPLATE.md",
                ));
            }
        }
        for (key, allowed) in &repo.schema.decision.enums {
            let Some(field) = decision.fields.get(key) else {
                continue;
            };
            if !allowed.contains(&field.value) {
                diagnostics.push(Diagnostic::error(
                    &decision.file,
                    field.line,
                    code::INVALID_ENUM,
                    format!("`{key}: {}` is not valid on `{}`", field.value, decision.id),
                    format!("allowed values: {}", allowed.join(" | ")),
                ));
            }
        }
        validate_surface_spikes(repo, decision, diagnostics);
    }

    validate_supersede_cycles(repo, diagnostics);
}

fn validate_status_coupling(
    task: &crate::model::Task,
    decision: &crate::model::Decision,
    diagnostics: &mut Diagnostics,
) {
    let status = task.status();
    let file_status = decision.status();
    let ok = match status {
        Status::Todo | Status::InProgress => file_status == "proposed",
        Status::Done => matches!(file_status, "accepted" | "rejected"),
        Status::Dropped => matches!(file_status, "superseded" | "proposed"),
    };
    if !ok {
        diagnostics.push(Diagnostic::error(
            &decision.file,
            decision.line,
            code::DECISION_TASK_MISMATCH,
            format!(
                "decision `{}` is `{file_status}` but task `{}` is `{status}`",
                decision.id, task.id
            ),
            "todo/in-progress ⇔ proposed; done ⇔ accepted|rejected; dropped ⇔ superseded",
        ));
    }
    if file_status == "accepted" && task.criteria.iter().any(|criterion| !criterion.ticked) {
        diagnostics.push(Diagnostic::error(
            &decision.file,
            decision.line,
            code::ADR_DECISION_NOT_CLOSED,
            format!(
                "decision `{}` is accepted while `{}` still has unticked criteria",
                decision.id, task.id
            ),
            "tick every acceptance box before accepting",
        ));
    }
}

fn validate_surface_spikes(
    repo: &Repo,
    decision: &crate::model::Decision,
    diagnostics: &mut Diagnostics,
) {
    let layer = decision.fields.value_or_empty("Layer");
    if layer != "L1" && layer != "L2" {
        return;
    }
    let surfaces = split_list(decision.fields.value_or_empty("Surfaces"));
    if surfaces.is_empty() {
        return;
    }
    let spike_ids = split_list(decision.fields.value_or_empty("Spikes"));
    for surface in surfaces {
        let found = spike_ids.iter().any(|id| {
            repo.task(id).is_some_and(|task| {
                task.task_type() == TaskType::Spike
                    && task.status() == Status::Done
                    && task.list("Explores").contains(&surface)
            })
        }) || repo.tasks.iter().any(|task| {
            task.task_type() == TaskType::Spike
                && task.status() == Status::Done
                && task.list("Explores").contains(&surface)
        });
        if !found {
            diagnostics.push(Diagnostic::error(
                &decision.file,
                decision.line,
                code::FREEZE_DISCIPLINE,
                format!(
                    "decision `{}` lists {layer} surface `{surface}` without a done spike that Explores it",
                    decision.id
                ),
                "finish a spike with Explores: naming the surface and cite it under Spikes:",
            ));
        }
    }
}

fn validate_supersede_cycles(repo: &Repo, diagnostics: &mut Diagnostics) {
    let mut index: BTreeMap<String, usize> = BTreeMap::new();
    for (position, decision) in repo.decisions.iter().enumerate() {
        if !decision.id.is_empty() {
            index.insert(decision.id.clone(), position);
        }
    }
    let mut graph = Graph::new(repo.decisions.len());
    for (position, decision) in repo.decisions.iter().enumerate() {
        for target in split_list(decision.fields.value_or_empty("Supersedes")) {
            if let Some(other) = index.get(&target) {
                graph.add_edge(position, *other);
            }
        }
    }
    for cycle in graph.cycles() {
        if cycle.len() <= 1 {
            continue;
        }
        let ids: Vec<String> = cycle
            .iter()
            .map(|item| repo.decisions[*item].id.clone())
            .collect();
        let decision = &repo.decisions[cycle[0]];
        diagnostics.push(Diagnostic::error(
            &decision.file,
            decision.line,
            code::DEPENDENCY_CYCLE,
            format!("decision supersession cycle: {}", ids.join(" -> ")),
            "break the Supersedes chain",
        ));
    }
}
