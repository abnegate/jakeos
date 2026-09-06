use crate::config::Config;
use crate::diagnostic::Diagnostic;
use crate::model::{
    BaselineIndex, CoverageItem, Decision, Milestone, Register, RepoAlias, Task, Workstream,
    split_list,
};
use crate::parser;
use crate::parser::slugs::SlugIndex;
use crate::schema::{Patterns, REGISTER_ORDER, Schema};
use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct LoadOptions {
    pub allow_drafts: bool,
    pub index_path: Option<PathBuf>,
}

pub struct Repo {
    pub root: PathBuf,
    pub config: Config,
    pub schema: Schema,
    pub patterns: Patterns,
    pub options: LoadOptions,
    pub workstreams: Vec<Workstream>,
    pub tasks: Vec<Task>,
    pub task_index: BTreeMap<String, usize>,
    pub milestones: Vec<Milestone>,
    pub decisions: Vec<Decision>,
    pub registers: BTreeMap<String, Register>,
    pub aliases: Vec<RepoAlias>,
    pub baseline: BaselineIndex,
    pub glossary: Vec<String>,
    pub inventory: Vec<CoverageItem>,
    pub gaps: Vec<CoverageItem>,
    pub extra: Vec<CoverageItem>,
    pub slugs: Option<SlugIndex>,
    pub spike_reports: BTreeSet<String>,
    pub benchmark_reports: BTreeMap<String, Vec<String>>,
    pub compat_reports: BTreeMap<String, Vec<String>>,
    pub report_files: BTreeSet<String>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn find_root(explicit: Option<&Path>) -> Result<PathBuf> {
    if let Some(path) = explicit {
        let canonical = path
            .canonicalize()
            .with_context(|| format!("resolving --root {}", path.display()))?;
        return Ok(canonical);
    }
    let mut current = std::env::current_dir()?;
    loop {
        if current.join("roadmap.toml").is_file()
            || current.join(crate::schema::SCHEMA_RELATIVE_PATH).is_file()
        {
            return Ok(current);
        }
        if !current.pop() {
            bail!(
                "could not find the roadmap repository root; run inside the repository or pass --root PATH"
            );
        }
    }
}

fn read(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn markdown_files(directory: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return Vec::new();
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "md"))
        .collect();
    files.sort();
    files
}

