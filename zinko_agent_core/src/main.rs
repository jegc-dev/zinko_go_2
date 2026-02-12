mod models;
mod telemetry;
mod simulator;

use std::time::Duration;
use tokio::time::sleep;
use chrono::Utc;
use sysinfo::System;
use crate::models::TelemetryData;
use crate::simulator::Simulator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut sys = System::new_all();
    let device_id = "ZINKO-DEMO-001".to_string();

    println!("🚀 Zinko Agent Core iniciado...");
    println!("Device ID: {}", device_id);

    loop {
        // 1. Gather Real Metrics (as far as possible in WSL/Demo env)
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

        // 2. Apply Simulation Overrides
        Simulator::apply_overrides(&mut data);

        // 3. Print JSON to Console (Data Transparency Dashboard requirement)
        let json = serde_json::to_string_pretty(&data)?;
        println!("\n--- TELEMETRY UPDATE ---");
        println!("{}", json);

        // 4. In Week 3, this would be sent to GCP
        // println!("Status: Waiting for cloud integration (Week 3)");

        sleep(Duration::from_secs(5)).await;
    }
}
