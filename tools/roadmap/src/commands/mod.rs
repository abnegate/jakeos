use crate::assign;
use crate::coverage;
use crate::derive;
use crate::diagnostic::{Diagnostic, Diagnostics, count_errors};
use crate::fmt as formatter;
use crate::generate;
use crate::repo::{LoadOptions, Repo, find_root};
use crate::validate;
use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "roadmap",
    version,
    about = "Validate, format and generate the JakeOS roadmap"
)]
pub struct Cli {
    #[arg(long, global = true)]
    pub root: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Check {
        #[arg(long)]
        strict: bool,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        allow_drafts: bool,
        #[arg(long)]
        index: Option<PathBuf>,
    },
    Fmt {
        #[arg(long)]
        check: bool,
    },
    #[command(name = "gen")]
    Generate {
        #[arg(long)]
        check: bool,
    },
    #[command(name = "assign-ids")]
    AssignIds {
        #[arg(long)]
        index: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    Coverage {
        #[arg(long)]
        json: bool,
    },
    Show {
        id: String,
    },
    Ready {
        #[arg(long)]
        workstream: Option<String>,
        #[arg(long)]
        milestone: Option<String>,
        #[arg(long)]
        size: Option<String>,
    },
    Blocked {
        #[arg(long)]
        by: Option<String>,
    },
    #[command(name = "critical-path")]
    CriticalPath {
        milestone: Option<String>,
        #[arg(long, default_value_t = 10)]
        top: usize,
    },
    Gate {
        milestone: String,
    },
    Impact {
        id: String,
        #[arg(long)]
        summary: bool,
    },
    Progress {
        #[arg(long)]
        json: bool,
    },
    Export {
        #[arg(long)]
        json: bool,
    },
    New {
        #[command(subcommand)]
        kind: NewKind,
    },
    Unclaim {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        owner: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum NewKind {
    Task {
        prefix: String,
        title: String,
        #[arg(long)]
        milestone: String,
        #[arg(long)]
        size: String,
        #[arg(long, default_value = "build")]
        r#type: String,
        #[arg(long)]
        depends: Option<String>,
    },
}

pub fn run() -> Result<i32> {
    let cli = Cli::parse();
    dispatch(cli)
}

pub fn dispatch(cli: Cli) -> Result<i32> {
    let root = find_root(cli.root.as_deref())?;
    match cli.command {
        Command::Check {
            strict,
            json,
            allow_drafts,
            index,
        } => cmd_check(root, strict, json, allow_drafts, index),
        Command::Fmt { check } => cmd_fmt(root, check),
        Command::Generate { check } => cmd_generate(root, check),
        Command::AssignIds { index, dry_run } => cmd_assign(root, index, dry_run),
        Command::Coverage { json } => cmd_coverage(root, json),
        Command::Show { id } => cmd_show(root, &id),
        Command::Ready {
            workstream,
            milestone,
            size,
        } => cmd_ready(
            root,
            workstream.as_deref(),
            milestone.as_deref(),
            size.as_deref(),
        ),
        Command::Blocked { by } => cmd_blocked(root, by.as_deref()),
        Command::CriticalPath { milestone, top } => {
            cmd_critical_path(root, milestone.as_deref(), top)
        }
        Command::Gate { milestone } => cmd_gate(root, &milestone),
        Command::Impact { id, summary } => cmd_impact(root, &id, summary),
        Command::Progress { json } => cmd_progress(root, json),
        Command::Export { json } => cmd_export(root, json),
        Command::New { kind } => cmd_new(root, kind),
        Command::Unclaim { all, owner } => cmd_unclaim(root, all, owner.as_deref()),
    }
}

fn load(
    root: PathBuf,
    allow_drafts: bool,
    index: Option<PathBuf>,
) -> Result<(Repo, derive::Derived)> {
    let repo = Repo::load(
        root,
        LoadOptions {
            allow_drafts,
            index_path: index,
        },
    )?;
    let derived = derive::build(&repo);
    Ok((repo, derived))
}

fn cmd_check(
    root: PathBuf,
    strict: bool,
    json: bool,
    allow_drafts: bool,
    index: Option<PathBuf>,
) -> Result<i32> {
    let (repo, derived) = load(root, allow_drafts, index)?;
    let mut diagnostics = validate::run(&repo, &derived);
    diagnostics.extend(generate::stale_diagnostics(&repo, &derived));
    let entries = diagnostics.sorted(strict);
    print_diagnostics(&entries, json)?;
    if count_errors(&entries) > 0 {
        Ok(1)
    } else {
        Ok(0)
    }
}

fn cmd_fmt(root: PathBuf, check: bool) -> Result<i32> {
    let (repo, _) = load(root, true, None)?;
    let dirty = formatter::apply(&repo, check)?;
    if dirty.is_empty() {
        return Ok(0);
    }
    for path in &dirty {
        if check {
            println!("would reformat {path}");
        } else {
            println!("reformatted {path}");
        }
    }
    Ok(if check { 1 } else { 0 })
}

fn cmd_generate(root: PathBuf, check: bool) -> Result<i32> {
    let (repo, derived) = load(root, true, None)?;
    let dirty = generate::apply(&repo, &derived, check)?;
    if dirty.is_empty() {
        return Ok(0);
    }
    for path in &dirty {
        if check {
            println!("stale {path}");
        } else {
            println!("wrote {path}");
        }
    }
    Ok(if check { 1 } else { 0 })
}

fn cmd_assign(root: PathBuf, index: PathBuf, dry_run: bool) -> Result<i32> {
    let (repo, _) = load(root, true, Some(index.clone()))?;
    let assigned = if dry_run {
        assign::mapping(&repo)
    } else {
        let mut options = repo.options.clone();
        options.index_path = Some(index);
        let repo = Repo { options, ..repo };
        assign::apply(&repo, false)?
    };
    if assigned.is_empty() {
        println!("no draft ids");
        return Ok(0);
    }
    for (from, to) in assigned {
        println!("{from} -> {to}");
    }
    Ok(0)
}

fn cmd_coverage(root: PathBuf, json: bool) -> Result<i32> {
    let (repo, derived) = load(root, true, None)?;
    let report = coverage::report(&repo, &derived);
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("{}", coverage::render(&report));
    }
    Ok(0)
}

fn cmd_show(root: PathBuf, id: &str) -> Result<i32> {
    let (repo, derived) = load(root, true, None)?;
    match generate::show_task(&repo, &derived, id) {
        Some(text) => {
            print!("{text}");
            Ok(0)
        }
        None => {
            bail!("unknown id `{id}`");
        }
    }
}

fn cmd_ready(
    root: PathBuf,
    workstream: Option<&str>,
    milestone: Option<&str>,
    size: Option<&str>,
) -> Result<i32> {
    let (repo, derived) = load(root, true, None)?;
    print!(
        "{}",
        generate::ready_markdown(&repo, &derived, workstream, milestone, size)
    );
    Ok(0)
}

fn cmd_blocked(root: PathBuf, by: Option<&str>) -> Result<i32> {
    let (repo, derived) = load(root, true, None)?;
    print!("{}", generate::blocked_markdown(&repo, &derived, by));
    Ok(0)
}

fn cmd_critical_path(root: PathBuf, milestone: Option<&str>, top: usize) -> Result<i32> {
    let (repo, derived) = load(root, true, None)?;
    print!(
        "{}",
        generate::critical_path_markdown(&repo, &derived, milestone, top)
    );
    Ok(0)
}

fn cmd_gate(root: PathBuf, milestone: &str) -> Result<i32> {
    let (repo, derived) = load(root, true, None)?;
    if repo.milestone(milestone).is_none() {
        bail!("unknown milestone `{milestone}`");
    }
    print!("{}", generate::gate_report(&repo, &derived, milestone));
    Ok(0)
}

fn cmd_impact(root: PathBuf, id: &str, summary: bool) -> Result<i32> {
    let (repo, derived) = load(root, true, None)?;
    match generate::impact_report(&repo, &derived, id, summary) {
        Some(text) => {
            print!("{text}");
            Ok(0)
        }
        None => bail!("unknown id `{id}`"),
    }
}

fn cmd_progress(root: PathBuf, json: bool) -> Result<i32> {
    let (repo, derived) = load(root, true, None)?;
    if json {
        let payload = serde_json::json!({
            "totals": derived.totals,
            "milestones": derived.milestone_progress,
            "workstreams": derived.workstream_progress,
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        print!("{}", generate::progress_markdown(&repo, &derived));
    }
    Ok(0)
}

fn cmd_export(root: PathBuf, json: bool) -> Result<i32> {
    if !json {
        bail!("export requires --json");
    }
    let (repo, derived) = load(root, true, None)?;
    let files = generate::render(&repo, &derived)?;
    let Some(index) = files.get("generated/index.json") else {
        bail!("generator did not produce index.json");
    };
    print!("{index}");
    Ok(0)
}

fn cmd_new(root: PathBuf, kind: NewKind) -> Result<i32> {
    let NewKind::Task {
        prefix,
        title,
        milestone,
        size,
        r#type,
        depends,
    } = kind;
    let (repo, _) = load(root.clone(), true, None)?;
    if !repo.schema.is_workstream(&prefix) {
        bail!("`{prefix}` is not a workstream prefix");
    }
    if !repo.schema.milestones.contains(&milestone) {
        bail!("`{milestone}` is not a milestone token");
    }
    if !repo
        .schema
        .task
        .enums
        .get("Size")
        .is_some_and(|values| values.contains(&size))
    {
        bail!("`{size}` is not a valid size");
    }
    if !repo
        .schema
        .task
        .enums
        .get("Type")
        .is_some_and(|values| values.contains(&r#type))
    {
        bail!("`{type}` is not a valid type");
    }
    let number = assign::next_for_prefix(&repo, &prefix);
    let id = crate::util::format_task_id(&prefix, number);
    let depends = depends
        .as_deref()
        .map(assign::parse_depends)
        .unwrap_or_default();
    let baseline = "none";
    let stub = formatter::stub_task(&id, &title, &milestone, &size, &r#type, &depends, baseline);
    let relative = format!("workstreams/{prefix}.md");
    let path = root.join(&relative);
    if path.is_file() {
        let mut content = std::fs::read_to_string(&path)?;
        if !content.ends_with('\n') {
            content.push('\n');
        }
        if !content.contains("## Tasks") {
            content.push_str("\n## Tasks\n\n");
        }
        content.push('\n');
        content.push_str(&stub);
        std::fs::write(&path, content)?;
    } else {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let header = format!(
            "# {prefix} · {prefix}\n- Prefix: {prefix}\n- Lead: none\n- Baseline: none\n\n## Scope\n\n## Out of scope\n\n## Tasks\n\n{stub}"
        );
        std::fs::write(&path, header)?;
    }
    println!("created {id} in {relative}");
    Ok(0)
}

fn cmd_unclaim(root: PathBuf, all: bool, owner: Option<&str>) -> Result<i32> {
    if !all {
        bail!("unclaim requires --all");
    }
    let (repo, _) = load(root, true, None)?;
    let mut changed = 0usize;
    for workstream in &repo.workstreams {
        let (start, end) = workstream.task_range;
        let mut tasks = repo.tasks[start..end].to_vec();
        let mut dirty = false;
        for task in &mut tasks {
            if matches!(
                task.status(),
                crate::model::Status::Done | crate::model::Status::Dropped
            ) {
                continue;
            }
            if let Some(pattern) = owner
                && !task.owner().contains(pattern)
            {
                continue;
            }
            let mut task_changed = false;
            if task.status() == crate::model::Status::InProgress {
                task.fields.set("Status", "todo");
                task_changed = true;
            }
            if task.owner() != "none" {
                task.fields.set("Owner", "none");
                task_changed = true;
            }
            if task_changed {
                dirty = true;
                changed += 1;
            }
        }
        if dirty {
            formatter::write_workstream_tasks(
                &repo.absolute(&workstream.file),
                workstream,
                &tasks,
                &repo.glossary,
                &repo.schema,
            )?;
        }
    }
    println!("unclaimed {changed} task(s)");
    Ok(0)
}

fn print_diagnostics(entries: &[Diagnostic], json: bool) -> Result<()> {
    if json {
        let payload = serde_json::json!({ "diagnostics": entries });
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }
    let mut stdout = std::io::stdout().lock();
    for entry in entries {
        writeln!(stdout, "{}", entry.render())?;
    }
    let errors = count_errors(entries);
    let warnings = entries.len().saturating_sub(errors);
    writeln!(
        stdout,
        "{errors} error{}, {warnings} warning{}",
        if errors == 1 { "" } else { "s" },
        if warnings == 1 { "" } else { "s" }
    )?;
    Ok(())
}

pub fn diagnostics_from(repo: &Repo, derived: &derive::Derived) -> Diagnostics {
    validate::run(repo, derived)
}

pub fn load_repo(root: PathBuf, options: LoadOptions) -> Result<Repo> {
    Repo::load(root, options).context("loading roadmap repository")
}
