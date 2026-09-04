pub mod baseline;
pub mod coverage;
pub mod decision;
pub mod glossary;
pub mod milestone;
pub mod register;
pub mod slugs;
pub mod workstream;

use crate::model::{Field, Fields};
use crate::schema::Patterns;

pub type NumberedLine = (usize, String);

pub fn numbered(text: &str) -> Vec<NumberedLine> {
    text.lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim_end().to_string()))
        .collect()
}

pub fn marker_begin_prefix() -> &'static str {
    "<!-- roadmap:generated:begin "
}

pub fn is_marker_begin(line: &str) -> bool {
    line.trim_start().starts_with(marker_begin_prefix())
}

pub fn is_marker_end(line: &str) -> bool {
    line.trim() == "<!-- roadmap:generated:end -->"
}

pub fn strip_generated_blocks(lines: &[NumberedLine]) -> Vec<NumberedLine> {
    let mut kept = Vec::with_capacity(lines.len());
    let mut inside = false;
    for (number, text) in lines {
        if is_marker_begin(text) {
            inside = true;
            continue;
        }
        if is_marker_end(text) {
            inside = false;
            continue;
        }
        if !inside {
            kept.push((*number, text.clone()));
        }
    }
    kept
}

pub fn parse_field_block(
    lines: &[NumberedLine],
    start: usize,
    patterns: &Patterns,
) -> (Fields, usize) {
    let mut fields = Fields::default();
    let mut cursor = start;
    while cursor < lines.len() {
        let (number, text) = &lines[cursor];
        let Some(captures) = patterns.field_line.captures(text) else {
            break;
        };
        fields.items.push(Field {
            key: captures[1].trim().to_string(),
            value: captures[2].trim().to_string(),
            line: *number,
        });
        cursor += 1;
    }
    (fields, cursor)
}

pub fn trim_blank_edges(lines: &[NumberedLine]) -> Vec<NumberedLine> {
    let mut slice = lines;
    while let Some((_, first)) = slice.first() {
        if first.trim().is_empty() {
            slice = &slice[1..];
        } else {
            break;
        }
    }
    while let Some((_, last)) = slice.last() {
        if last.trim().is_empty() {
            slice = &slice[..slice.len() - 1];
        } else {
            break;
        }
    }
    slice.to_vec()
}

pub fn text_of(lines: &[NumberedLine]) -> Vec<String> {
    lines.iter().map(|(_, text)| text.clone()).collect()
}
