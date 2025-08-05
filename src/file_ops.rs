use crate::parser::FileOperation;
use diff_match_patch_rs::{DiffMatchPatch, Compat};
use std::fs;
use std::path::Path;
use colored::*;

pub fn synchronize_files_to_disk(base_dir: &Path, operations: &[FileOperation]) {
    for op in operations {
        let full_path = base_dir.join(&op.path);
        let parent_dir = full_path.parent().unwrap();
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).unwrap();
        }

        let is_a_patch = is_patch(&op.path, &op.content);

        if !full_path.exists() {
            if is_a_patch {
                println!("    {} {} Can't apply patch to non-existent file: {}", "⚠️".yellow(), "[SKIPPING]".yellow().bold(), op.path.display());
                continue;
            }
            println!("    {} {} New file materialized: {}", "✨".cyan(), "[CREATING]".cyan().bold(), op.path.display().to_string().bold());
            fs::write(&full_path, &op.content).unwrap();
        } else if is_a_patch {
            println!("    {} {} Applying intelligent update to {}", "🔧".yellow(), "[PATCHING]".yellow().bold(), op.path.display().to_string().bold());
            let current_content = fs::read_to_string(&full_path).unwrap_or_default();
            let new_content = if op.path.ends_with("Cargo.toml") {
                apply_toml_patch(&current_content, &op.content)
            } else {
                apply_generic_patch(&current_content, &op.content)
            };
            fs::write(&full_path, new_content).unwrap();
        } else {
            println!("    {} {} Overwriting stale file: {}", "🔄".blue(), "[UPDATING]".blue().bold(), op.path.display().to_string().bold());
            fs::write(&full_path, &op.content).unwrap();
        }
    }
}

fn is_patch(path: &Path, content: &str) -> bool {
    if content.contains("\n...\n") { return true; }
    if path.ends_with("Cargo.toml") && !content.contains("[package]") {
        println!("      -> {} Cargo.toml without [package] section detected as a patch.", "HEURISTIC".cyan());
        return true;
    }
    false
}

fn apply_toml_patch(current: &str, patch: &str) -> String {
    let mut current_toml: toml::Value = toml::from_str(current).unwrap_or_else(|_| toml::Value::Table(Default::default()));
    let patch_toml: toml::Value = toml::from_str(patch).unwrap_or_else(|_| toml::Value::Table(Default::default()));

    if let (Some(current_table), Some(patch_table)) = (current_toml.as_table_mut(), patch_toml.as_table()) {
        for (section_key, patch_section_value) in patch_table {
            println!("        -> Merging section: {}", section_key.green());
            if let Some(current_section) = current_table.get_mut(section_key) {
                if let (Some(current_items), Some(patch_items)) = (current_section.as_table_mut(), patch_section_value.as_table()) {
                     current_items.extend(patch_items.clone());
                }
            } else {
                 current_table.insert(section_key.clone(), patch_section_value.clone());
            }
        }
    }
    toml::to_string_pretty(&current_toml).unwrap()
}

// Applies a patch to a file.
// It first tries to parse the patch as a standard diff format.
// If that fails, it falls back to a simple snippet replacement.
fn apply_generic_patch(current: &str, patch: &str) -> String {
    let dmp = DiffMatchPatch::new();
    let clean_snippet = patch.replace("...", "");

    // The diff-match-patch-rs crate expects a patch format that can be parsed.
    // We'll try to parse the snippet as a patch first.
    match dmp.patch_from_text::<Compat>(&clean_snippet) {
        Ok(patches) => {
            // If the patch is successfully parsed, we apply it.
            match dmp.patch_apply(&patches, current) {
                Ok((new_text, results)) => {
                    if results.iter().all(|r| *r) {
                        println!("      -> {} Fuzzy matched and merged changes.", "[PATCH APPLIED]".green());
                        return new_text;
                    } else {
                        println!("      -> {} Could not apply patch. File left unmodified.", "[PATCH FAILED]".yellow());
                        return current.to_string();
                    }
                }
                Err(_) => {
                    println!("      -> {} Error applying patch. File left unmodified.", "[PATCH FAILED]".yellow());
                    return current.to_string();
                }
            }
        }
        Err(_) => {
            // If parsing fails, we assume it's a raw code snippet with "..." as a separator.
            // This is a fallback and might not be robust, but it's better than nothing.
            println!("      -> {} Could not parse patch. Trying to apply as a snippet.", "[INFO]".yellow());
            let parts: Vec<&str> = patch.split("...").collect();
            if parts.len() == 2 {
                // We assume the first part is the prefix and the second part is the suffix.
                if let (Some(start), Some(end)) = (current.find(parts[0]), current.rfind(parts[1])) {
                    let mut new_content = String::new();
                    new_content.push_str(&current[..start]);
                    new_content.push_str(&clean_snippet);
                    new_content.push_str(&current[end + parts[1].len()..]);
                    println!("      -> {} Applied patch as a snippet.", "[SNIPPET APPLIED]".green());
                    return new_content;
                }
            }
            println!("      -> {} Could not apply patch as a snippet. File left unmodified.", "[SNIPPET FAILED]".yellow());
            current.to_string()
        }
    }
}