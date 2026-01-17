use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntentType {
    LaunchApp,
    OpenFile,
    SetBrightness,
    IncreaseBrightness,
    DecreaseBrightness,

    // new kernel intents
    SetCpuPerformance,
    SetCpuPowersave,
    GetCpuGovernor,
    WifiOn,
    WifiOff,
    SetVolume,
    SleepSystem,
}

pub struct Intent {
    pub intent_type: IntentType,
    pub parameters: Value,
}

