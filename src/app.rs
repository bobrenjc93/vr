use crate::comment::{Comment, ContextLine};
use crate::diff::{DiffLine, LineType};

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert { comment_text: String },
    Search { query: String },
    Command { command: String },
    Help,
}

pub struct App {
    pub diff_lines: Vec<DiffLine>,
    pub comments: Vec<Comment>,
    pub cursor: usize,
    pub scroll_offset: usize,
    pub mode: Mode,
    pub should_quit: bool,
    pub search_matches: Vec<usize>,
    pub current_match: usize,
    pub output_file: String,
}

impl App {
    pub fn new(diff_lines: Vec<DiffLine>, comments: Vec<Comment>, output_file: String) -> Self {
        Self {
            diff_lines,
            comments,
            cursor: 0,
            scroll_offset: 0,
            mode: Mode::Normal,
            should_quit: false,
            search_matches: Vec::new(),
            current_match: 0,
            output_file,
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor < self.diff_lines.len().saturating_sub(1) {
            self.cursor += 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_to_top(&mut self) {
        self.cursor = 0;
        self.scroll_offset = 0;
    }

    pub fn move_to_bottom(&mut self) {
        self.cursor = self.diff_lines.len().saturating_sub(1);
    }

    pub fn page_down(&mut self, page_size: usize) {
        self.cursor = (self.cursor + page_size).min(self.diff_lines.len().saturating_sub(1));
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.cursor = self.cursor.saturating_sub(page_size);
    }

    pub fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert {
            comment_text: String::new(),
        };
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search {
            query: String::new(),
        };
    }

    pub fn enter_command_mode(&mut self) {
        self.mode = Mode::Command {
            command: String::new(),
        };
    }

    pub fn execute_command(&mut self, command: &str) {
        match command.trim() {
            "wq" | "x" => {
                self.save_all_comments();
                self.should_quit = true;
            }
            "q!" => {
                self.should_quit = true;
            }
            "q" => {
                self.save_all_comments();
                self.should_quit = true;
            }
            "w" => {
                self.save_all_comments();
            }
            _ => {}
        }
    }

    pub fn save_all_comments(&self) {
        let _ = crate::comment::save_comments_to_file(&self.comments, &self.output_file);
    }

    pub fn exit_to_normal_mode(&mut self) {
        // If we were in insert mode, save the comment
        if let Mode::Insert { comment_text } = &self.mode {
            if !comment_text.trim().is_empty() {
                self.save_comment(comment_text.clone());
            }
        }

        self.mode = Mode::Normal;
    }

    pub fn save_comment(&mut self, text: String) {
        if let Some(line) = self.diff_lines.get(self.cursor) {
            // Only allow comments on added or context lines (not removed lines)
            if let Some(line_number) = line.new_line_no {
                // Extract context: 3 lines before (not after)
                let context = self.extract_context_before(self.cursor, 3);
                let comment = Comment::new(line.file_path.clone(), line_number, text, context);
                self.comments.push(comment);
            }
        }
    }

    /// Extract context lines before the given index (not after)
    fn extract_context_before(&self, index: usize, context_size: usize) -> Vec<ContextLine> {
        let mut context = Vec::new();

        // Get context lines before (including the current line)
        let start = index.saturating_sub(context_size);
        let end = index + 1; // Include current line

        for i in start..end {
            if let Some(diff_line) = self.diff_lines.get(i) {
                // Skip file headers and hunk headers
                if matches!(
                    diff_line.line_type,
                    LineType::FileHeader | LineType::HunkHeader
                ) {
                    continue;
                }

                // The content already includes the prefix (+, -, or space)
                // so we just use it directly
                context.push(ContextLine {
                    content: diff_line.content.clone(),
                    prefix: String::new(), // Not used since content already has prefix
                });
            }
        }

        context
    }

    pub fn delete_comment_at_cursor(&mut self) {
        if let Some(line) = self.diff_lines.get(self.cursor) {
            if let Some(line_number) = line.new_line_no {
                // Remove comment matching this file and line
                self.comments
                    .retain(|c| !(c.file_path == line.file_path && c.line_number == line_number));
            }
        }
    }

    pub fn search(&mut self, query: &str) {
        self.search_matches.clear();
        self.current_match = 0;

        if query.is_empty() {
            return;
        }

        for (idx, line) in self.diff_lines.iter().enumerate() {
            if line.content.to_lowercase().contains(&query.to_lowercase()) {
                self.search_matches.push(idx);
            }
        }

        // Jump to first match
        if !self.search_matches.is_empty() {
            self.cursor = self.search_matches[0];
        }
    }

    pub fn next_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }

        self.current_match = (self.current_match + 1) % self.search_matches.len();
        self.cursor = self.search_matches[self.current_match];
    }

    pub fn prev_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }

        self.current_match = if self.current_match == 0 {
            self.search_matches.len() - 1
        } else {
            self.current_match - 1
        };
        self.cursor = self.search_matches[self.current_match];
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn toggle_help(&mut self) {
        self.mode = if self.mode == Mode::Help {
            Mode::Normal
        } else {
            Mode::Help
        };
    }
}
