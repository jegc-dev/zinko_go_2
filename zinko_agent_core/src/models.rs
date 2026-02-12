use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelemetryData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub cpu: CpuMetrics,
    pub storage: StorageMetrics,
    pub battery: BatteryMetrics,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpuMetrics {
    pub usage_pct: f32,
    pub temp_c: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageMetrics {
    pub health_pct: f32,
    pub temp_c: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatteryMetrics {
    pub cycles: u32,
    pub health_pct: f32,
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
            cpu: CpuMetrics { usage_pct: 10.0, temp_c: 50.0 },
            storage: StorageMetrics { health_pct: 100.0, temp_c: 30.0 },
            battery: BatteryMetrics { cycles: 10, health_pct: 100.0, capacity_mah: 5000 },
        };
        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("\"device_id\":\"test\""));
    }
}
