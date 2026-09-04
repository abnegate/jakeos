use crate::derive::Derived;
use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::repo::Repo;

pub fn validate(repo: &Repo, derived: &Derived, diagnostics: &mut Diagnostics) {
    for (position, task) in repo.tasks.iter().enumerate() {
        if !derived.anchored[position] {
            diagnostics.push(Diagnostic::warning(
                &task.file,
                task.line,
                code::UNANCHORED,
                format!(
                    "task `{}` is not reachable from any gate or demo of a milestone at its rank",
                    task.id
                ),
                "cite it from a gate or demo Verified by, or depend from a task that is",
            ));
        }
    }
    for milestone in &repo.milestones {
        for gate in &milestone.gates {
            let verified = gate.verified_by();
            if verified.is_empty() {
                continue;
            }
            if verified.iter().all(|id| repo.is_example(id)) {
                diagnostics.push(Diagnostic::warning(
                    &milestone.file,
                    gate.line,
                    code::GATE_ONLY_EXAMPLES,
                    format!("gate `{}` is verified only by EX- example ids", gate.id),
                    "replace EX- placeholders with real task ids",
                ));
            }
        }
    }
}
