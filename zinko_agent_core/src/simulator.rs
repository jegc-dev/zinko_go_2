use crate::models::TelemetryData;
use std::path::Path;

/// The Simulator component allows for manual triggering of hardware failure states.
/// This is used during demonstrations to verify the agent's response to critical data.
pub struct Simulator;

impl Simulator {
    /// Checks for the existence of specific "trigger files" in the execution directory
    /// and overrides telemetry data if they are found.
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
    use crate::models::{BatteryMetrics, CpuMetrics, StorageMetrics};
    use chrono::Utc;
    use std::fs;

    #[test]
    fn test_simulator_overrides() {
        let mut data = TelemetryData {
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
            memory: crate::models::MemoryMetrics {
                total_kb: 16000000,
                used_kb: 8000000,
                usage_pct: 50.0,
            },
            os_name: "Test".to_string(),
            os_version: "1.0".to_string(),
            kernel_version: "5.0".to_string(),
            hostname: "test".to_string(),
            agent: crate::models::AgentMetrics {
                cpu_pct: 0.0,
                mem_kb: 0,
                mem_pct: 0.0,
            },
        };

        // Create trigger file in a temp directory to avoid test race conditions
        let temp_dir = std::env::temp_dir().join("zinko_test_overrides");
        let _ = fs::create_dir_all(&temp_dir);
        let trigger_path = temp_dir.join("fail_temp.trigger");

        fs::write(&trigger_path, "").unwrap();

        // Override uses current directory, so we temporarily change to temp dir
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        Simulator::apply_overrides(&mut data);
        assert_eq!(data.cpu.temp_c, 92.5);

        // Cleanup
        std::env::set_current_dir(&original_dir).unwrap();
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_no_overrides_when_files_missing() {
        let mut data = TelemetryData {
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
            memory: crate::models::MemoryMetrics {
                total_kb: 16000000,
                used_kb: 8000000,
                usage_pct: 50.0,
            },
            os_name: "Test".to_string(),
            os_version: "1.0".to_string(),
            kernel_version: "5.0".to_string(),
            hostname: "test".to_string(),
            agent: crate::models::AgentMetrics {
                cpu_pct: 0.0,
                mem_kb: 0,
                mem_pct: 0.0,
            },
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
