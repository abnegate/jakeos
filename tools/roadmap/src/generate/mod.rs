use crate::derive::{Derived, MilestoneStatus, format_percent};
use crate::diagnostic::{Diagnostic, code};
use crate::graph::critical_path;
use crate::model::{Status, TaskType, split_list};
use crate::repo::Repo;
use crate::schema::Schema;
use crate::util::{GENERATED_HEADER, display_list, ensure_trailing_newline};
use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub fn render(repo: &Repo, derived: &Derived) -> Result<BTreeMap<String, String>> {
    let mut files = BTreeMap::new();
    files.insert("ROADMAP.md".to_string(), roadmap(repo, derived));
    files.insert("STATUS.md".to_string(), status(repo, derived));
    files.insert(
        "generated/ready.md".to_string(),
        with_header(&ready_markdown(repo, derived, None, None, None)),
    );
    files.insert(
        "generated/blocked.md".to_string(),
        with_header(&blocked_markdown(repo, derived, None)),
    );
    files.insert(
        "generated/critical-path.md".to_string(),
        with_header(&critical_path_markdown(repo, derived, None, 10)),
    );
    files.insert(
        "generated/benchmarks.md".to_string(),
        with_header(&benchmarks_markdown(repo)),
    );
    files.insert("generated/graph.dot".to_string(), graph_dot(repo, derived));
    let index = index_json(repo, derived)?;
    files.insert(
        "generated/dashboard.html".to_string(),
        dashboard_html(&index),
    );
    files.insert("generated/index.json".to_string(), index);
    files.insert(
        "generated/coverage-items.md".to_string(),
        coverage_items_markdown(repo),
    );
    for token in repo.schema.ordered_milestones() {
        files.insert(
            format!("generated/by-milestone/{token}.md"),
            with_header(&by_milestone(repo, derived, &token)),
        );
    }
    for workstream in &repo.workstreams {
        files.insert(
            format!("generated/by-workstream/{}.md", workstream.prefix),
            with_header(&by_workstream(repo, derived, &workstream.prefix)),
        );
    }
    for workstream in &repo.workstreams {
        let original = std::fs::read_to_string(repo.absolute(&workstream.file)).unwrap_or_default();
        let body = workstream_summary(repo, derived, &workstream.prefix);
        files.insert(
            workstream.file.clone(),
            replace_marker(&original, "summary", &body, &repo.schema),
        );
    }
    for milestone in &repo.milestones {
        let original = std::fs::read_to_string(repo.absolute(&milestone.file)).unwrap_or_default();
        let body = milestone_block(repo, derived, &milestone.token);
        files.insert(
            milestone.file.clone(),
            replace_marker(&original, "milestone", &body, &repo.schema),
        );
    }
    let decisions_readme = repo.root.join("decisions/README.md");
    if decisions_readme.is_file() {
        let original = std::fs::read_to_string(&decisions_readme).unwrap_or_default();
        files.insert(
            "decisions/README.md".to_string(),
            replace_marker(&original, "index", &decisions_index(repo), &repo.schema),
        );
    }
    let reverse = collect_reverse(repo);
    for (family, name) in [
        ("R", "status"),
        ("B", "results"),
        ("Q", "status"),
        ("S", "status"),
    ] {
        let Some(register) = repo.register(family) else {
            continue;
        };
        let path = repo.absolute(&register.file);
        if !path.is_file() {
            continue;
        }
        let original = std::fs::read_to_string(&path).unwrap_or_default();
        let rewritten = rewrite_register(&original, repo, family, &reverse);
        let body = register_block(repo, derived, family);
        files.insert(
            register.file.clone(),
            replace_marker(&rewritten, name, &body, &repo.schema),
        );
    }
    Ok(files)
}

pub fn apply(repo: &Repo, derived: &Derived, check: bool) -> Result<Vec<String>> {
    let files = render(repo, derived)?;
    let mut dirty = Vec::new();
    for (relative, content) in &files {
        let path = repo.absolute(relative);
        let current = std::fs::read_to_string(&path).ok();
        if current.as_deref() == Some(content.as_str()) {
            continue;
        }
        dirty.push(relative.clone());
        if !check {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("creating {}", parent.display()))?;
            }
            std::fs::write(&path, content).with_context(|| format!("writing {relative}"))?;
        }
    }
    Ok(dirty)
}

pub fn stale_diagnostics(repo: &Repo, derived: &Derived) -> Vec<Diagnostic> {
    let Ok(files) = render(repo, derived) else {
        return Vec::new();
    };
    let mut diagnostics = Vec::new();
    for (relative, content) in files {
        let path = repo.absolute(&relative);
        let current = std::fs::read_to_string(&path).ok();
        if current.as_deref() != Some(content.as_str()) {
            diagnostics.push(Diagnostic::warning(
                relative,
                1,
                code::GENERATED_STALE,
                "generated output is stale".to_string(),
                "run `roadmap gen`",
            ));
        }
    }
    diagnostics
}

fn with_header(body: &str) -> String {
    ensure_trailing_newline(&format!("{GENERATED_HEADER}\n\n{}", body.trim_start()))
}

