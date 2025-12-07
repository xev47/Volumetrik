use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileStats {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub file_count: u64, // 0 for files, count of files inside for dirs
    pub modified: u64,   // Timestamp
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanRequest {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResponse {
    pub parent: Option<String>,
    pub current: String,
    pub files: Vec<FileStats>,
    pub total_size: u64,
    pub total_files: u64,
    pub disk_total: Option<u64>,
    pub disk_available: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrowseResponse {
    pub parent: Option<String>,
    pub current: String,
    pub directories: Vec<String>,
}
