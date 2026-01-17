use std::process::Command;
use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct SleepSystemExecutor;

impl IntentExecutor for SleepSystemExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        Command::new("systemctl")
            .arg("suspend")
            .status()
            .map_err(|_| IntentError::ProcessFailed)?;
        Ok(())
    }
}