fn roadmap(repo: &Repo, derived: &Derived) -> String {
    let mut lines = vec![
        GENERATED_HEADER.to_string(),
        String::new(),
        "# Roadmap".to_string(),
        String::new(),
        format!(
            "Size weights are estimates (S={}, M={}, L={}, XL={}). Progress is shown as count %, size-weighted %, and gate %. Count is done / (total excluding dropped). Weighted uses the same denominator in size-weight units. Gate progress is satisfied gates / gates.",
            repo.config.weights.small,
            repo.config.weights.medium,
            repo.config.weights.large,
            repo.config.weights.extra_large
        ),
        String::new(),
        "## Ladder".to_string(),
        String::new(),
        "| Token | Title | Status | Gates | Count | Weighted | Ready | Blocked |".to_string(),
        "| --- | --- | --- | --- | --- | --- | --- | --- |".to_string(),
    ];
    for token in repo.schema.ordered_milestones() {
        let title = repo
            .milestone(&token)
            .map(|milestone| milestone.display_title().to_string())
            .unwrap_or_else(|| token.clone());
        let status = derived
            .milestone_status
            .get(&token)
            .map(|value| value.label())
            .unwrap_or("planned");
        let progress = derived
            .milestone_progress
            .get(&token)
            .copied()
            .unwrap_or_default();
        let (ready, blocked) = counts_for_milestone(repo, derived, &token);
        lines.push(format!(
            "| {token} | {title} | {status} | {}/{} | {} ({}/{}) | {} | {ready} | {blocked} |",
            progress.gates_satisfied,
            progress.gates_total,
            format_percent(progress.count_percent()),
            progress.done,
            progress.total,
            format_percent(progress.weighted_percent()),
        ));
    }
    lines.push(String::new());
    lines.push("## Totals".to_string());
    lines.push(String::new());
    let totals = derived.totals;
    lines.push(format!(
        "Tasks {} done / {} live ({} dropped). Weighted {}. Gates {}/{}.",
        totals.done,
        totals.total,
        totals.dropped,
        format_percent(totals.weighted_percent()),
        totals.gates_satisfied,
        totals.gates_total
    ));
    lines.push(String::new());
    lines.push("## Workstream × milestone".to_string());
    lines.push(String::new());
    lines.push(workstream_grid(repo, derived));
    lines.push(String::new());
    lines.push("## Ready head".to_string());
    lines.push(String::new());
    lines.push(ready_table(repo, derived, 15));
    lines.push(String::new());
    lines.push("## Critical path".to_string());
    lines.push(String::new());
    lines.push(critical_path_markdown(
        repo,
        derived,
        active_token(derived).as_deref(),
        10,
    ));
    lines.push(String::new());
    lines.push("## Decision leverage".to_string());
    lines.push(String::new());
    lines.push(decision_leverage(repo, derived));
    lines.push(String::new());
    lines.push("## Blocked by".to_string());
    lines.push(String::new());
    lines.push(blocked_by_table(repo, derived));
    lines.push(String::new());
    ensure_trailing_newline(&lines.join("\n"))
}

fn status(repo: &Repo, derived: &Derived) -> String {
    let mut lines = vec![
        GENERATED_HEADER.to_string(),
        String::new(),
        "# Status".to_string(),
        String::new(),
        "| Workstream | Total | Done | In progress | Todo | Dropped | Ready | Blocked | Weighted |"
            .to_string(),
        "| --- | --- | --- | --- | --- | --- | --- | --- | --- |".to_string(),
    ];
    for workstream in &repo.workstreams {
        let progress = derived
            .workstream_progress
            .get(&workstream.prefix)
            .copied()
            .unwrap_or_default();
        let (ready, blocked) = counts_for_workstream(repo, derived, &workstream.prefix);
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {ready} | {blocked} | {} |",
            workstream.prefix,
            progress.total,
            progress.done,
            progress.in_progress,
            progress.todo,
            progress.dropped,
            format_percent(progress.weighted_percent())
        ));
    }
    lines.push(String::new());
    lines.push("## Unanchored".to_string());
    lines.push(String::new());
    let unanchored: Vec<&crate::model::Task> = repo
        .tasks
        .iter()
        .enumerate()
        .filter(|(index, _)| !derived.anchored[*index])
        .map(|(_, task)| task)
        .collect();
    if unanchored.is_empty() {
        lines.push("none".to_string());
    } else {
        for task in unanchored {
            lines.push(format!(
                "- {} · {} ({})",
                task.id,
                task.title,
                task.milestone()
            ));
        }
    }
    lines.push(String::new());
    lines.push("## Unowned in-progress".to_string());
    lines.push(String::new());
    let unowned: Vec<&crate::model::Task> = repo
        .tasks
        .iter()
        .filter(|task| {
            task.status() == Status::InProgress
                && (task.owner() == "none" || task.owner().is_empty())
        })
        .collect();
    if unowned.is_empty() {
        lines.push("none".to_string());
    } else {
        for task in unowned {
            lines.push(format!("- {} · {}", task.id, task.title));
        }
    }
    lines.push(String::new());
    lines.push("## Open questions blocking tasks".to_string());
    lines.push(String::new());
    lines.push(open_questions(repo, derived));
    lines.push(String::new());
    lines.push("## Steering signals".to_string());
    lines.push(String::new());
    lines.push(steering(repo, derived));
    lines.push(String::new());
    ensure_trailing_newline(&lines.join("\n"))
}

pub fn ready_markdown(
    repo: &Repo,
    derived: &Derived,
    workstream: Option<&str>,
    milestone: Option<&str>,
    size: Option<&str>,
) -> String {
    let mut lines = vec![
        "# Ready".to_string(),
        String::new(),
        "| ID | Title | Milestone | Size | Downstream | Owner |".to_string(),
        "| --- | --- | --- | --- | --- | --- |".to_string(),
    ];
    for (task, weight) in ready_ranked(repo, derived) {
        if let Some(filter) = workstream
            && task.prefix != filter
        {
            continue;
        }
        if let Some(filter) = milestone
            && task.milestone() != filter
        {
            continue;
        }
        if let Some(filter) = size
            && task.size() != filter
        {
            continue;
        }
        lines.push(format!(
            "| {} | {} | {} | {} | {weight} | {} |",
            task.id,
            task.title,
            task.milestone(),
            task.size(),
            task.owner()
        ));
    }
    if lines.len() == 4 {
        lines.push("| none |  |  |  |  |  |".to_string());
    }
    lines.join("\n") + "\n"
}

