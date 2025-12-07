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
            .with_inner_size([1600.0, 1000.0])
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
    filter_text: String,
}

#[derive(Clone)]
struct ScanResult {
    files: Vec<FileStats>,
    analysis: ScanAnalysis,
    scanned_path: String,
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
            filter_text: String::new(),
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
                        scanned_path: path.clone(),
                    });
                }
                Err(e) => {
                    *error_clone.lock().unwrap() = Some(e.to_string());
                }
            }
            *scanning_clone.lock().unwrap() = false;
        });
    }

    fn format_size(bytes: u64) -> String {
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

    fn open_path(path: &str) {
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("explorer")
                .arg(path)
                .spawn()
                .ok();
        }
        #[cfg(not(target_os = "windows"))]
        {
            // Fallback for other OSs if needed
            if let Ok(mut child) = std::process::Command::new("xdg-open").arg(path).spawn() {
                child.wait().ok();
            } else {
                std::process::Command::new("open").arg(path).spawn().ok();
            }
        }
    }
}

impl eframe::App for VolumetrikApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let next_path = std::cell::RefCell::new(None);
        let mut trigger_scan = false;

        // --- Header ---
        egui::TopBottomPanel::top("header")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(egui::Margin::symmetric(20.0, 8.0)))
            .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(texture) = &self.icon_texture {
                    ui.add(egui::Image::new(texture).max_height(32.0));
                } else {
                    ui.heading("📦");
                }
                ui.heading("Volumetrik");
                ui.add_space(20.0);

                // Breadcrumbs
                let path = std::path::Path::new(&self.current_path);
                let ancestors: Vec<_> = path.ancestors().collect();
                let ancestors_rev: Vec<_> = ancestors.into_iter().rev().collect();
                
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for (i, ancestor) in ancestors_rev.iter().enumerate() {
                            let label = if let Some(name) = ancestor.file_name() {
                                name.to_string_lossy().to_string()
                            } else {
                                ancestor.to_string_lossy().to_string()
                            };

                            if i > 0 {
                                ui.label(">");
                            }
                            
                            if ui.button(label).clicked() {
                                *next_path.borrow_mut() = Some(ancestor.to_string_lossy().to_string());
                            }
                        }
                    });
                });

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
                    
                    if ui.add(egui::Button::new(egui::RichText::new("🔍 Scan").size(20.0))).clicked() {
                        trigger_scan = true;
                    }

                    if ui.add(egui::Button::new(egui::RichText::new("📂").size(20.0))).on_hover_text("Browse").clicked() {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            *next_path.borrow_mut() = Some(path.to_string_lossy().to_string());
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.0))
            .show(ctx, |ui| {
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
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Frame::none().inner_margin(egui::Margin::symmetric(20.0, 10.0)).show(ui, |ui| {
                        ui.set_min_width(ui.available_width()); // Ensure content fills width to respect margins
                        // --- Dashboard & Charts ---
                        ui.add_space(10.0);
                    
                    let dashboard_width = 250.0 + 40.0 + 500.0;
                    let available_width = ui.available_width();
                    let margin = if available_width > dashboard_width {
                        (available_width - dashboard_width) / 2.0
                    } else {
                        0.0
                    };

                    ui.horizontal(|ui| {
                        ui.add_space(margin);
                        
                        // Left side: Cards
                        ui.vertical(|ui| {
                            ui.set_width(250.0);
                            
                            // Card 1: Total Size
                            egui::Frame::group(ui.style())
                                .fill(ui.style().visuals.faint_bg_color)
                                .stroke(egui::Stroke::new(1.0, ui.style().visuals.widgets.noninteractive.bg_stroke.color))
                                .rounding(egui::Rounding::same(4.0))
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new("Total Size").size(14.0).weak());
                                        ui.label(egui::RichText::new(Self::format_size(result.analysis.total_size)).size(24.0).strong());
                                    });
                                });
                            
                            ui.add_space(10.0);

                            // Card 2: Total Files
                            egui::Frame::group(ui.style())
                                .fill(ui.style().visuals.faint_bg_color)
                                .stroke(egui::Stroke::new(1.0, ui.style().visuals.widgets.noninteractive.bg_stroke.color))
                                .rounding(egui::Rounding::same(4.0))
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new("Total Files").size(14.0).weak());
                                        ui.label(egui::RichText::new(result.analysis.total_files.to_string()).size(24.0).strong());
                                    });
                                });
                        });

                        ui.add_space(40.0);

                        // Right side: Chart
                        ui.vertical(|ui| {
                            let folder_name = std::path::Path::new(&result.scanned_path)
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| result.scanned_path.clone());
                            
                            ui.heading(format!("File Type Distribution ({})", folder_name));
                            ui.add_space(10.0);
                            
                            let mut extensions: Vec<_> = result.analysis.extension_distribution.iter().collect();
                            extensions.sort_by(|a, b| b.1.size.cmp(&a.1.size));
                            let top_extensions: Vec<_> = extensions.into_iter().take(10).collect();

                            // Prepare data for donut chart
                            let chart_data: Vec<(String, u64, egui::Color32)> = top_extensions.iter().enumerate().map(|(i, (ext, stats))| {
                                let hue = (i as f32 * 0.61803398875) % 1.0;
                                let color = egui::Color32::from(egui::ecolor::Hsva::new(hue, 0.85, 0.9, 1.0));
                                (format!(".{}", ext), stats.size, color)
                            }).collect();

                            // Draw Donut Chart
                            let height = 230.0; // Increased height to fit legend
                            let width = 500.0;
                            let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
                            
                            let center = rect.min + egui::vec2(height / 2.0 + 20.0, height / 2.0);
                            let radius = height / 2.0 - 10.0;
                            let inner_radius = radius * 0.6;
                            
                            let painter = ui.painter();
                            let total_size: u64 = chart_data.iter().map(|(_, s, _)| *s).sum();
                            let mut current_angle = -std::f32::consts::FRAC_PI_2;

                            for (_name, size, color) in &chart_data {
                                if total_size > 0 {
                                    let percentage = *size as f64 / total_size as f64;
                                    let angle_span = percentage as f32 * std::f32::consts::TAU;
                                    
                                    if angle_span > 0.0 {
                                        let steps = (angle_span * radius).max(2.0) as usize;
                                        let mut mesh = egui::Mesh::default();
                                        
                                        for i in 0..=steps {
                                            let angle = current_angle + (i as f32 / steps as f32) * angle_span;
                                            let cos = angle.cos();
                                            let sin = angle.sin();
                                            
                                            let p_out = center + egui::vec2(cos, sin) * radius;
                                            let p_in = center + egui::vec2(cos, sin) * inner_radius;
                                            
                                            mesh.vertices.push(egui::epaint::Vertex {
                                                pos: p_out,
                                                uv: egui::Pos2::ZERO,
                                                color: *color,
                                            });
                                            mesh.vertices.push(egui::epaint::Vertex {
                                                pos: p_in,
                                                uv: egui::Pos2::ZERO,
                                                color: *color,
                                            });
                                            
                                            if i > 0 {
                                                let base = ((i - 1) * 2) as u32;
                                                mesh.add_triangle(base, base + 1, base + 3);
                                                mesh.add_triangle(base, base + 3, base + 2);
                                            }
                                        }
                                        painter.add(egui::Shape::mesh(mesh));
                                        current_angle += angle_span;
                                    }
                                }
                            }

                            // Legend
                            let legend_rect = egui::Rect::from_min_size(
                                rect.min + egui::vec2(height + 40.0, 0.0),
                                egui::vec2(rect.width() - (height + 40.0), height)
                            );
                            
                            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(legend_rect), |ui| {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    for (name, size, color) in &chart_data {
                                        ui.horizontal(|ui| {
                                            let (r, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                                            ui.painter().rect_filled(r, 2.0, *color);
                                            ui.label(egui::RichText::new(format!("{} - {}", name, Self::format_size(*size))).size(14.0));
                                        });
                                    }
                                });
                            });
                        });
                    });
                    
                    ui.add_space(20.0);

                    // --- File Browser ---
                    // Filter files
                    let filtered_files: Vec<FileStats> = result.files.iter()
                        .filter(|f| self.filter_text.is_empty() || f.name.to_lowercase().contains(&self.filter_text.to_lowercase()))
                        .cloned()
                        .collect();

                    let (mut directories, mut leaf_files): (Vec<_>, Vec<_>) = filtered_files.into_iter().partition(|f| f.is_dir);

                    // Sort helper
                    let sort_files = |files: &mut Vec<FileStats>, column: &SortColumn, descending: bool| {
                        files.sort_by(|a, b| {
                            let cmp = match column {
                                SortColumn::Name => a.name.cmp(&b.name),
                                SortColumn::Size => a.size.cmp(&b.size),
                                SortColumn::Count => a.file_count.cmp(&b.file_count),
                                SortColumn::Modified => a.modified.cmp(&b.modified),
                            };
                            if descending { cmp.reverse() } else { cmp }
                        });
                    };

                    sort_files(&mut directories, &self.sort_column, self.sort_descending);
                    sort_files(&mut leaf_files, &self.sort_column, self.sort_descending);

                    ui.push_id("directory_browser", |ui| {
                        ui.horizontal(|ui| {
                            ui.heading("Directory Browser");
                            ui.add_space(20.0);
                            ui.label("Filter:");
                            ui.add(egui::TextEdit::singleline(&mut self.filter_text).hint_text("Search..."));
                        });
                        ui.label(egui::RichText::new(&self.current_path).weak());
                        ui.add_space(5.0);

                        TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .vscroll(false)
                            .min_scrolled_height(0.0)
                            .column(Column::remainder().at_least(250.0).resizable(true))
                            .column(Column::auto().at_least(120.0).resizable(true))
                            .column(Column::auto().at_least(100.0).resizable(true))
                            .column(Column::auto().at_least(100.0).resizable(true))
                            .column(Column::auto().at_least(140.0).resizable(true))
                            .header(20.0, |mut header| {
                                header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("Name ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Name; self.sort_descending = !self.sort_descending; } });
                                header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("Size ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Size; self.sort_descending = !self.sort_descending; } });
                                header.col(|ui| { ui.label(egui::RichText::new("Usage %").heading().size(14.0)); });
                                header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("Count ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Count; self.sort_descending = !self.sort_descending; } });
                                header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("Modified ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Modified; self.sort_descending = !self.sort_descending; } });
                            })
                            .body(|mut body| {
                                if let Some(parent) = std::path::Path::new(&self.current_path).parent() {
                                    body.row(20.0, |mut row| {
                                        row.col(|ui| {
                                            if ui.add(egui::Button::new(egui::RichText::new("📁 ..").color(egui::Color32::from_rgb(100, 150, 255))).frame(false)).clicked() {
                                                *next_path.borrow_mut() = Some(parent.to_string_lossy().to_string());
                                            }
                                        });
                                        row.col(|ui| { ui.label(""); });
                                        row.col(|ui| { ui.label(""); });
                                        row.col(|ui| { ui.label(""); });
                                        row.col(|ui| { ui.label(""); });
                                    });
                                }

                                for file in &directories {
                                    body.row(20.0, |mut row| {
                                        row.col(|ui| {
                                            if ui.add(egui::Button::new(egui::RichText::new(format!("📁 {}", file.name)).color(egui::Color32::from_rgb(100, 150, 255))).frame(false)).clicked() {
                                                *next_path.borrow_mut() = Some(file.path.clone());
                                            }
                                        });
                                        row.col(|ui| { 
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(Self::format_size(file.size)); 
                                            });
                                        });
                                        row.col(|ui| {
                                            let percent = if result.analysis.total_size > 0 {
                                                file.size as f64 / result.analysis.total_size as f64
                                            } else { 0.0 };
                                            let bar = egui::ProgressBar::new(percent as f32)
                                                .show_percentage()
                                                .fill(egui::Color32::from_rgb(0, 120, 215))
                                                .animate(false);
                                            ui.add(bar);
                                        });
                                        row.col(|ui| { 
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(file.file_count.to_string());
                                            });
                                        });
                                        row.col(|ui| { ui.label(self.format_date(file.modified)); })
                                            .1
                                            .context_menu(|ui| {
                                                if ui.button("Open").clicked() {
                                                    Self::open_path(&file.path);
                                                    ui.close_menu();
                                                }
                                                if ui.button("Open in Explorer").clicked() {
                                                    #[cfg(target_os = "windows")]
                                                    std::process::Command::new("explorer")
                                                        .arg("/select,")
                                                        .arg(&file.path)
                                                        .spawn()
                                                        .ok();
                                                    ui.close_menu();
                                                }
                                            });
                                    });
                                }
                            });
                    });

                    ui.add_space(20.0);

                    ui.push_id("file_browser_list", |ui| {
                        ui.heading("File Browser");
                        ui.add_space(5.0);

                        TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .vscroll(false)
                            .min_scrolled_height(0.0)
                            .column(Column::remainder().at_least(250.0).resizable(true))
                            .column(Column::auto().at_least(120.0).resizable(true))
                            .column(Column::auto().at_least(100.0).resizable(true))
                            .column(Column::auto().at_least(100.0).resizable(true))
                            .column(Column::auto().at_least(140.0).resizable(true))
                            .header(20.0, |mut header| {
                                header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("Name ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Name; self.sort_descending = !self.sort_descending; } });
                                header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("Size ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Size; self.sort_descending = !self.sort_descending; } });
                                header.col(|ui| { ui.label(egui::RichText::new("Usage %").heading().size(14.0)); });
                                header.col(|ui| { ui.label(egui::RichText::new("").heading().size(14.0)); });
                                header.col(|ui| { if ui.add(egui::Button::new(egui::RichText::new("Modified ↕").heading().size(14.0)).frame(false)).clicked() { self.sort_column = SortColumn::Modified; self.sort_descending = !self.sort_descending; } });
                            })
                            .body(|mut body| {
                                for file in &leaf_files {
                                    body.row(20.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(format!("📄 {}", file.name));
                                        });
                                        row.col(|ui| { 
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(Self::format_size(file.size)); 
                                            });
                                        });
                                        row.col(|ui| {
                                            let percent = if result.analysis.total_size > 0 {
                                                file.size as f64 / result.analysis.total_size as f64
                                            } else { 0.0 };
                                            let bar = egui::ProgressBar::new(percent as f32)
                                                .show_percentage()
                                                .fill(egui::Color32::from_rgb(0, 120, 215))
                                                .animate(false);
                                            ui.add(bar);
                                        });
                                        row.col(|ui| { 
                                            ui.label("-");
                                        });
                                        row.col(|ui| { ui.label(self.format_date(file.modified)); })
                                            .1
                                            .context_menu(|ui| {
                                                if ui.button("Open").clicked() {
                                                    Self::open_path(&file.path);
                                                    ui.close_menu();
                                                }
                                                if ui.button("Open in Explorer").clicked() {
                                                    #[cfg(target_os = "windows")]
                                                    std::process::Command::new("explorer")
                                                        .arg("/select,")
                                                        .arg(&file.path)
                                                        .spawn()
                                                        .ok();
                                                    ui.close_menu();
                                                }
                                            });
                                    });
                                }
                            });
                    });

                    ui.add_space(20.0);
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
