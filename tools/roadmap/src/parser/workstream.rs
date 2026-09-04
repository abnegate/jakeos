use crate::diagnostic::{Diagnostic, code};
use crate::model::{Criterion, EvidenceLine, Fields, Task, VerificationLine, Workstream};
use crate::parser::{
    NumberedLine, numbered, parse_field_block, strip_generated_blocks, text_of, trim_blank_edges,
};
use crate::schema::{Patterns, SECTION_ORDER, Schema};

const HEADER_KEYS: [&str; 5] = ["Prefix", "Lead", "Baseline", "Baseline gap", "See also"];

pub struct ParsedWorkstream {
    pub workstream: Workstream,
    pub tasks: Vec<Task>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn parse(
    relative_path: &str,
    file_prefix: &str,
    content: &str,
    schema: &Schema,
    patterns: &Patterns,
    workstream_index: usize,
) -> ParsedWorkstream {
    let all_lines = numbered(content);
    let total_lines = all_lines.len();
    let lines = strip_generated_blocks(&all_lines);
    let mut diagnostics = Vec::new();

    let mut name = String::new();
    let mut heading_prefix = file_prefix.to_string();
    let mut cursor = 0usize;
    while cursor < lines.len() && lines[cursor].1.trim().is_empty() {
        cursor += 1;
    }
    if let Some((number, text)) = lines.get(cursor) {
        if let Some(rest) = text.strip_prefix("# ") {
            match rest.split_once(" · ") {
                Some((left, right)) => {
                    heading_prefix = left.trim().to_string();
                    name = right.trim().to_string();
                }
                None => {
                    name = rest.trim().to_string();
                    diagnostics.push(Diagnostic::error(
                        relative_path,
                        *number,
                        code::MALFORMED_BLOCK,
                        "workstream heading must read `# <PREFIX> · <Name>`",
                        "write `# KRN · Kernel fork and upstream tracking`",
                    ));
                }
            }
            cursor += 1;
        } else {
            diagnostics.push(Diagnostic::error(
                relative_path,
                *number,
                code::MALFORMED_BLOCK,
                "workstream file must start with a level-1 heading",
                "write `# <PREFIX> · <Name>` on the first line",
            ));
        }
    }

    while cursor < lines.len() && lines[cursor].1.trim().is_empty() {
        cursor += 1;
    }
    let (fields, _next) = parse_field_block(&lines, cursor, patterns);
    for field in &fields.items {
        if !HEADER_KEYS.contains(&field.key.as_str()) {
            diagnostics.push(Diagnostic::error(
                relative_path,
                field.line,
                code::UNKNOWN_FIELD,
                format!("unknown workstream header field `{}`", field.key),
                format!("allowed header fields: {}", HEADER_KEYS.join(", ")),
            ));
        }
    }
    if let Some(declared) = fields.value("Prefix")
        && declared != file_prefix
    {
        diagnostics.push(Diagnostic::error(
            relative_path,
            fields.line_of("Prefix", 1),
            code::PREFIX_FILE_MISMATCH,
            format!("header `Prefix: {declared}` does not match file name `{file_prefix}`"),
            format!("set `Prefix: {file_prefix}` or move the file"),
        ));
    }
    if heading_prefix != file_prefix {
        diagnostics.push(Diagnostic::error(
            relative_path,
            1,
            code::PREFIX_FILE_MISMATCH,
            format!("heading prefix `{heading_prefix}` does not match file name `{file_prefix}`"),
            format!("write `# {file_prefix} · <Name>`"),
        ));
    }
    if !schema.is_workstream(file_prefix) {
        diagnostics.push(Diagnostic::error(
            relative_path,
            1,
            code::UNKNOWN_PREFIX,
            format!("`{file_prefix}` is not a workstream prefix in fields.json"),
            "add the prefix to fields.json workstreams or rename the file",
        ));
    }
    if fields.contains("Baseline gap") && !schema.baseline_gap_allowed_for(file_prefix) {
        diagnostics.push(Diagnostic::error(
            relative_path,
            fields.line_of("Baseline gap", 1),
            code::BASELINE_GAP_FORBIDDEN,
            format!("`Baseline gap:` is not permitted for workstream `{file_prefix}`"),
            format!(
                "permitted prefixes: {}",
                schema.baseline_gap_allowed.join(", ")
            ),
        ));
    }

    let scope = collect_section(&lines, "## Scope");
    let out_of_scope = collect_section(&lines, "## Out of scope");

    let tasks_start = lines.iter().position(|(_, text)| text.trim() == "## Tasks");
    let mut tasks = Vec::new();
    if let Some(start) = tasks_start {
        let block_starts: Vec<usize> = (start + 1..lines.len())
            .filter(|index| lines[*index].1.starts_with("### "))
            .collect();
        for (position, block_start) in block_starts.iter().enumerate() {
            let end = block_starts
                .get(position + 1)
                .copied()
                .unwrap_or_else(|| next_boundary(&lines, *block_start + 1));
            let block = &lines[*block_start..end];
            if let Some(task) = parse_task(
                relative_path,
                file_prefix,
                block,
                patterns,
                workstream_index,
                &mut diagnostics,
            ) {
                tasks.push(task);
            }
        }
    }

    let workstream = Workstream {
        prefix: file_prefix.to_string(),
        name,
        file: relative_path.to_string(),
        fields,
        scope,
        out_of_scope,
        task_range: (0, 0),
        line_count: total_lines,
    };

    ParsedWorkstream {
        workstream,
        tasks,
        diagnostics,
    }
}

fn next_boundary(lines: &[NumberedLine], from: usize) -> usize {
    lines
        .iter()
        .enumerate()
        .skip(from)
        .find(|(_, (_, text))| {
            text.starts_with("### ") || (text.starts_with("## ") && !text.starts_with("### "))
        })
        .map(|(index, _)| index)
        .unwrap_or(lines.len())
}

fn collect_section(lines: &[NumberedLine], heading: &str) -> Vec<String> {
    let Some(start) = lines.iter().position(|(_, text)| text.trim() == heading) else {
        return Vec::new();
    };
    let mut collected = Vec::new();
    for (_, text) in lines.iter().skip(start + 1) {
        if text.starts_with("## ") {
            break;
        }
        collected.push(text.clone());
    }
    while collected.last().is_some_and(|line| line.trim().is_empty()) {
        collected.pop();
    }
    while collected.first().is_some_and(|line| line.trim().is_empty()) {
        collected.remove(0);
    }
    collected
}

fn parse_task(
    relative_path: &str,
    file_prefix: &str,
    block: &[NumberedLine],
    patterns: &Patterns,
    workstream_index: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<Task> {
    let (heading_line, heading_text) = &block[0];
    let Some(captures) = patterns.task_heading.captures(heading_text) else {
        diagnostics.push(Diagnostic::error(
            relative_path,
            *heading_line,
            code::INVALID_ID_SYNTAX,
            format!("task heading does not match the required shape: `{heading_text}`"),
            "write `### PREFIX-NNN · Imperative title`",
        ));
        return None;
    };
    let prefix = captures[1].to_string();
    let identifier = captures[2].to_string();
    let title = captures[3].trim().to_string();
    let (number, slug) = if let Some(slug) = identifier.strip_prefix('@') {
        (None, Some(slug.to_string()))
    } else {
        (identifier.parse::<u32>().ok(), None)
    };
    let id = format!("{prefix}-{identifier}");
    if prefix != file_prefix {
        diagnostics.push(Diagnostic::error(
            relative_path,
            *heading_line,
            code::PREFIX_FILE_MISMATCH,
            format!("task `{id}` carries prefix `{prefix}` in file for `{file_prefix}`"),
            format!("move the task to workstreams/{prefix}.md or renumber it"),
        ));
    }

    let (fields, mut cursor) = parse_field_block(block, 1, patterns);

    let section_starts: Vec<usize> = (cursor..block.len())
        .filter(|index| block[*index].1.starts_with("#### "))
        .collect();
    let description_end = section_starts.first().copied().unwrap_or(block.len());
    let description_lines = trim_blank_edges(&block[cursor..description_end]);
    cursor = description_end;

    let mut covers = Vec::new();
    for (_, text) in &description_lines {
        if let Some(captures) = patterns.covers_comment.captures(text.trim()) {
            covers.extend(
                captures[1]
                    .split(',')
                    .map(|token| token.trim().to_string())
                    .filter(|token| !token.is_empty()),
            );
        }
    }

    let mut out_of_scope = Vec::new();
    let mut criteria = Vec::new();
    let mut verification = Vec::new();
    let mut evidence = Vec::new();
    let mut present_sections = Vec::new();

    for (position, section_start) in section_starts.iter().enumerate() {
        let end = section_starts
            .get(position + 1)
            .copied()
            .unwrap_or(block.len());
        let (heading_number, heading) = &block[*section_start];
        let name = heading.trim_start_matches("#### ").trim().to_string();
        if !SECTION_ORDER.contains(&name.as_str()) {
            diagnostics.push(Diagnostic::error(
                relative_path,
                *heading_number,
                code::MALFORMED_BLOCK,
                format!("unknown task section `#### {name}`"),
                format!("allowed sections: {}", SECTION_ORDER.join(", ")),
            ));
            continue;
        }
        present_sections.push(name.clone());
        let body = trim_blank_edges(&block[section_start + 1..end]);
        match name.as_str() {
            "Out of scope" => out_of_scope = text_of(&body),
            "Acceptance criteria" => {
                for (number, text) in &body {
                    if text.trim().is_empty() {
                        continue;
                    }
                    if let Some(captures) = patterns.checkbox.captures(text) {
                        criteria.push(Criterion {
                            ticked: &captures[1] != " ",
                            text: captures[2].trim().to_string(),
                            line: *number,
                        });
                    } else {
                        diagnostics.push(Diagnostic::error(
                            relative_path,
                            *number,
                            code::INVALID_CHECKBOX,
                            format!("acceptance criterion is not a checkbox item: `{text}`"),
                            "write `- [ ] <observable statement>` or `- [x] …`",
                        ));
                    }
                }
            }
            "Verification" => {
                for (number, text) in &body {
                    if text.trim().is_empty() {
                        continue;
                    }
                    if let Some(captures) = patterns.verification_line.captures(text) {
                        verification.push(VerificationLine {
                            kind: captures[1].to_string(),
                            text: captures[2].trim().to_string(),
                            line: *number,
                        });
                    } else {
                        diagnostics.push(Diagnostic::error(
                            relative_path,
                            *number,
                            code::INVALID_VERIFICATION_KIND,
                            format!("verification line must read `- Kind: text`: `{text}`"),
                            "start the line with one of the allowed verification kinds",
                        ));
                    }
                }
            }
            "Evidence" => {
                for (number, text) in &body {
                    if text.trim().is_empty() {
                        continue;
                    }
                    match text.strip_prefix("- ") {
                        Some(value) => evidence.push(EvidenceLine {
                            text: value.trim().to_string(),
                            line: *number,
                        }),
                        None => diagnostics.push(Diagnostic::error(
                            relative_path,
                            *number,
                            code::MALFORMED_BLOCK,
                            format!("evidence entry must be a list item: `{text}`"),
                            "write `- none` while the task is not done",
                        )),
                    }
                }
            }
            _ => {}
        }
    }
    let _ = cursor;

    Some(Task {
        id,
        prefix,
        number,
        slug,
        title,
        fields,
        description: text_of(&description_lines),
        covers,
        out_of_scope,
        criteria,
        verification,
        evidence,
        present_sections,
        raw: text_of(block),
        file: relative_path.to_string(),
        workstream: workstream_index,
        line: *heading_line,
    })
}

pub fn empty_fields() -> Fields {
    Fields::default()
}