pub fn blocked_markdown(repo: &Repo, derived: &Derived, by: Option<&str>) -> String {
    let mut lines = vec![
        "# Blocked".to_string(),
        String::new(),
        "| ID | Title | Status | Blockers | Milestone |".to_string(),
        "| --- | --- | --- | --- | --- |".to_string(),
    ];
    let mut rows = 0usize;
    for (index, task) in repo.tasks.iter().enumerate() {
        if derived.state[index] != crate::model::DerivedState::Blocked {
            continue;
        }
        let blockers = &derived.blockers[index];
        if let Some(filter) = by
            && !blockers.iter().any(|item| item == filter)
        {
            continue;
        }
        rows += 1;
        lines.push(format!(
            "| {} | {} | {} | {} | {} |",
            task.id,
            task.title,
            task.status(),
            blockers.join(", "),
            task.milestone()
        ));
    }
    if rows == 0 {
        lines.push("| none |  |  |  |  |".to_string());
    }
    lines.push(String::new());
    lines.push("## Aggregated by blocker".to_string());
    lines.push(String::new());
    lines.push(blocked_by_table(repo, derived));
    lines.join("\n") + "\n"
}

pub fn critical_path_markdown(
    repo: &Repo,
    derived: &Derived,
    milestone: Option<&str>,
    top: usize,
) -> String {
    let token = milestone
        .map(ToString::to_string)
        .or_else(|| active_token(derived))
        .unwrap_or_else(|| "V0".to_string());
    let path = compute_critical(repo, derived, &token);
    let mut lines = vec![
        format!("# Critical path to {token}"),
        String::new(),
        format!("Makespan (size-weight estimates): {}.", path.makespan),
        String::new(),
    ];
    for (index, chain) in path.chains.iter().take(top).enumerate() {
        let ids: Vec<String> = chain
            .nodes
            .iter()
            .map(|node| repo.tasks[*node].id.clone())
            .collect();
        lines.push(format!(
            "{}. weight {} · {}",
            index + 1,
            chain.weight,
            ids.join(" → ")
        ));
    }
    if path.chains.is_empty() {
        lines.push("none".to_string());
    }
    lines.push(String::new());
    lines.push("## Slack".to_string());
    lines.push(String::new());
    let mut members: Vec<usize> = path.members.clone();
    members.sort_by_key(|node| (path.slack[*node], repo.tasks[*node].id.clone()));
    for node in members.iter().take(40) {
        lines.push(format!(
            "- {} slack {} ef {} lf {}",
            repo.tasks[*node].id,
            path.slack[*node],
            path.earliest_finish[*node],
            path.latest_finish[*node]
        ));
    }
    lines.join("\n") + "\n"
}

fn benchmarks_markdown(repo: &Repo) -> String {
    let mut lines = vec![
        "# Benchmarks".to_string(),
        String::new(),
        "| ID | Metric | Status | Latest reports |".to_string(),
        "| --- | --- | --- | --- |".to_string(),
    ];
    if let Some(register) = repo.register("B") {
        for entry in &register.entries {
            let reports = repo
                .benchmark_reports
                .get(&entry.id)
                .map(|files| files.join(", "))
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| "none".to_string());
            lines.push(format!(
                "| {} | {} | {} | {reports} |",
                entry.id,
                entry.fields.value_or_empty("Metric"),
                entry.status()
            ));
        }
    }
    if lines.len() == 4 {
        lines.push("| none |  |  |  |".to_string());
    }
    lines.join("\n") + "\n"
}

fn graph_dot(repo: &Repo, derived: &Derived) -> String {
    let mut lines = vec!["digraph roadmap {".to_string(), "  rankdir=LR;".to_string()];
    for task in &repo.tasks {
        if task.status() == Status::Dropped {
            continue;
        }
        lines.push(format!(
            "  \"{}\" [label=\"{}\\n{}\"];",
            task.id,
            task.id,
            task.milestone()
        ));
    }
    let mut edges: Vec<(String, String)> = Vec::new();
    for (dependent, task) in repo.tasks.iter().enumerate() {
        if task.status() == Status::Dropped {
            continue;
        }
        for dependency in &derived.graph.dependencies[dependent] {
            let source = &repo.tasks[*dependency];
            if source.status() == Status::Dropped {
                continue;
            }
            edges.push((source.id.clone(), task.id.clone()));
        }
    }
    edges.sort();
    edges.dedup();
    for (from, to) in edges {
        lines.push(format!("  \"{from}\" -> \"{to}\";"));
    }
    lines.push("}".to_string());
    lines.join("\n") + "\n"
}

