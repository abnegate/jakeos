use crate::derive::Derived;
use crate::diagnostic::{Diagnostic, Diagnostics, code};
use crate::model::{Status, Task, TaskType};
use crate::repo::Repo;
use crate::util::{SPIKE_HEADINGS, is_none};

pub fn validate(repo: &Repo, derived: &Derived, diagnostics: &mut Diagnostics) {
    for (position, task) in repo.tasks.iter().enumerate() {
        match task.status() {
            Status::Done => validate_done(repo, derived, task, position, diagnostics),
            Status::InProgress => validate_in_progress(task, diagnostics),
            Status::Todo | Status::Dropped => {}
        }
        if task.status() != Status::Done
            && !task.criteria.is_empty()
            && task.criteria.iter().all(|criterion| criterion.ticked)
        {
            diagnostics.push(Diagnostic::warning(
                &task.file,
                task.line,
                code::TICKED_NOT_DONE,
                format!(
                    "task `{}` has every acceptance box ticked but Status is {}",
                    task.id,
                    task.status()
                ),
                "set Status: done once evidence and verification hold, or untick remaining work",
            ));
        }
        if !task.list("Freezes").is_empty() {
            validate_freeze(repo, derived, task, position, diagnostics);
        }
    }
}

fn validate_in_progress(task: &Task, diagnostics: &mut Diagnostics) {
    if task.owner() == "none" || task.owner().is_empty() {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.fields.line_of("Owner", task.line),
            code::IN_PROGRESS_WITHOUT_OWNER,
            format!("in-progress task `{}` has Owner none", task.id),
            "set Owner to @handle or @agent/<name>, or unclaim the task",
        ));
    }
    if task.size() == "XL" {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.fields.line_of("Size", task.line),
            code::IN_PROGRESS_SIZE_XL,
            format!("in-progress task `{}` is Size XL", task.id),
            "split the XL task before claiming it",
        ));
    }
}

fn validate_done(
    repo: &Repo,
    derived: &Derived,
    task: &Task,
    position: usize,
    diagnostics: &mut Diagnostics,
) {
    for criterion in &task.criteria {
        if !criterion.ticked {
            diagnostics.push(Diagnostic::error(
                &task.file,
                criterion.line,
                code::DONE_UNTICKED_BOX,
                format!(
                    "done task `{}` has an unticked acceptance criterion",
                    task.id
                ),
                "tick every box or reopen the task",
            ));
        }
    }
    if task.verification.is_empty() {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::DONE_WITHOUT_VERIFICATION,
            format!("done task `{}` has no verification lines", task.id),
            "add at least one Verification line before marking done",
        ));
    }
    if !task.has_non_none_evidence() {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::DONE_WITHOUT_EVIDENCE,
            format!("done task `{}` has no evidence", task.id),
            "append a valid Evidence line",
        ));
    }
    if !derived.blockers[position].is_empty() {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.fields.line_of("Depends on", task.line),
            code::DONE_UNRESOLVED_DEPENDENCY,
            format!(
                "done task `{}` still depends on {}",
                task.id,
                derived.blockers[position].join(", ")
            ),
            "finish or drop-with-superseder every dependency first",
        ));
    }

    let verifier = task
        .fields
        .value("Verified by")
        .map(str::trim)
        .unwrap_or("");
    let needs_verifier = repo.config.policy.require_independent_verification
        || (repo.config.policy.verify_freezes_and_adr_always
            && (task.task_type() == TaskType::Adr || !task.list("Freezes").is_empty()));
    if needs_verifier && (verifier.is_empty() || verifier == "none") {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.fields.line_of("Verified by", task.line),
            code::DONE_WITHOUT_VERIFIER,
            format!("done task `{}` requires `Verified by: @handle`", task.id),
            "name a human verifier other than the owner",
        ));
    }
    if !verifier.is_empty() && verifier != "none" {
        if verifier == task.owner() {
            diagnostics.push(Diagnostic::error(
                &task.file,
                task.fields.line_of("Verified by", task.line),
                code::VERIFIER_IS_OWNER,
                format!("`Verified by` on `{}` matches Owner", task.id),
                "the verifier must be someone else",
            ));
        }
        if verifier.starts_with("@agent/") {
            diagnostics.push(Diagnostic::error(
                &task.file,
                task.fields.line_of("Verified by", task.line),
                code::VERIFIER_IS_AGENT,
                format!("`Verified by` on `{}` is an agent identity", task.id),
                "agents never verify tasks; use a human @handle",
            ));
        }
    }

    match task.task_type() {
        TaskType::Adr => validate_done_adr(repo, task, diagnostics),
        TaskType::Spike => validate_done_spike(repo, task, diagnostics),
        TaskType::Benchmark => validate_done_benchmark(repo, task, diagnostics),
        TaskType::Docs => validate_done_docs(task, diagnostics),
        TaskType::Build => {}
    }
}

