use crate::models::TelemetryData;
use eframe::egui;
use egui_plot::Plot;
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;

/// Represents the different views (tabs) within the application.
#[derive(PartialEq)]
pub enum Tab {
    Dashboard,
    Privacy,
    SystemInfo,
}

/// The main Zinko Application state and UI logic.
pub struct ZinkoApp {
    /// Circular buffer of recent telemetry data for plotting.
    pub telemetry_history: VecDeque<TelemetryData>,
    /// The most recent telemetry packet received from the agent.
    pub last_received: Option<TelemetryData>,
    /// Maximum number of data points to keep in history.
    pub max_history: usize,
    /// Channel receiver hooked into the background agent thread.
    pub receiver: Receiver<TelemetryData>,
    /// Currently active navigation tab.
    pub current_tab: Tab,
    /// Cached WMI hardware identifiers (collected once at startup).
    pub wmi_identifiers: Option<Vec<(String, String)>>,
}

impl ZinkoApp {
    /// Creates a new ZinkoApp with the given telemetry data receiver channel.
    pub fn new(receiver: Receiver<TelemetryData>) -> Self {
        Self {
            telemetry_history: VecDeque::new(),
            last_received: None,
            max_history: 100,
            receiver,
            current_tab: Tab::Dashboard,
            wmi_identifiers: None,
        }
    }

    /// Receives new telemetry data, updates history buffer, and caches WMI identifiers.
    pub fn update_data(&mut self, data: TelemetryData) {
        self.last_received = Some(data.clone());
        self.telemetry_history.push_back(data);
        if self.telemetry_history.len() > self.max_history {
            self.telemetry_history.pop_front();
        }
        // Collect WMI identifiers only once (they don't change at runtime)
        if self.wmi_identifiers.is_none() {
            self.wmi_identifiers = Some(crate::telemetry::get_all_wmi_identifiers());
        }
    }

    /// Renders the left navigation sidebar with tab selection and status indicators.
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
                        ui.ctx().output_mut(|o| {
                            o.open_url =
                                Some(egui::OpenUrl::new_tab("https://zinko.com/privacy-policy"))
                        });
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

