use crate::assign;
use crate::derive;
use crate::diagnostic::Severity;
use crate::fmt as formatter;
use crate::model::{EvidenceLine, Status, Task, TaskType};
use crate::repo::{LoadOptions, Repo};
use crate::validate;
use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const DROPPED_REASONS: [&str; 5] = [
    "duplicate",
    "descoped",
    "superseded",
    "infeasible",
    "merged",
];
const QUESTION_TITLE_WORDS: usize = 12;

pub fn claim(root: PathBuf, id: &str, owner: &str) -> Result<i32> {
    let repo = load(root)?;
    if !repo.patterns.owner.is_match(owner) || owner == "none" {
        bail!("`{owner}` is not a valid owner (@handle or @agent/name)");
    }
    let task = find(&repo, id)?;
    match task.status() {
        Status::Todo => {}
        other => bail!(
            "`{id}` is {}; only todo tasks can be claimed",
            other.label()
        ),
    }
    if task.size() == "XL" {
        bail!("`{id}` is XL and must be split before it leaves todo");
    }
    let mut edits = Edits::new(&repo);
    edits.update(id, |task| {
        task.fields.set("Owner", owner);
        task.fields.set("Status", "in-progress");
    });
    edits.commit(&repo, &format!("claimed {id} for {owner}"))
}

pub fn unclaim(root: PathBuf, id: &str) -> Result<i32> {
    let repo = load(root)?;
    let task = find(&repo, id)?;
    if task.status() != Status::InProgress {
        bail!(
            "`{id}` is {}; only in-progress tasks can be unclaimed",
            task.status().label()
        );
    }
    let mut edits = Edits::new(&repo);
    edits.update(id, |task| {
        task.fields.set("Owner", "none");
        task.fields.set("Status", "todo");
    });
    edits.commit(&repo, &format!("unclaimed {id}"))
}

pub fn block(root: PathBuf, id: &str, reason: &str) -> Result<i32> {
    let repo = load(root)?;
    let task = find(&repo, id)?;
    if matches!(task.status(), Status::Done | Status::Dropped) {
        bail!("`{id}` is {}; it cannot be blocked", task.status().label());
    }
    let reason = reason.trim();
    if reason.is_empty() {
        bail!("a blocking reason is required");
    }
    let question = mint_question(&repo, &task.prefix, reason)?;
    let mut edits = Edits::new(&repo);
    edits.update(id, |task| {
        let mut depends = task.depends_on();
        if !depends.iter().any(|item| item == &question) {
            depends.push(question.clone());
        }
        task.fields.set("Depends on", join(&depends));
    });
    edits.commit(&repo, &format!("blocked {id} on {question}"))
}

pub fn done(
    root: PathBuf,
    id: &str,
    evidence: &[String],
    verified_by: Option<&str>,
    tick: bool,
) -> Result<i32> {
    let repo = load(root)?;
    let task = find(&repo, id)?;
    match task.status() {
        Status::Todo | Status::InProgress => {}
        other => bail!("`{id}` is {}; it cannot be marked done", other.label()),
    }
    if evidence.is_empty() {
        bail!(
            "at least one --evidence entry is required (repo@sha, repo#pr, url, report:…, decision:D-NNNN)"
        );
    }
    if !tick && task.criteria.iter().any(|criterion| !criterion.ticked) {
        bail!(
            "`{id}` has unticked acceptance criteria; tick them in the file or pass --tick when every criterion holds"
        );
    }
    let owner = task.owner().to_string();
    let requires_verifier = repo.config.policy.require_independent_verification
        || (repo.config.policy.verify_freezes_and_adr_always
            && (task.task_type() == TaskType::Adr || task.fields.contains("Freezes")));
    match verified_by {
        Some(verifier) => {
            if verifier.starts_with("@agent/") {
                bail!("`Verified by` may never be an @agent/ identity");
            }
            if !repo.patterns.owner.is_match(verifier) || verifier == "none" {
                bail!("`{verifier}` is not a valid verifier handle");
            }
            if verifier == owner {
                bail!("the verifier must differ from the owner ({owner})");
            }
        }
        None if requires_verifier => {
            bail!(
                "`{id}` requires --verified-by @handle (policy: independent verification for this task)"
            )
        }
        None => {}
    }
    let evidence = evidence.to_vec();
    let verifier = verified_by.map(str::to_string);
    let mut edits = Edits::new(&repo);
    edits.update(id, |task| {
        if tick {
            for criterion in &mut task.criteria {
                criterion.ticked = true;
            }
        }
        task.evidence.retain(|line| line.text.trim() != "none");
        for entry in &evidence {
            let entry = entry.trim();
            if task.evidence.iter().all(|line| line.text != entry) {
                task.evidence.push(EvidenceLine {
                    text: entry.to_string(),
                    line: 0,
                });
            }
        }
        if let Some(verifier) = &verifier {
            task.fields.set("Verified by", verifier.as_str());
        }
        task.fields.set("Status", "done");
    });
    edits.commit(&repo, &format!("{id} done"))
}

