use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LineType {
    Added,
    Removed,
    Context,
    FileHeader,
    HunkHeader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub content: String,
    pub line_type: LineType,
    pub old_line_no: Option<usize>,
    pub new_line_no: Option<usize>,
    pub file_path: String,
}

/// Parse unified diff format into structured DiffLine objects
pub fn parse_diff(diff_text: &str) -> Result<Vec<DiffLine>> {
    let mut lines = Vec::new();
    let mut current_file = String::new();
    let mut old_line = 0;
    let mut new_line = 0;

    for line in diff_text.lines() {
        if line.starts_with("diff --git") {
            // Extract file path from "diff --git a/path b/path"
            if let Some(path) = line.split_whitespace().nth(2) {
                current_file = path.trim_start_matches("a/").to_string();
            }
            lines.push(DiffLine {
                content: line.to_string(),
                line_type: LineType::FileHeader,
                old_line_no: None,
                new_line_no: None,
                file_path: current_file.clone(),
            });
        } else if line.starts_with("---") || line.starts_with("+++") {
            // File header lines
            lines.push(DiffLine {
                content: line.to_string(),
                line_type: LineType::FileHeader,
                old_line_no: None,
                new_line_no: None,
                file_path: current_file.clone(),
            });
        } else if line.starts_with("@@") {
            // Hunk header: @@ -old_start,old_count +new_start,new_count @@
            if let Some(hunk_info) = parse_hunk_header(line) {
                old_line = hunk_info.0;
                new_line = hunk_info.1;
            }
            lines.push(DiffLine {
                content: line.to_string(),
                line_type: LineType::HunkHeader,
                old_line_no: None,
                new_line_no: None,
                file_path: current_file.clone(),
            });
        } else if line.starts_with('+') && !line.starts_with("+++") {
            // Added line
            lines.push(DiffLine {
                content: line.to_string(),
                line_type: LineType::Added,
                old_line_no: None,
                new_line_no: Some(new_line),
                file_path: current_file.clone(),
            });
            new_line += 1;
        } else if line.starts_with('-') && !line.starts_with("---") {
            // Removed line
            lines.push(DiffLine {
                content: line.to_string(),
                line_type: LineType::Removed,
                old_line_no: Some(old_line),
                new_line_no: None,
                file_path: current_file.clone(),
            });
            old_line += 1;
        } else if line.starts_with(' ') || (!line.starts_with("diff") && !line.is_empty()) {
            // Context line
            lines.push(DiffLine {
                content: line.to_string(),
                line_type: LineType::Context,
                old_line_no: Some(old_line),
                new_line_no: Some(new_line),
                file_path: current_file.clone(),
            });
            old_line += 1;
            new_line += 1;
        }
    }

    Ok(lines)
}

/// Parse hunk header to get starting line numbers
/// Returns (old_start, new_start)
fn parse_hunk_header(line: &str) -> Option<(usize, usize)> {
    // Parse "@@ -10,5 +12,6 @@" format
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }

    let old_part = parts[1].trim_start_matches('-');
    let new_part = parts[2].trim_start_matches('+');

    let old_start = old_part.split(',').next()?.parse::<usize>().ok()?;
    let new_start = new_part.split(',').next()?.parse::<usize>().ok()?;

    Some((old_start, new_start))
}
