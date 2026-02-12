mod models;
mod telemetry;
mod simulator;
mod app;

use std::time::Duration;
use std::sync::mpsc;
use std::thread;
use sysinfo::System;
use chrono::Utc;
use crate::models::TelemetryData;
use crate::simulator::Simulator;
use crate::app::ZinkoApp;

fn main() -> anyhow::Result<()> {
    // 1. Setup communication channel (Agent -> UI)
    let (tx, rx) = mpsc::channel();

    // 2. Spawn Agent Thread (Telemetría de bajo nivel)
    thread::spawn(move || {
        let mut sys = System::new_all();
        let device_id = "ZINKO-DEMO-001".to_string();

        loop {
            let cpu = telemetry::get_cpu_metrics(&mut sys);
            let storage = telemetry::get_storage_metrics();
            let battery = telemetry::get_battery_metrics();

            let mut data = TelemetryData {
                timestamp: Utc::now(),
                device_id: device_id.clone(),
                cpu,
                storage,
                battery,
            };

            Simulator::apply_overrides(&mut data);

            // Send to UI
            if tx.send(data).is_err() {
                break; // UI closed
            }

            thread::sleep(Duration::from_secs(2));
        }
    });

    // 3. Launch UI (Main Thread)
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Zinko Go 2.0 - Transparency Agent",
        native_options,
        Box::new(|_cc| {
            Box::new(ZinkoApp::new(rx))
        }),
    ).map_err(|e| anyhow::anyhow!("eframe error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use crate::models::{CpuMetrics, StorageMetrics, BatteryMetrics};

    #[test]
    fn test_channel_communication() {
        let (tx, rx) = mpsc::channel();
        let data = TelemetryData {
            timestamp: Utc::now(),
            device_id: "test".to_string(),
            cpu: CpuMetrics { usage_pct: 0.0, temp_c: 40.0 },
            storage: StorageMetrics { health_pct: 100.0, temp_c: 30.0 },
            battery: BatteryMetrics { cycles: 0, health_pct: 100.0, capacity_mah: 5000 },
        };

        tx.send(data.clone()).unwrap();
        let received = rx.try_recv().unwrap();
        assert_eq!(received.device_id, "test");
    }
}
