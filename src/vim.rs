use crate::app::{App, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_event(app: &mut App, key: KeyEvent, terminal_height: usize) {
    match &app.mode {
        Mode::Normal => handle_normal_mode(app, key, terminal_height),
        Mode::Insert { .. } => handle_insert_mode(app, key),
        Mode::Search { .. } => handle_search_mode(app, key),
        Mode::Command { .. } => handle_command_mode(app, key),
        Mode::Help => handle_help_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut App, key: KeyEvent, terminal_height: usize) {
    match key.code {
        // Quit
        KeyCode::Char('q') => app.quit(),

        // Navigation
        KeyCode::Char('j') | KeyCode::Down => app.move_cursor_down(),
        KeyCode::Char('k') | KeyCode::Up => app.move_cursor_up(),

        // Jump to top/bottom
        KeyCode::Char('g') => app.move_to_top(),
        KeyCode::Char('G') => app.move_to_bottom(),

        // Page navigation
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let page_size = terminal_height / 2;
            app.page_down(page_size);
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let page_size = terminal_height / 2;
            app.page_up(page_size);
        }

        // Insert mode (comment)
        KeyCode::Char('o') | KeyCode::Char('O') => app.enter_insert_mode(),

        // Delete comment
        KeyCode::Char('d') => {
            // Wait for second 'd'
            // For simplicity, we'll handle dd in a single press
            app.delete_comment_at_cursor();
        }

        // Search
        KeyCode::Char('/') => app.enter_search_mode(),
        KeyCode::Char('n') => app.next_match(),
        KeyCode::Char('N') => app.prev_match(),

        // Command mode
        KeyCode::Char(':') => app.enter_command_mode(),

        // Help
        KeyCode::Char('?') => app.toggle_help(),

        _ => {}
    }
}

fn handle_insert_mode(app: &mut App, key: KeyEvent) {
    if let Mode::Insert { comment_text } = &mut app.mode {
        match key.code {
            KeyCode::Esc => app.exit_to_normal_mode(),
            KeyCode::Char(c) => comment_text.push(c),
            KeyCode::Backspace => {
                comment_text.pop();
            }
            KeyCode::Enter => comment_text.push('\n'),
            _ => {}
        }
    }
}

fn handle_search_mode(app: &mut App, key: KeyEvent) {
    if let Mode::Search { query } = &mut app.mode {
        match key.code {
            KeyCode::Esc => {
                app.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                let q = query.clone();
                app.mode = Mode::Normal;
                app.search(&q);
            }
            KeyCode::Char(c) => query.push(c),
            KeyCode::Backspace => {
                query.pop();
            }
            _ => {}
        }
    }
}

fn handle_help_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => {
            app.mode = Mode::Normal;
        }
        _ => {}
    }
}

fn handle_command_mode(app: &mut App, key: KeyEvent) {
    if let Mode::Command { command } = &mut app.mode {
        match key.code {
            KeyCode::Esc => {
                app.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                let cmd = command.clone();
                app.mode = Mode::Normal;
                app.execute_command(&cmd);
            }
            KeyCode::Char(c) => command.push(c),
            KeyCode::Backspace => {
                command.pop();
            }
            _ => {}
        }
    }
}
