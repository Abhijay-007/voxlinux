use regex::Regex;
use serde_json::json;

use crate::{
    intent::{Intent, IntentType},
    error::IntentError,
};

pub fn parse_input(input: &str) -> Result<Intent, IntentError> {
    // STEP 1 already mentioned (you should also add trim for correctness)
    let text = input.trim().to_lowercase();

    // baseline rules
    let launch = Regex::new(r"(open|launch|start)\s+([a-z0-9_\-./]+)").unwrap();
    let open_file = Regex::new(r#"(open)\s+['"]?(.+?)['"]?$"#).unwrap();
    let set_brightness = Regex::new(r"(set|brightness)\s+(\d{1,3})").unwrap();
    let inc_brightness = Regex::new(r"(increase|raise|up)\s+brightness\s*(\d{1,3})?").unwrap();
    let dec_brightness = Regex::new(r"(decrease|lower|down)\s+brightness\s*(\d{1,3})?").unwrap();

    // kernel intent rules
    let cpu_perf = Regex::new(r"(set)\s+cpu\s+performance").unwrap();
    let cpu_power = Regex::new(r"(set)\s+cpu\s+powersave").unwrap();
    let cpu_get = Regex::new(r"(get)\s+cpu\s+governor").unwrap();
    let wifi_on = Regex::new(r"(wifi)\s+on").unwrap();
    let wifi_off = Regex::new(r"(wifi)\s+off").unwrap();
    let volume = Regex::new(r"(set)\s+volume\s+(\d{1,3})").unwrap();
    let sleep = Regex::new(r"(sleep)$").unwrap();

    if let Some(c) = open_file.captures(&text) {
        return Ok(Intent {
            intent_type: IntentType::OpenFile,
            parameters: json!({ "path": c[2].to_string() }),
        });
    }

    if let Some(c) = launch.captures(&text) {
        return Ok(Intent {
            intent_type: IntentType::LaunchApp,
            parameters: json!({ "app": c[2].to_string() }),
        });
    }

    if let Some(c) = set_brightness.captures(&text) {
        let percent: u64 = c[2].parse().map_err(|_| IntentError::InvalidParameters)?;
        return Ok(Intent {
            intent_type: IntentType::SetBrightness,
            parameters: json!({ "percent": percent }),
        });
    }

    if let Some(c) = inc_brightness.captures(&text) {
        let delta = c.get(2).map(|m| m.as_str().parse().unwrap_or(10)).unwrap_or(10);
        return Ok(Intent {
            intent_type: IntentType::IncreaseBrightness,
            parameters: json!({ "delta": delta }),
        });
    }

    if let Some(c) = dec_brightness.captures(&text) {
        let delta = c.get(2).map(|m| m.as_str().parse().unwrap_or(10)).unwrap_or(10);
        return Ok(Intent {
            intent_type: IntentType::DecreaseBrightness,
            parameters: json!({ "delta": delta }),
        });
    }

    // kernel intents
    if cpu_perf.is_match(&text) {
        return Ok(Intent {
            intent_type: IntentType::SetCpuPerformance,
            parameters: json!({}),
        });
    }

    if cpu_power.is_match(&text) {
        return Ok(Intent {
            intent_type: IntentType::SetCpuPowersave,
            parameters: json!({}),
        });
    }

    if cpu_get.is_match(&text) {
        return Ok(Intent {
            intent_type: IntentType::GetCpuGovernor,
            parameters: json!({}),
        });
    }

    if wifi_on.is_match(&text) {
        return Ok(Intent {
            intent_type: IntentType::WifiOn,
            parameters: json!({}),
        });
    }

    if wifi_off.is_match(&text) {
        return Ok(Intent {
            intent_type: IntentType::WifiOff,
            parameters: json!({}),
        });
    }

    if let Some(c) = volume.captures(&text) {
        let vol: u64 = c[2].parse().unwrap_or(50);
        return Ok(Intent {
            intent_type: IntentType::SetVolume,
            parameters: json!({ "percent": vol }),
        });
    }

    if sleep.is_match(&text) {
        return Ok(Intent {
            intent_type: IntentType::SleepSystem,
            parameters: json!({}),
        });
    }

    // ===========================
    // STEP 2 — NEW RULES MOVE HERE
    // ===========================

    // wifi connect <ssid> [password]
    // wifi connect (auto mode)
    let wifi_connect_auto = Regex::new(r#"^wifi\s+connect$"#).unwrap();
    if wifi_connect_auto.is_match(&text) {
        return Ok(Intent {
            intent_type: IntentType::WifiConnect,
            parameters: json!({}), // auto connect behavior
        });
    }

    // wifi connect SSID [PASSWORD]
    let wifi_connect = Regex::new(r#"^wifi\s+connect\s+['"]?(.+?)['"]?(?:\s+['"]?(.+?)['"]?)?$"#).unwrap();
    if let Some(c) = wifi_connect.captures(&text) {
        return Ok(Intent {
            intent_type: IntentType::WifiConnect,
            parameters: json!({
                "ssid": c.get(1).unwrap().as_str(),
                              "password": c.get(2).map(|m| m.as_str()),
            }),
        });
    }


    // battery
    let battery = Regex::new(r"^(battery|get\s+battery|battery\s+status)$").unwrap();
    if battery.is_match(&text) {
        return Ok(Intent {
            intent_type: IntentType::BatteryStatus,
            parameters: json!({}),
        });
    }

    // media
    let media = Regex::new(r"^media\s+(play|pause|next|prev)$").unwrap();
    if let Some(c) = media.captures(&text) {
        let action = c.get(1).unwrap().as_str();
        return Ok(match action {
            "next" => Intent { intent_type: IntentType::MediaNext, parameters: json!({}) },
                  "prev" => Intent { intent_type: IntentType::MediaPrev, parameters: json!({}) },
                  _ => Intent { intent_type: IntentType::MediaPlayPause, parameters: json!({}) },
        });
    }

    // lock / lock screen
    let lock = Regex::new(r"^lock(\s+screen)?$").unwrap();
    if lock.is_match(&text) {
        return Ok(Intent {
            intent_type: IntentType::LockScreen,
            parameters: json!({}),
        });
    }

    // bluetooth
    let bt = Regex::new(r"^bluetooth\s+(on|off|toggle)$").unwrap();
    if let Some(c) = bt.captures(&text) {
        return Ok(Intent {
            intent_type: IntentType::BluetoothToggle,
            parameters: json!({ "action": c.get(1).unwrap().as_str() }),
        });
    }

    // ===========================
    // DO NOT MOVE THIS
    // ===========================

    Err(IntentError::ParseError)
}
