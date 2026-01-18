use std::process::Command;

use crate::{
    error::IntentError,
    executor::IntentExecutor,
    intent::Intent,
};

pub struct BluetoothToggleExecutor;

impl IntentExecutor for BluetoothToggleExecutor {
    fn execute(&self, _intent: &Intent) -> Result<(), IntentError> {
        let output = Command::new("bluetoothctl")
        .args(["show"])
        .output()
        .map_err(|_| IntentError::ProcessFailed)?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let on = stdout.contains("Powered: yes");

        let cmd = if on { "off" } else { "on" };

        println!("Bluetooth toggling {}", cmd);

        let status = Command::new("bluetoothctl")
        .args(["power", cmd])
        .status()
        .map_err(|_| IntentError::ProcessFailed)?;

        if !status.success() {
            println!("Bluetooth toggle failed.");
            return Err(IntentError::ProcessFailed);
        }

        println!("Bluetooth is now {}", if on { "off" } else { "on" });
        Ok(())
    }
}
