# 🏆 Zinko Go 2.0 - Official Demo Protocol

This document outlines the standard operating procedure for the Zinko Go 2.0 technical demonstration, ensuring a seamless flow from local telemetry collection to predictive cloud alerting.

---

## 🛠️ 1. Preparation & Prerequisites

### Personnel Requirements
- **Hardware Agent:** Access to a terminal with Rust/Cargo.
- **Cloud Console:** Access to the `zinko-go-demo` GCP project.
- **Alert Channel:** Access to the destination Slack/Discord channel.

### Configuration
Ensure the communication bridge is configured in your environment:
- **Service URL:** `https://zinko-bridge-629795870953.us-central1.run.app`
- **Cmd (PowerShell):** `$env:ZINKO_WEBHOOK="https://zinko-bridge-629795870953.us-central1.run.app"`
- **Cmd (bash):** `export ZINKO_WEBHOOK="https://zinko-bridge-629795870953.us-central1.run.app"`

---

## 🚀 2. Phase 1: Local Transparency (Frontend)

**Goal:** Demonstrate the "Transparency Dashboard" and real-time hardware monitoring.

1.  **Launch the Agent:**
    ```bash
    cd zinko_agent_core
    cargo run --release
    ```
2.  **Verify Dashboard:**
    - Point out the **Real-time CPU Temperature Graph**.
    - Explain the **Hardware Health Indicators** (SSD/Battery).
    - Show the **Cloud Sync: Active** green status light.
3.  **Demonstrate Privacy Hub:**
    - Switch to the "🔍 Privacy Hub" tab.
    - Explain: *"This shows the exact hardware telemetry being shared. We only collect health metrics, never personal files."*

---

## 🎭 3. Phase 2: Predictive Failure Simulation

**Goal:** Show the agent detecting problems and reacting instantly.

1.  **Preparation:** Open your terminal side-by-side with the Zinko UI and the Slack/Discord channel.
2.  **Trigger Failure:**
    ```bash
    # Run this in the zinko_agent_core folder
    touch fail_temp.trigger
    ```
3.  **Observe UI Reaction:**
    - The Temperature Graph will spike and turn red.
    - The "Simulation Status" tab will show an active trigger.
    - A local `🚨 ALERT` will print to the terminal.
4.  **Observe Cloud Action:**
    - The notification should arrive at the destination channel in < 3 seconds.

---

## 🔍 4. Phase 3: Cloud Verification (GCP)

**Goal:** Prove that the data reached the cloud and was processed.

1.  **Open Log Explorer:** [Go to zinko-bridge Logs](https://console.cloud.google.com/logs/query?project=zinko-go-demo&query=resource.type%3D%22cloud_run_revision%22%0Aresource.labels.service_name%3D%22zinko-bridge%22%0A%22CRITICAL%22)
2.  **Verification Steps:**
    - Find the `POST` request with the `CRITICAL` alert message.
    - Expand the JSON payload to show the source metrics (e.g., `temp_c: 92.5`).
    - Note the **Latency**: Highlight the time difference between the event and the receipt.

---

## 🧹 5. Cleanup

To return the demo to a "healthy" state:
1.  Remove trigger files: `rm *.trigger`
2.  Verify the UI bars return to green/blue.
3.  Check that cloud traffic stabilizes to "Normal Pulse" levels.

---

## 🚩 Troubleshooting

| Symptom | Check | Resolution |
| :--- | :--- | :--- |
| **No UI window appears** | Graphics Drivers | Run with `cargo run --release -- --cli` for headless mode. |
| **Cloud Sync is Red** | Network | Verify outgoing HTTPS (Port 443) is not blocked. |
| **No alerts in Slack/Discord** | Webhook Env | Re-check `ZINKO_WEBHOOK` variable in the active terminal. |
| **GCP Permission Denied** | Google Account | Use the browser logged into the admin account (authuser=1). |