fn validate_done_adr(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    let Some(id) = task
        .fields
        .value("Decision")
        .filter(|value| !is_none(value))
    else {
        return;
    };
    let id = id.trim();
    let Some(decision) = repo.decision(id) else {
        return;
    };
    if !matches!(decision.status(), "accepted" | "rejected") {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::ADR_DECISION_NOT_CLOSED,
            format!(
                "done adr `{}` points at `{id}` with status {}",
                task.id,
                decision.status()
            ),
            "set the decision to accepted or rejected in the same change",
        ));
    }
    let has_decision_evidence = task
        .evidence
        .iter()
        .any(|entry| entry.text.trim() == format!("decision:{id}"));
    if !has_decision_evidence {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::ADR_DECISION_NOT_CLOSED,
            format!(
                "done adr `{}` must list `decision:{id}` in Evidence",
                task.id
            ),
            format!("add `- decision:{id}`"),
        ));
    }
}

fn validate_done_spike(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    let relative = format!("reports/spikes/{}.md", task.id);
    if !repo.has_report(&relative) {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::SPIKE_REPORT_MISSING,
            format!("done spike `{}` has no `{relative}`", task.id),
            "write the spike report using the skeleton in reports/README.md",
        ));
        return;
    }
    let path = repo.absolute(&relative);
    if let Ok(content) = std::fs::read_to_string(path) {
        for heading in SPIKE_HEADINGS {
            if !content.lines().any(|line| line.trim() == heading) {
                diagnostics.push(Diagnostic::error(
                    &task.file,
                    task.line,
                    code::SPIKE_REPORT_MISSING,
                    format!("spike report `{relative}` is missing `{heading}`"),
                    "use the required spike skeleton headings",
                ));
            }
        }
    }
    let referenced = task.evidence.iter().any(|entry| {
        entry.text.contains(&relative) || entry.text.contains(&format!("spikes/{}.md", task.id))
    });
    if !referenced {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::SPIKE_REPORT_MISSING,
            format!(
                "done spike `{}` does not reference `{relative}` in Evidence",
                task.id
            ),
            format!("add `- report:{relative}`"),
        ));
    }
}

fn validate_done_benchmark(repo: &Repo, task: &Task, diagnostics: &mut Diagnostics) {
    let Some(milestone) = repo.milestone(task.milestone()) else {
        return;
    };
    let hardware = milestone.list("Hardware scope");
    if hardware.is_empty() {
        return;
    }
    for benchmark in task.list("Benchmarks") {
        let reports = repo
            .benchmark_reports
            .get(&benchmark)
            .cloned()
            .unwrap_or_default();
        for machine in &hardware {
            if !benchmark_applies_to(repo, &benchmark, machine) {
                continue;
            }
            let found = reports.iter().any(|name| name.contains(machine));
            if !found {
                let deferred = task
                    .description_text()
                    .to_ascii_lowercase()
                    .contains("defer");
                if deferred {
                    diagnostics.push(Diagnostic::warning(
                        &task.file,
                        task.line,
                        code::BENCHMARK_UNREFERENCED,
                        format!(
                            "done benchmark `{}` defers `{benchmark}` on `{machine}`",
                            task.id
                        ),
                        "record the missing report when the target is no longer deferred",
                    ));
                    continue;
                }
                diagnostics.push(Diagnostic::error(
                    &task.file,
                    task.line,
                    code::DONE_WITHOUT_EVIDENCE,
                    format!(
                        "done benchmark `{}` has no `{benchmark}` report on `{machine}`",
                        task.id
                    ),
                    format!("add reports/benchmarks/{benchmark}/<alias>@<sha>-{machine}.md"),
                ));
            }
        }
    }
}

