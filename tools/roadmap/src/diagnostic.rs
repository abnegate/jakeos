use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

impl Severity {
    pub fn label(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Diagnostic {
    pub file: String,
    pub line: usize,
    pub code: &'static str,
    pub severity: Severity,
    pub message: String,
    pub hint: String,
}

impl Diagnostic {
    pub fn error(
        file: impl Into<String>,
        line: usize,
        code: &'static str,
        message: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        Self {
            file: file.into(),
            line,
            code,
            severity: Severity::Error,
            message: message.into(),
            hint: hint.into(),
        }
    }

    pub fn warning(
        file: impl Into<String>,
        line: usize,
        code: &'static str,
        message: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        Self {
            file: file.into(),
            line,
            code,
            severity: Severity::Warning,
            message: message.into(),
            hint: hint.into(),
        }
    }

    pub fn render(&self) -> String {
        format!(
            "{}:{}: {} [{}] {}\n    hint: {}",
            self.file,
            self.line,
            self.severity.label(),
            self.code,
            self.message,
            self.hint
        )
    }

    fn sort_key(&self) -> (&str, usize, &'static str, &str) {
        (&self.file, self.line, self.code, &self.message)
    }
}

#[derive(Default, Debug)]
pub struct Diagnostics {
    entries: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.entries.push(diagnostic);
    }

    pub fn extend(&mut self, other: impl IntoIterator<Item = Diagnostic>) {
        self.entries.extend(other);
    }

    pub fn entries(&self) -> &[Diagnostic] {
        &self.entries
    }

    pub fn into_entries(self) -> Vec<Diagnostic> {
        self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn sorted(mut self, strict: bool) -> Vec<Diagnostic> {
        if strict {
            for entry in &mut self.entries {
                entry.severity = Severity::Error;
            }
        }
        self.entries
            .sort_by(|left, right| left.sort_key().cmp(&right.sort_key()));
        self.entries.dedup_by(|left, right| {
            left.file == right.file
                && left.line == right.line
                && left.code == right.code
                && left.message == right.message
        });
        self.entries
    }
}

pub fn count_errors(entries: &[Diagnostic]) -> usize {
    entries
        .iter()
        .filter(|entry| entry.severity == Severity::Error)
        .count()
}

pub mod code {
    pub const UNKNOWN_FIELD: &str = "E-001";
    pub const DUPLICATE_FIELD: &str = "E-002";
    pub const INVALID_ENUM: &str = "E-003";
    pub const INVALID_ID_SYNTAX: &str = "E-004";
    pub const DUPLICATE_ID: &str = "E-005";
    pub const PREFIX_FILE_MISMATCH: &str = "E-006";
    pub const UNKNOWN_PREFIX: &str = "E-007";
    pub const NON_ASCENDING_ID: &str = "E-008";
    pub const MISSING_FIELD: &str = "E-009";
    pub const CONDITIONAL_REQUIRED: &str = "E-010";
    pub const CONDITIONAL_FORBIDDEN: &str = "E-011";
    pub const MISSING_SECTION: &str = "E-012";
    pub const SECTION_TOO_SHORT: &str = "E-013";
    pub const INVALID_VERIFICATION_KIND: &str = "E-014";
    pub const INVALID_CHECKBOX: &str = "E-015";
    pub const INVALID_OWNER: &str = "E-016";
    pub const MALFORMED_BLOCK: &str = "E-017";
    pub const SINGLE_VALUE_FIELD: &str = "E-018";

    pub const DANGLING_REFERENCE: &str = "E-020";
    pub const INVALID_ID_TOKEN: &str = "E-021";
    pub const BASELINE_UNRESOLVED: &str = "E-022";
    pub const BASELINE_NONE_FORBIDDEN: &str = "E-023";
    pub const BASELINE_GAP_FORBIDDEN: &str = "E-024";
    pub const UNKNOWN_MILESTONE: &str = "E-025";
    pub const DRAFT_NOT_ALLOWED: &str = "E-026";
    pub const UNKNOWN_DRAFT: &str = "E-027";

    pub const DEPENDENCY_CYCLE: &str = "E-030";
    pub const SELF_DEPENDENCY: &str = "E-031";
    pub const MILESTONE_MONOTONICITY: &str = "E-032";
    pub const DROPPED_WITHOUT_SUPERSEDER: &str = "E-033";

    pub const DONE_UNTICKED_BOX: &str = "E-040";
    pub const DONE_WITHOUT_VERIFICATION: &str = "E-041";
    pub const DONE_WITHOUT_EVIDENCE: &str = "E-042";
    pub const DONE_UNRESOLVED_DEPENDENCY: &str = "E-043";
    pub const DONE_WITHOUT_VERIFIER: &str = "E-044";
    pub const VERIFIER_IS_OWNER: &str = "E-045";
    pub const VERIFIER_IS_AGENT: &str = "E-046";
    pub const ADR_DECISION_NOT_CLOSED: &str = "E-047";
    pub const SPIKE_REPORT_MISSING: &str = "E-048";
    pub const FREEZE_DISCIPLINE: &str = "E-049";
    pub const IN_PROGRESS_WITHOUT_OWNER: &str = "E-051";
    pub const IN_PROGRESS_SIZE_XL: &str = "E-052";
    pub const DROPPED_REASON_ENUM: &str = "E-053";

    pub const SPIKE_WITHOUT_REPORT_LINE: &str = "E-060";
    pub const BENCHMARK_WITHOUT_BENCH_LINE: &str = "E-061";

    pub const DECISION_FILE_MISSING: &str = "E-070";
    pub const DECISION_TASK_MISMATCH: &str = "E-071";
    pub const DECISION_TASK_NOT_UNIQUE: &str = "E-072";
    pub const DECISION_TOO_FEW_OPTIONS: &str = "E-073";
    pub const DECISION_MISSING_SECTION: &str = "E-074";

    pub const GATE_RANK: &str = "E-080";
    pub const MILESTONE_WITHOUT_GATES: &str = "E-081";
    pub const GATE_WITHOUT_BENCHMARK: &str = "E-082";
    pub const BENCHMARK_WITHOUT_TARGET: &str = "E-083";
    pub const GATE_WITHOUT_CORPUS: &str = "E-084";
    pub const UNKNOWN_MILESTONE_FILE: &str = "E-085";

    pub const REGISTER_UNKNOWN_FIELD: &str = "E-090";
    pub const REGISTER_INVALID_ENUM: &str = "E-091";
    pub const REGISTER_INVALID_ID_LIST: &str = "E-092";
    pub const REGISTER_ID_FAMILY: &str = "E-093";
    pub const REGISTER_TARGET_GRAMMAR: &str = "E-094";

    pub const CALENDAR_DATE: &str = "E-100";

    pub const TICKED_NOT_DONE: &str = "W-001";
    pub const UNANCHORED: &str = "W-002";
    pub const GATE_ONLY_EXAMPLES: &str = "W-003";
    pub const GENERATED_STALE: &str = "W-004";
    pub const XL_WITHOUT_SPLIT: &str = "W-005";
    pub const BENCHMARK_UNREFERENCED: &str = "W-006";
    pub const QUESTION_UNBOUND: &str = "W-007";
    pub const BANNED_CRITERIA_WORD: &str = "W-008";
    pub const NON_IMPERATIVE_TITLE: &str = "W-009";
    pub const TASK_TOO_LONG: &str = "W-010";
    pub const WORKSTREAM_TOO_LONG: &str = "W-011";
    pub const FAN_IN: &str = "W-012";
    pub const HAND_TYPED_PERCENT: &str = "W-013";
    pub const PERFORMANCE_NUMBER: &str = "W-014";
    pub const GLOSSARY_CASING: &str = "W-015";
}
