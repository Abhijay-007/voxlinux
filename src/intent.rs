use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntentType {
    LaunchApp,
    OpenFile,
    SetBrightness,
    IncreaseBrightness,
    DecreaseBrightness,
    // kernel
    SetCpuPerformance,
    SetCpuPowersave,
    GetCpuGovernor,
    WifiOn,
    WifiOff,
    SleepSystem,
    SetVolume,
    // new
    WifiConnect,
    BatteryStatus,
    MediaPlayPause,
    MediaNext,
    MediaPrev,
    LockScreen,
    BluetoothToggle,
}

#[derive(Debug, Clone)]
pub struct Intent {
    pub intent_type: IntentType,
    pub parameters: Value,
}

