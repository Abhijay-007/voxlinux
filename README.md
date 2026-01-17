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

LaunchApp

Pattern: (open|launch|start) <appname>
appname character constraints (from regex): [a-z0-9_-./]+ (so lowercased, alphanumeric, underscore, hyphen, dot, slash)
Parameters produced: { "app": "<appname>" }
Executor action: runs the app command (Command::new(app).spawn()) and propagates environment variables for GUI.
Example inputs:
launch firefox
open /usr/bin/code
start my-app
Notes: app string passed directly to Command::new; must be in PATH or be an executable path.
OpenFile

Pattern: open <path> (open may be followed by quoted or unquoted path)
Regex allows optional quotes: (open)\s+['"]?(.+?)['"]?$
Parameters produced: { "path": "<path>" }
Executor action: runs xdg-open <path> (Command::new("xdg-open").arg(path).spawn())
Example inputs:
open /home/user/Documents/notes.txt
open "My File.pdf"
open 'some/relative/path.md'
Notes: uses xdg-open; requires that xdg-open exists and a desktop environment that handles the file MIME type.
SetBrightness

Pattern: (set|brightness) <percent> (percent is 1–3 digits)
Parameters produced: { "percent": <u64> } (value parsed from input)
Executor action: writes a scaled value to /sys/class/backlight/*/brightness using the matching max_brightness; finds the first matching device and writes value = (percent * max_brightness) / 100
Example inputs:
set brightness 40
brightness 75
Constraints / notes:
percent parsed as integer; parse error => InvalidParameters
Writes to /sys/class/backlight — requires appropriate permissions (usually root) or a helper (sysfs write permission).
IncreaseBrightness

Pattern: (increase|raise|up) brightness [<delta>]? (delta optional, 1–3 digits)
Parameters produced: { "delta": <u64> } (if omitted defaults to 10)
Executor action: reads current brightness from /sys/class/backlight/*/brightness and max_brightness, computes new = min(cur + delta, maxv), writes new value.
Example inputs:
increase brightness
raise brightness 5
up brightness 20
DecreaseBrightness

Pattern: (decrease|lower|down) brightness [<delta>]? (delta optional)
Parameters produced: { "delta": <u64> } (if omitted defaults to 10)
Executor action: reads current brightness and writes cur - delta (saturating at 0).
Example inputs:
decrease brightness
lower brightness 15
down brightness 3
SetCpuPerformance

Pattern: set cpu performance
Parameters produced: {} (none)
Executor action: iterates /sys/devices/system/cpu/* and writes "performance" into cpufreq/scaling_governor for each CPU that has one.
Example input:
set cpu performance
Notes: requires write access to cpufreq sysfs entries (root).
SetCpuPowersave

Pattern: set cpu powersave
Parameters produced: {} (none)
Executor action: writes "powersave" to cpufreq/scaling_governor entries.
Example input:
set cpu powersave
Notes: requires write access (root).
GetCpuGovernor

Pattern: get cpu governor
Parameters produced: {} (none)
Executor action: Executor exists to query governor; the parser recognizes "get cpu governor" and constructs the intent. (Check executor implementation for exact output behavior.)
Example input:
get cpu governor
WifiOn

Pattern: wifi on
Parameters produced: {} (none)
Executor action: runs rfkill unblock wifi
Example input:
wifi on
Notes: uses rfkill; may require privileges.
WifiOff

Pattern: wifi off
Parameters produced: {} (none)
Executor action: runs rfkill block wifi
Example input:
wifi off
Notes: uses rfkill; may require privileges.
SetVolume

Pattern: set volume <percent> (percent is 1–3 digits)
Parameters produced: { "percent": <u64> }
Executor action:
First tries PipeWire native: wpctl set-volume @DEFAULT_AUDIO_SINK@ <N>%
If wpctl not available, falls back to pactl set-sink-volume @DEFAULT_SINK@ <N>% (PulseAudio)
If the command succeeds the executor returns OK.
Example inputs:
set volume 30
set volume 100
Defaults / notes:
If parsing fails parser may default to 50 in some locations; executor uses the parsed percent or fallback inside the executor implementation.
Requires wpctl or pactl to be present for the action to actually change volume.
SleepSystem

Pattern: sleep (the regex is anchored to end of line: (sleep)$)
Parameters produced: {} (none)
Executor action: runs systemctl suspend
Example input:
sleep
Notes: systemctl suspend requires system privileges / policy (usually works for logged-in desktop sessions).
