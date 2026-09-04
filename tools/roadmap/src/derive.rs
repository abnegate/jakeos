use crate::graph::Graph;
use crate::model::{DerivedState, Status, TaskType};
use crate::repo::Repo;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MilestoneStatus {
    Complete,
    Active,
    Next,
    Planned,
}

impl MilestoneStatus {
    pub fn label(self) -> &'static str {
        match self {
            MilestoneStatus::Complete => "complete",
            MilestoneStatus::Active => "active",
            MilestoneStatus::Next => "next",
            MilestoneStatus::Planned => "planned",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Progress {
    pub total: usize,
    pub done: usize,
    pub todo: usize,
    pub in_progress: usize,
    pub dropped: usize,
    pub weight_total: u32,
    pub weight_done: u32,
    pub gates_total: usize,
    pub gates_satisfied: usize,
}

impl Progress {
    pub fn count_percent(&self) -> f64 {
        percent(self.done, self.total)
    }

    pub fn weighted_percent(&self) -> f64 {
        percent(self.weight_done as usize, self.weight_total as usize)
    }

    pub fn gate_percent(&self) -> f64 {
        percent(self.gates_satisfied, self.gates_total)
    }
}

pub fn percent(part: usize, whole: usize) -> f64 {
    if whole == 0 {
        return 0.0;
    }
    (part as f64) * 100.0 / (whole as f64)
}

pub fn format_percent(value: f64) -> String {
    format!("{value:.0}%")
}

#[derive(Clone, Debug, Default)]
struct Bitset {
    words: Vec<u64>,
}

impl Bitset {
    fn new(size: usize) -> Self {
        Self {
            words: vec![0; size.div_ceil(64)],
        }
    }

    fn set(&mut self, index: usize) {
        self.words[index / 64] |= 1u64 << (index % 64);
    }

    fn union_with(&mut self, other: &Bitset) {
        for (target, source) in self.words.iter_mut().zip(other.words.iter()) {
            *target |= *source;
        }
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.words
            .iter()
            .enumerate()
            .flat_map(|(word_index, word)| {
                (0..64).filter_map(move |bit| {
                    if word & (1u64 << bit) != 0 {
                        Some(word_index * 64 + bit)
                    } else {
                        None
                    }
                })
            })
    }
}

pub struct Derived {
    pub graph: Graph,
    pub question_dependencies: Vec<Vec<String>>,
    pub resolved: Vec<bool>,
    pub state: Vec<DerivedState>,
    pub blockers: Vec<Vec<String>>,
    pub downstream: Vec<Vec<usize>>,
    pub downstream_weight: Vec<u32>,
    pub anchored: Vec<bool>,
    pub gate_satisfied: BTreeMap<String, bool>,
    pub gate_reasons: BTreeMap<String, Vec<String>>,
    pub milestone_status: BTreeMap<String, MilestoneStatus>,
    pub milestone_progress: BTreeMap<String, Progress>,
    pub workstream_progress: BTreeMap<String, Progress>,
    pub totals: Progress,
}

pub fn build(repo: &Repo) -> Derived {
    let size = repo.tasks.len();
    let mut graph = Graph::new(size);
    let mut question_dependencies = vec![Vec::new(); size];

    for (position, task) in repo.tasks.iter().enumerate() {
        for dependency in task.depends_on() {
            if dependency.starts_with("Q-") {
                question_dependencies[position].push(dependency);
                continue;
            }
            if let Some(target) = repo.task_position(&dependency) {
                graph.add_edge(position, target);
            }
        }
    }

    let resolved = compute_resolution(repo);
    let mut state = vec![DerivedState::Waiting; size];
    let mut blockers: Vec<Vec<String>> = vec![Vec::new(); size];
    for (position, task) in repo.tasks.iter().enumerate() {
        let mut unresolved = Vec::new();
        for dependency in task.depends_on() {
            if dependency.starts_with("Q-") {
                if !question_answered(repo, &dependency) {
                    unresolved.push(dependency);
                }
                continue;
            }
            match repo.task_position(&dependency) {
                Some(target) => {
                    if !resolved[target] {
                        unresolved.push(dependency);
                    }
                }
                None => {
                    if !repo.is_example(&dependency) {
                        unresolved.push(dependency);
                    }
                }
            }
        }
        blockers[position] = unresolved.clone();
        state[position] = match task.status() {
            Status::Done => DerivedState::Done,
            Status::Dropped => DerivedState::Dropped,
            Status::Todo => {
                if unresolved.is_empty() {
                    DerivedState::Ready
                } else {
                    DerivedState::Blocked
                }
            }
            Status::InProgress => {
                if !unresolved.is_empty() {
                    DerivedState::Blocked
                } else if task.has_non_none_evidence() {
                    DerivedState::InReview
                } else {
                    DerivedState::Waiting
                }
            }
        };
    }

    let mut sets: Vec<Bitset> = vec![Bitset::new(size); size];
    let mut order = graph.topological_order();
    order.reverse();
    let mut visited = vec![false; size];
    for node in &order {
        visited[*node] = true;
        let mut accumulated = Bitset::new(size);
        for dependent in &graph.dependents[*node] {
            accumulated.set(*dependent);
            accumulated.union_with(&sets[*dependent]);
        }
        sets[*node] = accumulated;
    }
    for node in 0..size {
        if !visited[node] {
            let mut accumulated = Bitset::new(size);
            for dependent in graph.transitive_dependents(node) {
                accumulated.set(dependent);
            }
            sets[node] = accumulated;
        }
    }
    let downstream: Vec<Vec<usize>> = sets.iter().map(|set| set.iter().collect()).collect();
    let downstream_weight: Vec<u32> = downstream
        .iter()
        .map(|members| {
            members
                .iter()
                .map(|member| repo.weight(&repo.tasks[*member]))
                .sum()
        })
        .collect();

    let (gate_satisfied, gate_reasons) = compute_gates(repo, &resolved);
    let anchored = compute_anchoring(repo, &graph);
    let (milestone_status, milestone_progress, workstream_progress, totals) =
        compute_progress(repo, &gate_satisfied);

    Derived {
        graph,
        question_dependencies,
        resolved,
        state,
        blockers,
        downstream,
        downstream_weight,
        anchored,
        gate_satisfied,
        gate_reasons,
        milestone_status,
        milestone_progress,
        workstream_progress,
        totals,
    }
}

pub fn question_answered(repo: &Repo, id: &str) -> bool {
    repo.register("Q")
        .and_then(|register| register.get(id))
        .map(|entry| entry.status() == "answered")
        .unwrap_or(false)
}

fn compute_resolution(repo: &Repo) -> Vec<bool> {
    let mut resolved = vec![false; repo.tasks.len()];
    for (position, task) in repo.tasks.iter().enumerate() {
        resolved[position] = match task.status() {
            Status::Done => true,
            Status::Dropped => task.list("Superseded by").iter().any(|superseder| {
                repo.task(superseder)
                    .map(|target| target.status() == Status::Done)
                    .unwrap_or(false)
            }),
            _ => false,
        };
    }
    resolved
}

pub fn reference_resolved(repo: &Repo, resolved: &[bool], id: &str) -> bool {
    if repo.is_example(id) {
        return false;
    }
    repo.task_position(id)
        .map(|position| resolved[position])
        .unwrap_or(false)
}

fn compute_gates(
    repo: &Repo,
    resolved: &[bool],
) -> (BTreeMap<String, bool>, BTreeMap<String, Vec<String>>) {
    let mut satisfied = BTreeMap::new();
    let mut reasons: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for milestone in &repo.milestones {
        for gate in &milestone.gates {
            let mut failing = Vec::new();
            let verified_by = gate.verified_by();
            if verified_by.is_empty() {
                failing.push("gate lists no verifying task".to_string());
            }
            for identifier in &verified_by {
                if repo.is_example(identifier) {
                    failing.push(format!("`{identifier}` is an example placeholder"));
                    continue;
                }
                match repo.task(identifier) {
                    Some(task) => {
                        if !reference_resolved(repo, resolved, identifier) {
                            failing.push(format!(
                                "`{identifier}` is {} — {}",
                                task.status(),
                                task.title
                            ));
                        }
                    }
                    None => failing.push(format!("`{identifier}` does not exist")),
                }
            }
            let alternative = gate
                .fields
                .value("Or")
                .map(|value| value.trim().to_string());
            let alternative_done = alternative
                .as_ref()
                .map(|identifier| reference_resolved(repo, resolved, identifier))
                .unwrap_or(false);
            let is_satisfied = failing.is_empty() || alternative_done;
            if !is_satisfied && let Some(identifier) = &alternative {
                failing.push(format!("alternative `{identifier}` is not done"));
            }
            satisfied.insert(gate.id.clone(), is_satisfied);
            reasons.insert(gate.id.clone(), failing);
        }
    }
    (satisfied, reasons)
}

fn compute_anchoring(repo: &Repo, graph: &Graph) -> Vec<bool> {
    let mut anchored = vec![true; repo.tasks.len()];
    let ranks: Vec<u32> = repo
        .schema
        .ordered_milestones()
        .iter()
        .filter(|token| token.as_str() != "LATER")
        .filter_map(|token| repo.schema.rank(token))
        .collect();
    for rank in ranks {
        let mut roots = Vec::new();
        for milestone in &repo.milestones {
            if repo.rank(&milestone.token) < rank {
                continue;
            }
            let identifiers = milestone
                .gates
                .iter()
                .flat_map(|gate| gate.verified_by())
                .chain(milestone.demos.iter().flat_map(|demo| demo.verified_by()))
                .chain(
                    milestone
                        .gates
                        .iter()
                        .filter_map(|gate| gate.fields.value("Or").map(|value| value.to_string())),
                );
            for identifier in identifiers {
                if let Some(position) = repo.task_position(&identifier) {
                    roots.push(position);
                }
            }
        }
        let closure = graph.closure_of(&roots);
        for (position, task) in repo.tasks.iter().enumerate() {
            if repo.rank(task.milestone()) != rank {
                continue;
            }
            if task.status() == Status::Dropped {
                continue;
            }
            anchored[position] = closure.contains(&position);
        }
    }
    for (position, task) in repo.tasks.iter().enumerate() {
        if task.milestone() == "LATER" || task.status() == Status::Dropped {
            anchored[position] = true;
        }
    }
    anchored
}

type ProgressTables = (
    BTreeMap<String, MilestoneStatus>,
    BTreeMap<String, Progress>,
    BTreeMap<String, Progress>,
    Progress,
);

fn compute_progress(repo: &Repo, gate_satisfied: &BTreeMap<String, bool>) -> ProgressTables {
    let mut milestone_progress: BTreeMap<String, Progress> = BTreeMap::new();
    let mut workstream_progress: BTreeMap<String, Progress> = BTreeMap::new();
    let mut totals = Progress::default();

    for token in repo.schema.ordered_milestones() {
        milestone_progress.insert(token, Progress::default());
    }
    for workstream in &repo.workstreams {
        workstream_progress.insert(workstream.prefix.clone(), Progress::default());
    }

    for task in &repo.tasks {
        let weight = repo.weight(task);
        let status = task.status();
        for bucket in [
            milestone_progress.get_mut(task.milestone()),
            workstream_progress.get_mut(&task.prefix),
        ]
        .into_iter()
        .flatten()
        {
            accumulate(bucket, status, weight);
        }
        accumulate(&mut totals, status, weight);
    }

    for milestone in &repo.milestones {
        let entry = milestone_progress
            .entry(milestone.token.clone())
            .or_default();
        entry.gates_total = milestone.gates.len();
        entry.gates_satisfied = milestone
            .gates
            .iter()
            .filter(|gate| gate_satisfied.get(&gate.id).copied().unwrap_or(false))
            .count();
        totals.gates_total += entry.gates_total;
        totals.gates_satisfied += entry.gates_satisfied;
    }

    let mut status_of: BTreeMap<String, MilestoneStatus> = BTreeMap::new();
    let mut complete: BTreeMap<String, bool> = BTreeMap::new();
    for milestone in &repo.milestones {
        complete.insert(
            milestone.token.clone(),
            milestone_complete(repo, &milestone.token, gate_satisfied, &milestone_progress),
        );
    }
    let mut ordered: Vec<&crate::model::Milestone> = repo.milestones.iter().collect();
    ordered.sort_by_key(|milestone| (milestone.sequence(), milestone.token.clone()));
    let active_position = ordered
        .iter()
        .position(|milestone| !complete.get(&milestone.token).copied().unwrap_or(false));
    for (position, milestone) in ordered.iter().enumerate() {
        let value = if complete.get(&milestone.token).copied().unwrap_or(false) {
            MilestoneStatus::Complete
        } else if Some(position) == active_position {
            MilestoneStatus::Active
        } else if active_position.map(|active| position == active + 1) == Some(true) {
            MilestoneStatus::Next
        } else {
            MilestoneStatus::Planned
        };
        status_of.insert(milestone.token.clone(), value);
    }

    (status_of, milestone_progress, workstream_progress, totals)
}

fn accumulate(progress: &mut Progress, status: Status, weight: u32) {
    match status {
        Status::Dropped => {
            progress.dropped += 1;
            return;
        }
        Status::Done => {
            progress.done += 1;
            progress.weight_done += weight;
        }
        Status::Todo => progress.todo += 1,
        Status::InProgress => progress.in_progress += 1,
    }
    progress.total += 1;
    progress.weight_total += weight;
}

fn milestone_complete(
    repo: &Repo,
    token: &str,
    gate_satisfied: &BTreeMap<String, bool>,
    progress: &BTreeMap<String, Progress>,
) -> bool {
    let Some(milestone) = repo.milestone(token) else {
        return false;
    };
    if milestone.gates.is_empty() {
        return false;
    }
    if !milestone
        .gates
        .iter()
        .all(|gate| gate_satisfied.get(&gate.id).copied().unwrap_or(false))
    {
        return false;
    }
    for surface in milestone.list("Surfaces to freeze") {
        let frozen = repo
            .register("S")
            .and_then(|register| register.get(&surface))
            .map(|entry| entry.fields.value_or_empty("State") == "frozen")
            .unwrap_or(false);
        if !frozen {
            return false;
        }
    }
    for risk in milestone.list("Risks to retire") {
        let retired = repo
            .register("R")
            .and_then(|register| register.get(&risk))
            .map(|entry| matches!(entry.status(), "mitigated" | "accepted" | "closed"))
            .unwrap_or(false);
        if !retired {
            return false;
        }
    }
    let bucket = progress.get(token).copied().unwrap_or_default();
    bucket.todo == 0 && bucket.in_progress == 0
}

pub fn task_state_label(state: DerivedState) -> &'static str {
    state.label()
}

pub fn is_adr(task: &crate::model::Task) -> bool {
    task.task_type() == TaskType::Adr
}
