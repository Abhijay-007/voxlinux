voxlinux — Intent Engine for Linux

voxlinux is a lightweight intent-based control layer for Linux. It parses natural commands into structured intents and executes them using system tools or kernel interfaces. The design makes it possible to launch apps, open files, adjust brightness, control CPU governors, toggle Wi-Fi, change volume, and suspend the system from a simple REPL-like interface.

Build & Run

git clone https://github.com/Abhijay-007/voxlinux

cd voxlinux
cargo build --release
./target/release/voxlinux-intent

General parsing notes

The parser lowercases input before pattern matching.
Many intents use regular expressions to match natural-ish phrases.
Some patterns accept quoted or unquoted file paths / app names.
Numeric parameters are parsed from 1–3 digit groups in the input (0–999).
If a numeric parse fails for a required parameter the parser will return an invalid-parameter error.
Intents, input patterns, parameters, examples

Intents that are used:
Invocation / App & File Handling
LaunchApp
OpenFile

Display / Brightness Controls
SetBrightness
IncreaseBrightness
DecreaseBrightness

CPU Power Governors
SetCpuPerformance
SetCpuPowersave
GetCpuGovernor

Networking
WifiOn
WifiOff

Audio
SetVolume

System Power
SleepSystem
