use std::fs;

use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct IncreaseBrightnessExecutor;

impl IntentExecutor for IncreaseBrightnessExecutor {
    fn execute(&self, intent: &Intent) -> Result<(), IntentError> {
        let delta = intent.parameters["delta"].as_u64().unwrap_or(10);

        let base = "/sys/class/backlight";
        for entry in fs::read_dir(base)? {
            let path = entry?.path();
            let b = path.join("brightness");
            let max = path.join("max_brightness");

            if b.exists() && max.exists() {
                let cur: u64 = fs::read_to_string(&b)?.trim().parse().unwrap();
                let maxv: u64 = fs::read_to_string(&max)?.trim().parse().unwrap();
                let new = (cur + delta).min(maxv);
                fs::write(b, new.to_string())?;
                return Ok(());
            }
        }

        Err(IntentError::ProcessFailed)
    }
}

