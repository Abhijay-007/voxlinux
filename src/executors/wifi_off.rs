use std::process::Command;
use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct WifiOffExecutor;

impl IntentExecutor for WifiOffExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        Command::new("rfkill")
            .arg("block")
            .arg("wifi")
            .status()
            .map_err(|_| IntentError::ProcessFailed)?;
        Ok(())
    }
}

