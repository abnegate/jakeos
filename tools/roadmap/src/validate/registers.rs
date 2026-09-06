use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::model::{parse_target_clauses, split_list};
use crate::repo::Repo;
use crate::util::is_none;
use std::collections::BTreeSet;

pub fn validate(repo: &Repo, diagnostics: &mut Diagnostics) {
    let mut referenced_benchmarks: BTreeSet<String> = BTreeSet::new();
    for task in &repo.tasks {
        for id in task.list("Benchmarks") {
            referenced_benchmarks.insert(id);
        }
    }
    for milestone in &repo.milestones {
        for gate in &milestone.gates {
            if let Some(id) = gate.fields.value("Benchmark")
                && !is_none(id)
            {
                referenced_benchmarks.insert(id.trim().to_string());
            }
        }
    }

    for (family, register) in &repo.registers {
        let Some(schema) = repo.schema.registers.get(family) else {
            continue;
        };
        for entry in &register.entries {
            for field in &entry.fields.items {
                if let Some(allowed) = schema.enums.get(&field.key)
                    && !allowed.contains(&field.value)
                    && field.value != "none"
                {
                    diagnostics.push(Diagnostic::error(
                        &register.file,
                        field.line,
                        code::REGISTER_INVALID_ENUM,
                        format!(
                            "`{}: {}` is not valid on `{}`",
                            field.key, field.value, entry.id
                        ),
                        format!("allowed values: {}", allowed.join(" | ")),
                    ));
                }
            }
            match family.as_str() {
                "B" => {
                    if let Some(field) = entry.fields.get("Targets") {
                        match parse_target_clauses(&field.value) {
                            Ok(targets) => {
                                for target in targets {
                                    if !repo.schema.milestones.contains(&target.milestone) {
                                        diagnostics.push(Diagnostic::error(
                                            &register.file,
                                            field.line,
                                            code::UNKNOWN_MILESTONE,
                                            format!(
                                                "target clause on `{}` uses unknown milestone `{}`",
                                                entry.id, target.milestone
                                            ),
                                            "use a token from fields.json milestones",
                                        ));
                                    }
                                    if !schema.target_kinds.is_empty()
                                        && !schema.target_kinds.contains(&target.kind)
                                    {
                                        diagnostics.push(Diagnostic::error(
                                            &register.file,
                                            field.line,
                                            code::REGISTER_TARGET_GRAMMAR,
                                            format!(
                                                "`{}` target kind `{}` is not allowed",
                                                entry.id, target.kind
                                            ),
                                            format!(
                                                "allowed kinds: {}",
                                                schema.target_kinds.join(" | ")
                                            ),
                                        ));
                                    }
                                }
                            }
                            Err(clause) => diagnostics.push(Diagnostic::error(
                                &register.file,
                                field.line,
                                code::REGISTER_TARGET_GRAMMAR,
                                format!("`{}` has a malformed target clause `{clause}`", entry.id),
                                "use `<TOKEN> publish`, `<TOKEN> absolute …`, or `<TOKEN> regression N% vs TOKEN`",
                            )),
                        }
                    }
                    if !referenced_benchmarks.contains(&entry.id) {
                        diagnostics.push(Diagnostic::warning(
                            &register.file,
                            entry.line,
                            code::BENCHMARK_UNREFERENCED,
                            format!("benchmark `{}` is referenced by no task or gate", entry.id),
                            "cite it from a benchmark task or a Kind: benchmark gate",
                        ));
                    }
                }
                "C" => {
                    if let Some(field) = entry.fields.get("Thresholds")
                        && parse_target_clauses(&field.value).is_err()
                        && !field.value.contains(';')
                        && !is_none(&field.value)
                    {
                        let looks_like_clauses = field
                            .value
                            .split(';')
                            .any(|clause| clause.split_whitespace().count() >= 2);
                        if looks_like_clauses && parse_target_clauses(&field.value).is_err() {
                            diagnostics.push(Diagnostic::error(
                                &register.file,
                                field.line,
                                code::REGISTER_TARGET_GRAMMAR,
                                format!("`{}` has a malformed Thresholds clause", entry.id),
                                "use per-milestone clauses separated by semicolons",
                            ));
                        }
                    }
                }
                "Q" => {
                    if let Some(field) = entry.fields.get("Workstream")
                        && field.value != "none"
                        && !repo.schema.is_workstream(&field.value)
                    {
                        diagnostics.push(Diagnostic::error(
                            &register.file,
                            field.line,
                            code::UNKNOWN_PREFIX,
                            format!(
                                "question `{}` lists unknown workstream `{}`",
                                entry.id, field.value
                            ),
                            "use a prefix from fields.json workstreams",
                        ));
                    }
                    if entry.status() == "open"
                        && is_none(entry.fields.value_or_empty("Answered by"))
                    {
                        diagnostics.push(Diagnostic::warning(
                            &register.file,
                            entry.line,
                            code::QUESTION_UNBOUND,
                            format!("open question `{}` names no answering task", entry.id),
                            "name the adr or spike that answers it in Answered by (a consumer in Depends on does not answer it), or withdraw it",
                        ));
                    }
                    if let Some(field) = entry.fields.get("Answered by")
                        && !is_none(&field.value)
                    {
                        for identifier in split_list(&field.value) {
                            let exists = repo.task(&identifier).is_some()
                                || repo.decision(&identifier).is_some()
                                || repo.is_example(&identifier);
                            if !exists {
                                diagnostics.push(Diagnostic::error(
                                    &register.file,
                                    field.line,
                                    code::REGISTER_INVALID_ID_LIST,
                                    format!(
                                        "`Answered by: {identifier}` on `{}` does not exist",
                                        entry.id
                                    ),
                                    "name a task id or D-id",
                                ));
                            }
                        }
                    }
                }
                "H" => {
                    let providers = entry.list("Provided by");
                    let declares_entry = !is_none(entry.fields.value_or_empty("Matrix entry"));
                    if declares_entry && providers.is_empty() {
                        diagnostics.push(Diagnostic::warning(
                            &register.file,
                            entry.line,
                            code::HARDWARE_UNPROVIDED,
                            format!(
                                "hardware `{}` declares a CI matrix entry but no `Provided by` task",
                                entry.id
                            ),
                            "name the procurement, bring-up or CI-profile task that makes the entry real",
                        ));
                    }
                    let first_milestone = entry.fields.value_or_empty("First milestone");
                    for provider in providers {
                        match repo.task(&provider) {
                            None => diagnostics.push(Diagnostic::error(
                                &register.file,
                                entry.fields.line_of("Provided by", entry.line),
                                code::DANGLING_REFERENCE,
                                format!(
                                    "`Provided by` on `{}` names unknown task `{provider}`",
                                    entry.id
                                ),
                                "name an existing task",
                            )),
                            Some(task) => {
                                if let (Some(first), Some(rank)) = (
                                    repo.schema.rank(first_milestone),
                                    repo.schema.rank(task.milestone()),
                                ) && rank > first
                                {
                                    diagnostics.push(Diagnostic::warning(
                                        &register.file,
                                        entry.line,
                                        code::HARDWARE_UNPROVIDED,
                                        format!(
                                            "hardware `{}` is first used at {first_milestone} but `{provider}` provides it at {}",
                                            entry.id,
                                            task.milestone()
                                        ),
                                        "move the provider earlier or the first milestone later",
                                    ));
                                }
                            }
                        }
                    }
                    if let Some(field) = entry.fields.get("First milestone")
                        && !is_none(&field.value)
                        && let Some(milestone) = repo.milestone(&field.value)
                        && !milestone
                            .list("Hardware scope")
                            .iter()
                            .any(|identifier| identifier == &entry.id)
                    {
                        diagnostics.push(Diagnostic::warning(
                            &register.file,
                            field.line,
                            code::HARDWARE_SCOPE_MISSING,
                            format!(
                                "`{}` names `First milestone: {}` but that milestone's Hardware scope omits it",
                                entry.id, field.value
                            ),
                            "add the H-ID to the milestone's Hardware scope or change First milestone",
                        ));
                    }
                    if let Some(field) = entry.fields.get("First milestone")
                        && !is_none(&field.value)
                        && !repo.schema.milestones.contains(&field.value)
                    {
                        diagnostics.push(Diagnostic::error(
                            &register.file,
                            field.line,
                            code::UNKNOWN_MILESTONE,
                            format!(
                                "`First milestone: {}` on `{}` is not a milestone token",
                                field.value, entry.id
                            ),
                            "use a token from fields.json milestones",
                        ));
                    }
                }
                "R" => {
                    if let Some(field) = entry.fields.get("Retire by")
                        && !is_none(&field.value)
                        && !repo.schema.milestones.contains(&field.value)
                    {
                        diagnostics.push(Diagnostic::error(
                            &register.file,
                            field.line,
                            code::UNKNOWN_MILESTONE,
                            format!(
                                "`Retire by: {}` on `{}` is not a milestone token",
                                field.value, entry.id
                            ),
                            "use a milestone token",
                        ));
                    }
                    validate_id_list(repo, register, entry, "Mitigated by", "TASK", diagnostics);
                }
                "T" => validate_id_list(repo, register, entry, "Addressed by", "TASK", diagnostics),
                "I" => validate_id_list(repo, register, entry, "Enforced by", "TASK", diagnostics),
                "S" => {
                    if let Some(field) = entry.fields.get("Owner")
                        && !is_none(&field.value)
                        && !repo.schema.is_workstream(&field.value)
                    {
                        diagnostics.push(Diagnostic::error(
                            &register.file,
                            field.line,
                            code::UNKNOWN_PREFIX,
                            format!(
                                "surface `{}` owner `{}` is not a workstream prefix",
                                entry.id, field.value
                            ),
                            "use a workstream prefix",
                        ));
                    }
                    validate_id_list(repo, register, entry, "Explored by", "TASK", diagnostics);
                    validate_id_list(repo, register, entry, "Decided by", "TASK", diagnostics);
                    validate_id_list(repo, register, entry, "Frozen by", "TASK", diagnostics);
                }
                _ => {}
            }
        }
    }
}

fn validate_id_list(
    repo: &Repo,
    register: &crate::model::Register,
    entry: &crate::model::RegisterEntry,
    key: &str,
    family: &str,
    diagnostics: &mut Diagnostics,
) {
    let Some(field) = entry.fields.get(key) else {
        return;
    };
    for identifier in split_list(&field.value) {
        if repo.is_example(&identifier) {
            continue;
        }
        let ok = match family {
            "TASK" => {
                repo.task(&identifier).is_some()
                    || (repo.options.allow_drafts && repo.family_of(&identifier) == Some("DRAFT"))
            }
            other => repo.register_entry(other, &identifier).is_some(),
        };
        if !ok {
            diagnostics.push(Diagnostic::error(
                &register.file,
                field.line,
                code::REGISTER_INVALID_ID_LIST,
                format!("`{key}` on `{}` lists unknown `{identifier}`", entry.id),
                "name existing ids or write none",
            ));
        }
    }
}
