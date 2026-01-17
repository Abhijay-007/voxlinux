use std::process::Command;
use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct WifiOnExecutor;

impl IntentExecutor for WifiOnExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        Command::new("rfkill")
            .arg("unblock")
            .arg("wifi")
            .status()
            .map_err(|_| IntentError::ProcessFailed)?;
        Ok(())
    }
}