impl Repo {
    pub fn load(root: PathBuf, options: LoadOptions) -> Result<Self> {
        let schema = Schema::load(&root)?;
        let patterns = Patterns::build(&schema)?;
        let config = Config::load(&root);
        let mut diagnostics = Vec::new();

        let mut workstreams = Vec::new();
        let mut tasks = Vec::new();
        for prefix in &schema.workstreams {
            let path = root.join("workstreams").join(format!("{prefix}.md"));
            let Some(content) = read(&path) else {
                continue;
            };
            let relative = format!("workstreams/{prefix}.md");
            let index = workstreams.len();
            let parsed =
                parser::workstream::parse(&relative, prefix, &content, &schema, &patterns, index);
            diagnostics.extend(parsed.diagnostics);
            let start = tasks.len();
            tasks.extend(parsed.tasks);
            let mut workstream = parsed.workstream;
            workstream.task_range = (start, tasks.len());
            workstreams.push(workstream);
        }
        for path in markdown_files(&root.join("workstreams")) {
            let stem = path
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_string();
            if schema.is_workstream(&stem) {
                continue;
            }
            diagnostics.push(Diagnostic::error(
                format!("workstreams/{stem}.md"),
                1,
                crate::diagnostic::code::UNKNOWN_PREFIX,
                format!("`{stem}` is not a workstream prefix in fields.json"),
                "rename the file or add the prefix to fields.json workstreams",
            ));
        }

        let mut task_index = BTreeMap::new();
        for (position, task) in tasks.iter().enumerate() {
            task_index.entry(task.id.clone()).or_insert(position);
        }

        let mut milestones = Vec::new();
        for path in markdown_files(&root.join("milestones")) {
            let token = path
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_string();
            let relative = format!("milestones/{token}.md");
            let Some(content) = read(&path) else { continue };
            if !schema.milestones.contains(&token) {
                diagnostics.push(Diagnostic::error(
                    relative.clone(),
                    1,
                    crate::diagnostic::code::UNKNOWN_MILESTONE_FILE,
                    format!("`{token}` is not a milestone token in fields.json"),
                    format!("valid tokens: {}", schema.milestones.join(", ")),
                ));
                continue;
            }
            let parsed = parser::milestone::parse(&relative, &token, &content, &schema, &patterns);
            diagnostics.extend(parsed.diagnostics);
            milestones.push(parsed.milestone);
        }
        milestones.sort_by_key(|milestone| {
            (
                schema.rank(&milestone.token).unwrap_or(u32::MAX),
                milestone.token.clone(),
            )
        });

        let mut decisions = Vec::new();
        for path in markdown_files(&root.join("decisions")) {
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_string();
            if !name.starts_with("D-")
                || name.len() < 7
                || !name[2..6].chars().all(|c| c.is_ascii_digit())
            {
                continue;
            }
            let relative = format!("decisions/{name}");
            let Some(content) = read(&path) else { continue };
            if let Some(parsed) = parser::decision::parse(&relative, &content, &schema, &patterns) {
                diagnostics.extend(parsed.diagnostics);
                decisions.push(parsed.decision);
            }
        }
        decisions.sort_by(|left, right| left.id.cmp(&right.id));

        let mut registers = BTreeMap::new();
        for family in REGISTER_ORDER {
            let Some(register_schema) = schema.registers.get(family) else {
                continue;
            };
            let path = root.join(&register_schema.file);
            let Some(content) = read(&path) else { continue };
            let parsed = parser::register::parse(
                &register_schema.file,
                family,
                &content,
                register_schema,
                &patterns,
            );
            diagnostics.extend(parsed.diagnostics);
            registers.insert(family.to_string(), parsed.register);
        }

        let aliases = read(&root.join("registers/repos.md"))
            .map(|content| parser::register::parse_aliases(&content, &patterns))
            .unwrap_or_default();

        let baseline = read(&root.join("BASELINE.md"))
            .map(|content| parser::baseline::parse(&content))
            .unwrap_or_default();
        let glossary = read(&root.join("GLOSSARY.md"))
            .map(|content| parser::glossary::parse(&content))
            .unwrap_or_default();

        let inventory = read(&root.join("tools/coverage/inventory.jsonl"))
            .map(|content| parser::coverage::parse(&content))
            .unwrap_or_default();
        let gaps = read(&root.join("tools/coverage/gaps.jsonl"))
            .map(|content| parser::coverage::parse(&content))
            .unwrap_or_default();
        let extra = read(&root.join("tools/coverage/extra.jsonl"))
            .map(|content| parser::coverage::parse(&content))
            .unwrap_or_default();

        let slugs = options
            .index_path
            .as_ref()
            .map(|path| {
                if path.is_absolute() {
                    path.clone()
                } else {
                    root.join(path)
                }
            })
            .and_then(|path| read(&path))
            .map(|content| parser::slugs::parse(&content));

        let mut spike_reports = BTreeSet::new();
        for path in markdown_files(&root.join("reports/spikes")) {
            if let Some(stem) = path.file_stem().and_then(|value| value.to_str()) {
                spike_reports.insert(stem.to_string());
            }
        }

        let mut benchmark_reports: BTreeMap<String, Vec<String>> = BTreeMap::new();
        if let Ok(entries) = std::fs::read_dir(root.join("reports/benchmarks")) {
            let mut directories: Vec<PathBuf> = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| path.is_dir())
                .collect();
            directories.sort();
            for directory in directories {
                let Some(name) = directory.file_name().and_then(|value| value.to_str()) else {
                    continue;
                };
                let files: Vec<String> = markdown_files(&directory)
                    .iter()
                    .filter_map(|path| path.file_name().and_then(|value| value.to_str()))
                    .map(|value| value.to_string())
                    .collect();
                benchmark_reports.insert(name.to_string(), files);
            }
        }

