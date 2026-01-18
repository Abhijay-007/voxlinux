use std::process::Command;

use crate::{
    error::IntentError,
    executor::IntentExecutor,
    intent::Intent,
};

pub struct MediaPlayPauseExecutor;

impl IntentExecutor for MediaPlayPauseExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        let status = Command::new("playerctl")
        .arg("play-pause")
        .status()
        .map_err(|_| IntentError::ProcessFailed)?;

        if !status.success() {
            println!("Media play/pause failed.");
            return Err(IntentError::ProcessFailed);
        }

        println!("Media toggled (play/pause).");
        Ok(())
    }
}
