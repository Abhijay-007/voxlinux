use std::io::{self, Write};

mod error;
mod intent;
mod executor;
mod dispatcher;
mod parser;
mod executors;

use dispatcher::Dispatcher;
use intent::IntentType;
use executors::*;
use parser::parse_input;

fn main() -> Result<(), error::IntentError> {
    let mut dispatcher = Dispatcher::new();

    dispatcher.register(IntentType::LaunchApp, launch_app::LaunchAppExecutor);
    dispatcher.register(IntentType::OpenFile, open_file::OpenFileExecutor);
    dispatcher.register(IntentType::SetBrightness, set_brightness::SetBrightnessExecutor);
    dispatcher.register(IntentType::IncreaseBrightness, inc_brightness::IncreaseBrightnessExecutor);
    dispatcher.register(IntentType::DecreaseBrightness, dec_brightness::DecreaseBrightnessExecutor);

    // new kernel intent registrations
    dispatcher.register(IntentType::SetCpuPerformance, cpu_performance::SetCpuPerformanceExecutor);
    dispatcher.register(IntentType::SetCpuPowersave, cpu_powersave::SetCpuPowersaveExecutor);
    dispatcher.register(IntentType::GetCpuGovernor, cpu_get::GetCpuGovernorExecutor);
    dispatcher.register(IntentType::WifiOn, wifi_on::WifiOnExecutor);
    dispatcher.register(IntentType::WifiOff, wifi_off::WifiOffExecutor);
    dispatcher.register(IntentType::SetVolume, set_volume::SetVolumeExecutor);
    dispatcher.register(IntentType::SleepSystem, sleep_system::SleepSystemExecutor);

    loop {
        print!("voxlinux> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let intent = parse_input(input)?;
        dispatcher.dispatch(&intent)?;
        println!("✔ intent executed");
    }

    Ok(())
}

