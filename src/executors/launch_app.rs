
use std::process::Command;

use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct LaunchAppExecutor;

impl IntentExecutor for LaunchAppExecutor {
    fn execute(&self, intent: &Intent) -> Result<(), IntentError> {
        let app = intent.parameters["app"]
            .as_str()
            .ok_or(IntentError::InvalidParameters)?;

        Command::new(app)
            .envs(std::env::vars()) // propagate user env for GUI
            .spawn()
            .map_err(|_| IntentError::ProcessFailed)?;

        Ok(())
    }
}

