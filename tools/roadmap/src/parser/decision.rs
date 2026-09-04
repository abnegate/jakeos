use crate::diagnostic::{Diagnostic, code};
use crate::model::Decision;
use crate::parser::{numbered, parse_field_block, strip_generated_blocks, trim_blank_edges};
use crate::schema::{Patterns, Schema};
use std::collections::BTreeMap;

pub struct ParsedDecision {
    pub decision: Decision,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn parse(
    relative_path: &str,
    content: &str,
    schema: &Schema,
    patterns: &Patterns,
) -> Option<ParsedDecision> {
    let lines = strip_generated_blocks(&numbered(content));
    let mut diagnostics = Vec::new();

    let mut cursor = 0usize;
    while cursor < lines.len() && lines[cursor].1.trim().is_empty() {
        cursor += 1;
    }
    let (line_number, heading_text) = lines.get(cursor)?;
    let Some(captures) = patterns.decision_heading.captures(heading_text) else {
        diagnostics.push(Diagnostic::error(
            relative_path,
            *line_number,
            code::INVALID_ID_SYNTAX,
            format!("decision heading does not match the required shape: `{heading_text}`"),
            "write `# D-0007 · Title`",
        ));
        return Some(ParsedDecision {
            decision: Decision {
                id: String::new(),
                title: String::new(),
                file: relative_path.to_string(),
                fields: crate::model::Fields::default(),
                sections: Vec::new(),
                options: Vec::new(),
                body: BTreeMap::new(),
                line: *line_number,
            },
            diagnostics,
        });
    };
    let id = captures[1].to_string();
    let title = captures[2].trim().to_string();
    cursor += 1;
    while cursor < lines.len() && lines[cursor].1.trim().is_empty() {
        cursor += 1;
    }
    let (fields, _) = parse_field_block(&lines, cursor, patterns);
    for field in &fields.items {
        if !schema.decision.field_order.contains(&field.key) {
            diagnostics.push(Diagnostic::error(
                relative_path,
                field.line,
                code::UNKNOWN_FIELD,
                format!("unknown decision field `{}`", field.key),
                format!("allowed fields: {}", schema.decision.field_order.join(", ")),
            ));
        }
    }

    let section_starts: Vec<usize> = (0..lines.len())
        .filter(|index| lines[*index].1.starts_with("## "))
        .collect();
    let mut sections = Vec::new();
    let mut body = BTreeMap::new();
    let mut options = Vec::new();
    for (position, section_start) in section_starts.iter().enumerate() {
        let end = section_starts
            .get(position + 1)
            .copied()
            .unwrap_or(lines.len());
        let name = lines[*section_start]
            .1
            .trim_start_matches("## ")
            .trim()
            .to_string();
        sections.push(name.clone());
        let content_lines = trim_blank_edges(&lines[section_start + 1..end]);
        if name == "Options" {
            options = content_lines
                .iter()
                .filter(|(_, text)| text.starts_with("### "))
                .map(|(_, text)| text.trim_start_matches("### ").trim().to_string())
                .collect();
        }
        body.insert(
            name,
            content_lines
                .iter()
                .map(|(_, text)| text.clone())
                .collect::<Vec<String>>(),
        );
    }

    Some(ParsedDecision {
        decision: Decision {
            id,
            title,
            file: relative_path.to_string(),
            fields,
            sections,
            options,
            body,
            line: *line_number,
        },
        diagnostics,
    })
}
