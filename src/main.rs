use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::process::Command;

/// Simple CLI tool to clean up merged git branches
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target branch (e.g. main or master). If not provided, tries to auto-detect.
    #[arg(short, long)]
    target: Option<String>,

    /// Dry-run mode
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Determine target branch (Auto-detect if not provided)
    let target = match args.target {
        Some(t) => t,
        None => detect_default_branch()?,
    };

    println!(
        "{} {} {}",
        "🔍 Searching for branches merged into".blue(),
        target.bold(),
        "..."
    );

    // 2. Git: Fetch list of merged branches
    let output = Command::new("git")
        .arg("branch")
        .arg("--merged")
        .arg(&target)
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        eprintln!("{}", "Error: Target branch not found or not a git repository.".red());
        return Ok(());
    }

    let output_str = String::from_utf8(output.stdout)?;

    // 3. Parsing and filtering
    let branches_to_clean: Vec<String> = output_str
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with('*')) // Ignore current
        .filter(|line| line != &target)        // Ignore target (main/master)
        .collect();

    if branches_to_clean.is_empty() {
        println!("{}", "✨ Clean! No merged branches to delete.".green());
        return Ok(());
    }

    // 4. Interactive selection (UI)
    println!("Found {} branches to delete:", branches_to_clean.len());

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Space to select/unselect, Enter to confirm")
        .items(&branches_to_clean)
        .defaults(&vec![true; branches_to_clean.len()])
        .interact()?;

    if selections.is_empty() {
        println!("Cancelled. No branches were deleted.");
        return Ok(());
    }

    // 5. Deletion process
    for index in selections {
        let branch_name = &branches_to_clean[index];

        if args.dry_run {
            println!("{} {}", "[Dry-Run] Would delete:".yellow(), branch_name);
        } else {
            delete_branch(branch_name)?;
        }
    }

    if !args.dry_run {
        println!("{}", "Done! 🧹".green().bold());
    }

    Ok(())
}

/// Helper to check if branch exists
fn branch_exists(name: &str) -> bool {
    Command::new("git")
        .args(["rev-parse", "--verify", name])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Logic to find main or master
fn detect_default_branch() -> Result<String> {
    if branch_exists("main") {
        return Ok("main".to_string());
    }
    if branch_exists("master") {
        return Ok("master".to_string());
    }
    // Fallback if neither exists (unlikely but possible)
    Ok("main".to_string())
}

fn delete_branch(branch_name: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(branch_name)
        .status()?;

    if status.success() {
        println!("{} {}", "🗑️  Deleted:".green(), branch_name);
    } else {
        eprintln!("{} {}", "❌ Error deleting:".red(), branch_name);
    }
    Ok(())
}