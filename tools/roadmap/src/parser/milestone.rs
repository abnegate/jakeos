use crate::diagnostic::{Diagnostic, code};
use crate::model::{Demo, Gate, Milestone};
use crate::parser::{
    NumberedLine, numbered, parse_field_block, strip_generated_blocks, text_of, trim_blank_edges,
};
use crate::schema::{Patterns, Schema};

pub struct ParsedMilestone {
    pub milestone: Milestone,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn parse(
    relative_path: &str,
    token: &str,
    content: &str,
    schema: &Schema,
    patterns: &Patterns,
) -> ParsedMilestone {
    let lines = strip_generated_blocks(&numbered(content));
    let mut diagnostics = Vec::new();

    let mut cursor = 0usize;
    while cursor < lines.len() && lines[cursor].1.trim().is_empty() {
        cursor += 1;
    }
    let mut title = String::new();
    if let Some((number, text)) = lines.get(cursor) {
        if let Some(rest) = text.strip_prefix("# ") {
            title = rest
                .split_once(" — ")
                .map(|(_, right)| right.trim().to_string())
                .unwrap_or_else(|| rest.trim().to_string());
            cursor += 1;
        } else {
            diagnostics.push(Diagnostic::error(
                relative_path,
                *number,
                code::MALFORMED_BLOCK,
                "milestone file must start with `# <TOKEN> — <Title>`",
                "write `# V0 — Execution model proof`",
            ));
        }
    }
    while cursor < lines.len() && lines[cursor].1.trim().is_empty() {
        cursor += 1;
    }
    let (fields, _) = parse_field_block(&lines, cursor, patterns);
    for field in &fields.items {
        if !schema.milestone_file.field_order.contains(&field.key) {
            diagnostics.push(Diagnostic::error(
                relative_path,
                field.line,
                code::UNKNOWN_FIELD,
                format!("unknown milestone field `{}`", field.key),
                format!(
                    "allowed fields: {}",
                    schema.milestone_file.field_order.join(", ")
                ),
            ));
        }
    }

    let sections: Vec<String> = lines
        .iter()
        .filter(|(_, text)| text.starts_with("## "))
        .map(|(_, text)| text.trim_start_matches("## ").trim().to_string())
        .collect();

    let gates = parse_blocks(
        relative_path,
        &lines,
        "## Gates",
        patterns,
        true,
        &mut diagnostics,
    );
    let demos = parse_blocks(
        relative_path,
        &lines,
        "## Demos",
        patterns,
        false,
        &mut diagnostics,
    );

    let gate_blocks = gates
        .into_iter()
        .map(|block| Gate {
            id: block.id,
            title: block.title,
            fields: block.fields,
            prose: block.prose,
            line: block.line,
        })
        .collect();
    let demo_blocks = demos
        .into_iter()
        .map(|block| Demo {
            id: block.id,
            title: block.title,
            fields: block.fields,
            prose: block.prose,
            line: block.line,
        })
        .collect();

    ParsedMilestone {
        milestone: Milestone {
            token: token.to_string(),
            title,
            file: relative_path.to_string(),
            fields,
            gates: gate_blocks,
            demos: demo_blocks,
            sections,
            line: 1,
        },
        diagnostics,
    }
}

struct Block {
    id: String,
    title: String,
    fields: crate::model::Fields,
    prose: Vec<String>,
    line: usize,
}

fn parse_blocks(
    relative_path: &str,
    lines: &[NumberedLine],
    heading: &str,
    patterns: &Patterns,
    is_gate: bool,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<Block> {
    let Some(start) = lines.iter().position(|(_, text)| text.trim() == heading) else {
        return Vec::new();
    };
    let end = lines
        .iter()
        .skip(start + 1)
        .position(|(_, text)| text.starts_with("## "))
        .map(|offset| start + 1 + offset)
        .unwrap_or(lines.len());
    let region = &lines[start + 1..end];
    let block_starts: Vec<usize> = (0..region.len())
        .filter(|index| region[*index].1.starts_with("### "))
        .collect();
    let mut blocks = Vec::new();
    for (position, block_start) in block_starts.iter().enumerate() {
        let block_end = block_starts
            .get(position + 1)
            .copied()
            .unwrap_or(region.len());
        let block = &region[*block_start..block_end];
        let (line_number, heading_text) = &block[0];
        let pattern = if is_gate {
            &patterns.gate_heading
        } else {
            &patterns.demo_heading
        };
        let Some(captures) = pattern.captures(heading_text) else {
            diagnostics.push(Diagnostic::error(
                relative_path,
                *line_number,
                code::INVALID_ID_SYNTAX,
                format!("heading does not match the required shape: `{heading_text}`"),
                if is_gate {
                    "write `### V0-G01 · Title`"
                } else {
                    "write `### V0-D01 · Title`"
                },
            ));
            continue;
        };
        let (fields, next) = parse_field_block(block, 1, patterns);
        let prose = trim_blank_edges(&block[next..]);
        blocks.push(Block {
            id: captures[1].to_string(),
            title: captures[2].trim().to_string(),
            fields,
            prose: text_of(&prose),
            line: *line_number,
        });
    }
    blocks
}
