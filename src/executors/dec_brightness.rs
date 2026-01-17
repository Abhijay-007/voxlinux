use std::fs;

use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct DecreaseBrightnessExecutor;

impl IntentExecutor for DecreaseBrightnessExecutor {
    fn execute(&self, intent: &Intent) -> Result<(), IntentError> {
        let delta = intent.parameters["delta"].as_u64().unwrap_or(10);

        let base = "/sys/class/backlight";
        for entry in fs::read_dir(base)? {
            let path = entry?.path();
            let b = path.join("brightness");

            if b.exists() {
                let cur: u64 = fs::read_to_string(&b)?.trim().parse().unwrap();
                let new = cur.saturating_sub(delta);
                fs::write(b, new.to_string())?;
                return Ok(());
            }
        }

        Err(IntentError::ProcessFailed)
    }
}

