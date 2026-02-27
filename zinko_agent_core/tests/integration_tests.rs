use chrono::Utc;
use zinko_agent_core::models::{
    AgentMetrics, BatteryMetrics, CpuMetrics, MemoryMetrics, StorageMetrics, TelemetryData,
};
use zinko_agent_core::simulator::Simulator;

#[test]
fn test_end_to_end_telemetry_flow() {
    // 1. Create a base telemetry packet
    let mut data = TelemetryData {
        timestamp: Utc::now(),
        device_id: "INT-TEST-001".to_string(),
        cpu: CpuMetrics {
            usage_pct: 10.0,
            temp_c: 45.0,
        },
        storage: StorageMetrics {
            health_pct: 100.0,
            temp_c: 35.0,
        },
        battery: BatteryMetrics {
            cycles: 10,
            health_pct: 95.0,
            capacity_mah: 4500,
        },
        memory: MemoryMetrics {
            total_kb: 16000000,
            used_kb: 8000000,
            usage_pct: 50.0,
        },
        os_name: "Test".to_string(),
        os_version: "1.0".to_string(),
        kernel_version: "5.0".to_string(),
        hostname: "int-test".to_string(),
        agent: AgentMetrics {
            cpu_pct: 0.0,
            mem_kb: 0,
            mem_pct: 0.0,
        },
    };

    // 2. Verify initial state
    assert_eq!(data.device_id, "INT-TEST-001");
    assert!(data.cpu.temp_c < 90.0);

    // 3. Apply simulator overrides (Integration point)
    // In a real integration test we might check for file presence,
    // but here we are testing the logic's integration.
    Simulator::apply_overrides(&mut data);

    // 4. Verify that data is still valid JSON (Serialization test)
    let json = serde_json::to_string(&data).expect("Should serialize to JSON");
    let deserialized: TelemetryData =
        serde_json::from_str(&json).expect("Should deserialize from JSON");

    assert_eq!(deserialized.device_id, "INT-TEST-001");
}

#[test]
fn test_telemetry_history_window() {
    use std::sync::mpsc;
    use zinko_agent_core::app::ZinkoApp;

    let (_tx, rx) = mpsc::channel();
    let mut app = ZinkoApp::new(rx);

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
        memory: MemoryMetrics {
            total_kb: 16000000,
            used_kb: 8000000,
            usage_pct: 50.0,
        },
        os_name: "Test".to_string(),
        os_version: "1.0".to_string(),
        kernel_version: "5.0".to_string(),
        hostname: "test".to_string(),
        agent: AgentMetrics {
            cpu_pct: 0.0,
            mem_kb: 0,
            mem_pct: 0.0,
        },
    };

    // Push more than max_history
    for _ in 0..110 {
        app.update_data(data.clone());
    }

    assert!(app.telemetry_history.len() <= 100);
}
