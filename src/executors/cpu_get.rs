
use std::fs;

use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct GetCpuGovernorExecutor;

impl IntentExecutor for GetCpuGovernorExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        let gov = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";

        let content = fs::read_to_string(gov)
            .map_err(|_| IntentError::ProcessFailed)?;

        println!("cpu governor: {}", content.trim());

        Ok(())
    }
}