pub fn drop(root: PathBuf, id: &str, because: &str, superseded_by: &[String]) -> Result<i32> {
    let repo = load(root)?;
    let task = find(&repo, id)?;
    match task.status() {
        Status::Todo | Status::InProgress => {}
        Status::Done => bail!("`{id}` is done; done work is history, add a new task instead"),
        Status::Dropped => bail!("`{id}` is already dropped"),
    }
    let because = because.trim();
    if !DROPPED_REASONS.iter().any(|reason| {
        because == *reason
            || because.starts_with(&format!("{reason} "))
            || because.starts_with(&format!("{reason}:"))
    }) {
        bail!(
            "`Dropped because` must start with one of: {}",
            DROPPED_REASONS.join(", ")
        );
    }
    for superseder in superseded_by {
        find(&repo, superseder)?;
    }
    let dependents = dependents_of(&repo, id);
    if !dependents.is_empty() && superseded_by.is_empty() {
        bail!(
            "{} task(s) depend on `{id}` ({}); pass --superseded-by so they can be repointed",
            dependents.len(),
            dependents.join(", ")
        );
    }
    let superseded: Vec<String> = superseded_by.to_vec();
    let because = because.to_string();
    let mut edits = Edits::new(&repo);
    edits.update(id, |task| {
        task.fields.set("Status", "dropped");
        if !superseded.is_empty() {
            task.fields.set("Superseded by", join(&superseded));
        }
        task.fields.set("Dropped because", because.as_str());
    });
    for dependent in &dependents {
        edits.repoint(dependent, id, &superseded);
    }
    edits.commit(&repo, &format!("dropped {id}"))
}

pub fn split(root: PathBuf, id: &str, titles: &[String], size: Option<&str>) -> Result<i32> {
    let repo = load(root)?;
    let task = find(&repo, id)?;
    match task.status() {
        Status::Todo | Status::InProgress => {}
        other => bail!("`{id}` is {}; only live tasks can be split", other.label()),
    }
    if titles.len() < 2 {
        bail!("split needs at least two --into titles");
    }
    let size = size.unwrap_or("M");
    if size == "XL" {
        bail!("split children may not be XL");
    }
    let milestone = task.milestone().to_string();
    let task_type = task.task_type().label().to_string();
    let depends = task.depends_on();
    let baseline = task.fields.value("Baseline").unwrap_or("none").to_string();
    let prefix = task.prefix.clone();
    let first = assign::next_for_prefix(&repo, &prefix);
    let mut children = Vec::new();
    let mut stubs = String::new();
    for (number, title) in (first..).zip(titles.iter()) {
        let child = crate::util::format_task_id(&prefix, number);
        stubs.push('\n');
        stubs.push_str(&formatter::stub_task(
            &child, title, &milestone, size, &task_type, &depends, &baseline,
        ));
        children.push(child);
    }
    let dependents = dependents_of(&repo, id);
    let mut edits = Edits::new(&repo);
    edits.update(id, |task| {
        task.fields.set("Status", "dropped");
        task.fields.set("Superseded by", join(&children));
        task.fields.set("Dropped because", "superseded by split");
    });
    for dependent in &dependents {
        edits.repoint(dependent, id, &children);
    }
    edits.append(&task.file, stubs);
    edits.commit(&repo, &format!("split {id} into {}", children.join(", ")))
}

