use crate::model::DraftRow;

pub struct SlugIndex {
    pub rows: Vec<DraftRow>,
    pub header: Vec<String>,
}

impl SlugIndex {
    pub fn get(&self, draft: &str) -> Option<&DraftRow> {
        self.rows.iter().find(|row| row.draft == draft)
    }
}

pub fn parse(content: &str) -> SlugIndex {
    let mut rows = Vec::new();
    let mut header = Vec::new();
    for (index, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns: Vec<&str> = line.split('\t').collect();
        if index == 0 && columns.first().map(|value| value.trim()) == Some("draft") {
            header = columns
                .iter()
                .map(|value| value.trim().to_string())
                .collect();
            continue;
        }
        let assigned_column = header.iter().position(|name| name == "assigned");
        rows.push(DraftRow {
            draft: columns.first().unwrap_or(&"").trim().to_string(),
            milestone: columns.get(1).unwrap_or(&"").trim().to_string(),
            title: columns.get(2).unwrap_or(&"").trim().to_string(),
            covers: columns
                .get(3)
                .unwrap_or(&"")
                .split(',')
                .map(|token| token.trim().to_string())
                .filter(|token| !token.is_empty())
                .collect(),
            assigned: assigned_column
                .and_then(|position| columns.get(position))
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
            line: index + 1,
        });
    }
    SlugIndex { rows, header }
}
