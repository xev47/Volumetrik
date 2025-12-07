#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_extras::{TableBuilder, Column};
use std::sync::{Arc, Mutex};
use std::thread;
use rfd::FileDialog;
use chrono::DateTime;

mod models;
mod scan;

use models::{FileStats, ScanAnalysis};
use scan::scan_path;

fn load_icon() -> eframe::egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(include_bytes!("../logo.png"))
            .unwrap_or_else(|_| {
                image::DynamicImage::new_rgba8(1, 1)
            })
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    
    eframe::egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title("Volumetrik")
            .with_min_inner_size([800.0, 600.0])
            .with_icon(load_icon()),
        ..Default::default()
    };
    
    eframe::run_native(
        "Volumetrik",
        options,
        Box::new(|cc| {
            // Set dark theme by default to match screenshot
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(VolumetrikApp::new(cc)))
        }),
    )
}

struct VolumetrikApp {
    current_path: String,
    scan_result: Arc<Mutex<Option<ScanResult>>>,
    is_scanning: Arc<Mutex<bool>>,
    error_message: Arc<Mutex<Option<String>>>,
    
    // UI State
    sort_column: SortColumn,
    sort_descending: bool,
    icon_texture: Option<egui::TextureHandle>,
}

#[derive(Clone)]
struct ScanResult {
    files: Vec<FileStats>,
    analysis: ScanAnalysis,
}

#[derive(PartialEq)]
enum SortColumn {
    Name,
    Size,
    Count,
    Modified,
}

impl VolumetrikApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(24.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(14.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Button, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
        ].into();
        cc.egui_ctx.set_style(style);

        let icon_texture = if let Ok(image) = image::load_from_memory(include_bytes!("../logo.png")) {
            let image_buffer = image.into_rgba8();
            let size = [image_buffer.width() as _, image_buffer.height() as _];
            let pixels = image_buffer.into_flat_samples();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                size,
                pixels.as_slice(),
            );
            Some(cc.egui_ctx.load_texture(
                "app_icon",
                color_image,
                egui::TextureOptions::LINEAR
            ))
        } else {
            None
        };

        Self {
            current_path: std::env::current_dir().unwrap_or_default().to_string_lossy().to_string(),
            scan_result: Arc::new(Mutex::new(None)),
            is_scanning: Arc::new(Mutex::new(false)),
            error_message: Arc::new(Mutex::new(None)),
            sort_column: SortColumn::Size,
            sort_descending: true,
            icon_texture,
        }
    }

    fn start_scan(&self) {
        let path = self.current_path.clone();
        let result_clone = self.scan_result.clone();
        let scanning_clone = self.is_scanning.clone();
        let error_clone = self.error_message.clone();

        *scanning_clone.lock().unwrap() = true;
        *error_clone.lock().unwrap() = None;

        thread::spawn(move || {
            match scan_path(&path) {
                Ok((files, analysis)) => {
                    *result_clone.lock().unwrap() = Some(ScanResult {
                        files,
                        analysis,
                    });
                }
                Err(e) => {
                    *error_clone.lock().unwrap() = Some(e.to_string());
                }
            }
            *scanning_clone.lock().unwrap() = false;
        });
    }

    fn format_size(&self, bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if bytes >= TB {
            format!("{:.2} TB", bytes as f64 / TB as f64)
        } else if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} Bytes", bytes)
        }
    }

    fn format_date(&self, timestamp: u64) -> String {
        if let Some(dt) = DateTime::from_timestamp(timestamp as i64, 0) {
            dt.format("%d/%m/%Y").to_string()
        } else {
            "-".to_string()
        }
    }
}

