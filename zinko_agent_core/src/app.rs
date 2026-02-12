use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::VecDeque;
use crate::models::TelemetryData;

use std::sync::mpsc::Receiver;

pub struct ZinkoApp {
    pub telemetry_history: VecDeque<TelemetryData>,
    pub last_received: Option<TelemetryData>,
    pub max_history: usize,
    pub receiver: Receiver<TelemetryData>,
}

impl ZinkoApp {
    pub fn new(receiver: Receiver<TelemetryData>) -> Self {
        Self {
            telemetry_history: VecDeque::new(),
            last_received: None,
            max_history: 100,
            receiver,
        }
    }

    pub fn update_data(&mut self, data: TelemetryData) {
        self.last_received = Some(data.clone());
        self.telemetry_history.push_back(data);
        if self.telemetry_history.len() > self.max_history {
            self.telemetry_history.pop_front();
        }
    }
}

impl eframe::App for ZinkoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll for new data
        while let Ok(data) = self.receiver.try_recv() {
            self.update_data(data);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zinko Go 2.0 - Transparency Dashboard");
            ui.add_space(10.0);

            if let Some(data) = &self.last_received {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.set_min_width(300.0);
                        ui.group(|ui| {
                            ui.label(format!("Device: {}", data.device_id));
                            ui.label(format!("Last Update: {}", data.timestamp.format("%H:%M:%S")));
                            
                            ui.add_space(5.0);
                            ui.label("Hardware Status:");
                            
                            // SSD Health
                            ui.label(format!("SSD Health: {:.1}%", data.storage.health_pct));
                            ui.add(egui::ProgressBar::new(data.storage.health_pct / 100.0)
                                .text(format!("{:.0}%", data.storage.health_pct)));

                            ui.add_space(5.0);
                            // Battery
                            ui.label(format!("Battery Cycles: {}", data.battery.cycles));
                            ui.label(format!("Battery Health: {:.1}%", data.battery.health_pct));
                            ui.add(egui::ProgressBar::new(data.battery.health_pct / 100.0)
                                .text(format!("{:.0}%", data.battery.health_pct)));
                        });
                    });

                    ui.vertical(|ui| {
                        ui.group(|ui| {
                            ui.label("Real-time Temperature (°C)");
                            let points: PlotPoints = self.telemetry_history
                                .iter()
                                .enumerate()
                                .map(|(i, d)| [i as f64, d.cpu.temp_c as f64])
                                .collect();
                            
                            let line = Line::new(points).color(egui::Color32::RED);
                            Plot::new("temp_plot")
                                .view_aspect(2.0)
                                .show(ui, |plot_ui| plot_ui.line(line));
                        });
                    });
                });

                ui.add_space(10.0);
                ui.collapsing("Privacy Console (Raw JSON)", |ui| {
                    egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                        let json = serde_json::to_string_pretty(data).unwrap_or_default();
                        ui.add(egui::TextEdit::multiline(&mut json.as_str())
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY));
                    });
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Auditar Conexión (EULA)").clicked() {
                        let _ = open::that("https://zinko.com/privacy-policy");
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("Secure Cloud Connection: Active");
                        ui.colored_label(egui::Color32::GREEN, "●");
                    });
                });

            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Waiting for agent telemetry...");
                });
            }
        });

        // Continuous repaint to keep the graph moving
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}
