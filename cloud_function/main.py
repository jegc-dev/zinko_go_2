"""
Zinko Go 2.0 - GCP Cloud Function (Alert Webhook Receiver)

Receives telemetry alerts from the Zinko Rust Agent via HTTP POST,
enriches the Slack notification with device identification (ITAM),
and forwards it to the configured Slack webhook.

Deployed on: Google Cloud Functions (2nd Gen)
Runtime: Python 3.11+
Trigger: HTTP
"""

import functions_framework
import requests
import json
import os

# Slack Incoming Webhook URL loaded from GCP environment variable (set via Secret Manager or env config)
SLACK_URL = os.environ.get("SLACK_WEBHOOK_URL", "")


@functions_framework.http
def zinko_alerts(request):
    """
    HTTP entry point for the Zinko alert webhook.

    Expected JSON payload from the Rust Agent:
    {
        "content": "🚨 **Zinko Alert [CRITICAL]:** High CPU Temperature detected: 92.5°C",
        "username": "Zinko Transparency Agent",
        "device_id": "ARB20A113312H2676",
        "hostname": "DESKTOP-A5UQ8T8",
        "os_name": "Windows",
        "os_version": "11 (26200)",
        "cpu_usage_pct": 12.5,
        "cpu_temp_c": 92.5,
        "ram_usage_pct": 45.2,
        "ssd_health_pct": 98.0,
        "battery_health_pct": 95.0,
        "battery_cycles": 120,
        "alert_level": "CRITICAL",
        "alert_message": "High CPU Temperature detected: 92.5°C"
    }
    """
    request_json = request.get_json(silent=True)

    # Log the full payload for debugging and audit trail in GCP Cloud Logging
    print(f"DEBUG: Data Received from Rust Agent: {json.dumps(request_json, indent=2)}")

    if not request_json or "content" not in request_json:
        print("ERROR: Invalid payload - missing 'content' field")
        return "No data", 400

    if not SLACK_URL:
        print("ERROR: SLACK_WEBHOOK_URL environment variable is not configured")
        return "Slack webhook not configured", 500

    # Extract device identification fields (ITAM context)
    device_id = request_json.get("device_id", "Unknown")
    hostname = request_json.get("hostname", "Unknown")
    os_info = request_json.get("os_name", "Unknown")
    os_version = request_json.get("os_version", "")
    alert_level = request_json.get("alert_level", "INFO")
    alert_message = request_json.get("alert_message", request_json["content"])

    # Extract telemetry context
    cpu_temp = request_json.get("cpu_temp_c", "N/A")
    cpu_usage = request_json.get("cpu_usage_pct", "N/A")
    ram_usage = request_json.get("ram_usage_pct", "N/A")
    ssd_health = request_json.get("ssd_health_pct", "N/A")
    battery_health = request_json.get("battery_health_pct", "N/A")
    battery_cycles = request_json.get("battery_cycles", "N/A")

    # Build enriched Slack message with ITAM device context
    slack_text = (
        f"🛡️ *Zinko Go Cloud Protection*\n"
        f"━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
        f"*Alert Level:* `{alert_level}`\n"
        f"*Message:* {alert_message}\n"
        f"━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
        f"📟 *Device Identification (ITAM)*\n"
        f"• Device ID: `{device_id}`\n"
        f"• Hostname: `{hostname}`\n"
        f"• OS: {os_info} {os_version}\n"
        f"━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
        f"📊 *Telemetry Snapshot*\n"
        f"• CPU Temp: {cpu_temp}°C | CPU Usage: {cpu_usage}%\n"
        f"• RAM Usage: {ram_usage}%\n"
        f"• SSD Health: {ssd_health}% | Battery: {battery_health}% ({battery_cycles} cycles)\n"
    )

    slack_data = {
        "text": slack_text,
        "username": "Zinko Cloud Logic",
    }

    # Forward the enriched alert to Slack
    response = requests.post(SLACK_URL, json=slack_data)
    print(f"DEBUG: Slack response code: {response.status_code}")

    if response.status_code != 200:
        print(f"ERROR: Slack API Error: {response.text}")
        return f"Slack error: {response.text}", 502

    print(f"INFO: Alert forwarded to Slack for device {device_id} ({hostname})")
    return "Alert Sent", 200
