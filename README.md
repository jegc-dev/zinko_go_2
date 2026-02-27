# 🛡️ Zinko Go 2.0 — Transparency Agent

**A lightweight, real-time IT asset monitoring agent built in Rust with native GUI, cloud alerting, and ITAM-grade device identification.**

Zinko Go is an endpoint telemetry agent designed for IT Asset Management (ITAM). It continuously monitors hardware health, collects system metrics, and sends intelligent alerts to the cloud when anomalies are detected — all while maintaining full transparency with the end user through a real-time desktop interface.

---

## 📸 Architecture Overview

```
┌──────────────────────┐       HTTPS/JSON        ┌──────────────────────┐
│   Zinko Agent (Rust)  │ ──────────────────────► │  GCP Cloud Run       │
│   Desktop App         │   Alert + Telemetry     │  (Python Webhook)    │
│   - Hardware Metrics   │   + Device ID (ITAM)    │                      │
│   - System Info        │                         └──────────┬───────────┘
│   - Alert Engine       │                                    │
│   - Native GUI (egui)  │                                    ▼
└──────────────────────┘                          ┌──────────────────────┐
                                                  │  Slack Workspace     │
                                                  │  #zinko-alerts       │
                                                  └──────────────────────┘
```

---

## ✨ Key Features

### 🖥️ Real-Time Desktop Dashboard
- **Hardware Health** — SSD life percentage, battery health, charge cycles
- **Resource Utilization** — Live CPU usage, RAM consumption with progress bars
- **CPU Temperature** — Real-time bar chart with thermal color coding (cyan → orange → red)
- **Agent Footprint** — Self-monitoring: shows Zinko's own CPU and RAM usage vs. total system resources, proving how lightweight the agent is

### 🔍 Privacy Hub
- Full transparency: displays the **exact JSON payload** being sent to the cloud
- Users can audit every data point collected in real time

### ⚙️ System Info (ITAM)
- **Device ID** — Hardware serial number extracted via WMI (csproduct → baseboard → bios → diskdrive fallback chain)
- **Hostname, OS, OS Version, Kernel Version**
- **WMI Hardware Audit** — Disaggregated view of all hardware identifiers with visual indicators for invalid/placeholder values
- **Agent Version** tracking

### 🚨 Heuristic Alert Engine
- Rule-based anomaly detection with configurable thresholds:
  - **CPU Temperature** > 90°C → `CRITICAL`
  - **SSD Health** < 10% → `CRITICAL`
  - **Battery Health** < 70% → `WARNING`
- Debounce mechanism to prevent alert spam
- Alerts sent to cloud webhook with full ITAM context

### 🎭 Simulation Engine
- File-based trigger system for demo/testing:
  - `fail_temp.trigger` → Simulates CPU overheating (92.5°C)
  - `fail_battery.trigger` → Simulates battery degradation (65% health, 1050 cycles)
  - `fail_disk.trigger` → Simulates SSD failure (5% health)

---

## 🏗️ Project Structure

```
zinko_go/
├── README.md
├── zinko_agent_core/           # Rust Agent (main application)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs             # Entry point, agent thread, UI launch
│   │   ├── lib.rs              # Library exports
│   │   ├── models.rs           # Data structures (TelemetryData, AgentMetrics, etc.)
│   │   ├── app.rs              # GUI application (egui/eframe)
│   │   ├── alerts.rs           # Heuristic alert engine + webhook sender
│   │   ├── simulator.rs        # File-based failure simulation
│   │   └── telemetry/
│   │       └── mod.rs          # Hardware metrics collection (CPU, RAM, WMI serials)
│   └── tests/
│       └── integration_tests.rs
├── cloud_function/             # GCP Cloud Run service (Python)
│   ├── main.py                 # Webhook receiver + Slack forwarder
│   └── requirements.txt
└── doc/
    ├── PRODUCTION_ROADMAP.md   # Productionization roadmap
    └── OFFICIAL_DEMO_GUIDE.md  # Step-by-step demo script
```

---

## 🔧 Tech Stack

| Layer | Technology | Purpose |
|:---|:---|:---|
| **Agent** | Rust | High-performance, memory-safe system monitoring |
| **GUI** | egui / eframe | Native, immediate-mode desktop interface |
| **Metrics** | sysinfo | Cross-platform CPU, RAM, process introspection |
| **Device ID** | WMI (wmic.exe) | ITAM-grade hardware serial extraction |
| **Serialization** | serde / serde_json | Telemetry data serialization |
| **HTTP** | reqwest | Async webhook delivery to cloud |
| **Async Runtime** | tokio | Async operations from synchronous agent thread |
| **Cloud** | GCP Cloud Run | Serverless webhook processing |
| **Notifications** | Slack Incoming Webhooks | Real-time alert delivery |
| **Charting** | egui_plot | Temperature bar charts in the dashboard |

---

## 🧩 Software Architecture

### Design Patterns

The project follows established software architecture patterns to ensure maintainability, testability, and separation of concerns:

#### 1. Modular Architecture (Separation of Concerns)
Each module has a single, well-defined responsibility:

