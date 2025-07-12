const WPCTL: &str = "wpctl";
const AUDIO_SINK: &str = "@DEFAULT_AUDIO_SINK@";
const AUDIO_SOURCE: &str = "@DEFAULT_AUDIO_SOURCE@";

const BRIGHTNESSCTL: &str = "brightnessctl";

fn main() {
    media_controller::MediaControllerApp {
        toggle_volume_mute,
        get_volume_mute,
        toggle_microphone_mute,
        get_microphone_mute,
        get_volume,
        get_brightness,
        inc_volume,
        inc_brightness,
        custom_controller: None,
    }
    .run();
}

fn run_get_volume_output() -> String {
    let stdout = std::process::Command::new(WPCTL)
        .args(["get-volume", AUDIO_SINK])
        .output()
        .unwrap()
        .stdout;
    String::from_utf8(stdout).unwrap()
}

fn run_get_microphone_volume_output() -> String {
    let stdout = std::process::Command::new(WPCTL)
        .args(["get-volume", AUDIO_SOURCE])
        .output()
        .unwrap()
        .stdout;
    String::from_utf8(stdout).unwrap()
}

fn get_formatted_value(value: i8) -> String {
    format!("{}%{}", value.abs(), if value < 0 { '-' } else { '+' })
}

fn get_volume_mute() -> bool {
    run_get_volume_output().contains("MUTED")
}

fn get_microphone_mute() -> bool {
    run_get_microphone_volume_output().contains("MUTED")
}

fn get_volume() -> u8 {
    let f32_vol = run_get_volume_output()
        .split(' ')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<f32>()
        .unwrap()
        * 100.0;
    f32_vol.round() as u8
}

fn force_mute(mute: bool) {
    std::process::Command::new(WPCTL)
        .args(["set-mute", AUDIO_SINK, if mute { "1" } else { "0" }])
        .output()
        .unwrap();
}

fn toggle_volume_mute() {
    std::process::Command::new(WPCTL)
        .args(["set-mute", AUDIO_SINK, "toggle"])
        .output()
        .unwrap();
}

fn toggle_microphone_mute() {
    std::process::Command::new(WPCTL)
        .args(["set-mute", AUDIO_SOURCE, "toggle"])
        .output()
        .unwrap();
}

fn inc_volume(inc: i8) {
    force_mute(false);
    if inc > 0 && get_volume() >= 100 {
        return;
    }
    std::process::Command::new(WPCTL)
        .args(["set-volume", AUDIO_SINK, &get_formatted_value(inc)])
        .output()
        .unwrap();
}
fn get_brightness() -> u8 {
    std::fs::read_to_string("/sys/class/backlight/nvidia_0/brightness")
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

fn inc_brightness(inc: i8) {
    std::process::Command::new(BRIGHTNESSCTL)
        .args(["s", &get_formatted_value(inc)])
        .output()
        .unwrap();
}
