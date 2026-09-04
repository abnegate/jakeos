use crate::derive::Derived;
use crate::model::Status;
use crate::repo::Repo;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize)]
pub struct CoverageReport {
    pub uncovered: Vec<UncoveredItem>,
    pub uncovering_tasks: Vec<UncoveringTask>,
    pub covered: usize,
    pub items: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct UncoveredItem {
    pub id: String,
    pub workstream: String,
    pub milestone: String,
    pub text: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct UncoveringTask {
    pub id: String,
    pub milestone: String,
    pub title: String,
}

pub fn report(repo: &Repo, _derived: &Derived) -> CoverageReport {
    let mut covered_by: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut gate_cited: BTreeSet<String> = BTreeSet::new();
    for milestone in &repo.milestones {
        for gate in &milestone.gates {
            for id in gate.verified_by() {
                gate_cited.insert(id);
            }
            if let Some(id) = gate.fields.value("Or") {
                let trimmed = id.trim();
                if trimmed != "none" && !trimmed.is_empty() {
                    gate_cited.insert(trimmed.to_string());
                }
            }
        }
        for demo in &milestone.demos {
            for id in demo.verified_by() {
                gate_cited.insert(id);
            }
        }
    }
    for task in &repo.tasks {
        if task.status() == Status::Dropped {
            continue;
        }
        for cover in &task.covers {
            covered_by
                .entry(cover.clone())
                .or_default()
                .insert(task.id.clone());
        }
    }

    let mut uncovered = Vec::new();
    let items = repo.coverage_items();
    for item in &items {
        if let Some(target) = &item.merged_into
            && (covered_by.contains_key(target) || covered_by.contains_key(&item.id))
        {
            continue;
        }
        if covered_by.contains_key(&item.id) {
            continue;
        }
        if item.merged_into.is_some() {
            continue;
        }
        uncovered.push(UncoveredItem {
            id: item.id.clone(),
            workstream: item.workstream.clone(),
            milestone: item.milestone.clone(),
            text: item.text.clone(),
        });
    }
    uncovered.sort_by(|left, right| left.id.cmp(&right.id));

    let mut uncovering_tasks = Vec::new();
    for task in &repo.tasks {
        if task.status() == Status::Dropped {
            continue;
        }
        if !task.covers.is_empty() {
            continue;
        }
        if gate_cited.contains(&task.id) {
            continue;
        }
        uncovering_tasks.push(UncoveringTask {
            id: task.id.clone(),
            milestone: task.milestone().to_string(),
            title: task.title.clone(),
        });
    }

    CoverageReport {
        covered: items.len().saturating_sub(uncovered.len()),
        items: items.len(),
        uncovered,
        uncovering_tasks,
    }
}

pub fn render(report: &CoverageReport) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "coverage: {} / {} items have a non-dropped task",
        report.covered, report.items
    ));
    if report.uncovered.is_empty() {
        lines.push("uncovered items: none".to_string());
    } else {
        lines.push(format!("uncovered items: {}", report.uncovered.len()));
        for item in &report.uncovered {
            lines.push(format!(
                "  {} ({}/{}): {}",
                item.id, item.workstream, item.milestone, item.text
            ));
        }
    }
    if report.uncovering_tasks.is_empty() {
        lines.push("tasks without covers or gate citation: none".to_string());
    } else {
        lines.push(format!(
            "tasks without covers or gate citation: {}",
            report.uncovering_tasks.len()
        ));
        for task in &report.uncovering_tasks {
            lines.push(format!("  {} · {}", task.id, task.title));
        }
    }
    lines.join("\n")
}
