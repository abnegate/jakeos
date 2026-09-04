use crate::diagnostic::{Diagnostic, code};
use crate::model::{Register, RegisterEntry, RepoAlias};
use crate::parser::{
    numbered, parse_field_block, strip_generated_blocks, text_of, trim_blank_edges,
};
use crate::schema::{Patterns, RegisterSchema};

pub struct ParsedRegister {
    pub register: Register,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn parse(
    relative_path: &str,
    family: &str,
    content: &str,
    register_schema: &RegisterSchema,
    patterns: &Patterns,
) -> ParsedRegister {
    let lines = strip_generated_blocks(&numbered(content));
    let mut diagnostics = Vec::new();
    let title = lines
        .iter()
        .find(|(_, text)| text.starts_with("# "))
        .map(|(_, text)| text.trim_start_matches("# ").trim().to_string())
        .unwrap_or_default();

    let block_starts: Vec<usize> = (0..lines.len())
        .filter(|index| lines[*index].1.starts_with("### "))
        .collect();
    let mut entries = Vec::new();
    for (position, block_start) in block_starts.iter().enumerate() {
        let end = block_starts
            .get(position + 1)
            .copied()
            .unwrap_or(lines.len());
        let block = &lines[*block_start..end];
        let (line_number, heading_text) = &block[0];
        let Some(captures) = patterns.register_heading.captures(heading_text) else {
            diagnostics.push(Diagnostic::error(
                relative_path,
                *line_number,
                code::INVALID_ID_SYNTAX,
                format!("register entry heading is malformed: `{heading_text}`"),
                format!("write `### {family}-001 · Title`"),
            ));
            continue;
        };
        let id = captures[1].to_string();
        if !patterns.matches_family(family, &id) {
            diagnostics.push(Diagnostic::error(
                relative_path,
                *line_number,
                code::REGISTER_ID_FAMILY,
                format!("`{id}` does not belong to the `{family}` register"),
                format!("entries in {relative_path} use the `{family}-NNN` id family"),
            ));
        }
        let (fields, next) = parse_field_block(block, 1, patterns);
        for field in &fields.items {
            if !register_schema.fields.contains(&field.key) {
                diagnostics.push(Diagnostic::error(
                    relative_path,
                    field.line,
                    code::REGISTER_UNKNOWN_FIELD,
                    format!("unknown field `{}` in register entry `{id}`", field.key),
                    format!("allowed fields: {}", register_schema.fields.join(", ")),
                ));
            }
        }
        let prose = trim_blank_edges(&block[next..]);
        entries.push(RegisterEntry {
            id,
            title: captures[2].trim().to_string(),
            fields,
            prose: text_of(&prose),
            line: *line_number,
        });
    }

    ParsedRegister {
        register: Register {
            family: family.to_string(),
            file: relative_path.to_string(),
            title,
            entries,
        },
        diagnostics,
    }
}

pub fn parse_aliases(content: &str, patterns: &Patterns) -> Vec<RepoAlias> {
    let lines = strip_generated_blocks(&numbered(content));
    let mut aliases = Vec::new();
    let mut current: Option<(String, usize)> = None;
    for (number, text) in &lines {
        if let Some(captures) = patterns.alias_heading.captures(text) {
            current = Some((captures[1].to_string(), *number));
            continue;
        }
        if let Some(url) = text.strip_prefix("- URL: ")
            && let Some((alias, line)) = current.take()
        {
            aliases.push(RepoAlias {
                alias,
                url: url.trim().to_string(),
                line,
            });
        }
    }
    aliases
}
