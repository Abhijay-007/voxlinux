use std::process::Command;

use crate::{
    error::IntentError,
    executor::IntentExecutor,
    intent::Intent,
};

pub struct MediaNextExecutor;

impl IntentExecutor for MediaNextExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        let status = Command::new("playerctl")
        .arg("next")
        .status()
        .map_err(|_| IntentError::ProcessFailed)?;

        if !status.success() {
            println!("Media next failed.");
            return Err(IntentError::ProcessFailed);
        }

        println!("Media skipped to next.");
        Ok(())
    }
}
