mod app;
mod comment;
mod diff;
mod tui;
mod vcs;
mod vim;

use anyhow::Result;
use std::env;
use uuid::Uuid;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Auto-detect VCS type (git or mercurial)
    let vcs_type = vcs::VcsType::detect()?;

    // Parse VCS ref argument (default to uncommitted changes)
    let vcs_ref = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    // Get diff from VCS
    let diff_text = vcs_type.get_diff(vcs_ref.as_deref())?;

    if diff_text.is_empty() {
        println!("No changes to review");
        return Ok(());
    }

    // Parse diff into structured format
    let diff_lines = diff::parse_diff(&diff_text)?;

    if diff_lines.is_empty() {
        println!("No changes to review");
        return Ok(());
    }

    // Generate UUID for output file
    let uuid = Uuid::new_v4();
    let output_file = format!("/tmp/vr/{}.txt", uuid);

    // Load existing comments (empty for new sessions)
    let comments = Vec::new();

    // Create app state
    let mut app = app::App::new(diff_lines, comments, output_file.clone());

    // Run TUI
    tui::run(&mut app)?;

    // Print output file path if comments were added
    if !app.comments.is_empty() {
        println!("\nComments saved to: {}", output_file);
    }

    Ok(())
}
