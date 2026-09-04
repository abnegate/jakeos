pub fn parse(content: &str) -> Vec<String> {
    let mut terms = Vec::new();
    let mut inside_fence = false;
    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            inside_fence = !inside_fence;
            continue;
        }
        if inside_fence {
            continue;
        }
        if let Some(rest) = line.strip_prefix("## ") {
            let term = rest.trim().to_string();
            if !term.is_empty() && !terms.contains(&term) {
                terms.push(term);
            }
        }
    }
    terms.sort_by(|left, right| right.len().cmp(&left.len()).then(left.cmp(right)));
    terms
}
