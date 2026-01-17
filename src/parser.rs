use regex::Regex;
use serde_json::json;

use crate::{
    intent::{Intent, IntentType},
    error::IntentError,
};

pub fn parse_input(input: &str) -> Result<Intent, IntentError> {
    let text = input.to_lowercase();

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

    Err(IntentError::ParseError)
}


