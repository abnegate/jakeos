use crate::derive::Derived;
use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::model::Status;
use crate::repo::Repo;

pub fn validate(repo: &Repo, derived: &Derived, diagnostics: &mut Diagnostics) {
    for (position, task) in repo.tasks.iter().enumerate() {
        let line = task.fields.line_of("Depends on", task.line);
        for identifier in task.depends_on() {
            if identifier == task.id {
                diagnostics.push(Diagnostic::error(
                    &task.file,
                    line,
                    code::SELF_DEPENDENCY,
                    format!("task `{}` depends on itself", task.id),
                    "remove the self-dependency",
                ));
            }
            let Some(target) = repo.task(&identifier) else {
                continue;
            };
            if repo.is_example(&identifier) {
                continue;
            }
            let from_rank = repo.rank(target.milestone());
            let to_rank = repo.rank(task.milestone());
            if from_rank > to_rank {
                diagnostics.push(Diagnostic::error(
                    &task.file,
                    line,
                    code::MILESTONE_MONOTONICITY,
                    format!(
                        "`{}` at {} depends on `{}` at {} (later rank)",
                        task.id,
                        task.milestone(),
                        target.id,
                        target.milestone()
                    ),
                    "move the dependency to this rung or earlier, or retarget the dependent",
                ));
            }
            if target.milestone() == "LATER" && task.milestone() != "LATER" {
                diagnostics.push(Diagnostic::error(
                    &task.file,
                    line,
                    code::MILESTONE_MONOTONICITY,
                    format!(
                        "`{}` is not LATER but depends on LATER task `{}`",
                        task.id, target.id
                    ),
                    "promote the LATER task before depending on it",
                ));
            }
            if target.status() == Status::Dropped {
                let superseders = target.list("Superseded by");
                let has_done = superseders.iter().any(|superseder| {
                    repo.task(superseder)
                        .is_some_and(|entry| entry.status() == Status::Done)
                });
                if superseders.is_empty() || !has_done {
                    diagnostics.push(Diagnostic::error(
                        &task.file,
                        line,
                        code::DROPPED_WITHOUT_SUPERSEDER,
                        format!(
                            "`{}` depends on dropped task `{}` with no done superseder",
                            task.id, target.id
                        ),
                        "repoint Depends on to a live task, or add Superseded by naming a done task",
                    ));
                }
            }
        }
        let fan_in = derived.graph.dependents[position].len();
        if fan_in > repo.config.policy.fan_in_warning {
            diagnostics.push(Diagnostic::warning(
                &task.file,
                task.line,
                code::FAN_IN,
                format!(
                    "task `{}` has {fan_in} direct dependents (threshold {})",
                    task.id, repo.config.policy.fan_in_warning
                ),
                "split the task or fan the work out through more intermediate tasks",
            ));
        }
    }

    for cycle in derived.graph.cycles() {
        if cycle.len() == 1 {
            continue;
        }
        let ids: Vec<String> = cycle
            .iter()
            .map(|index| repo.tasks[*index].id.clone())
            .collect();
        let Some(first) = cycle.first() else {
            continue;
        };
        let task = &repo.tasks[*first];
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::DEPENDENCY_CYCLE,
            format!("dependency cycle: {}", ids.join(" -> ")),
            "break the cycle by removing or reversing one Depends on edge",
        ));
    }
}
