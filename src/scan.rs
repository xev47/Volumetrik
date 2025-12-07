use crate::models::FileStats;
use rayon::prelude::*;
use std::fs::{self, Metadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Recursively calculates the size and file count of a directory.
pub fn calculate_size(path: &Path) -> (u64, u64) {
    let mut size = 0;
    let mut count = 0;

    if let Ok(entries) = fs::read_dir(path) {
        // We collect entries to a vector to parallelize if the directory is huge,
        // but for deep recursion, spawning too many tasks can be overhead.
        // A hybrid approach: sequential for small dirs, parallel for top levels?
        // For simplicity and robustness in deep recursion, let's stick to sequential
        // inside the deep recursion to avoid thread exhaustion, but parallelize the top-level scan.
        // Actually, rayon handles work-stealing well.
        
        // However, for a simple `calculate_size`, sequential is often faster due to syscall overhead
        // unless we are on a very slow filesystem.
        // Let's try a pure sequential recursive approach for the "deep" calculation
        // to ensure we don't explode the stack or thread pool, 
        // but we will parallelize the *immediate children* of the requested folder in `scan_path`.
        
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_symlink() {
                continue;
            }

            if path.is_dir() {
                let (sub_size, sub_count) = calculate_size(&path);
                size += sub_size;
                count += sub_count;
            } else {
                if let Ok(metadata) = entry.metadata() {
                    size += metadata.len();
                    count += 1;
                }
            }
        }
    }
    (size, count)
}

fn get_metadata_time(metadata: &Metadata) -> u64 {
    metadata
        .modified()
        .unwrap_or(SystemTime::UNIX_EPOCH)
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn scan_path(path_str: &str) -> Result<(Vec<FileStats>, u64, u64), std::io::Error> {
    let path = PathBuf::from(path_str);
    let entries = fs::read_dir(&path)?;

    // Collect entries first to parallelize
    let dir_entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();

    let results: Vec<FileStats> = dir_entries
        .par_iter()
        .map(|entry| {
            let entry_path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let metadata = entry.metadata().ok();
            
            let is_dir = entry_path.is_dir();
            let mut size = 0;
            let mut file_count = 0;
            let mut modified = 0;

            if let Some(meta) = metadata {
                modified = get_metadata_time(&meta);
                if is_dir {
                    // Heavy lifting here: calculate size of this subdirectory
                    let (s, c) = calculate_size(&entry_path);
                    size = s;
                    file_count = c;
                } else {
                    size = meta.len();
                    file_count = 1;
                }
            }

            FileStats {
                path: entry_path.to_string_lossy().to_string(),
                name,
                is_dir,
                size,
                file_count,
                modified,
            }
        })
        .collect();

    // Calculate totals for the current directory
    let total_size: u64 = results.iter().map(|f| f.size).sum();
    let total_files: u64 = results.iter().map(|f| f.file_count).sum();

    Ok((results, total_size, total_files))
}
