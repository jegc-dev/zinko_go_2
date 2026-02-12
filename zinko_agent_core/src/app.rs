use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::VecDeque;
use crate::models::TelemetryData;
use std::sync::mpsc::Receiver;

#[derive(PartialEq)]
pub enum Tab {
    Dashboard,
    Privacy,
    SystemInfo,
}

pub struct ZinkoApp {
    pub telemetry_history: VecDeque<TelemetryData>,
    pub last_received: Option<TelemetryData>,
    pub max_history: usize,
    pub receiver: Receiver<TelemetryData>,
    pub current_tab: Tab,
}

impl ZinkoApp {
    pub fn new(receiver: Receiver<TelemetryData>) -> Self {
        Self {
            telemetry_history: VecDeque::new(),
            last_received: None,
            max_history: 100,
            receiver,
            current_tab: Tab::Dashboard,
        }
    }

    pub fn update_data(&mut self, data: TelemetryData) {
        self.last_received = Some(data.clone());
        self.telemetry_history.push_back(data);
        if self.telemetry_history.len() > self.max_history {
            self.telemetry_history.pop_front();
        }
    }

    fn render_sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(180.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading("ZINKO GO 2.0");
                    ui.label("Transparency Agent");
                    ui.add_space(20.0);
                });

                ui.separator();
                ui.add_space(10.0);

                ui.selectable_value(&mut self.current_tab, Tab::Dashboard, "📊 Dashboard");
                ui.add_space(5.0);
                ui.selectable_value(&mut self.current_tab, Tab::Privacy, "🔍 Privacy Hub");
                ui.add_space(5.0);
                ui.selectable_value(&mut self.current_tab, Tab::SystemInfo, "⚙️ System Info");

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    if ui.button("🌐 Auditar (EULA)").clicked() {
                        ui.ctx().output_mut(|o| o.open_url = Some(egui::OpenUrl::new_tab("https://zinko.com/privacy-policy")));
                    }
                    
                    ui.separator();
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        let dot_color = if self.last_received.is_some() {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        };
                        ui.colored_label(dot_color, "●");
                        ui.label("Cloud Sync: Active");
                    });
                    ui.add_space(5.0);
                });
            });
    }

    fn render_dashboard(&self, ui: &mut egui::Ui, data: &TelemetryData) {
        ui.heading("System Diagnostics");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // Health Cards
            ui.vertical(|ui| {
                ui.set_min_width(320.0);
                ui.group(|ui| {
                    ui.strong("Hardware Health");
                    ui.add_space(5.0);
                    
                    ui.label(format!("SSD Life: {:.1}%", data.storage.health_pct));
                    ui.add(egui::ProgressBar::new(data.storage.health_pct / 100.0)
                        .text(format!("{:.0}%", data.storage.health_pct)));

                    ui.add_space(10.0);
                    
                    ui.label(format!("Battery Health: {:.1}%", data.battery.health_pct));
                    ui.add(egui::ProgressBar::new(data.battery.health_pct / 100.0)
                        .text(format!("{:.0}%", data.battery.health_pct)));
                    
                    ui.label(format!("Cycles: {}", data.battery.cycles));
                });
            });

            // Temperature Graph
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.strong("Real-time CPU Temperature (°C)");
                    let points: PlotPoints = self.telemetry_history
                        .iter()
                        .enumerate()
                        .map(|(i, d)| [i as f64, d.cpu.temp_c as f64])
                        .collect();
                    
                    let line = Line::new(points).color(egui::Color32::RED);
                    Plot::new("temp_plot")
                        .view_aspect(2.0)
                        .allow_zoom(false)
                        .allow_drag(false)
                        .show(ui, |plot_ui| plot_ui.line(line));
                });
            });
        });
    }

    fn render_privacy_hub(&self, ui: &mut egui::Ui, data: &TelemetryData) {
        ui.heading("Privacy Hub - Real-time Data Stream");
        ui.label("This is the exact JSON data packet sent to Zinko Cloud for analysis.");
        ui.add_space(10.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let json = serde_json::to_string_pretty(data).unwrap_or_default();
                ui.add(egui::TextEdit::multiline(&mut json.as_str())
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY));
            });
    }

    fn render_system_info(&self, ui: &mut egui::Ui, data: &TelemetryData) {
        ui.heading("System & Simulation Status");
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(format!("Device ID: {}", data.device_id));
            ui.label(format!("Kernel Version: Demo 2.0"));
            ui.label(format!("Agent Logic: Standard Heuristic"));
        });

        ui.add_space(20.0);
        ui.heading("Active Simulation Triggers");
        ui.label("Checking for .trigger files in root...");
        
        ui.group(|ui| {
            if data.cpu.temp_c > 90.0 {
                ui.colored_label(egui::Color32::RED, "⚠ Critical Temperature Simulation Active");
            }
            if data.storage.health_pct < 10.0 {
                ui.colored_label(egui::Color32::RED, "⚠ SSD Failure Simulation Active");
            }
            if data.battery.cycles > 1000 {
                ui.colored_label(egui::Color32::RED, "⚠ Battery Wear Simulation Active");
            }
            if data.cpu.temp_c <= 90.0 && data.storage.health_pct >= 10.0 && data.battery.cycles <= 1000 {
                ui.label("No simulation triggers detected. Running in standard monitoring mode.");
            }
        });
    }
}

impl eframe::App for ZinkoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll for new data
        while let Ok(data) = self.receiver.try_recv() {
            self.update_data(data);
        }

        self.render_sidebar(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(data) = &self.last_received {
                match self.current_tab {
                    Tab::Dashboard => self.render_dashboard(ui, data),
                    Tab::Privacy => self.render_privacy_hub(ui, data),
                    Tab::SystemInfo => self.render_system_info(ui, data),
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.spinner();
                    ui.label("Connecting to Zinko Agent...");
                });
            }
        });

        // Request repaint to keep animations and graphs smooth
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}
