use std::collections::{BTreeSet, VecDeque};

pub struct Graph {
    pub dependencies: Vec<Vec<usize>>,
    pub dependents: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Self {
            dependencies: vec![Vec::new(); size],
            dependents: vec![Vec::new(); size],
        }
    }

    pub fn add_edge(&mut self, dependent: usize, dependency: usize) {
        if !self.dependencies[dependent].contains(&dependency) {
            self.dependencies[dependent].push(dependency);
        }
        if !self.dependents[dependency].contains(&dependent) {
            self.dependents[dependency].push(dependent);
        }
    }

    pub fn size(&self) -> usize {
        self.dependencies.len()
    }

    pub fn strongly_connected_components(&self) -> Vec<Vec<usize>> {
        let size = self.size();
        let mut index_of = vec![usize::MAX; size];
        let mut low_link = vec![0usize; size];
        let mut on_stack = vec![false; size];
        let mut stack: Vec<usize> = Vec::new();
        let mut next_index = 0usize;
        let mut components = Vec::new();

        for root in 0..size {
            if index_of[root] != usize::MAX {
                continue;
            }
            let mut call_stack: Vec<(usize, usize)> = vec![(root, 0)];
            while let Some((node, child_position)) = call_stack.pop() {
                if child_position == 0 {
                    index_of[node] = next_index;
                    low_link[node] = next_index;
                    next_index += 1;
                    stack.push(node);
                    on_stack[node] = true;
                }
                let neighbours = &self.dependencies[node];
                if child_position < neighbours.len() {
                    let child = neighbours[child_position];
                    call_stack.push((node, child_position + 1));
                    if index_of[child] == usize::MAX {
                        call_stack.push((child, 0));
                    } else if on_stack[child] {
                        low_link[node] = low_link[node].min(index_of[child]);
                    }
                    continue;
                }
                if low_link[node] == index_of[node] {
                    let mut component = Vec::new();
                    while let Some(member) = stack.pop() {
                        on_stack[member] = false;
                        component.push(member);
                        if member == node {
                            break;
                        }
                    }
                    component.sort_unstable();
                    components.push(component);
                }
                if let Some((parent, _)) = call_stack.last().copied() {
                    low_link[parent] = low_link[parent].min(low_link[node]);
                }
            }
        }
        components
    }

    pub fn cycles(&self) -> Vec<Vec<usize>> {
        let mut cycles: Vec<Vec<usize>> = self
            .strongly_connected_components()
            .into_iter()
            .filter(|component| component.len() > 1)
            .collect();
        for node in 0..self.size() {
            if self.dependencies[node].contains(&node) {
                cycles.push(vec![node]);
            }
        }
        cycles.sort();
        cycles
    }

    pub fn topological_order(&self) -> Vec<usize> {
        let size = self.size();
        let mut remaining: Vec<usize> = (0..size)
            .map(|node| self.dependencies[node].len())
            .collect();
        let mut queue: VecDeque<usize> = (0..size).filter(|node| remaining[*node] == 0).collect();
        let mut order = Vec::with_capacity(size);
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for dependent in &self.dependents[node] {
                remaining[*dependent] = remaining[*dependent].saturating_sub(1);
                if remaining[*dependent] == 0 {
                    queue.push_back(*dependent);
                }
            }
        }
        order
    }

    pub fn transitive_dependents(&self, start: usize) -> BTreeSet<usize> {
        let mut seen = BTreeSet::new();
        let mut queue = VecDeque::from([start]);
        while let Some(node) = queue.pop_front() {
            for dependent in &self.dependents[node] {
                if seen.insert(*dependent) {
                    queue.push_back(*dependent);
                }
            }
        }
        seen.remove(&start);
        seen
    }

    pub fn transitive_dependencies(&self, start: usize) -> BTreeSet<usize> {
        let mut seen = BTreeSet::new();
        let mut queue = VecDeque::from([start]);
        while let Some(node) = queue.pop_front() {
            for dependency in &self.dependencies[node] {
                if seen.insert(*dependency) {
                    queue.push_back(*dependency);
                }
            }
        }
        seen.remove(&start);
        seen
    }

    pub fn closure_of(&self, roots: &[usize]) -> BTreeSet<usize> {
        let mut seen: BTreeSet<usize> = roots.iter().copied().collect();
        let mut queue: VecDeque<usize> = roots.iter().copied().collect();
        while let Some(node) = queue.pop_front() {
            for dependency in &self.dependencies[node] {
                if seen.insert(*dependency) {
                    queue.push_back(*dependency);
                }
            }
        }
        seen
    }
}

