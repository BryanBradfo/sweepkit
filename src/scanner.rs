use crate::languages::{self, DetectionPattern, LanguageCleaner};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FoundItem {
    pub path: PathBuf,
    pub ecosystem: String,
    pub icon: String,
    pub size: u64,
}

/// Calculate directory size recursively
fn calculate_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum()
}

/// Check if a sibling file exists (in the parent directory)
fn has_sibling_file(dir: &Path, filename: &str) -> bool {
    if let Some(parent) = dir.parent() {
        parent.join(filename).exists()
    } else {
        false
    }
}

/// Check if directory matches a pattern and return ecosystem info
fn check_pattern(
    dir_name: &str,
    full_path: &Path,
    pattern: &DetectionPattern,
    cleaner: &dyn LanguageCleaner,
) -> Option<(String, String)> {
    match pattern {
        DetectionPattern::DirectoryName(name) => {
            if dir_name == name {
                Some((cleaner.name().to_string(), cleaner.icon().to_string()))
            } else {
                None
            }
        }
        DetectionPattern::DirectoryWithSibling {
            dir_name: dn,
            sibling,
        } => {
            if dir_name == dn && has_sibling_file(full_path, sibling) {
                Some((cleaner.name().to_string(), cleaner.icon().to_string()))
            } else {
                None
            }
        }
        DetectionPattern::GlobPattern(glob_pattern) => {
            // Simple glob matching for patterns like "*.egg-info" or "cmake-build-*"
            if let Some(suffix) = glob_pattern.strip_prefix('*') {
                if dir_name.ends_with(suffix) {
                    return Some((cleaner.name().to_string(), cleaner.icon().to_string()));
                }
            } else if let Some(prefix) = glob_pattern.strip_suffix('*') {
                if dir_name.starts_with(prefix) {
                    return Some((cleaner.name().to_string(), cleaner.icon().to_string()));
                }
            }
            None
        }
    }
}

/// Scan directory recursively for dev dependencies using language cleaners
fn scan_with_cleaners(root: &Path, cleaners: &[Box<dyn LanguageCleaner>]) -> Vec<FoundItem> {
    let mut found_items = Vec::new();
    let mut found_paths = std::collections::HashSet::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
            // Try each cleaner's patterns
            for cleaner in cleaners {
                for pattern in cleaner.project_patterns() {
                    if let Some((ecosystem, icon)) =
                        check_pattern(dir_name, path, &pattern, &**cleaner)
                    {
                        // Check if this path is already inside a found item
                        let is_nested = found_paths
                            .iter()
                            .any(|found_path: &PathBuf| path.starts_with(found_path));

                        if !is_nested {
                            let size = calculate_dir_size(path);

                            if size > 0 {
                                found_items.push(FoundItem {
                                    path: path.to_path_buf(),
                                    ecosystem,
                                    icon,
                                    size,
                                });
                                found_paths.insert(path.to_path_buf());
                            }
                        }
                        break; // Found a match, no need to check other patterns
                    }
                }
            }
        }
    }

    // Sort by size descending
    found_items.sort_by_key(|item| std::cmp::Reverse(item.size));

    found_items
}

/// Scan directory recursively for all dev dependencies
pub fn scan_directory(root: &Path) -> Vec<FoundItem> {
    let cleaners = languages::get_all_cleaners();
    scan_with_cleaners(root, &cleaners)
}

/// Scan directory filtered by a specific language
pub fn scan_directory_filtered(root: &Path, language: &str) -> Vec<FoundItem> {
    match languages::get_cleaner_by_name(language) {
        Some(cleaner) => {
            let cleaners = vec![cleaner];
            scan_with_cleaners(root, &cleaners)
        }
        None => {
            let available: Vec<String> = languages::get_all_cleaners()
                .iter()
                .map(|c| c.name().to_string())
                .collect();
            eprintln!(
                "⚠️  Unknown language '{}'. Available: {}",
                language,
                available.join(", ")
            );
            Vec::new()
        }
    }
}
