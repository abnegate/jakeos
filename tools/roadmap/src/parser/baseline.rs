use crate::model::BaselineIndex;
use regex::Regex;
use std::sync::LazyLock;

static HEADING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#{1,2} (\d+)(?:\.(\d+))?\.? ").expect("valid heading pattern"));

pub fn parse(content: &str) -> BaselineIndex {
    let mut sections = Vec::new();
    let mut inside_fence = false;
    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            inside_fence = !inside_fence;
            continue;
        }
        if inside_fence {
            continue;
        }
        if let Some(captures) = HEADING.captures(line) {
            let key = match captures.get(2) {
                Some(minor) => format!("{}.{}", &captures[1], minor.as_str()),
                None => captures[1].to_string(),
            };
            if !sections.contains(&key) {
                sections.push(key);
            }
        }
    }
    BaselineIndex { sections }
}
