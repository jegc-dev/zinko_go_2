use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the full telemetry packet sent by the agent.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelemetryData {
    /// Timestamp of the telemetry collection.
    pub timestamp: DateTime<Utc>,
    /// Unique identifier for the device.
    pub device_id: String,
    /// CPU related metrics.
    pub cpu: CpuMetrics,
    /// Storage related metrics.
    pub storage: StorageMetrics,
    /// Battery related metrics.
    pub battery: BatteryMetrics,
    /// System memory (RAM) metrics.
    pub memory: MemoryMetrics,
    /// Operating system name.
    pub os_name: String,
    /// Operating system version.
    pub os_version: String,
    /// Kernel version.
    pub kernel_version: String,
    /// System hostname.
    pub hostname: String,
    /// Zinko Agent's own resource consumption metrics.
    pub agent: AgentMetrics,
}

/// Metrics representing CPU state.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpuMetrics {
    /// Current CPU usage percentage (0.0 - 100.0).
    pub usage_pct: f32,
    /// Current CPU temperature in Celsius.
    pub temp_c: f32,
}

/// Metrics representing system memory (RAM) usage.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryMetrics {
    /// Total RAM in KB.
    pub total_kb: u64,
    /// Used RAM in KB.
    pub used_kb: u64,
    /// RAM usage percentage (0.0 - 100.0).
    pub usage_pct: f32,
}

/// Metrics representing the Zinko Agent's own resource footprint.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentMetrics {
    /// Agent's CPU usage percentage.
    pub cpu_pct: f32,
    /// Agent's memory usage in KB.
    pub mem_kb: u64,
    /// Agent's memory usage as percentage of total system RAM.
    pub mem_pct: f32,
}

/// Metrics representing storage device health and state.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageMetrics {
    /// Storage device health percentage (0.0 - 100.0).
    pub health_pct: f32,
    /// Current storage device temperature in Celsius.
    pub temp_c: f32,
}

/// Metrics representing battery health and capacity.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatteryMetrics {
    /// Total charge cycles the battery has completed.
    pub cycles: u32,
    /// Battery health percentage based on original capacity.
    pub health_pct: f32,
    /// Current estimated battery capacity in mAh.
    pub capacity_mah: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_serialization() {
        let data = TelemetryData {
            timestamp: Utc::now(),
            device_id: "test".to_string(),
            cpu: CpuMetrics {
                usage_pct: 10.0,
                temp_c: 50.0,
            },
            storage: StorageMetrics {
                health_pct: 100.0,
                temp_c: 30.0,
            },
            battery: BatteryMetrics {
                cycles: 10,
                health_pct: 100.0,
                capacity_mah: 5000,
            },
            memory: MemoryMetrics {
                total_kb: 16000000,
                used_kb: 8000000,
                usage_pct: 50.0,
            },
            os_name: "Windows".to_string(),
            os_version: "11".to_string(),
            kernel_version: "23H2".to_string(),
            hostname: "ZINKO-PC".to_string(),
            agent: AgentMetrics {
                cpu_pct: 0.1,
                mem_kb: 5000,
                mem_pct: 0.03,
            },
        };
        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("\"device_id\":\"test\""));
    }

    #[test]
    fn test_telemetry_deserialization() {
        let data = TelemetryData {
            timestamp: Utc::now(),
            device_id: "roundtrip".to_string(),
            cpu: CpuMetrics {
                usage_pct: 25.0,
                temp_c: 55.0,
            },
            storage: StorageMetrics {
                health_pct: 90.0,
                temp_c: 35.0,
            },
            battery: BatteryMetrics {
                cycles: 200,
                health_pct: 85.0,
                capacity_mah: 4000,
            },
            memory: MemoryMetrics {
                total_kb: 8000000,
                used_kb: 4000000,
                usage_pct: 50.0,
            },
            os_name: "Linux".to_string(),
            os_version: "22.04".to_string(),
            kernel_version: "5.15".to_string(),
            hostname: "test-pc".to_string(),
            agent: AgentMetrics {
                cpu_pct: 0.5,
                mem_kb: 10000,
                mem_pct: 0.12,
            },
        };

        // Serialize and deserialize to verify round-trip integrity
        let json = serde_json::to_string(&data).unwrap();
        let deserialized: TelemetryData = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.device_id, "roundtrip");
        assert_eq!(deserialized.cpu.temp_c, 55.0);
        assert_eq!(deserialized.agent.mem_kb, 10000);
    }

    #[test]
    fn test_memory_metrics_percentage() {
        let mem = MemoryMetrics {
            total_kb: 16000000,
            used_kb: 12000000,
            usage_pct: 75.0,
        };
        assert!(mem.usage_pct > 0.0 && mem.usage_pct <= 100.0);
        assert!(mem.used_kb <= mem.total_kb);
    }
}
