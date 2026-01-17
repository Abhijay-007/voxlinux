use std::fs;

use crate::{executor::IntentExecutor, intent::Intent, error::IntentError};

pub struct SetCpuPowersaveExecutor;

impl IntentExecutor for SetCpuPowersaveExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        let base = "/sys/devices/system/cpu";

        for entry in fs::read_dir(base)? {
            let cpu = entry?.path();
            let gov = cpu.join("cpufreq/scaling_governor");
            if gov.exists() {
                fs::write(gov, "powersave")?;
            }
        }

        Ok(())
    }
}

