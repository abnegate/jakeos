use anyhow::{Context, Result};
use regex::Regex;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

pub const SECTION_ORDER: [&str; 4] = [
    "Out of scope",
    "Acceptance criteria",
    "Verification",
    "Evidence",
];

pub const REGISTER_ORDER: [&str; 8] = ["R", "B", "C", "T", "I", "Q", "H", "S"];

pub const SCHEMA_RELATIVE_PATH: &str = "tools/schema/fields.json";

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionSchema {
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub min_items: usize,
    #[serde(default)]
    pub checkbox: bool,
    #[serde(default)]
    pub line_kinds: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conditional {
    #[serde(default)]
    pub required_when: Option<BTreeMap<String, String>>,
    #[serde(default)]
    pub forbidden_unless: Option<BTreeMap<String, String>>,
    #[serde(default)]
    pub forbidden_when: Option<BTreeMap<String, String>>,
    #[serde(default)]
    pub id_family: Option<String>,
    #[serde(default)]
    pub single: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSchema {
    pub heading_pattern: String,
    pub field_order: Vec<String>,
    pub required: Vec<String>,
    pub enums: BTreeMap<String, Vec<String>>,
    pub conditional: BTreeMap<String, Conditional>,
    pub depends_on_families: Vec<String>,
    pub dropped_reasons: Vec<String>,
    pub owner_pattern: String,
    pub sections: BTreeMap<String, SectionSchema>,
    pub verification_required_kind: BTreeMap<String, String>,
    pub covers_comment: String,
    pub null_token: String,
    pub max_lines_warning: usize,
    pub banned_criteria_words: Vec<String>,
    pub size_weights: BTreeMap<String, u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockSchema {
    pub heading_pattern: String,
    pub field_order: Vec<String>,
    #[serde(default)]
    pub enums: BTreeMap<String, Vec<String>>,
    pub required: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneFileSchema {
    pub field_order: Vec<String>,
    pub sections: Vec<String>,
    pub gate: BlockSchema,
    pub demo: BlockSchema,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionSchema {
    pub heading_pattern: String,
    pub field_order: Vec<String>,
    pub enums: BTreeMap<String, Vec<String>>,
    pub sections: Vec<String>,
    pub min_options: usize,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSchema {
    pub file: String,
    pub fields: Vec<String>,
    #[serde(default)]
    pub enums: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub target_kinds: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedSchema {
    pub whole_files: Vec<String>,
    pub directory: String,
    pub marker_begin: String,
    pub marker_end: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub version: u32,
    pub milestones: Vec<String>,
    pub milestone_rank: BTreeMap<String, u32>,
    pub workstreams: Vec<String>,
    pub baseline_gap_allowed: Vec<String>,
    pub example_prefix: String,
    pub task: TaskSchema,
    pub id_families: BTreeMap<String, String>,
    pub milestone_file: MilestoneFileSchema,
    pub decision: DecisionSchema,
    pub registers: BTreeMap<String, RegisterSchema>,
    pub generated: GeneratedSchema,
    pub date_pattern: String,
}

impl Schema {
    pub fn load(root: &Path) -> Result<Self> {
        let path = root.join(SCHEMA_RELATIVE_PATH);
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }

    pub fn rank(&self, milestone: &str) -> Option<u32> {
        self.milestone_rank.get(milestone).copied()
    }

    pub fn is_workstream(&self, prefix: &str) -> bool {
        self.workstreams.iter().any(|entry| entry == prefix)
    }

    pub fn workstream_position(&self, prefix: &str) -> usize {
        self.workstreams
            .iter()
            .position(|entry| entry == prefix)
            .unwrap_or(usize::MAX)
    }

    pub fn baseline_gap_allowed_for(&self, prefix: &str) -> bool {
        self.baseline_gap_allowed
            .iter()
            .any(|entry| entry == prefix)
    }

    pub fn marker_begin(&self, name: &str) -> String {
        self.generated.marker_begin.replace("{name}", name)
    }

    pub fn marker_end(&self) -> &str {
        &self.generated.marker_end
    }

    pub fn ordered_milestones(&self) -> Vec<String> {
        let mut tokens = self.milestones.clone();
        tokens.sort_by_key(|token| (self.rank(token).unwrap_or(u32::MAX), token.clone()));
        tokens
    }
}

pub struct Patterns {
    pub task_heading: Regex,
    pub gate_heading: Regex,
    pub demo_heading: Regex,
    pub decision_heading: Regex,
    pub covers_comment: Regex,
    pub owner: Regex,
    pub date: Regex,
    pub families: BTreeMap<String, Regex>,
    pub field_line: Regex,
    pub baseline_reference: Regex,
    pub checkbox: Regex,
    pub verification_line: Regex,
    pub register_heading: Regex,
    pub alias_heading: Regex,
    pub percentage: Regex,
    pub performance_number: Regex,
    pub benchmark_identifier: Regex,
    pub evidence: Regex,
    pub imperative_title: Regex,
}

impl Patterns {
    pub fn build(schema: &Schema) -> Result<Self> {
        let mut families = BTreeMap::new();
        for (name, pattern) in &schema.id_families {
            families.insert(
                name.clone(),
                Regex::new(pattern)
                    .with_context(|| format!("compiling id family pattern {name}"))?,
            );
        }
        Ok(Self {
            task_heading: Regex::new(&schema.task.heading_pattern)?,
            gate_heading: Regex::new(&schema.milestone_file.gate.heading_pattern)?,
            demo_heading: Regex::new(&schema.milestone_file.demo.heading_pattern)?,
            decision_heading: Regex::new(&schema.decision.heading_pattern)?,
            covers_comment: Regex::new(&schema.task.covers_comment)?,
            owner: Regex::new(&schema.task.owner_pattern)?,
            date: Regex::new(&schema.date_pattern)?,
            families,
            field_line: Regex::new(r"^- ([A-Z][A-Za-z ]{0,30}):[ ]?(.*)$")?,
            baseline_reference: Regex::new(r"^§\d+(\.\d+)?$")?,
            checkbox: Regex::new(r"^- \[([ xX])\] (.*)$")?,
            verification_line: Regex::new(r"^- ([A-Z][A-Za-z]*): (.*)$")?,
            register_heading: Regex::new(r"^### ([A-Z]-\d{3,4}) · (.+)$")?,
            alias_heading: Regex::new(r"^### ([a-z0-9][a-z0-9-]*)$")?,
            percentage: Regex::new(r"\d+%")?,
            performance_number: Regex::new(
                r"(?:≤|<|>=|<=|>|≥)\s*\d+(?:\.\d+)?\s*(?:µs|us|ms|ns|s|MB|GB|KB|MiB|GiB)\b",
            )?,
            benchmark_identifier: Regex::new(r"\bB-\d{3,}\b")?,
            evidence: Regex::new(
                r"^(none|[a-z0-9][a-z0-9-]*@[0-9a-fA-F]{7,40}|[a-z0-9][a-z0-9-]*#\d+|https://\S+|report:reports/(spikes/[A-Z]{2,4}-(?:\d{3,}|@[a-z0-9][a-z0-9-]*)\.md|benchmarks/B-\d{3,}/[a-z0-9][a-z0-9-]*@[0-9a-fA-F]{7,40}-H-\d{3,}\.md|compat/C-\d{3,}/[a-z0-9][a-z0-9-]*@[0-9a-fA-F]{7,40}-H-\d{3,}\.md)|decision:D-\d{4})$",
            )?,
            imperative_title: Regex::new(r"^[A-Z][A-Za-z]+(?:[-/][A-Za-z]+)*\b")?,
        })
    }

    pub fn family(&self, name: &str) -> Option<&Regex> {
        self.families.get(name)
    }

    pub fn matches_family(&self, name: &str, value: &str) -> bool {
        self.families
            .get(name)
            .map(|pattern| pattern.is_match(value))
            .unwrap_or(false)
    }
}
