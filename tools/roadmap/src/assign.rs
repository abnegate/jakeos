use crate::model::split_list;
use crate::repo::Repo;
use crate::util::{apply_replacements, format_task_id, next_number, walk_markdown};
use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::Path;

pub fn mapping(repo: &Repo) -> Vec<(String, String)> {
    let mut drafts: Vec<(u32, String, String)> = repo
        .tasks
        .iter()
        .filter(|task| task.is_draft())
        .map(|task| {
            (
                repo.rank(task.milestone()),
                task.slug.clone().unwrap_or_default(),
                task.id.clone(),
            )
        })
        .collect();
    drafts.sort();
    let mut next_by_prefix: BTreeMap<String, u32> = BTreeMap::new();
    for task in &repo.tasks {
        if let Some(number) = task.number {
            let entry = next_by_prefix.entry(task.prefix.clone()).or_insert(1);
            *entry = (*entry).max(number + 1);
        }
    }
    let mut assigned = Vec::new();
    for (_, _, id) in drafts {
        let prefix = id.split('-').next().unwrap_or(&id).to_string();
        let number = next_by_prefix.entry(prefix.clone()).or_insert(1);
        if *number == 0 {
            *number = 1;
        }
        let fresh = format_task_id(&prefix, *number);
        *number += 1;
        assigned.push((id, fresh));
    }
    assigned.sort_by(|left, right| left.0.cmp(&right.0));
    assigned
}

pub fn apply(repo: &Repo, dry_run: bool) -> Result<Vec<(String, String)>> {
    let assigned = mapping(repo);
    if assigned.is_empty() || dry_run {
        return Ok(assigned);
    }
    let mut replacements = assigned.clone();
    replacements.sort_by(|left, right| right.0.len().cmp(&left.0.len()).then(left.0.cmp(&right.0)));
    rewrite_tree(&repo.root.join("workstreams"), &replacements)?;
    rewrite_tree(&repo.root.join("milestones"), &replacements)?;
    rewrite_tree(&repo.root.join("decisions"), &replacements)?;
    rewrite_tree(&repo.root.join("registers"), &replacements)?;
    rewrite_tree(&repo.root.join("reports"), &replacements)?;
    if let Some(path) = &repo.options.index_path {
        let absolute = if path.is_absolute() {
            path.clone()
        } else {
            repo.root.join(path)
        };
        rewrite_slugs(&absolute, &assigned)?;
    }
    Ok(assigned)
}

fn rewrite_tree(directory: &Path, replacements: &[(String, String)]) -> Result<()> {
    if !directory.is_dir() {
        return Ok(());
    }
    for path in walk_markdown(directory) {
        let original = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let updated = apply_replacements(&original, replacements);
        if updated != original {
            std::fs::write(&path, updated)
                .with_context(|| format!("writing {}", path.display()))?;
        }
    }
    Ok(())
}

fn rewrite_slugs(path: &Path, assigned: &[(String, String)]) -> Result<()> {
    if !path.is_file() {
        return Ok(());
    }
    let original = std::fs::read_to_string(path)?;
    let lookup: BTreeMap<&str, &str> = assigned
        .iter()
        .map(|(from, to)| (from.as_str(), to.as_str()))
        .collect();
    let mut lines = Vec::new();
    for (index, line) in original.lines().enumerate() {
        if index == 0 {
            let mut header: Vec<String> = line.split('\t').map(ToString::to_string).collect();
            if !header.iter().any(|column| column == "assigned") {
                header.push("assigned".to_string());
            }
            lines.push(header.join("\t"));
            continue;
        }
        if line.trim().is_empty() {
            lines.push(line.to_string());
            continue;
        }
        let mut columns: Vec<String> = line.split('\t').map(ToString::to_string).collect();
        let draft = columns.first().cloned().unwrap_or_default();
        if let Some(fresh) = lookup.get(draft.as_str()) {
            let header = original
                .lines()
                .next()
                .unwrap_or_default()
                .split('\t')
                .collect::<Vec<_>>();
            let assigned_column = header.iter().position(|name| *name == "assigned");
            match assigned_column {
                Some(position) => {
                    while columns.len() <= position {
                        columns.push(String::new());
                    }
                    columns[position] = (*fresh).to_string();
                }
                None => columns.push((*fresh).to_string()),
            }
        }
        lines.push(columns.join("\t"));
    }
    let mut output = lines.join("\n");
    if original.ends_with('\n') {
        output.push('\n');
    }
    std::fs::write(path, output)?;
    Ok(())
}

pub fn next_for_prefix(repo: &Repo, prefix: &str) -> u32 {
    next_number(
        repo.tasks
            .iter()
            .filter(|task| task.prefix == prefix)
            .filter_map(|task| task.number),
    )
}

pub fn parse_depends(value: &str) -> Vec<String> {
    split_list(value)
}
