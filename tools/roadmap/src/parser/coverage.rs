use crate::model::CoverageItem;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawItem {
    id: String,
    #[serde(default)]
    workstream: String,
    #[serde(default)]
    milestone: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    merged_into: Option<String>,
}

pub fn parse(content: &str) -> Vec<CoverageItem> {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<RawItem>(line).ok())
        .map(|raw| CoverageItem {
            id: raw.id,
            workstream: raw.workstream,
            milestone: raw.milestone,
            text: raw.text,
            merged_into: raw.merged_into.filter(|value| !value.is_empty()),
        })
        .collect()
}
