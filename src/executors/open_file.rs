use std::process::Command;

use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct OpenFileExecutor;

impl IntentExecutor for OpenFileExecutor {
    fn execute(&self, intent: &Intent) -> Result<(), IntentError> {
        let path = intent.parameters["path"]
            .as_str()
            .ok_or(IntentError::InvalidParameters)?;

        Command::new("xdg-open")
            .arg(path)
            .envs(std::env::vars()) // propagate session env
            .spawn()
            .map_err(|_| IntentError::ProcessFailed)?;

        Ok(())
    }
}