pub struct Chain {
    pub nodes: Vec<usize>,
    pub weight: u32,
}

pub struct CriticalPath {
    pub members: Vec<usize>,
    pub earliest_finish: Vec<u32>,
    pub latest_finish: Vec<u32>,
    pub slack: Vec<u32>,
    pub makespan: u32,
    pub chains: Vec<Chain>,
}

pub fn critical_path(
    graph: &Graph,
    weights: &[u32],
    targets: &[usize],
    include: &BTreeSet<usize>,
) -> CriticalPath {
    let size = graph.size();
    let mut earliest_start = vec![0u32; size];
    let mut earliest_finish = vec![0u32; size];
    let mut best_predecessor = vec![usize::MAX; size];

    for node in graph.topological_order() {
        if !include.contains(&node) {
            continue;
        }
        let mut start = 0u32;
        let mut predecessor = usize::MAX;
        for dependency in &graph.dependencies[node] {
            if !include.contains(dependency) {
                continue;
            }
            if earliest_finish[*dependency] > start {
                start = earliest_finish[*dependency];
                predecessor = *dependency;
            }
        }
        earliest_start[node] = start;
        earliest_finish[node] = start + weights[node];
        best_predecessor[node] = predecessor;
    }

    let makespan = targets
        .iter()
        .filter(|node| include.contains(node))
        .map(|node| earliest_finish[*node])
        .max()
        .unwrap_or(0);

    let mut latest_finish = vec![makespan; size];
    let mut order = graph.topological_order();
    order.reverse();
    for node in order {
        if !include.contains(&node) {
            continue;
        }
        let relevant: Vec<usize> = graph.dependents[node]
            .iter()
            .copied()
            .filter(|dependent| include.contains(dependent))
            .collect();
        if relevant.is_empty() {
            latest_finish[node] = if targets.contains(&node) {
                makespan
            } else {
                latest_finish[node].min(makespan)
            };
        } else {
            let mut latest = u32::MAX;
            for dependent in relevant {
                let start = latest_finish[dependent].saturating_sub(weights[dependent]);
                latest = latest.min(start);
            }
            if targets.contains(&node) {
                latest = latest.min(makespan);
            }
            latest_finish[node] = latest;
        }
    }

    let slack: Vec<u32> = (0..size)
        .map(|node| {
            if include.contains(&node) {
                latest_finish[node].saturating_sub(earliest_finish[node])
            } else {
                0
            }
        })
        .collect();

    let mut chains: Vec<Chain> = targets
        .iter()
        .copied()
        .filter(|node| include.contains(node))
        .map(|target| {
            let mut nodes = Vec::new();
            let mut current = target;
            while current != usize::MAX {
                nodes.push(current);
                current = best_predecessor[current];
            }
            nodes.reverse();
            Chain {
                nodes,
                weight: earliest_finish[target],
            }
        })
        .collect();
    chains.sort_by(|left, right| {
        right
            .weight
            .cmp(&left.weight)
            .then(left.nodes.cmp(&right.nodes))
    });

    CriticalPath {
        members: include.iter().copied().collect(),
        earliest_finish,
        latest_finish,
        slack,
        makespan,
        chains,
    }
}
