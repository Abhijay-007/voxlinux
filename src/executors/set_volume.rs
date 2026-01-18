use std::process::Command;
use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct SetVolumeExecutor;

impl IntentExecutor for SetVolumeExecutor {
    fn execute(&self, intent: &Intent) -> Result<(), IntentError> {
        let percent = intent.parameters["percent"].as_u64().unwrap_or(50);
        let percent_str = format!("{}%", percent);

        // Try wpctl first (PipeWire native)
        if Command::new("wpctl").arg("--help").output().is_ok() {
            let status = Command::new("wpctl")
                .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &percent_str])
                .status()
                .map_err(|_| IntentError::ProcessFailed)?;
            if status.success() {
                return Ok(());
            }
        }

        // Fallback: pactl (PulseAudio or pulse-bridge)
        if Command::new("pactl").arg("--version").output().is_ok() {
            let status = Command::new("pactl")
                .args(["set-sink-volume", "@DEFAULT_SINK@", &percent_str])
                .status()
                .map_err(|_| IntentError::ProcessFailed)?;
            if status.success() {
                return Ok(());
            }
        }

        Err(IntentError::ProcessFailed)
    }
}
<<<<<<< HEAD
=======



>>>>>>> 244ee09 (message)
