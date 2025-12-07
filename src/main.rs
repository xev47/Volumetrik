mod alerts;
mod models;
mod scan;
mod server;
mod settings;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use settings::{load_settings, SharedSettings, ThresholdType};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::Disks;
use tokio::time;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let is_docker = std::env::var("APP_ENV").unwrap_or_default() == "docker";
    println!("Starting Volumetrik server at http://localhost:{}", 8080);
    println!("Docker mode: {}", is_docker);

    // Load settings
    let settings = load_settings();
    let shared_settings: SharedSettings = Arc::new(Mutex::new(settings));

    // Spawn background monitoring task
    let monitor_settings = shared_settings.clone();
    tokio::spawn(async move {
        // Initial delay to let server start
        time::sleep(Duration::from_secs(5)).await;
        let mut disks = Disks::new_with_refreshed_list();
        
        loop {
            let (enabled, paths, check_interval, alert_config) = {
                let s = monitor_settings.lock().unwrap();
                (
                    s.monitoring.enabled,
                    s.monitoring.paths.clone(),
                    s.monitoring.check_interval_minutes,
                    s.alerts.clone(),
                )
            };

            if enabled {
                disks.refresh(true);
                for item in paths {
                    let path = &item.path;
                    let threshold = item.threshold_value;
                    
                    // We need to map UI path to system path if in docker
                    // This logic duplicates server.rs logic, ideally should be shared but for now it's fine
                    let system_path = if is_docker {
                        let p = path.replace('\\', "/");
                        let clean = p.trim_start_matches('/');
                        if clean.is_empty() { "/host".to_string() } else { format!("/host/{}", clean) }
                    } else {
                        path.clone()
                    };

                    println!("Monitoring: Checking {} ({:?})", system_path, item.threshold_type);
                    
                    match item.threshold_type {
                        ThresholdType::MaxUsed => {
                            match scan::scan_path(&system_path) {
                                Ok((_, total_size, _)) => {
                                    let size_gb = total_size as f64 / 1_073_741_824.0;
                                    if size_gb > threshold {
                                        let msg = if let Some(custom) = &alert_config.custom_message {
                                            custom.replace("{path}", path)
                                                  .replace("{threshold}", &threshold.to_string())
                                                  .replace("{current}", &format!("{:.2}", size_gb))
                                        } else {
                                            format!(
                                                "⚠️ Volumetrik Alert: Folder '{}' size is {:.2} GB, exceeding threshold of {:.2} GB.",
                                                path, size_gb, threshold
                                            )
                                        };
                                        
                                        println!("{}", msg);
                                        alerts::send_alert(&alert_config, &msg);
                                    }
                                }
                                Err(e) => println!("Monitoring error for {}: {}", path, e),
                            }
                        },
                        ThresholdType::MinRemaining => {
                            // Find the disk that contains this path
                            let path_obj = std::path::Path::new(&system_path);
                            let mut best_match_len = 0;
                            let mut available_space = None;

                            for disk in disks.list() {
                                if path_obj.starts_with(disk.mount_point()) {
                                    let len = disk.mount_point().as_os_str().len();
                                    if len >= best_match_len {
                                        best_match_len = len;
                                        available_space = Some(disk.available_space());
                                    }
                                }
                            }

                            if let Some(bytes) = available_space {
                                let free_gb = bytes as f64 / 1_073_741_824.0;
                                if free_gb < threshold {
                                     let msg = if let Some(custom) = &alert_config.custom_message {
                                            custom.replace("{path}", path)
                                                  .replace("{threshold}", &threshold.to_string())
                                                  .replace("{current}", &format!("{:.2}", free_gb))
                                        } else {
                                            format!(
                                                "⚠️ Volumetrik Alert: Volume for '{}' has {:.2} GB remaining, below threshold of {:.2} GB.",
                                                path, free_gb, threshold
                                            )
                                        };
                                    println!("{}", msg);
                                    alerts::send_alert(&alert_config, &msg);
                                }
                            } else {
                                println!("Monitoring warning: Could not determine disk for path {}", system_path);
                            }
                        }
                    }
                }

                // Wait for the configured interval
                time::sleep(Duration::from_secs(check_interval * 60)).await;
            } else {
                // If disabled, check again in 10 seconds
                time::sleep(Duration::from_secs(10)).await;
            }
        }
    });

    let app_settings = shared_settings.clone();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(app_settings.clone()))
            .wrap(cors)
            .route("/api/scan", web::get().to(server::scan))
            .route("/api/select-folder", web::get().to(server::select_folder))
            .route("/api/browse", web::get().to(server::browse))
            .route("/api/health", web::get().to(server::health))
            .route("/api/settings", web::get().to(server::get_settings))
            .route("/api/settings", web::post().to(server::update_settings))
            .route("/api/layout", web::post().to(server::update_layout))
            // Serve static files
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