```
┌─────────────┐   ┌──────────────┐   ┌──────────────┐   ┌────────────┐
│  telemetry  │   │    models    │   │    alerts    │   │ simulator  │
│  (collect)  │──►│   (data)     │──►│  (evaluate)  │   │  (testing) │
└─────────────┘   └──────┬───────┘   └──────┬───────┘   └────────────┘
                         │                  │
                         ▼                  ▼
                  ┌──────────────┐   ┌──────────────┐
                  │     app      │   │   webhook    │
                  │    (GUI)     │   │   (cloud)    │
                  └──────────────┘   └──────────────┘
```

| Module | Responsibility | Pattern |
|:---|:---|:---|
| `models.rs` | Data structures and serialization contracts | **Data Transfer Object (DTO)** |
| `telemetry/mod.rs` | Hardware metrics collection and WMI queries | **Service / Adapter** |
| `alerts.rs` | Rule evaluation, debounce, and webhook delivery | **Strategy + Observer** |
| `simulator.rs` | File-based telemetry override for testing | **Decorator** |
| `app.rs` | Desktop UI rendering and state management | **Immediate Mode GUI (IMGUI)** |
| `main.rs` | Orchestration of threads, channels, and lifecycle | **Mediator** |

#### 2. Producer-Consumer (Channel-Based Concurrency)
The agent uses Rust's `mpsc` (Multi-Producer, Single-Consumer) channels to decouple the telemetry collection thread from the UI thread:

```
┌──────────────────┐     mpsc::channel     ┌──────────────────┐
│  Agent Thread    │ ────────────────────► │  UI Thread       │
│  (producer)      │    TelemetryData      │  (consumer)      │
│  - collect()     │                       │  - render()      │
│  - evaluate()    │                       │  - update_data() │
│  - send_alert()  │                       │                  │
└──────────────────┘                       └──────────────────┘
```

This pattern ensures:
- **Non-blocking UI** — The GUI never waits for hardware queries
- **Thread safety** — Data crosses thread boundaries via owned values (no shared state)
- **Graceful shutdown** — If the UI closes, `tx.send()` returns `Err` and the agent thread exits

#### 3. Pipeline Architecture (Cloud Alert Flow)
Alerts follow a linear pipeline with clear transformation stages:

```
Telemetry → Rules Engine → Alert → Enrichment (ITAM) → HTTP POST → GCP → Slack
```

Each stage is independently testable and replaceable.

#### 4. Strategy Pattern (Alert Rules)
The `AlertSystem` evaluates multiple independent rules against the same telemetry data. Each rule is a self-contained strategy with its own threshold, severity level, and debounce tracking. New rules can be added without modifying existing ones.

#### 5. Fallback Chain (Device Identification)
The WMI serial number extraction follows the **Chain of Responsibility** pattern, trying multiple sources in priority order (`csproduct → baseboard → bios → diskdrive → hostname`) until a valid identifier is found.

#### 6. Decorator Pattern (Simulation)
The `Simulator::apply_overrides()` function acts as a decorator that can modify telemetry data before it reaches the alert engine and UI, without changing the collection logic. This enables testing and demonstration of failure scenarios without actual hardware faults.

---

## 🚀 Getting Started

### Prerequisites

