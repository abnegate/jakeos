use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Status {
    Todo,
    InProgress,
    Done,
    Dropped,
}

impl Status {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "todo" => Some(Status::Todo),
            "in-progress" => Some(Status::InProgress),
            "done" => Some(Status::Done),
            "dropped" => Some(Status::Dropped),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Status::Todo => "todo",
            Status::InProgress => "in-progress",
            Status::Done => "done",
            Status::Dropped => "dropped",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TaskType {
    Build,
    Adr,
    Spike,
    Benchmark,
    Docs,
}

impl TaskType {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "build" => Some(TaskType::Build),
            "adr" => Some(TaskType::Adr),
            "spike" => Some(TaskType::Spike),
            "benchmark" => Some(TaskType::Benchmark),
            "docs" => Some(TaskType::Docs),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            TaskType::Build => "build",
            TaskType::Adr => "adr",
            TaskType::Spike => "spike",
            TaskType::Benchmark => "benchmark",
            TaskType::Docs => "docs",
        }
    }
}

impl fmt::Display for TaskType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DerivedState {
    Ready,
    Blocked,
    InReview,
    Done,
    Dropped,
    Waiting,
}

impl DerivedState {
    pub fn label(self) -> &'static str {
        match self {
            DerivedState::Ready => "ready",
            DerivedState::Blocked => "blocked",
            DerivedState::InReview => "in-review",
            DerivedState::Done => "done",
            DerivedState::Dropped => "dropped",
            DerivedState::Waiting => "active",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub key: String,
    pub value: String,
    pub line: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Fields {
    pub items: Vec<Field>,
}

impl Fields {
    pub fn get(&self, key: &str) -> Option<&Field> {
        self.items.iter().find(|field| field.key == key)
    }

    pub fn value(&self, key: &str) -> Option<&str> {
        self.get(key).map(|field| field.value.as_str())
    }

    pub fn value_or_empty(&self, key: &str) -> &str {
        self.value(key).unwrap_or("")
    }

    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn line_of(&self, key: &str, fallback: usize) -> usize {
        self.get(key).map(|field| field.line).unwrap_or(fallback)
    }

    pub fn set(&mut self, key: &str, value: impl Into<String>) {
        let value = value.into();
        if let Some(field) = self.items.iter_mut().find(|field| field.key == key) {
            field.value = value;
            return;
        }
        self.items.push(Field {
            key: key.to_string(),
            value,
            line: 0,
        });
    }

    pub fn remove(&mut self, key: &str) {
        self.items.retain(|field| field.key != key);
    }
}

#[derive(Clone, Debug)]
pub struct Criterion {
    pub ticked: bool,
    pub text: String,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub struct VerificationLine {
    pub kind: String,
    pub text: String,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub struct EvidenceLine {
    pub text: String,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub prefix: String,
    pub number: Option<u32>,
    pub slug: Option<String>,
    pub title: String,
    pub fields: Fields,
    pub description: Vec<String>,
    pub covers: Vec<String>,
    pub out_of_scope: Vec<String>,
    pub criteria: Vec<Criterion>,
    pub verification: Vec<VerificationLine>,
    pub evidence: Vec<EvidenceLine>,
    pub present_sections: Vec<String>,
    pub raw: Vec<String>,
    pub file: String,
    pub workstream: usize,
    pub line: usize,
}

impl Task {
    pub fn is_draft(&self) -> bool {
        self.number.is_none()
    }

    pub fn status(&self) -> Status {
        Status::parse(self.fields.value_or_empty("Status")).unwrap_or(Status::Todo)
    }

    pub fn task_type(&self) -> TaskType {
        TaskType::parse(self.fields.value_or_empty("Type")).unwrap_or(TaskType::Build)
    }

    pub fn size(&self) -> &str {
        self.fields.value_or_empty("Size")
    }

    pub fn milestone(&self) -> &str {
        self.fields.value_or_empty("Milestone")
    }

    pub fn owner(&self) -> &str {
        self.fields.value_or_empty("Owner")
    }

    pub fn list(&self, key: &str) -> Vec<String> {
        split_list(self.fields.value_or_empty(key))
    }

    pub fn depends_on(&self) -> Vec<String> {
        self.list("Depends on")
    }

    pub fn has_non_none_evidence(&self) -> bool {
        self.evidence
            .iter()
            .any(|entry| entry.text.trim() != "none" && !entry.text.trim().is_empty())
    }

    pub fn line_count(&self) -> usize {
        self.raw.len()
    }

    pub fn field_is_none(&self, key: &str) -> bool {
        match self.fields.value(key) {
            None => true,
            Some(value) => value.is_empty() || value == "none",
        }
    }

    pub fn description_text(&self) -> String {
        self.description
            .iter()
            .filter(|line| !line.trim().starts_with("<!-- covers:"))
            .cloned()
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub fn split_list(value: &str) -> Vec<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "none" {
        return Vec::new();
    }
    trimmed
        .split(',')
        .map(|token| token.trim().to_string())
        .filter(|token| !token.is_empty())
        .collect()
}

#[derive(Clone, Debug)]
pub struct Workstream {
    pub prefix: String,
    pub name: String,
    pub file: String,
    pub fields: Fields,
    pub scope: Vec<String>,
    pub out_of_scope: Vec<String>,
    pub task_range: (usize, usize),
    pub line_count: usize,
}

impl Workstream {
    pub fn lead(&self) -> &str {
        self.fields.value("Lead").unwrap_or("none")
    }

    pub fn has_baseline_gap(&self) -> bool {
        self.fields.contains("Baseline gap")
    }
}

#[derive(Clone, Debug)]
pub struct Gate {
    pub id: String,
    pub title: String,
    pub fields: Fields,
    pub prose: Vec<String>,
    pub line: usize,
}

impl Gate {
    pub fn kind(&self) -> &str {
        self.fields.value_or_empty("Kind")
    }

    pub fn verified_by(&self) -> Vec<String> {
        split_list(self.fields.value_or_empty("Verified by"))
    }
}

#[derive(Clone, Debug)]
pub struct Demo {
    pub id: String,
    pub title: String,
    pub fields: Fields,
    pub prose: Vec<String>,
    pub line: usize,
}

impl Demo {
    pub fn verified_by(&self) -> Vec<String> {
        split_list(self.fields.value_or_empty("Verified by"))
    }
}

#[derive(Clone, Debug)]
pub struct Milestone {
    pub token: String,
    pub title: String,
    pub file: String,
    pub fields: Fields,
    pub gates: Vec<Gate>,
    pub demos: Vec<Demo>,
    pub sections: Vec<String>,
    pub line: usize,
}

impl Milestone {
    pub fn sequence(&self) -> u32 {
        self.fields
            .value("Sequence")
            .and_then(|value| value.trim().parse().ok())
            .unwrap_or(u32::MAX)
    }

    pub fn display_title(&self) -> &str {
        self.fields.value("Title").unwrap_or(&self.title)
    }

    pub fn list(&self, key: &str) -> Vec<String> {
        split_list(self.fields.value_or_empty(key))
    }
}

#[derive(Clone, Debug)]
pub struct Decision {
    pub id: String,
    pub title: String,
    pub file: String,
    pub fields: Fields,
    pub sections: Vec<String>,
    pub options: Vec<String>,
    pub body: BTreeMap<String, Vec<String>>,
    pub line: usize,
}

impl Decision {
    pub fn status(&self) -> &str {
        self.fields.value_or_empty("Status")
    }

    pub fn task(&self) -> &str {
        self.fields.value_or_empty("Task")
    }

    pub fn list(&self, key: &str) -> Vec<String> {
        split_list(self.fields.value_or_empty(key))
    }
}

#[derive(Clone, Debug)]
pub struct RegisterEntry {
    pub id: String,
    pub title: String,
    pub fields: Fields,
    pub prose: Vec<String>,
    pub line: usize,
}

impl RegisterEntry {
    pub fn list(&self, key: &str) -> Vec<String> {
        split_list(self.fields.value_or_empty(key))
    }

    pub fn status(&self) -> &str {
        self.fields.value_or_empty("Status")
    }
}

#[derive(Clone, Debug)]
pub struct Register {
    pub family: String,
    pub file: String,
    pub title: String,
    pub entries: Vec<RegisterEntry>,
}

impl Register {
    pub fn get(&self, id: &str) -> Option<&RegisterEntry> {
        self.entries.iter().find(|entry| entry.id == id)
    }
}

#[derive(Clone, Debug)]
pub struct RepoAlias {
    pub alias: String,
    pub url: String,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub struct BenchmarkTarget {
    pub milestone: String,
    pub kind: String,
    pub detail: String,
}

pub fn parse_target_clauses(value: &str) -> Result<Vec<BenchmarkTarget>, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "none" {
        return Ok(Vec::new());
    }
    let mut targets = Vec::new();
    for clause in trimmed.split(';') {
        let clause = clause.trim();
        if clause.is_empty() {
            continue;
        }
        let tokens: Vec<&str> = clause.split_whitespace().collect();
        if tokens.len() < 2 {
            return Err(clause.to_string());
        }
        let milestone = tokens[0].to_string();
        let kind = tokens[1].to_string();
        let detail = tokens[2..].join(" ");
        match kind.as_str() {
            "publish" => {}
            "absolute" => {
                if detail.is_empty() {
                    return Err(clause.to_string());
                }
            }
            "regression" => {
                if !detail.contains("% vs ") && !detail.contains("% vs") {
                    return Err(clause.to_string());
                }
            }
            _ => return Err(clause.to_string()),
        }
        targets.push(BenchmarkTarget {
            milestone,
            kind,
            detail,
        });
    }
    Ok(targets)
}

#[derive(Clone, Debug, Default)]
pub struct BaselineIndex {
    pub sections: Vec<String>,
}

impl BaselineIndex {
    pub fn resolves(&self, reference: &str) -> bool {
        let key = reference.trim_start_matches('§');
        self.sections.iter().any(|section| section == key)
    }
}

#[derive(Clone, Debug)]
pub struct CoverageItem {
    pub id: String,
    pub workstream: String,
    pub milestone: String,
    pub text: String,
    pub merged_into: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DraftRow {
    pub draft: String,
    pub milestone: String,
    pub title: String,
    pub covers: Vec<String>,
    pub assigned: Option<String>,
    pub line: usize,
}
