use std::path::Path;
use crate::models::TelemetryData;

pub struct Simulator;

impl Simulator {
    pub fn apply_overrides(data: &mut TelemetryData) {
        // CPU Temperature Override
        if Path::new("fail_temp.trigger").exists() {
            data.cpu.temp_c = 92.5;
        }

        // Battery Cycles Override
        if Path::new("fail_battery.trigger").exists() {
            data.battery.cycles = 1050;
            data.battery.health_pct = 65.0;
        }

        // SSD Health Override
        if Path::new("fail_disk.trigger").exists() {
            // Simulation logic: health drops over time if we tracked state, 
            // but for simple demo, we force a low value.
            data.storage.health_pct = 5.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CpuMetrics, StorageMetrics, BatteryMetrics};
    use chrono::Utc;
    use std::fs;

    #[test]
    fn test_simulator_overrides() {
        let mut data = TelemetryData {
            timestamp: Utc::now(),
            device_id: "test".to_string(),
            cpu: CpuMetrics { usage_pct: 0.0, temp_c: 40.0 },
            storage: StorageMetrics { health_pct: 100.0, temp_c: 30.0 },
            battery: BatteryMetrics { cycles: 0, health_pct: 100.0, capacity_mah: 5000 },
        };

        // Create trigger file
        fs::write("fail_temp.trigger", "").unwrap();
        Simulator::apply_overrides(&mut data);
        assert_eq!(data.cpu.temp_c, 92.5);

        // Cleanup
        fs::remove_file("fail_temp.trigger").unwrap();
    }

    #[test]
    fn test_no_overrides_when_files_missing() {
        let mut data = TelemetryData {
            timestamp: Utc::now(),
            device_id: "test".to_string(),
            cpu: CpuMetrics { usage_pct: 0.0, temp_c: 40.0 },
            storage: StorageMetrics { health_pct: 100.0, temp_c: 30.0 },
            battery: BatteryMetrics { cycles: 0, health_pct: 100.0, capacity_mah: 5000 },
        };

        // Ensure no trigger files exist (they shouldn't in a clean test environment, but just in case)
        let _ = fs::remove_file("fail_temp.trigger");
        let _ = fs::remove_file("fail_battery.trigger");
        let _ = fs::remove_file("fail_disk.trigger");

        Simulator::apply_overrides(&mut data);

        // Values should remain unchanged
        assert_eq!(data.cpu.temp_c, 40.0);
        assert_eq!(data.storage.health_pct, 100.0);
        assert_eq!(data.battery.cycles, 0);
    }
}
