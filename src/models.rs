use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileStats {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub file_count: u64, // 0 for files, count of files inside for dirs
    pub modified: u64,   // Timestamp
}

#[derive(Debug, Clone, Default)]
pub struct ExtensionStats {
    pub size: u64,
    pub count: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ScanAnalysis {
    pub total_size: u64,
    #[allow(dead_code)]
    pub total_files: u64,
    #[allow(dead_code)]
    pub extension_distribution: HashMap<String, ExtensionStats>,
}
