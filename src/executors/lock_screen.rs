use std::process::Command;

use crate::{
    error::IntentError,
    executor::IntentExecutor,
    intent::Intent,
};

pub struct LockScreenExecutor;

impl IntentExecutor for LockScreenExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        let status = Command::new("loginctl")
        .arg("lock-session")
        .status()
        .map_err(|_| IntentError::ProcessFailed)?;

        if !status.success() {
            println!("Failed to lock session.");
            return Err(IntentError::ProcessFailed);
        }

        println!("Session locked.");
        Ok(())
    }
}
