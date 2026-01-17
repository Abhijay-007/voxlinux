use std::process::Command;
use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct SetVolumeExecutor;

impl IntentExecutor for SetVolumeExecutor {
    fn execute(&self, intent: &Intent) -> Result<(), IntentError> {
        let percent = intent.parameters["percent"].as_u64().unwrap_or(50);

        Command::new("pactl")
            .arg("set-sink-volume")
            .arg("@DEFAULT_SINK@")
            .arg(format!("{}%", percent))
            .status()
            .map_err(|_| IntentError::ProcessFailed)?;

        Ok(())
    }
}


