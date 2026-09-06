pub mod anchoring;
pub mod base;
pub mod decisions;
pub mod dependencies;
pub mod fields;
pub mod ids;
pub mod milestones;
pub mod references;
pub mod registers;
pub mod status;
pub mod text;

use crate::derive::Derived;
use crate::diagnostic::Diagnostics;
use crate::repo::Repo;

pub fn run(repo: &Repo, derived: &Derived) -> Diagnostics {
    let mut diagnostics = Diagnostics::new();
    diagnostics.extend(repo.diagnostics.iter().cloned());
    fields::validate(repo, &mut diagnostics);
    ids::validate(repo, &mut diagnostics);
    references::validate(repo, &mut diagnostics);
    dependencies::validate(repo, derived, &mut diagnostics);
    status::validate(repo, derived, &mut diagnostics);
    decisions::validate(repo, &mut diagnostics);
    milestones::validate(repo, &mut diagnostics);
    registers::validate(repo, &mut diagnostics);
    text::validate(repo, &mut diagnostics);
    anchoring::validate(repo, derived, &mut diagnostics);
    diagnostics
}

pub fn policy_flag(repo: &Repo, name: &str) -> bool {
    match name {
        "require_independent_verification" => repo.config.policy.require_independent_verification,
        "verify_freezes_and_adr_always" => repo.config.policy.verify_freezes_and_adr_always,
        "verify_gate_tasks" => repo.config.policy.verify_gate_tasks,
        _ => false,
    }
}
