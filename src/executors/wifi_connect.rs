use std::process::Command;
use crate::{
    error::IntentError,
    executor::IntentExecutor,
    intent::Intent,
};
use serde_json::Value;

pub struct WifiConnectExecutor;

impl IntentExecutor for WifiConnectExecutor {
    fn execute(&self, intent: &Intent) -> Result<(), IntentError> {
        // Extract SSID + password from parser parameters if provided
        let (ssid, password) = match &intent.parameters {
            Value::Object(map) => {
                let ssid = map.get("ssid").and_then(|v| v.as_str()).unwrap_or("").trim();
                let password = map.get("password").and_then(|v| v.as_str()).unwrap_or("").trim();
                (ssid, password)
            }
            _ => ("", "")
        };

        // AUTO MODE (no params)
        if ssid.is_empty() {
            return auto_connect();
        }

        // Check saved connections
        let output = Command::new("nmcli")
        .args(["-t", "-f", "NAME,TYPE", "connection", "show"])
        .output()
        .map_err(|_| IntentError::ProcessFailed)?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        let saved_wifi: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(':');
            let name = parts.next()?;
            let ty = parts.next()?;
            if ty == "wifi" { Some(name.to_string()) } else { None }
        })
        .collect();

        // Case 2: SSID exists as saved
        if saved_wifi.contains(&ssid.to_string()) {
            println!("Connecting to saved WiFi: {}", ssid);
            let status = Command::new("nmcli")
            .args(["connection", "up", ssid])
            .status()
            .map_err(|_| IntentError::ProcessFailed)?;

            if !status.success() {
                println!("Failed to activate saved WiFi: {}", ssid);
                return Err(IntentError::ProcessFailed);
            }

            println!("Connected to WiFi: {}", ssid);
            return Ok(());
        }

        // Case 3: New SSID but password missing
        if password.is_empty() {
            println!("WiFi '{}' is not saved. Password required.", ssid);
            return Err(IntentError::InvalidParameters);
        }

        // Case 4: New SSID + password → full connect
        println!("Connecting to new WiFi: {}", ssid);
        let status = Command::new("nmcli")
        .args(["device", "wifi", "connect", ssid, "password", password])
        .status()
        .map_err(|_| IntentError::ProcessFailed)?;

        if !status.success() {
            println!("Failed to connect to WiFi: {}", ssid);
            return Err(IntentError::ProcessFailed);
        }

        println!("Connected to WiFi: {}", ssid);
        Ok(())
    }
}

// Auto mode for saved WiFi
fn auto_connect() -> Result<(), IntentError> {
    let output = Command::new("nmcli")
    .args(["-t", "-f", "NAME,TYPE", "connection", "show"])
    .output()
    .map_err(|_| IntentError::ProcessFailed)?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let wifi_cons: Vec<String> = stdout
    .lines()
    .filter_map(|l| {
        let mut parts = l.split(':');
        let name = parts.next()?;
        let ty = parts.next()?;
        if ty == "wifi" { Some(name.to_string()) } else { None }
    })
    .collect();

    if wifi_cons.is_empty() {
        println!("No saved WiFi connections found.");
        return Err(IntentError::ProcessFailed);
    }

    let target = &wifi_cons[0];

    println!("Auto-connecting to WiFi: {}", target);

    let status = Command::new("nmcli")
    .args(["connection", "up", target])
    .status()
    .map_err(|_| IntentError::ProcessFailed)?;

    if !status.success() {
        println!("WiFi connection failed.");
        return Err(IntentError::ProcessFailed);
    }

    println!("Connected to WiFi: {}", target);
    Ok(())
}