- **Rust** (1.75+ recommended) — [Install](https://rustup.rs/)
- **Windows** machine (or WSL on Windows) for WMI serial number extraction
- **GCP Account** (optional, for cloud alerting)
- **Slack Workspace** (optional, for notifications)

### Build & Run

```bash
# Clone the repository
git clone https://github.com/jegc-dev/zinko_go_2.git
cd zinko_go_2/zinko_agent_core

# Build the agent
cargo build --release

# Run with GUI
cargo run --release

# Run in headless/CLI mode
cargo run --release -- --cli

# Run with cloud alerting enabled
ZINKO_WEBHOOK='https://your-cloud-function-url.run.app' cargo run --release
```

### Run Tests

```bash
cargo test
```

**Test Suite (11 tests):**

| Module | Tests |
|:---|:---|
| `models` | Serialization, deserialization (round-trip), memory bounds |
| `alerts` | CPU threshold, SSD threshold, battery alert, healthy baseline |
| `simulator` | Override application, no-override baseline |
| `main` | Cross-thread channel communication |
| `integration` | End-to-end telemetry flow, history window bounds |

### Unit Tests

Unit tests validate individual module behavior in isolation. All tests use mock `TelemetryData` instances with controlled values.

#### `models` — Data Integrity (3 tests)

| Test | What It Validates |
|:---|:---|
| `test_telemetry_serialization` | Verifies that a `TelemetryData` struct serializes to JSON correctly, including the `device_id` field. Ensures the serde configuration works for all fields. |
| `test_telemetry_deserialization` | Performs a full **JSON round-trip** (serialize → deserialize) and validates that all field values survive the transformation intact, including nested structs like `AgentMetrics`. |
| `test_memory_metrics_percentage` | Validates that `MemoryMetrics` field values satisfy logical constraints: `usage_pct` is between 0-100% and `used_kb` does not exceed `total_kb`. |

#### `alerts` — Heuristic Alert Engine (3 tests)

| Test | What It Validates |
|:---|:---|
| `test_alert_thresholds` | Injects telemetry with CPU temp at 95°C (above 90°C threshold). Asserts exactly one `CRITICAL` alert fires with the correct message. Then modifies SSD health to 5% (below 10% threshold) and verifies the SSD alert fires independently. |
| `test_battery_alert` | Injects telemetry with battery health at 60% (below 70% threshold). Asserts a `WARNING`-level alert fires with the "Battery Degradation" message. Validates that battery rules use `WARNING` severity, not `CRITICAL`. |
| `test_no_alerts_when_healthy` | Injects perfectly healthy telemetry (temp 45°C, SSD 95%, battery 95%). Asserts that **zero alerts** fire, verifying there are no false positives under normal operating conditions. |

#### `simulator` — Failure Simulation (2 tests)

| Test | What It Validates |
|:---|:---|
| `test_simulator_overrides` | Creates a `fail_temp.trigger` file in an isolated temp directory, runs `apply_overrides()`, and asserts that `cpu.temp_c` is overridden to `92.5°C`. Cleans up the temp directory after execution to prevent race conditions with parallel tests. |
| `test_no_overrides_when_files_missing` | Ensures that no trigger files exist, runs `apply_overrides()`, and asserts that all telemetry values remain **unchanged** (temp stays at 40°C, SSD at 100%, battery at 0 cycles). |

#### `main` — Thread Communication (1 test)

| Test | What It Validates |
|:---|:---|
| `test_channel_communication` | Creates an `mpsc` channel, sends a `TelemetryData` packet through it, and verifies the receiver gets the same `device_id`. Validates the Producer-Consumer pattern that connects the agent thread to the UI. |

### Integration Tests

Integration tests (`tests/integration_tests.rs`) validate the interaction between multiple modules working together.

| Test | Modules Tested | What It Validates |
|:---|:---|:---|
| `test_end_to_end_telemetry_flow` | `models` + `simulator` + `serde` | Creates a telemetry packet, passes it through `Simulator::apply_overrides()`, serializes to JSON, deserializes back, and verifies the `device_id` survives the full pipeline. Tests the complete data flow from creation → transformation → serialization → deserialization. |
| `test_telemetry_history_window` | `models` + `app` | Creates a `ZinkoApp`, pushes 110 data packets into it (exceeding the `max_history` of 100), and asserts the circular buffer never exceeds 100 entries. Validates the sliding window mechanism that prevents unbounded memory growth. |

---

## 🌐 Cloud Integration

### Alert Flow

1. **Agent** detects anomaly via heuristic rules
2. **Agent** sends HTTP POST to GCP Cloud Run with enriched payload:
   ```json
   {
     "content": "🚨 Zinko Alert [CRITICAL]: High CPU Temperature detected: 92.5°C",
     "device_id": "ARB20A113312H2676",
     "hostname": "DESKTOP-A5UQ8T8",
     "os_name": "Windows",
     "os_version": "11 (26200)",
     "cpu_temp_c": 92.5,
     "cpu_usage_pct": 12.5,
     "ram_usage_pct": 45.2,
     "ssd_health_pct": 98.0,
     "battery_health_pct": 95.0,
     "battery_cycles": 120,
     "alert_level": "CRITICAL"
   }
   ```
3. **GCP Cloud Run** receives, logs, and forwards enriched notification to Slack
4. **Slack** displays formatted alert with ITAM device context and telemetry snapshot

### GCP Deployment

```bash
cd cloud_function

gcloud run deploy zinko-bridge \
  --source . \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars SLACK_WEBHOOK_URL="https://hooks.slack.com/services/YOUR/WEBHOOK/URL"
```

---

## 🎮 Demo Simulation

Trigger hardware failure scenarios by creating files in the agent's working directory:

```bash
# Simulate CPU overheating (92.5°C)
touch fail_temp.trigger

# Simulate battery degradation
touch fail_battery.trigger

# Simulate SSD failure
touch fail_disk.trigger

# Remove triggers to restore normal operation
rm -f fail_temp.trigger fail_battery.trigger fail_disk.trigger
```

---

## 📊 Device Identification (ITAM)

The agent extracts a unique hardware identifier using a multi-source WMI fallback chain:

| Priority | WMI Source | Property | Description |
|:---:|:---|:---|:---|
| 1 | `csproduct` | `identifyingnumber` | System product serial (most reliable) |
| 2 | `baseboard` | `serialnumber` | Motherboard serial |
| 3 | `bios` | `serialnumber` | BIOS serial |
| 4 | `diskdrive` | `serialnumber` | Primary disk serial |

Generic placeholder values (`"Default string"`, `"To be filled by O.E.M."`, etc.) are automatically filtered. If no valid serial is found, the system hostname is used as fallback.

---

## 📄 License

This project is proprietary to Zinko. All rights reserved.

---

## 👤 Author

**Jorge Granados** — [jegc-dev](https://github.com/jegc-dev)
