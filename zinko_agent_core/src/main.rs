use chrono::Utc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::System;
use zinko_agent_core::AlertSystem;
use zinko_agent_core::app::ZinkoApp;
use zinko_agent_core::models::TelemetryData;
use zinko_agent_core::simulator::Simulator;
use zinko_agent_core::telemetry;

/// The entry point for the Zinko Agent application.
/// It initializes the background telemetry agent and the graphical UI.
fn main() -> anyhow::Result<()> {
    // 1. Check for CLI fallback flag to support headless environments or debug rendering issues.
    let args: Vec<String> = std::env::args().collect();
    let cli_mode = args.contains(&"--cli".to_string());

    // 2. Setup communication channel (Agent -> UI) for cross-thread telemetry delivery.
    let (tx, rx) = mpsc::channel();

    // 3. Spawn Agent Thread (Low-level hardware telemetry collection)
    thread::spawn(move || {
        let mut sys = System::new_all();
        let device_id = telemetry::get_system_serial();
        let mut alert_system = AlertSystem::new();

        // Runtime for async calls (webhooks) from the synchronous agent thread.
        let rt = tokio::runtime::Runtime::new().unwrap();

        loop {
            // Collect metrics from different hardware components.
            let cpu = telemetry::get_cpu_metrics(&mut sys);
            let storage = telemetry::get_storage_metrics();
            let battery = telemetry::get_battery_metrics();
            let memory = telemetry::get_memory_metrics(&mut sys);
            let agent = telemetry::get_agent_metrics(&mut sys);

            let mut data = TelemetryData {
                timestamp: Utc::now(),
                device_id: device_id.clone(),
                cpu,
                storage,
                battery,
                memory,
                os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
                os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
                kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
                hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
                agent,
            };

            // Apply simulation triggers if specific files are present in the filesystem.
            Simulator::apply_overrides(&mut data);

            // Evaluate heuristic alert rules against the collected telemetry.
            let alerts = alert_system.check_rules(&data);
            for alert in alerts {
                if let Ok(webhook_url) = std::env::var("ZINKO_WEBHOOK") {
                    let alert_clone = alert.clone();
                    let data_clone = data.clone();
                    rt.block_on(async {
                        match alert_system
                            .send_to_webhook(&webhook_url, &alert_clone, &data_clone)
                            .await
                        {
                            Ok(_) => println!("✅ Cloud Sync: Alert sent successfully."),
                            Err(e) => println!("❌ Cloud Sync Error: Failed to send alert: {}", e),
                        }
                    });
                }
                println!("🚨 ALERT [{}]: {}", alert.level, alert.message);
            }

            // Send processed telemetry data to the UI thread.
            if tx.send(data).is_err() {
                break; // Exit loop if the UI has been closed.
            }

            // Wait for 2 seconds before the next telemetry cycle.
            thread::sleep(Duration::from_secs(2));
        }
    });

    // Handle CLI mode if requested, providing an interactive heartbeat in the terminal.
    if cli_mode {
        println!("🚀 Zinko Agent running in CLI mode (Headless)");
        println!("Press Ctrl+C to stop.");
        let mut count = 0;
        loop {
            if let Ok(data) = rx.recv() {
                count += 1;
                print!(
                    "\r[Pulse #{} | Temp: {:.1}°C | SSD Health: {:.1}%] Heartbeat OK",
                    count, data.cpu.temp_c, data.storage.health_pct
                );
                use std::io::{Write, stdout};
                let _ = stdout().flush();
            }
        }
    }

    // 4. Launch UI (Graphical User Interface using egui and eframe)
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([900.0, 850.0])
            .with_min_inner_size([600.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Zinko Go 2.0 - Transparency Agent",
        native_options,
        Box::new(|_cc| Box::new(ZinkoApp::new(rx))),
    )
    .map_err(|e| anyhow::anyhow!("eframe error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use zinko_agent_core::models::{BatteryMetrics, CpuMetrics, StorageMetrics};

    #[test]
    fn test_channel_communication() {
        let (tx, rx) = mpsc::channel();
        let data = TelemetryData {
            timestamp: Utc::now(),
            device_id: "test".to_string(),
            cpu: CpuMetrics {
                usage_pct: 0.0,
                temp_c: 40.0,
            },
            storage: StorageMetrics {
                health_pct: 100.0,
                temp_c: 30.0,
            },
            battery: BatteryMetrics {
                cycles: 0,
                health_pct: 100.0,
                capacity_mah: 5000,
            },
            memory: zinko_agent_core::models::MemoryMetrics {
                total_kb: 16000000,
                used_kb: 8000000,
                usage_pct: 50.0,
            },
            os_name: "TestOS".to_string(),
            os_version: "1.0".to_string(),
            kernel_version: "5.0".to_string(),
            hostname: "test-host".to_string(),
            agent: zinko_agent_core::models::AgentMetrics {
                cpu_pct: 0.0,
                mem_kb: 0,
                mem_pct: 0.0,
            },
        };

        tx.send(data.clone()).unwrap();
        let received = rx.try_recv().unwrap();
        assert_eq!(received.device_id, "test");
    }
}
