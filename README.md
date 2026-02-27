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
