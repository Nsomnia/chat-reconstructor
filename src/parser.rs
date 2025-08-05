use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use regex::Regex;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileOperation {
    pub path: PathBuf,
    pub content: String,
}

// Splits the transcript into a Vec of assistant responses.
pub fn extract_assistant_turns(transcript: &str) -> Vec<String> {
    transcript
        .split("\n### ASSISTANT\n")
        .skip(1) // The first element will be the initial USER prompt
        .map(|s| s.split("\n### USER\n").next().unwrap_or("").trim().to_string())
        .collect()
}

// The core parsing logic for a single assistant turn.
pub fn parse_turn_for_ops(content: &str) -> Vec<FileOperation> {
    let mut operations = Vec::new();
    let parser = Parser::new(content);

    let mut current_prose = String::new();
    let mut in_code_block = false;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                current_prose.clear(); // Reset context after a block is processed
            }
            Event::Text(text) => {
                if in_code_block {
                    if let Some(path) = identify_path_from_context(&current_prose) {
                        operations.push(FileOperation {
                            path,
                            content: text.to_string(),
                        });
                    }
                } else {
                    current_prose.push_str(&text);
                }
            }
            _ => {
                // We only care about text and code blocks for now
            }
        }
    }
    operations
}

// Finds a filename in the text preceding a code block.
fn identify_path_from_context(context: &str) -> Option<PathBuf> {
    // Priority 1: Look for a markdown header like `### filename.rs`
    // This is a strong indicator of a file name.
    let header_re = Regex::new(r"###\s+([\w\./\-_]+)\s*$").unwrap();
    if let Some(caps) = header_re.captures(context) {
        return Some(PathBuf::from(&caps[1]));
    }

    // Priority 2: Look for a path-like string in the prose, like `src/main.rs` or `Cargo.toml`.
    // This is a weaker indicator, so we take the last match to be the most likely candidate.
    let prose_re = Regex::new(r"[`]?((?:[\w\-_]+/)*[\w\-_]+\.(?:rs|toml|md|json|gitignore|zsh))[`]?").unwrap();
    if let Some(last_match) = prose_re.find_iter(context).last() {
        return Some(PathBuf::from(last_match.as_str().replace('`', "")));
    }
    
    None
}