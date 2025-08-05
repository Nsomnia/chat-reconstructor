use clap::Parser;
use colored::*;
use std::fs;
use std::path::PathBuf;

mod file_ops;
mod parser;

/// A Chad-Tier tool to reconstruct a project from an LLM's .txt chat export.
#[derive(Parser, Debug)]
#[command(version, about, long_about = "This tool reads a chat log and applies the code changes sequentially, like a true agent.", author = "Built by a Gemini, inspired by a Chad.")]
struct Args {
    /// Path to the glorious, multi-line .txt chat export.
    #[arg(required = true)]
    text_file: PathBuf,

    /// Output directory for the reconstructed project.
    #[arg(short, long, default_value = "reconstructed_project")]
    output_dir: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("\n{} {}", "⚙️".bold(), "Target directory set. Let's get this bread.".bold());

    let transcript = match fs::read_to_string(&args.text_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} Error reading file '{}': {}", "💥".red(), args.text_file.display(), e);
            return;
        }
    };

    if !args.output_dir.exists() {
        fs::create_dir_all(&args.output_dir).expect("Failed to create output directory");
    }

    println!("{}", "--- Commencing Agentic Synchronization ---".purple().bold());

    let assistant_turns = parser::extract_assistant_turns(&transcript);
    if assistant_turns.is_empty() {
        println!("{}", "No assistant messages found. Was it all just talk?".yellow());
        return;
    }

    for (i, turn_content) in assistant_turns.iter().enumerate() {
        println!("\n{}[Turn {}]{} {}", "[".cyan(), i + 1, "]".cyan(), "Processing assistant output...".cyan());
        let operations = parser::parse_turn_for_ops(turn_content);
        if operations.is_empty() {
            println!("  -> {}", "No file operations found in this turn. Probably just chit-chat.".italic());
        } else {
            println!("  -> Found {} file operations in this turn.", operations.len());
            file_ops::synchronize_files_to_disk(&args.output_dir, &operations);
        }
    }

    println!("\n{} {}", "🎉".green().bold(), "Project synchronization complete. Go build something legendary.".green().bold());
}