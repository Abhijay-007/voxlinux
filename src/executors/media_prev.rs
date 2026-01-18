use std::process::Command;

use crate::{
    error::IntentError,
    executor::IntentExecutor,
    intent::Intent,
};

pub struct MediaPrevExecutor;

impl IntentExecutor for MediaPrevExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        let status = Command::new("playerctl")
        .arg("previous")
        .status()
        .map_err(|_| IntentError::ProcessFailed)?;

        if !status.success() {
            println!("Media previous failed.");
            return Err(IntentError::ProcessFailed);
        }

        println!("Media skipped to previous.");
        Ok(())
    }
}
