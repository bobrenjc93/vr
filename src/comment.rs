use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub file_path: String,
    pub line_number: usize,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommentStorage {
    comments: Vec<Comment>,
}

const COMMENT_DIR: &str = ".vim-review";
const COMMENT_FILE: &str = ".vim-review/comments.json";

/// Load comments from disk
pub fn load_comments() -> Result<Vec<Comment>> {
    if !Path::new(COMMENT_FILE).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(COMMENT_FILE)?;
    let storage: CommentStorage = serde_json::from_str(&content)?;
    Ok(storage.comments)
}

/// Save comments to disk
pub fn save_comments(comments: &[Comment]) -> Result<()> {
    // Create directory if it doesn't exist
    fs::create_dir_all(COMMENT_DIR)?;

    let storage = CommentStorage {
        comments: comments.to_vec(),
    };

    let json = serde_json::to_string_pretty(&storage)?;
    fs::write(COMMENT_FILE, json)?;

    Ok(())
}

/// Save comments to a text file
pub fn save_comments_to_file(comments: &[Comment], file_path: &str) -> Result<()> {
    use std::io::Write;

    // Create parent directory if it doesn't exist
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut output = String::new();

    for comment in comments {
        output.push_str(&format!(
            "{}:{}\n{}\n\n",
            comment.file_path, comment.line_number, comment.text
        ));
    }

    let mut file = fs::File::create(file_path)?;
    file.write_all(output.as_bytes())?;

    Ok(())
}

impl Comment {
    pub fn new(file_path: String, line_number: usize, text: String) -> Self {
        Self {
            file_path,
            line_number,
            text,
            timestamp: Utc::now(),
        }
    }
}
