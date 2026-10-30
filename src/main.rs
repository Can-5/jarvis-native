use eframe::egui;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{System, SystemExt, ProcessorExt, DiskExt, NetworkExt};

mod app_state;
mod panels;
mod system_monitor;

use app_state::{AppState, Mode, ChatMessage};
use panels::{left_panel, central_panel, right_panel, bottom_panel};
use system_monitor::SystemMonitor;

fn main() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("JARVIS")
            .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])?),
        ..Default::default()
    };

    let app_state = Arc::new(Mutex::new(AppState::default()));
    let system_monitor = SystemMonitor::new();

    eframe::run_native(
        "JARVIS",
        options,
        Box::new(move |cc| {
            // Install custom fonts if needed
            setup_custom_fonts(&cc.egui_ctx);

            Box::new(JarvisApp {
                state: app_state.clone(),
                system_monitor,
                last_update: std::time::Instant::now(),
            })
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    // Optionally add custom fonts here
    ctx.set_fonts(fonts);
}

struct JarvisApp {
    state: Arc<parking_lot::Mutex<AppState>>,
    system_monitor: SystemMonitor,
    last_update: std::time::Instant,
}

impl eframe::App for JarvisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update system stats every second
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            self.system_monitor.refresh();
            *self.state.lock().system_stats() = self.system_monitor.stats();
            self.last_update = std::time::Instant::now();
        }

        // Request repaint for smooth animations
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        // Layout
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("J.A.R.V.I.S.").size(24.0).color(egui::Color32::from_rgb(0, 212, 255)).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("{}", chrono::Local::now().format("%H:%M:%S"))).monospace().size(16.0));
                });
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            left_panel(ui, &mut *self.state.lock());
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            central_panel(ui, &mut *self.state.lock());
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            right_panel(ui, &mut *self.state.lock());
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            bottom_panel(ui, &mut *self.state.lock());
        });
    }
}