fn benchmark_applies_to(repo: &Repo, benchmark: &str, machine: &str) -> bool {
    let Some(hardware) = repo.register_entry("H", machine) else {
        return true;
    };
    if hardware.fields.value_or_empty("Kind") != "qemu" {
        return true;
    }
    let Some(entry) = repo.register_entry("B", benchmark) else {
        return true;
    };
    let text = format!(
        "{} {}",
        entry.fields.value_or_empty("Method"),
        entry.fields.value_or_empty("Harness")
    );
    text.contains(machine) || text.to_ascii_lowercase().contains("qemu")
}

fn validate_done_docs(task: &Task, diagnostics: &mut Diagnostics) {
    let has_review = task.verification.iter().any(|line| line.kind == "Review");
    let has_url = task
        .evidence
        .iter()
        .any(|entry| entry.text.trim().starts_with("https://"));
    if !has_review && !has_url {
        diagnostics.push(Diagnostic::error(
            &task.file,
            task.line,
            code::DONE_WITHOUT_EVIDENCE,
            format!(
                "done docs task `{}` needs a Review: line or https:// evidence",
                task.id
            ),
            "add Review verification or a review URL",
        ));
    }
}

fn validate_freeze(
    repo: &Repo,
    derived: &Derived,
    task: &Task,
    position: usize,
    diagnostics: &mut Diagnostics,
) {
    let mut closure: Vec<usize> = derived
        .graph
        .transitive_dependencies(position)
        .into_iter()
        .collect();
    closure.push(position);
    for surface in task.list("Freezes") {
        let line = task.fields.line_of("Freezes", task.line);
        let mut found_spike = false;
        let mut found_decision = false;
        let mut layer = String::new();
        if let Some(entry) = repo.register_entry("S", &surface) {
            layer = entry.fields.value_or_empty("Layer").to_string();
        }
        for index in &closure {
            let candidate = &repo.tasks[*index];
            if candidate.task_type() == TaskType::Spike
                && candidate.list("Explores").contains(&surface)
            {
                found_spike = true;
            }
            if candidate.task_type() == TaskType::Adr
                && let Some(decision_id) = candidate.fields.value("Decision")
                && let Some(decision) = repo.decision(decision_id.trim())
                && decision
                    .list("Surfaces")
                    .iter()
                    .any(|item| item == &surface)
            {
                found_decision = true;
                if layer == "L1" && task.status() == Status::Done {
                    let cites_benchmark = decision.body.values().flatten().any(|text| {
                        text.contains("reports/benchmarks/")
                            || repo.patterns.benchmark_identifier.is_match(text)
                    });
                    if !cites_benchmark {
                        diagnostics.push(Diagnostic::error(
                            &task.file,
                            line,
                            code::FREEZE_DISCIPLINE,
                            format!(
                                "L1 surface `{surface}` frozen by `{}` has no benchmark cited on `{}`",
                                task.id, decision.id
                            ),
                            "cite a benchmark report on the accepted decision",
                        ));
                    }
                }
            }
        }
        if !found_spike || !found_decision {
            diagnostics.push(Diagnostic::error(
                &task.file,
                line,
                code::FREEZE_DISCIPLINE,
                format!(
                    "task `{}` lists `Freezes: {surface}` without a spike and adr in its closure",
                    task.id
                ),
                "depend on a spike that Explores the surface and an adr whose decision lists it",
            ));
        }
        if task.status() == Status::Done
            && let Some(entry) = repo.register_entry("S", &surface)
        {
            let state = entry.fields.value_or_empty("State");
            if state != "frozen" && state != "superseded" {
                diagnostics.push(Diagnostic::error(
                    &task.file,
                    line,
                    code::FREEZE_DISCIPLINE,
                    format!(
                        "done task `{}` freezes `{surface}` but the register state is `{state}`",
                        task.id
                    ),
                    "run roadmap gen to back-fill Frozen by, or set State: frozen",
                ));
            }
        }
    }
}