pub fn move_task(root: PathBuf, id: &str, milestone: &str) -> Result<i32> {
    let repo = load(root)?;
    let task = find(&repo, id)?;
    if !repo.schema.milestones.contains(&milestone.to_string()) {
        bail!("`{milestone}` is not a milestone token");
    }
    if task.status() == Status::Done {
        bail!("`{id}` is done; its milestone is frozen");
    }
    let mut edits = Edits::new(&repo);
    edits.update(id, |task| task.fields.set("Milestone", milestone));
    edits.commit(&repo, &format!("moved {id} to {milestone}"))
}

pub fn renumber(root: PathBuf, old: &str, new: &str, base_ref: &str) -> Result<i32> {
    let repo = load(root)?;
    let task = find(&repo, old)?;
    if !repo
        .patterns
        .families
        .get("TASK")
        .is_some_and(|pattern| pattern.is_match(new))
    {
        bail!("`{new}` is not a task id");
    }
    if repo.task(new).is_some() {
        bail!("`{new}` already exists");
    }
    let prefix = new.split('-').next().unwrap_or_default();
    if prefix != task.prefix {
        bail!("renumbering may not change the prefix");
    }
    if on_base(&repo, base_ref, &task.file, old) {
        bail!(
            "`{old}` already exists on {base_ref}; ids that reached the base branch are never renumbered"
        );
    }
    let pattern = regex::Regex::new(&format!(r"\b{}\b", regex::escape(old)))?;
    let mut rewritten = 0usize;
    for directory in ["workstreams", "milestones", "decisions", "registers"] {
        let path = repo.root.join(directory);
        let Ok(entries) = std::fs::read_dir(&path) else {
            continue;
        };
        for entry in entries.flatten() {
            let file = entry.path();
            if file.extension().is_none_or(|extension| extension != "md") {
                continue;
            }
            let content = std::fs::read_to_string(&file)?;
            if !pattern.is_match(&content) {
                continue;
            }
            let updated = pattern.replace_all(&content, new).into_owned();
            std::fs::write(&file, updated)?;
            rewritten += 1;
        }
    }
    println!("renumbered {old} to {new} across {rewritten} file(s)");
    verify(&repo.root)
}

fn on_base(repo: &Repo, base_ref: &str, relative: &str, id: &str) -> bool {
    std::process::Command::new("git")
        .arg("-C")
        .arg(&repo.root)
        .arg("show")
        .arg(format!("{base_ref}:{relative}"))
        .output()
        .ok()
        .filter(|output| output.status.success())
        .is_some_and(|output| {
            String::from_utf8_lossy(&output.stdout).contains(&format!("### {id} ·"))
        })
}

fn mint_question(repo: &Repo, prefix: &str, reason: &str) -> Result<String> {
    let path = repo.root.join("registers/questions.md");
    let content = std::fs::read_to_string(&path).context("reading registers/questions.md")?;
    let pattern = regex::Regex::new(r"(?m)^### Q-(\d{3}) ·")?;
    let next = pattern
        .captures_iter(&content)
        .filter_map(|capture| capture[1].parse::<u32>().ok())
        .max()
        .unwrap_or(0)
        + 1;
    let id = format!("Q-{next:03}");
    let title: String = reason
        .split_whitespace()
        .take(QUESTION_TITLE_WORDS)
        .collect::<Vec<_>>()
        .join(" ")
        .trim_end_matches(['.', '?', '!'])
        .to_string();
    let entry = format!(
        "### {id} · {title}\n- Workstream: {prefix}\n- Status: open\n- Answered by: none\n{reason}\n\n"
    );
    let marker = "<!-- roadmap:generated:begin";
    let updated = match content.find(marker) {
        Some(position) => {
            let (head, tail) = content.split_at(position);
            format!("{}\n\n{entry}{tail}", head.trim_end())
        }
        None => format!("{}\n\n{entry}", content.trim_end()),
    };
    std::fs::write(&path, updated)?;
    Ok(id)
}

