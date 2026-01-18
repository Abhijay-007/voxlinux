use std::process::Command;

use crate::{
    error::IntentError,
    executor::IntentExecutor,
    intent::Intent,
};

pub struct BatteryStatusExecutor;

impl IntentExecutor for BatteryStatusExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        let output = Command::new("acpi")
        .arg("-b")
        .output()
        .map_err(|_| IntentError::ProcessFailed)?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.trim().is_empty() {
            println!("Battery info not available.");
            return Err(IntentError::ProcessFailed);
        }

        print!("Battery: {}", stdout.trim());
        println!();
        Ok(())
    }
}