    /// Renders the main dashboard with health metrics, resource utilization, temperature, and agent footprint.
    fn render_dashboard(&self, ui: &mut egui::Ui, data: &TelemetryData) {
        ui.heading("System Diagnostics");
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Hardware Health Section
            ui.group(|ui| {
                ui.strong("Hardware Health");
                ui.add_space(5.0);

                ui.label(format!("SSD Life: {:.1}%", data.storage.health_pct));
                ui.add(
                    egui::ProgressBar::new(data.storage.health_pct / 100.0)
                        .fill(if data.storage.health_pct < 10.0 {
                            egui::Color32::RED
                        } else {
                            egui::Color32::from_rgb(0, 102, 128)
                        })
                        .text(format!("{:.0}%", data.storage.health_pct)),
                );

                ui.add_space(10.0);

                ui.label(format!("Battery Health: {:.1}%", data.battery.health_pct));
                ui.add(
                    egui::ProgressBar::new(data.battery.health_pct / 100.0)
                        .fill(if data.battery.health_pct < 80.0 {
                            egui::Color32::RED
                        } else {
                            egui::Color32::from_rgb(0, 102, 128)
                        })
                        .text(format!("{:.0}%", data.battery.health_pct)),
                );

                ui.label(format!("Cycles: {}", data.battery.cycles));
            });

            ui.add_space(10.0);

            // Resource Utilization Section
            ui.group(|ui| {
                ui.strong("Resource Utilization");
                ui.add_space(5.0);

                ui.label(format!("CPU Usage: {:.1}%", data.cpu.usage_pct));
                ui.add(
                    egui::ProgressBar::new(data.cpu.usage_pct / 100.0)
                        .fill(if data.cpu.usage_pct > 80.0 {
                            egui::Color32::RED
                        } else {
                            egui::Color32::from_rgb(0, 102, 128)
                        })
                        .text(format!("{:.1}%", data.cpu.usage_pct)),
                );

                ui.add_space(10.0);

                ui.label(format!(
                    "RAM Usage: {:.1}% ({:.1} GB / {:.1} GB)",
                    data.memory.usage_pct,
                    data.memory.used_kb as f32 / 1024.0 / 1024.0,
                    data.memory.total_kb as f32 / 1024.0 / 1024.0
                ));
                ui.add(
                    egui::ProgressBar::new(data.memory.usage_pct / 100.0)
                        .fill(if data.memory.usage_pct > 85.0 {
                            egui::Color32::RED
                        } else {
                            egui::Color32::from_rgb(0, 102, 128)
                        })
                        .text(format!("{:.1}%", data.memory.usage_pct)),
                );
            });

            ui.add_space(10.0);

            // Temperature Section with bar chart
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.strong("CPU Temperature:");
                    let color = if data.cpu.temp_c > 80.0 {
                        egui::Color32::RED
                    } else {
                        egui::Color32::LIGHT_BLUE
                    };
                    ui.colored_label(color, format!("{:.1}°C", data.cpu.temp_c));
                });

                let temp_progress = (data.cpu.temp_c - 20.0) / 80.0;
                ui.add(
                    egui::ProgressBar::new(temp_progress.clamp(0.0, 1.0))
                        .fill(if data.cpu.temp_c > 80.0 {
                            egui::Color32::RED
                        } else {
                            egui::Color32::from_rgb(0, 102, 128)
                        })
                        .text(format!("{:.1}°C", data.cpu.temp_c)),
                );

                ui.add_space(5.0);
                ui.strong("Temperature History (Bar Chart):");

                // Build vertical bars from history, each bar represents a sample
                let bars: Vec<egui_plot::Bar> = self
                    .telemetry_history
                    .iter()
                    .enumerate()
                    .map(|(i, d)| {
                        let color = if d.cpu.temp_c > 80.0 {
                            egui::Color32::from_rgb(220, 50, 50) // Hot red
                        } else if d.cpu.temp_c > 60.0 {
                            egui::Color32::from_rgb(230, 150, 30) // Warm orange
                        } else {
                            egui::Color32::from_rgb(0, 180, 200) // Cool cyan
                        };
                        egui_plot::Bar::new(i as f64, d.cpu.temp_c as f64)
                            .width(0.8)
                            .fill(color)
                    })
                    .collect();

                let chart = egui_plot::BarChart::new(bars).name("Temp °C");
                Plot::new("temp_bar_plot")
                    .height(120.0)
                    .allow_zoom(false)
                    .allow_drag(false)
                    .show(ui, |plot_ui| plot_ui.bar_chart(chart));
            });

            ui.add_space(10.0);

            // Agent Footprint Section (at the end)
            ui.group(|ui| {
                ui.strong("Zinko Agent Footprint");
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Agent CPU:");
                    ui.colored_label(
                        egui::Color32::LIGHT_GREEN,
                        format!("{:.2}%", data.agent.cpu_pct),
                    );
                    ui.label(format!("of total ({:.1}%)", data.cpu.usage_pct));
                });
                ui.add(
                    egui::ProgressBar::new(data.agent.cpu_pct / 100.0)
                        .fill(egui::Color32::from_rgb(0, 180, 100))
                        .text(format!("{:.2}%", data.agent.cpu_pct)),
                );

                ui.add_space(5.0);

                let agent_mem_mb = data.agent.mem_kb as f32 / 1024.0;
                ui.horizontal(|ui| {
                    ui.label("Agent RAM:");
                    ui.colored_label(
                        egui::Color32::LIGHT_GREEN,
                        format!("{:.1} MB ({:.2}%)", agent_mem_mb, data.agent.mem_pct),
                    );
                    ui.label(format!(
                        "of {:.1} GB total",
                        data.memory.total_kb as f32 / 1024.0 / 1024.0
                    ));
                });
                ui.add(
                    egui::ProgressBar::new(data.agent.mem_pct / 100.0)
                        .fill(egui::Color32::from_rgb(0, 180, 100))
                        .text(format!("{:.2}%", data.agent.mem_pct)),
                );
            });
        });
    }

    /// Renders the Privacy Hub tab showing the raw JSON telemetry payload.
    fn render_privacy_hub(&self, ui: &mut egui::Ui, data: &TelemetryData) {
        ui.heading("Privacy Hub - Real-time Data Stream");
        ui.label("This is the exact JSON data packet sent to Zinko Cloud for analysis.");
        ui.add_space(10.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let json = serde_json::to_string_pretty(data).unwrap_or_default();
                ui.add(
                    egui::TextEdit::multiline(&mut json.as_str())
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
            });
    }

    /// Renders the System Info tab with asset identification, platform details, and WMI audit.
    fn render_system_info(&self, ui: &mut egui::Ui, data: &TelemetryData) {
        ui.heading("System Information");
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("Asset Identification")
                        .strong()
                        .size(14.0),
                );
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Device ID (BIOS):");
                    ui.strong(&data.device_id);
                });

                ui.horizontal(|ui| {
                    ui.label("Hostname:");
                    ui.strong(&data.hostname);
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label(egui::RichText::new("Platform Details").strong().size(14.0));
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Operating System:");
                    ui.strong(&data.os_name);
                });

                ui.horizontal(|ui| {
                    ui.label("OS Version:");
                    ui.strong(&data.os_version);
                });

                ui.horizontal(|ui| {
                    ui.label("Kernel Version:");
                    ui.strong(&data.kernel_version);
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Agent Version:");
                    ui.strong("2.0 (Stable)");
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label(
                    egui::RichText::new("WMI Hardware Audit")
                        .strong()
                        .size(14.0),
                );
                ui.add_space(5.0);

                if let Some(identifiers) = &self.wmi_identifiers {
                    for (label, value) in identifiers {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", label));
                            if value == "N/A"
                                || value.to_lowercase().contains("default")
                                || value.to_lowercase().contains("o.e.m.")
                            {
                                ui.colored_label(egui::Color32::DARK_GRAY, value);
                            } else {
                                ui.strong(value);
                            }
                        });
                    }
                } else {
                    ui.spinner();
                    ui.label("Querying WMI...");
                }
            });
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
