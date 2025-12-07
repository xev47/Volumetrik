use actix_web::{web, HttpResponse, Responder};
use crate::models::{ScanRequest, ScanResponse, BrowseResponse};
use crate::scan::scan_path;
use crate::settings::{SharedSettings, Settings, save_settings};
use std::path::Path;
use rfd::FileDialog;
use std::env;
use sysinfo::Disks;

fn is_docker() -> bool {
    env::var("APP_ENV").unwrap_or_default() == "docker"
}

fn map_to_system_path(path: &str) -> String {
    if is_docker() {
        // Normalize path separators
        let path = path.replace('\\', "/");
        let clean_path = path.trim_start_matches('/');
        
        if clean_path.is_empty() {
            return "/host".to_string();
        }
        return format!("/host/{}", clean_path);
    }
    path.to_string()
}

fn map_to_ui_path(path: &str) -> String {
    if is_docker() {
        // Normalize path separators for comparison
        let path = path.replace('\\', "/");
        
        if path == "/host" || path == "/host/" {
            return "/".to_string();
        }
        if path.starts_with("/host/") {
            return path.replacen("/host", "", 1);
        }
        // Fallback: if path doesn't start with /host, it might be a relative path or error
        return path;
    }
    path.to_string()
}

pub async fn select_folder() -> impl Responder {
    // Check if running in Docker
    if is_docker() {
        return HttpResponse::Ok().json(serde_json::json!({ 
            "supported": false,
            "error": "File dialog is not available in Docker mode." 
        }));
    }

    // Run the dialog in a blocking task to avoid blocking the async runtime
    let task = web::block(|| {
        FileDialog::new().pick_folder()
    }).await;

    match task {
        Ok(Some(path)) => HttpResponse::Ok().json(serde_json::json!({ "path": path })),
        Ok(None) => HttpResponse::Ok().json(serde_json::json!({ "path": null })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn browse(info: web::Query<ScanRequest>) -> impl Responder {
    let mut ui_path = info.path.clone();
    
    // Default logic
    if ui_path.is_empty() {
        if is_docker() {
            ui_path = "/".to_string();
        } else {
            #[cfg(target_os = "windows")]
            { ui_path = "C:\\".to_string(); }
            #[cfg(not(target_os = "windows"))]
            { ui_path = "/".to_string(); }
        }
    }

    let system_path_str = map_to_system_path(&ui_path);
    println!("Browsing UI path: '{}' -> System path: '{}'", ui_path, system_path_str);
    
    let path = Path::new(&system_path_str);
    let mut directories = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        directories.push(name.to_string());
                    }
                }
            }
        }
    }
    
    directories.sort_by_key(|name| name.to_lowercase());

    let parent = path.parent().map(|p| {
        let s = p.to_string_lossy().to_string();
        // If we are at /host (system path), parent should be None for UI
        if is_docker() && s == "/host" {
             return "".to_string(); // Will be handled as None or empty
        }
        // Map back to UI path
        map_to_ui_path(&s)
    });
    
    // Filter out empty parent if it results in empty string and we are not at root
    let parent = match parent {
        Some(p) if p.is_empty() && ui_path != "/" => Some("/".to_string()), // Parent of /Users is /
        Some(p) if p.is_empty() => None, // Parent of / is None
        Some(p) => Some(p),
        None => None,
    };

    // Special case for Docker root parent
    let parent = if is_docker() && ui_path == "/" {
        None
    } else {
        parent
    };

    HttpResponse::Ok().json(BrowseResponse {
        parent,
        current: ui_path,
        directories,
    })
}