impl eframe::App for VolumetrikApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let next_path = std::cell::RefCell::new(None);
        let mut trigger_scan = false;

        // --- Header ---
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if let Some(texture) = &self.icon_texture {
                    ui.add(egui::Image::new(texture).max_height(32.0));
                } else {
                    ui.heading("📦");
                }
                ui.heading("Volumetrik");
                ui.add_space(20.0);

                let response = ui.add_sized(
                    [ui.available_width() - 250.0, 24.0], 
                    egui::TextEdit::singleline(&mut self.current_path)
                );
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    trigger_scan = true;
                }

                if ui.add(egui::Button::new(egui::RichText::new("📂").size(20.0))).on_hover_text("Browse").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        *next_path.borrow_mut() = Some(path.to_string_lossy().to_string());
                    }
                }

                if ui.add(egui::Button::new(egui::RichText::new("🔍 Scan").size(20.0))).clicked() {
                    trigger_scan = true;
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let is_dark = ctx.style().visuals.dark_mode;
                    let icon = if is_dark { "☀" } else { "🌙" };
                    if ui.add(egui::Button::new(egui::RichText::new(icon).size(20.0)).frame(false)).on_hover_text("Toggle Theme").clicked() {
                        if is_dark {
                            ctx.set_visuals(egui::Visuals::light());
                        } else {
                            ctx.set_visuals(egui::Visuals::dark());
                        }
                    }
                });
            });
            ui.add_space(8.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let is_scanning = *self.is_scanning.lock().unwrap();
            if is_scanning {
                ui.centered_and_justified(|ui| {
                    ui.spinner();
                });
                ctx.request_repaint();
                return;
            }

            if let Some(err) = &*self.error_message.lock().unwrap() {
                ui.centered_and_justified(|ui| {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", err));
                });
                return;
            }

            let scan_result_arc = self.scan_result.clone();
            let result_lock = scan_result_arc.lock().unwrap();
            if let Some(result) = &*result_lock {
                // Sort files
                let mut files = result.files.clone();
                files.sort_by(|a, b| {
                    let cmp = match self.sort_column {
                        SortColumn::Name => a.name.cmp(&b.name),
                        SortColumn::Size => a.size.cmp(&b.size),
                        SortColumn::Count => a.file_count.cmp(&b.file_count),
                        SortColumn::Modified => a.modified.cmp(&b.modified),
                    };
                    if self.sort_descending { cmp.reverse() } else { cmp }
                });

                // File Browser
                ui.push_id("file_browser", |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("File Browser");
                        if let Some(parent) = std::path::Path::new(&self.current_path).parent() {
                            if ui.button("⬆ Up").clicked() {
                                *next_path.borrow_mut() = Some(parent.to_string_lossy().to_string());
                            }
                        }
                    });
                    ui.label(egui::RichText::new(&self.current_path).weak());
                    ui.add_space(5.0);

                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .vscroll(true)
                        .column(Column::remainder().at_least(250.0).resizable(true)) // Name
                        .column(Column::auto().at_least(120.0).resizable(true)) // Size
                        .column(Column::auto().at_least(100.0).resizable(true)) // Usage %
                        .column(Column::auto().at_least(100.0).resizable(true))  // Count
                        .column(Column::auto().at_least(140.0).resizable(true)) // Modified
                        .header(20.0, |mut header| {
                            header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("Filename ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Name; self.sort_descending = !self.sort_descending; } });
                            header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("Storage Usage ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Size; self.sort_descending = !self.sort_descending; } });
                            header.col(|ui| { ui.label(egui::RichText::new("Usage %").heading().size(14.0)); });
                            header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("File Count ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Count; self.sort_descending = !self.sort_descending; } });
                            header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("Last Modified ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Modified; self.sort_descending = !self.sort_descending; } });
                        })
                        .body(|mut body| {
                            for file in &files {
                                body.row(20.0, |mut row| {
                                    row.col(|ui| {
                                        let icon = if file.is_dir { "📁" } else { "📄" };
                                        if file.is_dir {
                                            if ui.link(format!("{} {}", icon, file.name)).clicked() {
                                                *next_path.borrow_mut() = Some(file.path.clone());
                                            }
                                        } else {
                                            ui.label(format!("{} {}", icon, file.name));
                                        }
                                    });
                                    row.col(|ui| { ui.label(self.format_size(file.size)); });
                                    row.col(|ui| {
                                        let percent = if result.analysis.total_size > 0 {
                                            file.size as f64 / result.analysis.total_size as f64
                                        } else { 0.0 };
                                        let bar = egui::ProgressBar::new(percent as f32).show_percentage().animate(false);
                                        ui.add(bar);
                                    });
                                    row.col(|ui| { 
                                        if file.is_dir {
                                            ui.label(file.file_count.to_string());
                                        } else {
                                            ui.label("-");
                                        }
                                    });
                                    row.col(|ui| { ui.label(self.format_date(file.modified)); });
                                });
                            }
                        });
                });

                // Handle navigation clicks (hacky workaround for closure capture)
                // We need to detect clicks inside the table. 
                // Since we can't mutate `next_path` easily inside the table closure if it's not moved,
                // we can use `ui.ctx().data()` to store intent or just use a RefCell if we really need to.
                // Or we can just iterate again to check clicks? No.
                // Actually, `next_path` is mutable in `update`. The closures borrow `self`.
                // If we move `next_path` into the closure, we can't use it later.
                // We can use a `std::cell::RefCell` for `next_path` locally.
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Select a folder and click Scan to begin.");
                });
            }
        });

        // Apply navigation if needed
        if let Some(path) = next_path.into_inner() {
            self.current_path = path;
            self.start_scan();
        }
        
        if trigger_scan {
            self.start_scan();
        }
    }
}