fn dependents_of(repo: &Repo, id: &str) -> Vec<String> {
    repo.tasks
        .iter()
        .filter(|task| task.status() != Status::Dropped)
        .filter(|task| task.depends_on().iter().any(|dependency| dependency == id))
        .map(|task| task.id.clone())
        .collect()
}

fn find<'a>(repo: &'a Repo, id: &str) -> Result<&'a Task> {
    repo.task(id)
        .ok_or_else(|| anyhow::anyhow!("unknown task `{id}`"))
}

fn join(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}

fn load(root: PathBuf) -> Result<Repo> {
    Repo::load(
        root,
        LoadOptions {
            allow_drafts: false,
            index_path: None,
        },
    )
    .context("loading roadmap repository")
}

fn verify(root: &Path) -> Result<i32> {
    let repo = load(root.to_path_buf())?;
    let derived = derive::build(&repo);
    let entries = validate::run(&repo, &derived).sorted(false);
    let errors: Vec<_> = entries
        .iter()
        .filter(|entry| entry.severity == Severity::Error)
        .collect();
    if errors.is_empty() {
        return Ok(0);
    }
    for entry in &errors {
        eprintln!("{}", entry.render());
    }
    Ok(1)
}

struct Edits {
    tasks: BTreeMap<usize, Vec<Task>>,
    originals: BTreeMap<String, String>,
    appended: BTreeMap<String, String>,
    touched: BTreeSet<usize>,
}

impl Edits {
    fn new(repo: &Repo) -> Self {
        let mut tasks = BTreeMap::new();
        for (index, workstream) in repo.workstreams.iter().enumerate() {
            let (start, end) = workstream.task_range;
            tasks.insert(index, repo.tasks[start..end].to_vec());
        }
        Self {
            tasks,
            originals: BTreeMap::new(),
            appended: BTreeMap::new(),
            touched: BTreeSet::new(),
        }
    }

    fn update(&mut self, id: &str, change: impl FnOnce(&mut Task)) {
        for (index, tasks) in &mut self.tasks {
            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                change(task);
                self.touched.insert(*index);
                return;
            }
        }
    }

    fn repoint(&mut self, dependent: &str, from: &str, to: &[String]) {
        let to = to.to_vec();
        self.update(dependent, |task| {
            let mut depends: Vec<String> = task
                .depends_on()
                .into_iter()
                .filter(|dependency| dependency != from)
                .collect();
            for target in &to {
                if target != &task.id && !depends.contains(target) {
                    depends.push(target.clone());
                }
            }
            task.fields.set("Depends on", join(&depends));
        });
    }

    fn append(&mut self, file: &str, content: String) {
        self.appended
            .entry(file.to_string())
            .or_default()
            .push_str(&content);
    }

    fn commit(mut self, repo: &Repo, message: &str) -> Result<i32> {
        for index in &self.touched {
            let workstream = &repo.workstreams[*index];
            let path = repo.absolute(&workstream.file);
            self.originals
                .entry(workstream.file.clone())
                .or_insert_with(|| std::fs::read_to_string(&path).unwrap_or_default());
            formatter::write_workstream_tasks(
                &path,
                workstream,
                &self.tasks[index],
                &repo.glossary,
                &repo.schema,
            )?;
        }
        for (file, content) in &self.appended {
            let path = repo.absolute(file);
            self.originals
                .entry(file.clone())
                .or_insert_with(|| std::fs::read_to_string(&path).unwrap_or_default());
            let mut existing = std::fs::read_to_string(&path).unwrap_or_default();
            if !existing.ends_with('\n') {
                existing.push('\n');
            }
            existing.push_str(content);
            std::fs::write(&path, existing)?;
        }
        let status = verify(&repo.root)?;
        if status != 0 {
            for (file, original) in &self.originals {
                std::fs::write(repo.absolute(file), original)?;
            }
            eprintln!("{message}: rejected, the change was reverted");
            return Ok(status);
        }
        println!("{message}");
        Ok(0)
    }
}
