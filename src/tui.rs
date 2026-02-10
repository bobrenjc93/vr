use crate::app::{App, Mode};
use crate::diff::LineType;
use crate::vim;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

pub fn run(app: &mut App) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    loop {
        terminal.draw(|f| {
            let size = f.area();

            // Create layout with main area and status bar
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(size);

            // Show help screen if in help mode
            if app.mode == Mode::Help {
                render_help(f, chunks[0]);
                let status = Paragraph::new(" Press '?' or ESC to close help")
                    .style(Style::default().bg(Color::Blue).fg(Color::White));
                f.render_widget(status, chunks[1]);
                return;
            }

            // Adjust scroll offset to keep cursor in view
            let visible_height = chunks[0].height as usize;
            if app.cursor >= app.scroll_offset + visible_height {
                app.scroll_offset = app.cursor - visible_height + 1;
            } else if app.cursor < app.scroll_offset {
                app.scroll_offset = app.cursor;
            }

            // Render diff lines
            let mut lines = Vec::new();
            let visible_end = (app.scroll_offset + visible_height).min(app.diff_lines.len());

            for i in app.scroll_offset..visible_end {
                let diff_line = &app.diff_lines[i];

                // Check if there's a comment for this line
                let has_comment = if let Some(line_no) = diff_line.new_line_no {
                    app.comments.iter().any(|c| {
                        c.file_path == diff_line.file_path && c.line_number == line_no
                    })
                } else {
                    false
                };

                // Style based on line type
                let style = match diff_line.line_type {
                    LineType::Added => Style::default().fg(Color::Green),
                    LineType::Removed => Style::default().fg(Color::Red),
                    LineType::Context => Style::default().fg(Color::White),
                    LineType::FileHeader => Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                    LineType::HunkHeader => Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                };

                // Highlight cursor line
                let style = if i == app.cursor {
                    style.bg(Color::DarkGray)
                } else if has_comment {
                    style.bg(Color::Yellow).fg(Color::Black)
                } else {
                    style
                };

                lines.push(Line::from(Span::styled(
                    diff_line.content.clone(),
                    style,
                )));

                // Show comment being typed if in insert mode at this line
                if i == app.cursor {
                    if let Mode::Insert { comment_text } = &app.mode {
                        // Split multi-line comments and display each line
                        for (idx, line) in comment_text.lines().enumerate() {
                            let display_line = if idx == comment_text.lines().count() - 1
                                && !comment_text.ends_with('\n') {
                                format!("  💬 {}_", line)
                            } else {
                                format!("  💬 {}", line)
                            };
                            lines.push(Line::from(Span::styled(
                                display_line,
                                Style::default()
                                    .fg(Color::Black)
                                    .bg(Color::Yellow),
                            )));
                        }
                        // If text ends with newline, show cursor on new line
                        if comment_text.ends_with('\n') {
                            lines.push(Line::from(Span::styled(
                                "  💬 _",
                                Style::default()
                                    .fg(Color::Black)
                                    .bg(Color::Yellow),
                            )));
                        }
                        // Show empty line with cursor if no text yet
                        if comment_text.is_empty() {
                            lines.push(Line::from(Span::styled(
                                "  💬 _",
                                Style::default()
                                    .fg(Color::Black)
                                    .bg(Color::Yellow),
                            )));
                        }
                    }
                }

                // Show comment text if present
                if has_comment {
                    if let Some(line_no) = diff_line.new_line_no {
                        for comment in app.comments.iter() {
                            if comment.file_path == diff_line.file_path
                                && comment.line_number == line_no {
                                // Split multi-line comments and display each line
                                for line in comment.text.lines() {
                                    lines.push(Line::from(Span::styled(
                                        format!("  💬 {}", line),
                                        Style::default()
                                            .fg(Color::Black)
                                            .bg(Color::Yellow),
                                    )));
                                }
                            }
                        }
                    }
                }
            }

            let paragraph = Paragraph::new(lines)
                .block(Block::default().borders(Borders::NONE));
            f.render_widget(paragraph, chunks[0]);

            // Render status bar
            let status_text = match &app.mode {
                Mode::Normal => {
                    let current_file = if app.cursor < app.diff_lines.len() {
                        &app.diff_lines[app.cursor].file_path
                    } else {
                        ""
                    };
                    format!(
                        " {} | Line {}/{} | Press ':wq' to save & quit, 'o' to comment, '?' for help",
                        current_file,
                        app.cursor + 1,
                        app.diff_lines.len()
                    )
                }
                Mode::Insert { comment_text } => {
                    let lines_count = comment_text.lines().count();
                    if lines_count <= 1 {
                        format!(" -- INSERT -- Press ESC to save, Enter for new line")
                    } else {
                        format!(" -- INSERT -- {} lines | Press ESC to save, Enter for new line", lines_count)
                    }
                }
                Mode::Search { query } => {
                    format!(" /{}", query)
                }
                Mode::Command { command } => {
                    format!(" :{}", command)
                }
                Mode::Help => {
                    " Press '?' or ESC to close help".to_string()
                }
            };

            let status = Paragraph::new(status_text)
                .style(Style::default().bg(Color::Blue).fg(Color::White));
            f.render_widget(status, chunks[1]);
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Handle quit in any mode
                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                    app.quit();
                }

                let terminal_height = terminal.size()?.height as usize;
                vim::handle_key_event(app, key, terminal_height);
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn render_help(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    let help_text = vec![
        Line::from(Span::styled(
            "Vim Review - Keyboard Shortcuts",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Navigation:", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  j / ↓         Move cursor down"),
        Line::from("  k / ↑         Move cursor up"),
        Line::from("  g             Jump to top"),
        Line::from("  G             Jump to bottom"),
        Line::from("  Ctrl+d        Page down"),
        Line::from("  Ctrl+u        Page up"),
        Line::from(""),
        Line::from(Span::styled("Comments:", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  o / O         Add comment at current line"),
        Line::from("  Enter         Add new line (in insert mode)"),
        Line::from("  ESC           Save comment and exit insert mode"),
        Line::from("  d             Delete comment at current line"),
        Line::from(""),
        Line::from(Span::styled("Search:", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  /             Start search"),
        Line::from("  n             Next match"),
        Line::from("  N             Previous match"),
        Line::from(""),
        Line::from(Span::styled("Other:", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  ?             Toggle this help screen"),
        Line::from("  :wq           Save comments and quit"),
        Line::from("  :w            Save comments"),
        Line::from("  :q            Save and quit"),
        Line::from("  :q!           Quit without saving"),
        Line::from("  Ctrl+c        Force quit"),
        Line::from(""),
        Line::from(Span::styled(
            "Comments are saved to /tmp/vr/{uuid}.txt",
            Style::default().fg(Color::Yellow),
        )),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(paragraph, area);
}