pub async fn scan(info: web::Query<ScanRequest>) -> impl Responder {
    let ui_path = &info.path;
    
    // Basic security check (very minimal)
    if ui_path.contains("..") {
        return HttpResponse::BadRequest().json("Invalid path");
    }

    let mut system_path = map_to_system_path(ui_path);
    
    // If not in Docker, resolve relative paths to absolute to ensure parent navigation works
    if !is_docker() {
        if let Ok(p) = std::fs::canonicalize(&system_path) {
            let s = p.to_string_lossy().to_string();
            #[cfg(target_os = "windows")]
            let s = s.trim_start_matches(r"\\?\").to_string();
            #[cfg(not(target_os = "windows"))]
            let s = s; // No-op
            system_path = s;
        }
    }

    println!("Scanning UI path: '{}' -> System path: '{}' (Docker mode: {})", ui_path, system_path, is_docker());

    match scan_path(&system_path) {
        Ok((files, total_size, total_files)) => {
            // Sort by size descending by default
            let mut sorted_files = files;
            sorted_files.sort_by(|a, b| b.size.cmp(&a.size));

            // Map paths back to UI paths
            for file in &mut sorted_files {
                file.path = map_to_ui_path(&file.path);
            }

            let parent = Path::new(&system_path)
                .parent()
                .map(|p| {
                    let s = p.to_string_lossy().to_string();
                    map_to_ui_path(&s)
                });
            
            // Filter out empty parent if it results in empty string and we are not at root
            let parent = match parent {
                Some(p) if p.is_empty() && ui_path != "/" => Some("/".to_string()), // Parent of /Users is /
                Some(p) if p.is_empty() => None, // Parent of / is None
                Some(p) => Some(p),
                None => None,
            };

            // Fix parent for root
            let parent = if is_docker() && ui_path == "/" {
                None
            } else {
                parent
            };

            // Get Disk Info
            let disks = Disks::new_with_refreshed_list();
            let mut disk_total = None;
            let mut disk_available = None;

            // Find the disk that contains the scanned path
            // This is a simple heuristic: find the mount point that is a prefix of the path
            // and has the longest length (most specific match)
            let mut best_match_len = 0;
            
            for disk in &disks {
                let mount_point = disk.mount_point().to_string_lossy();
                // In Docker, we might be scanning /host/..., but disks might show / or /etc/hosts etc.
                // If we are in docker and scanning /host, we want the disk info for /host (which is the bind mount)
                // But sysinfo inside docker sees the container's disks.
                // If we are scanning /host, we are actually scanning the host's disk mounted at /host.
                // sysinfo inside container usually sees the overlayfs or the bind mount.
                
                // Simple check: if path starts with mount point
                if system_path.starts_with(mount_point.as_ref()) {
                    if mount_point.len() > best_match_len {
                        best_match_len = mount_point.len();
                        disk_total = Some(disk.total_space());
                        disk_available = Some(disk.available_space());
                    }
                }
            }

            // Fallback: if no match found (e.g. Windows paths vs sysinfo), try to just get the first disk or root
            if disk_total.is_none() {
                 if let Some(disk) = disks.iter().next() {
                     disk_total = Some(disk.total_space());
                     disk_available = Some(disk.available_space());
                 }
            }

            HttpResponse::Ok().json(ScanResponse {
                parent,
                current: ui_path.clone(),
                files: sorted_files,
                total_size,
                total_files,
                disk_total,
                disk_available,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("Error scanning path: {}", e)),
    }
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Volumetrik is running")
}

pub async fn get_settings(data: web::Data<SharedSettings>) -> impl Responder {
    let settings = data.lock().unwrap();
    HttpResponse::Ok().json(settings.clone())
}

pub async fn update_settings(
    data: web::Data<SharedSettings>,
    new_settings: web::Json<Settings>,
) -> impl Responder {
    let mut settings = data.lock().unwrap();
    *settings = new_settings.into_inner();
    
    if let Err(e) = save_settings(&settings) {
        return HttpResponse::InternalServerError().body(format!("Failed to save settings: {}", e));
    }

    HttpResponse::Ok().json(settings.clone())
}

pub async fn update_layout(
    data: web::Data<SharedSettings>,
    layout: web::Json<serde_json::Value>,
) -> impl Responder {
    let mut settings = data.lock().unwrap();
    let layout_val = layout.into_inner();

    // Check if it's an empty array, which we treat as a reset request
    if let Some(arr) = layout_val.as_array() {
        if arr.is_empty() {
             settings.layout = Some(crate::settings::default_layout());
        } else {
             settings.layout = Some(layout_val);
        }
    } else {
        settings.layout = Some(layout_val);
    }
    
    if let Err(e) = save_settings(&settings) {
        return HttpResponse::InternalServerError().body(format!("Failed to save layout: {}", e));
    }

    HttpResponse::Ok().json(serde_json::json!({ "status": "success" }))
}