fn coverage_items_markdown(repo: &Repo) -> String {
    let mut lines = vec![
        GENERATED_HEADER.to_string(),
        "# Coverage items".to_string(),
        String::new(),
        "Requirement inventory, gap sweep and critique items that tasks cite in `<!-- covers: -->` comments and in prose. Generated from `tools/coverage/*.jsonl`; edit the JSONL, never this file.".to_string(),
        String::new(),
        "| Item | Workstream | Milestone | Text |".to_string(),
        "| --- | --- | --- | --- |".to_string(),
    ];
    let mut items = repo.coverage_items();
    items.sort_by(|left, right| left.id.cmp(&right.id));
    for item in items {
        lines.push(format!(
            "| {} | {} | {} | {} |",
            item.id,
            item.workstream,
            item.milestone,
            item.text.replace('|', "\\|").replace('\n', " ")
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn index_json(repo: &Repo, derived: &Derived) -> Result<String> {
    let mut tasks = Vec::new();
    for (index, task) in repo.tasks.iter().enumerate() {
        tasks.push(TaskExport {
            id: task.id.clone(),
            title: task.title.clone(),
            r#type: task.task_type().label().to_string(),
            milestone: task.milestone().to_string(),
            status: task.status().label().to_string(),
            derived: derived.state[index].label().to_string(),
            size: task.size().to_string(),
            owner: task.owner().to_string(),
            depends_on: task.depends_on(),
            blockers: derived.blockers[index].clone(),
            covers: task.covers.clone(),
            file: task.file.clone(),
            downstream_weight: derived.downstream_weight[index],
            anchored: derived.anchored[index],
        });
    }
    let payload = IndexExport {
        tasks,
        workstreams: repo
            .workstreams
            .iter()
            .map(|workstream| WorkstreamExport {
                prefix: workstream.prefix.clone(),
                name: workstream.name.clone(),
                lead: workstream.lead().to_string(),
                file: workstream.file.clone(),
            })
            .collect(),
        milestones: repo
            .milestones
            .iter()
            .map(|milestone| MilestoneExport {
                token: milestone.token.clone(),
                title: milestone.display_title().to_string(),
                sequence: milestone.sequence(),
                file: milestone.file.clone(),
                demos: milestone
                    .demos
                    .iter()
                    .map(|demo| DemoExport {
                        id: demo.id.clone(),
                        title: demo.title.clone(),
                        verified_by: demo.verified_by(),
                    })
                    .collect(),
                status: derived
                    .milestone_status
                    .get(&milestone.token)
                    .map(|value| value.label().to_string())
                    .unwrap_or_else(|| "planned".to_string()),
                gates: milestone
                    .gates
                    .iter()
                    .map(|gate| GateExport {
                        id: gate.id.clone(),
                        title: gate.title.clone(),
                        kind: gate.kind().to_string(),
                        verified_by: gate.verified_by(),
                        satisfied: derived
                            .gate_satisfied
                            .get(&gate.id)
                            .copied()
                            .unwrap_or(false),
                        reasons: derived
                            .gate_reasons
                            .get(&gate.id)
                            .cloned()
                            .unwrap_or_default(),
                    })
                    .collect(),
            })
            .collect(),
        totals: derived.totals,
        milestone_progress: derived.milestone_progress.clone(),
        workstream_progress: derived.workstream_progress.clone(),
    };
    Ok(serde_json::to_string_pretty(&payload)? + "\n")
}

#[derive(Serialize)]
struct IndexExport {
    tasks: Vec<TaskExport>,
    workstreams: Vec<WorkstreamExport>,
    milestones: Vec<MilestoneExport>,
    totals: crate::derive::Progress,
    milestone_progress: BTreeMap<String, crate::derive::Progress>,
    workstream_progress: BTreeMap<String, crate::derive::Progress>,
}

#[derive(Serialize)]
struct TaskExport {
    id: String,
    title: String,
    r#type: String,
    milestone: String,
    status: String,
    derived: String,
    size: String,
    owner: String,
    depends_on: Vec<String>,
    blockers: Vec<String>,
    covers: Vec<String>,
    file: String,
    downstream_weight: u32,
    anchored: bool,
}

#[derive(Serialize)]
struct WorkstreamExport {
    prefix: String,
    name: String,
    lead: String,
    file: String,
}

#[derive(Serialize)]
struct DemoExport {
    id: String,
    title: String,
    verified_by: Vec<String>,
}

#[derive(Serialize)]
struct MilestoneExport {
    token: String,
    title: String,
    sequence: u32,
    file: String,
    status: String,
    gates: Vec<GateExport>,
    demos: Vec<DemoExport>,
}

#[derive(Serialize)]
struct GateExport {
    id: String,
    title: String,
    kind: String,
    verified_by: Vec<String>,
    satisfied: bool,
    reasons: Vec<String>,
}

const DASHBOARD_TEMPLATE: &str = include_str!("dashboard.html");
const DASHBOARD_DATA_MARKER: &str = "__INDEX_JSON__";

fn dashboard_html(index: &str) -> String {
    let safe = index.replace("</", "<\\/");
    DASHBOARD_TEMPLATE.replacen(DASHBOARD_DATA_MARKER, &safe, 1)
}

fn by_milestone(repo: &Repo, derived: &Derived, token: &str) -> String {
    let mut lines = vec![format!("# {token}"), String::new()];
    let mut grouped: BTreeMap<String, Vec<&crate::model::Task>> = BTreeMap::new();
    for task in &repo.tasks {
        if task.milestone() == token {
            grouped.entry(task.prefix.clone()).or_default().push(task);
        }
    }
    if grouped.is_empty() {
        lines.push("none".to_string());
        return lines.join("\n") + "\n";
    }
    for (prefix, tasks) in grouped {
        lines.push(format!("## {prefix}"));
        lines.push(String::new());
        lines.push("| ID | Title | Status | Derived | Size | Owner |".to_string());
        lines.push("| --- | --- | --- | --- | --- | --- |".to_string());
        for task in tasks {
            let index = repo.task_position(&task.id).unwrap_or(0);
            lines.push(format!(
                "| {} | {} | {} | {} | {} | {} |",
                task.id,
                task.title,
                task.status(),
                derived.state[index].label(),
                task.size(),
                task.owner()
            ));
        }
        lines.push(String::new());
    }
    lines.join("\n") + "\n"
}

fn by_workstream(repo: &Repo, derived: &Derived, prefix: &str) -> String {
    let mut lines = vec![format!("# {prefix}"), String::new()];
    let mut grouped: BTreeMap<String, Vec<&crate::model::Task>> = BTreeMap::new();
    for task in &repo.tasks {
        if task.prefix == prefix {
            grouped
                .entry(task.milestone().to_string())
                .or_default()
                .push(task);
        }
    }
    for token in repo.schema.ordered_milestones() {
        let Some(tasks) = grouped.get(&token) else {
            continue;
        };
        lines.push(format!("## {token}"));
        lines.push(String::new());
        lines.push("| ID | Title | Status | Derived | Size | Depends on |".to_string());
        lines.push("| --- | --- | --- | --- | --- | --- |".to_string());
        for task in tasks {
            let index = repo.task_position(&task.id).unwrap_or(0);
            lines.push(format!(
                "| {} | {} | {} | {} | {} | {} |",
                task.id,
                task.title,
                task.status(),
                derived.state[index].label(),
                task.size(),
                display_list(&task.depends_on())
            ));
        }
        lines.push(String::new());
    }
    lines.join("\n") + "\n"
}

fn workstream_summary(repo: &Repo, derived: &Derived, prefix: &str) -> String {
    let progress = derived
        .workstream_progress
        .get(prefix)
        .copied()
        .unwrap_or_default();
    let (ready, blocked) = counts_for_workstream(repo, derived, prefix);
    format!(
        "Tasks: {} live, {} done, {} in-progress, {} todo, {} dropped. Ready: {ready}. Blocked: {blocked}. Weighted: {}.",
        progress.total,
        progress.done,
        progress.in_progress,
        progress.todo,
        progress.dropped,
        format_percent(progress.weighted_percent())
    )
}

fn milestone_block(repo: &Repo, derived: &Derived, token: &str) -> String {
    let status = derived
        .milestone_status
        .get(token)
        .map(|value| value.label())
        .unwrap_or("planned");
    let progress = derived
        .milestone_progress
        .get(token)
        .copied()
        .unwrap_or_default();
    let mut lines = vec![
        format!("Status: {status}."),
        format!(
            "Gates: {}/{}. Count: {} ({}/{}). Weighted: {}.",
            progress.gates_satisfied,
            progress.gates_total,
            format_percent(progress.count_percent()),
            progress.done,
            progress.total,
            format_percent(progress.weighted_percent())
        ),
        String::new(),
        "| Gate | Kind | Satisfied |".to_string(),
        "| --- | --- | --- |".to_string(),
    ];
    if let Some(milestone) = repo.milestone(token) {
        for gate in &milestone.gates {
            let satisfied = derived
                .gate_satisfied
                .get(&gate.id)
                .copied()
                .unwrap_or(false);
            lines.push(format!(
                "| {} | {} | {} |",
                gate.id,
                gate.kind(),
                if satisfied { "yes" } else { "no" }
            ));
        }
    }
    lines.join("\n")
}

fn decisions_index(repo: &Repo) -> String {
    let mut lines = vec![
        "| ID | Title | Status | Task | Surfaces |".to_string(),
        "| --- | --- | --- | --- | --- |".to_string(),
    ];
    for decision in &repo.decisions {
        lines.push(format!(
            "| {} | {} | {} | {} | {} |",
            decision.id,
            decision.title,
            decision.status(),
            decision.task(),
            display_list(&decision.list("Surfaces"))
        ));
    }
    if repo.decisions.is_empty() {
        lines.push("| none |  |  |  |  |".to_string());
    }
    lines.join("\n")
}

fn register_block(repo: &Repo, _derived: &Derived, family: &str) -> String {
    let Some(register) = repo.register(family) else {
        return String::new();
    };
    let mut lines = vec![
        "| ID | Title | Status |".to_string(),
        "| --- | --- | --- |".to_string(),
    ];
    for entry in &register.entries {
        lines.push(format!(
            "| {} | {} | {} |",
            entry.id,
            entry.title,
            entry.status()
        ));
    }
    lines.join("\n")
}

fn replace_marker(content: &str, name: &str, body: &str, schema: &Schema) -> String {
    let begin = schema.marker_begin(name);
    let end = schema.marker_end();
    let replacement = {
        let mut block = begin.clone();
        block.push('\n');
        let trimmed = body.trim_end();
        if !trimmed.is_empty() {
            block.push_str(trimmed);
            block.push('\n');
        }
        block.push_str(end);
        block
    };
    if let Some(start) = content.find(&begin) {
        let search_from = start + begin.len();
        if let Some(rel) = content[search_from..].find(end) {
            let end_at = search_from + rel + end.len();
            let mut output = String::new();
            output.push_str(&content[..start]);
            output.push_str(&replacement);
            output.push_str(&content[end_at..]);
            if !output.ends_with('\n') && content.ends_with('\n') {
                output.push('\n');
            }
            return ensure_trailing_newline(&output);
        }
    }
    let mut output = content.trim_end().to_string();
    output.push('\n');
    output.push('\n');
    output.push_str(&replacement);
    output.push('\n');
    output
}

fn collect_reverse(repo: &Repo) -> BTreeMap<String, BTreeMap<String, Vec<String>>> {
    let mut map: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    let add = |map: &mut BTreeMap<String, BTreeMap<String, Vec<String>>>,
               id: &str,
               field: &str,
               task_id: &str| {
        let list = map
            .entry(id.to_string())
            .or_default()
            .entry(field.to_string())
            .or_default();
        if !list.iter().any(|item| item == task_id) {
            list.push(task_id.to_string());
        }
    };
    for task in &repo.tasks {
        if task.status() == Status::Dropped {
            continue;
        }
        for id in task.list("Risks") {
            add(&mut map, &id, "Mitigated by", &task.id);
        }
        for id in task.list("Threats") {
            add(&mut map, &id, "Addressed by", &task.id);
        }
        for id in task.list("Invariants") {
            add(&mut map, &id, "Enforced by", &task.id);
        }
        for id in task.list("Explores") {
            add(&mut map, &id, "Explored by", &task.id);
        }
        if task.status() == Status::Done {
            for id in task.list("Freezes") {
                add(&mut map, &id, "Frozen by", &task.id);
            }
        }
        if task.task_type() == TaskType::Adr
            && let Some(decision_id) = task.fields.value("Decision")
            && let Some(decision) = repo.decision(decision_id.trim())
        {
            for surface in decision.list("Surfaces") {
                add(&mut map, &surface, "Decided by", &task.id);
            }
        }
    }
    for lists in map.values_mut() {
        for values in lists.values_mut() {
            values.sort();
            values.dedup();
        }
    }
    map
}

fn rewrite_register(
    original: &str,
    repo: &Repo,
    family: &str,
    reverse: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
) -> String {
    let Some(register) = repo.register(family) else {
        return original.to_string();
    };
    let mut lines: Vec<String> = original.lines().map(ToString::to_string).collect();
    let fields: &[&str] = match family {
        "R" => &["Mitigated by"],
        "T" => &["Addressed by"],
        "I" => &["Enforced by"],
        "S" => &["Explored by", "Decided by", "Frozen by"],
        _ => &[],
    };
    for entry in &register.entries {
        let values = reverse.get(&entry.id);
        for field in fields {
            let rendered = values
                .and_then(|map| map.get(*field))
                .filter(|list| !list.is_empty())
                .map(|list| list.join(", "))
                .unwrap_or_else(|| "none".to_string());
            if let Some(existing) = entry.fields.get(field) {
                let index = existing.line.saturating_sub(1);
                if index < lines.len() {
                    lines[index] = format!("- {field}: {rendered}");
                }
            }
        }
        if family == "S" {
            let frozen = values
                .and_then(|map| map.get("Frozen by"))
                .is_some_and(|list| !list.is_empty());
            if frozen
                && let Some(state) = entry.fields.get("State")
                && state.value != "superseded"
            {
                let index = state.line.saturating_sub(1);
                if index < lines.len() {
                    lines[index] = "- State: frozen".to_string();
                }
            }
        }
    }
    let mut output = lines.join("\n");
    if original.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn ready_ranked<'a>(repo: &'a Repo, derived: &Derived) -> Vec<(&'a crate::model::Task, u32)> {
    let mut rows: Vec<(&crate::model::Task, u32, &str)> = repo
        .tasks
        .iter()
        .enumerate()
        .filter(|(index, _)| derived.state[*index] == crate::model::DerivedState::Ready)
        .map(|(index, task)| (task, derived.downstream_weight[index], task.id.as_str()))
        .collect();
    rows.sort_by(|left, right| right.1.cmp(&left.1).then(left.2.cmp(right.2)));
    rows.into_iter()
        .map(|(task, weight, _)| (task, weight))
        .collect()
}

fn ready_table(repo: &Repo, derived: &Derived, limit: usize) -> String {
    let mut lines = vec![
        "| ID | Title | Milestone | Size | Downstream |".to_string(),
        "| --- | --- | --- | --- | --- |".to_string(),
    ];
    for (task, weight) in ready_ranked(repo, derived).into_iter().take(limit) {
        lines.push(format!(
            "| {} | {} | {} | {} | {weight} |",
            task.id,
            task.title,
            task.milestone(),
            task.size()
        ));
    }
    if lines.len() == 2 {
        lines.push("| none |  |  |  |  |".to_string());
    }
    lines.join("\n")
}

fn blocked_by_table(repo: &Repo, derived: &Derived) -> String {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for (index, _) in repo.tasks.iter().enumerate() {
        if derived.state[index] != crate::model::DerivedState::Blocked {
            continue;
        }
        for blocker in &derived.blockers[index] {
            *counts.entry(blocker.clone()).or_insert(0) += 1;
        }
    }
    if counts.is_empty() {
        return "none".to_string();
    }
    let mut rows: Vec<(usize, String)> =
        counts.into_iter().map(|(id, count)| (count, id)).collect();
    rows.sort_by(|left, right| right.0.cmp(&left.0).then(left.1.cmp(&right.1)));
    let mut lines = vec![
        "| Blocker | Tasks |".to_string(),
        "| --- | --- |".to_string(),
    ];
    for (count, id) in rows {
        lines.push(format!("| {id} | {count} |"));
    }
    lines.join("\n")
}

fn decision_leverage(repo: &Repo, derived: &Derived) -> String {
    let mut rows: Vec<(u32, usize, String, String)> = Vec::new();
    for (index, task) in repo.tasks.iter().enumerate() {
        if task.task_type() != TaskType::Adr {
            continue;
        }
        if task.status() == Status::Done || task.status() == Status::Dropped {
            continue;
        }
        rows.push((
            derived.downstream_weight[index],
            derived.downstream[index].len(),
            task.id.clone(),
            task.title.clone(),
        ));
    }
    rows.sort_by(|left, right| right.0.cmp(&left.0).then(left.2.cmp(&right.2)));
    if rows.is_empty() {
        return "none".to_string();
    }
    let mut lines = vec![
        "| ID | Title | Downstream tasks | Downstream weight |".to_string(),
        "| --- | --- | --- | --- |".to_string(),
    ];
    for (weight, count, id, title) in rows.into_iter().take(20) {
        lines.push(format!("| {id} | {title} | {count} | {weight} |"));
    }
    lines.join("\n")
}

fn workstream_grid(repo: &Repo, derived: &Derived) -> String {
    let tokens = repo.schema.ordered_milestones();
    let mut header = String::from("| Workstream |");
    for token in &tokens {
        header.push(' ');
        header.push_str(token);
        header.push_str(" |");
    }
    let mut divider = String::from("| --- |");
    for _ in &tokens {
        divider.push_str(" --- |");
    }
    let mut lines = vec![header, divider];
    for workstream in &repo.workstreams {
        let mut row = format!("| {} |", workstream.prefix);
        for token in &tokens {
            let count = repo
                .tasks
                .iter()
                .filter(|task| {
                    task.prefix == workstream.prefix
                        && task.milestone() == token
                        && task.status() != Status::Dropped
                })
                .count();
            let done = repo
                .tasks
                .iter()
                .filter(|task| {
                    task.prefix == workstream.prefix
                        && task.milestone() == token
                        && task.status() == Status::Done
                })
                .count();
            let _ = derived;
            row.push_str(&format!(" {done}/{count} |"));
        }
        lines.push(row);
    }
    if repo.workstreams.is_empty() {
        lines.push("| none |".to_string());
    }
    lines.join("\n")
}

fn counts_for_milestone(repo: &Repo, derived: &Derived, token: &str) -> (usize, usize) {
    let mut ready = 0usize;
    let mut blocked = 0usize;
    for (index, task) in repo.tasks.iter().enumerate() {
        if task.milestone() != token {
            continue;
        }
        match derived.state[index] {
            crate::model::DerivedState::Ready => ready += 1,
            crate::model::DerivedState::Blocked => blocked += 1,
            _ => {}
        }
    }
    (ready, blocked)
}

fn counts_for_workstream(repo: &Repo, derived: &Derived, prefix: &str) -> (usize, usize) {
    let mut ready = 0usize;
    let mut blocked = 0usize;
    for (index, task) in repo.tasks.iter().enumerate() {
        if task.prefix != prefix {
            continue;
        }
        match derived.state[index] {
            crate::model::DerivedState::Ready => ready += 1,
            crate::model::DerivedState::Blocked => blocked += 1,
            _ => {}
        }
    }
    (ready, blocked)
}

fn active_token(derived: &Derived) -> Option<String> {
    derived
        .milestone_status
        .iter()
        .find(|(_, status)| **status == MilestoneStatus::Active)
        .map(|(token, _)| token.clone())
}

fn open_questions(repo: &Repo, derived: &Derived) -> String {
    let mut lines = Vec::new();
    for (index, task) in repo.tasks.iter().enumerate() {
        for blocker in &derived.blockers[index] {
            if blocker.starts_with("Q-") {
                lines.push(format!("- {} blocks {}", blocker, task.id));
            }
        }
    }
    if lines.is_empty() {
        "none".to_string()
    } else {
        lines.sort();
        lines.join("\n")
    }
}

fn steering(repo: &Repo, derived: &Derived) -> String {
    let active = active_token(derived);
    let active_rank = active.as_deref().map(|token| repo.rank(token)).unwrap_or(1);
    let next_rank = derived
        .milestone_status
        .iter()
        .find(|(_, status)| **status == MilestoneStatus::Next)
        .map(|(token, _)| repo.rank(token));
    let path = active
        .as_deref()
        .map(|token| compute_critical(repo, derived, token));
    let mut lines = Vec::new();
    for (index, task) in repo.tasks.iter().enumerate() {
        if task.status() == Status::Dropped {
            continue;
        }
        let rank = repo.rank(task.milestone());
        if task.size() == "XL" && (Some(rank) == Some(active_rank) || Some(rank) == next_rank) {
            lines.push(format!(
                "- XL task {} sits on the active or next rung ({})",
                task.id,
                task.milestone()
            ));
        }
        if let Some(path) = &path
            && path.members.contains(&index)
            && task.owner() == "none"
            && Some(rank) == Some(active_rank)
        {
            lines.push(format!(
                "- critical-path task {} has Owner none on the active rung",
                task.id
            ));
        }
        if task.status() == Status::InProgress && rank > active_rank.saturating_add(1) {
            lines.push(format!(
                "- in-progress task {} is more than one rung ahead of active",
                task.id
            ));
        }
    }
    if lines.is_empty() {
        "none".to_string()
    } else {
        lines.sort();
        lines.join("\n")
    }
}

fn compute_critical(repo: &Repo, derived: &Derived, token: &str) -> crate::graph::CriticalPath {
    let mut targets = Vec::new();
    if let Some(milestone) = repo.milestone(token) {
        for gate in &milestone.gates {
            for id in gate.verified_by() {
                if let Some(index) = repo.task_position(&id) {
                    targets.push(index);
                }
            }
        }
    }
    targets.sort_unstable();
    targets.dedup();
    let include: BTreeSet<usize> = if targets.is_empty() {
        repo.tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| task.status() != Status::Dropped)
            .map(|(index, _)| index)
            .collect()
    } else {
        derived
            .graph
            .closure_of(&targets)
            .into_iter()
            .filter(|index| repo.tasks[*index].status() != Status::Dropped)
            .collect()
    };
    let weights: Vec<u32> = repo.tasks.iter().map(|task| repo.weight(task)).collect();
    critical_path(&derived.graph, &weights, &targets, &include)
}

pub fn gate_report(repo: &Repo, derived: &Derived, token: &str) -> String {
    let Some(milestone) = repo.milestone(token) else {
        return format!("unknown milestone `{token}`\n");
    };
    let mut lines = vec![format!("# {} gates", milestone.token), String::new()];
    for gate in &milestone.gates {
        let satisfied = derived
            .gate_satisfied
            .get(&gate.id)
            .copied()
            .unwrap_or(false);
        lines.push(format!(
            "## {} · {} ({})",
            gate.id,
            gate.title,
            if satisfied {
                "satisfied"
            } else {
                "unsatisfied"
            }
        ));
        let reasons = derived
            .gate_reasons
            .get(&gate.id)
            .cloned()
            .unwrap_or_default();
        if reasons.is_empty() {
            lines.push("ok".to_string());
        } else {
            for reason in reasons {
                lines.push(format!("- {reason}"));
            }
        }
        lines.push(String::new());
    }
    lines.join("\n")
}

pub fn impact_report(repo: &Repo, derived: &Derived, id: &str, summary: bool) -> Option<String> {
    let index = repo.task_position(id)?;
    let dependents = &derived.downstream[index];
    let weight = derived.downstream_weight[index];
    if summary {
        return Some(format!(
            "{id} unblocks {} tasks (weight {weight})\n",
            dependents.len()
        ));
    }
    let mut lines = vec![
        format!("# Impact of {id}"),
        String::new(),
        format!("{} tasks, weight {weight}.", dependents.len()),
        String::new(),
        "| ID | Title | Milestone | Status | Size |".to_string(),
        "| --- | --- | --- | --- | --- |".to_string(),
    ];
    for dependent in dependents {
        let task = &repo.tasks[*dependent];
        lines.push(format!(
            "| {} | {} | {} | {} | {} |",
            task.id,
            task.title,
            task.milestone(),
            task.status(),
            task.size()
        ));
    }
    Some(lines.join("\n") + "\n")
}

pub fn progress_markdown(repo: &Repo, derived: &Derived) -> String {
    let mut lines = vec![
        "# Progress".to_string(),
        String::new(),
        "## Milestones".to_string(),
        String::new(),
    ];
    lines.push("| Token | Done | Total | Weighted | Gates |".to_string());
    lines.push("| --- | --- | --- | --- | --- |".to_string());
    for token in repo.schema.ordered_milestones() {
        let progress = derived
            .milestone_progress
            .get(&token)
            .copied()
            .unwrap_or_default();
        lines.push(format!(
            "| {token} | {} | {} | {} | {}/{} |",
            progress.done,
            progress.total,
            format_percent(progress.weighted_percent()),
            progress.gates_satisfied,
            progress.gates_total
        ));
    }
    lines.push(String::new());
    lines.push("## Workstreams".to_string());
    lines.push(String::new());
    lines.push("| Prefix | Done | Total | Weighted |".to_string());
    lines.push("| --- | --- | --- | --- |".to_string());
    for workstream in &repo.workstreams {
        let progress = derived
            .workstream_progress
            .get(&workstream.prefix)
            .copied()
            .unwrap_or_default();
        lines.push(format!(
            "| {} | {} | {} | {} |",
            workstream.prefix,
            progress.done,
            progress.total,
            format_percent(progress.weighted_percent())
        ));
    }
    lines.join("\n") + "\n"
}

pub fn show_task(repo: &Repo, derived: &Derived, id: &str) -> Option<String> {
    let index = repo.task_position(id)?;
    let task = &repo.tasks[index];
    let mut lines = vec![crate::fmt::format_task(task, &repo.glossary, &repo.schema)];
    lines.push(format!("Derived: {}", derived.state[index].label()));
    if !derived.blockers[index].is_empty() {
        lines.push(format!("Blockers: {}", derived.blockers[index].join(", ")));
    }
    let dependents: Vec<String> = derived.graph.dependents[index]
        .iter()
        .map(|item| repo.tasks[*item].id.clone())
        .collect();
    if !dependents.is_empty() {
        lines.push(format!("Dependents: {}", dependents.join(", ")));
    }
    lines.push(format!(
        "Downstream: {} tasks, weight {}.",
        derived.downstream[index].len(),
        derived.downstream_weight[index]
    ));
    let mut gates = Vec::new();
    for milestone in &repo.milestones {
        for gate in &milestone.gates {
            if gate.verified_by().iter().any(|item| item == id)
                || gate
                    .fields
                    .value("Or")
                    .is_some_and(|value| value.trim() == id)
            {
                gates.push(format!("{} · {}", gate.id, gate.title));
            }
        }
        for demo in &milestone.demos {
            if demo.verified_by().iter().any(|item| item == id) {
                gates.push(format!("{} · {}", demo.id, demo.title));
            }
        }
    }
    if !gates.is_empty() {
        lines.push("Gates:".to_string());
        for gate in gates {
            lines.push(format!("- {gate}"));
        }
    }
    let mut cited: BTreeSet<String> = task.covers.iter().cloned().collect();
    let item_pattern = regex::Regex::new(r"\b(?:INV|GAP|EXTRA)-\d{3,4}\b").expect("static pattern");
    for capture in item_pattern.find_iter(&task.raw.join("\n")) {
        cited.insert(capture.as_str().to_string());
    }
    let items: Vec<String> = repo
        .coverage_items()
        .into_iter()
        .filter(|item| cited.contains(&item.id))
        .map(|item| format!("- {} · {}", item.id, item.text))
        .collect();
    if !items.is_empty() {
        lines.push("Coverage items:".to_string());
        lines.extend(items);
    }
    let mut closure: BTreeSet<usize> = derived.graph.transitive_dependencies(index);
    closure.insert(index);
    for member in &closure {
        let candidate = &repo.tasks[*member];
        if candidate.task_type() != TaskType::Adr {
            continue;
        }
        if let Some(decision_id) = candidate.fields.value("Decision")
            && let Some(decision) = repo.decision(decision_id.trim())
        {
            lines.push(String::new());
            lines.push(format!("## {} · {}", decision.id, decision.title));
            if let Some(body) = decision.body.get("Decision") {
                lines.push("### Decision".to_string());
                lines.extend(body.clone());
            }
            if let Some(body) = decision.body.get("Consequences") {
                lines.push("### Consequences".to_string());
                lines.extend(body.clone());
            }
        }
    }
    for term in &repo.glossary {
        if task
            .title
            .split_whitespace()
            .any(|word| word.eq_ignore_ascii_case(term))
            || task.title.contains(term)
        {
            lines.push(format!("Glossary: {term}"));
        }
    }
    Some(ensure_trailing_newline(&lines.join("\n")))
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

pub fn split_list_export(value: &str) -> Vec<String> {
    split_list(value)
}
