use std::fs;

use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct SetBrightnessExecutor;

impl IntentExecutor for SetBrightnessExecutor {
    fn execute(&self, intent: &Intent) -> Result<(), IntentError> {
        let percent = intent.parameters["percent"]
            .as_u64()
            .ok_or(IntentError::InvalidParameters)?;

        let base = "/sys/class/backlight";
        for entry in fs::read_dir(base)? {
            let path = entry?.path();
            let b = path.join("brightness");
            let max = path.join("max_brightness");

            if b.exists() && max.exists() {
                let maxv: u64 = fs::read_to_string(&max)?.trim().parse().unwrap();
                let value = (percent * maxv) / 100;
                fs::write(b, value.to_string())?;
                return Ok(());
            }
        }

        Err(IntentError::ProcessFailed)
    }
}