        let mut compat_reports: BTreeMap<String, Vec<String>> = BTreeMap::new();
        if let Ok(entries) = std::fs::read_dir(root.join("reports/compat")) {
            let mut directories: Vec<PathBuf> = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| path.is_dir())
                .collect();
            directories.sort();
            for directory in directories {
                let Some(name) = directory.file_name().and_then(|value| value.to_str()) else {
                    continue;
                };
                let files: Vec<String> = markdown_files(&directory)
                    .iter()
                    .filter_map(|path| path.file_name().and_then(|value| value.to_str()))
                    .map(|value| value.to_string())
                    .collect();
                compat_reports.insert(name.to_string(), files);
            }
        }

        let report_files = collect_report_files(&root);

        Ok(Self {
            root,
            config,
            schema,
            patterns,
            options,
            workstreams,
            tasks,
            task_index,
            milestones,
            decisions,
            registers,
            aliases,
            baseline,
            glossary,
            inventory,
            gaps,
            extra,
            slugs,
            spike_reports,
            benchmark_reports,
            compat_reports,
            report_files,
            diagnostics,
        })
    }

    pub fn task(&self, id: &str) -> Option<&Task> {
        self.task_index.get(id).map(|index| &self.tasks[*index])
    }

    pub fn task_position(&self, id: &str) -> Option<usize> {
        self.task_index.get(id).copied()
    }

    pub fn milestone(&self, token: &str) -> Option<&Milestone> {
        self.milestones
            .iter()
            .find(|milestone| milestone.token == token)
    }

    pub fn decision(&self, id: &str) -> Option<&Decision> {
        self.decisions.iter().find(|decision| decision.id == id)
    }

    pub fn register(&self, family: &str) -> Option<&Register> {
        self.registers.get(family)
    }

    pub fn rank(&self, milestone: &str) -> u32 {
        self.schema.rank(milestone).unwrap_or(u32::MAX)
    }

    pub fn weight(&self, task: &Task) -> u32 {
        self.config.weight_of(task.size())
    }

    pub fn coverage_items(&self) -> Vec<&CoverageItem> {
        self.inventory
            .iter()
            .chain(self.gaps.iter())
            .chain(self.extra.iter())
            .collect()
    }

    pub fn absolute(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    pub fn is_example(&self, identifier: &str) -> bool {
        identifier.starts_with(&format!("{}-", self.schema.example_prefix))
    }

    pub fn cited_by_gate(&self, id: &str) -> bool {
        self.milestones.iter().any(|milestone| {
            milestone.gates.iter().any(|gate| {
                split_list(gate.fields.value_or_empty("Verified by"))
                    .iter()
                    .any(|cited| cited == id)
            }) || milestone.demos.iter().any(|demo| {
                split_list(demo.fields.value_or_empty("Verified by"))
                    .iter()
                    .any(|cited| cited == id)
            })
        })
    }

    pub fn needs_verifier(&self, task: &crate::model::Task) -> bool {
        let policy = &self.config.policy;
        policy.require_independent_verification
            || (policy.verify_freezes_and_adr_always
                && (task.task_type() == crate::model::TaskType::Adr
                    || !task.list("Freezes").is_empty()))
            || (policy.verify_gate_tasks && self.cited_by_gate(&task.id))
    }

    pub fn hardware_for_matrix_entry(&self, entry: &str) -> Option<&crate::model::RegisterEntry> {
        self.register("H")?
            .entries
            .iter()
            .find(|hardware| hardware.fields.value_or_empty("Matrix entry") == entry)
    }

    pub fn register_entry(&self, family: &str, id: &str) -> Option<&crate::model::RegisterEntry> {
        self.register(family).and_then(|register| register.get(id))
    }

    pub fn has_report(&self, relative: &str) -> bool {
        self.report_files.contains(relative) || self.absolute(relative).is_file()
    }

    pub fn alias_exists(&self, alias: &str) -> bool {
        self.aliases.iter().any(|entry| entry.alias == alias)
    }

    pub fn family_of(&self, identifier: &str) -> Option<&str> {
        [
            "DRAFT", "TASK", "D", "R", "B", "C", "T", "I", "Q", "H", "S", "GATE", "DEMO",
        ]
        .into_iter()
        .find(|name| self.patterns.matches_family(name, identifier))
    }
}

fn collect_report_files(root: &Path) -> BTreeSet<String> {
    let mut files = BTreeSet::new();
    let reports = root.join("reports");
    if !reports.is_dir() {
        return files;
    }
    let mut stack = vec![reports];
    while let Some(directory) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.filter_map(|entry| entry.ok()) {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_some_and(|extension| extension == "md")
                && let Ok(relative) = path.strip_prefix(root)
            {
                files.insert(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    files
}
