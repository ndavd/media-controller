use crate::{Action, Color, MediaController};

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const TAB: &str = "    ";

const ARG_WIDTH: &str = "width";
const ARG_HEIGHT: &str = "height";
const ARG_BOTTOM: &str = "bottom";
const ARG_FONT_DESCRIPTION: &str = "font-description";
const ARG_COLOR: &str = "color";
const ARG_DURATION: &str = "duration";
const ARG_FILLED: &str = "filled";
const ARG_HALF_FILLED: &str = "half-filled";
const ARG_EMPTY: &str = "empty";

const ARGS: &[&str] = &[
    ARG_WIDTH,
    ARG_HEIGHT,
    ARG_BOTTOM,
    ARG_FONT_DESCRIPTION,
    ARG_COLOR,
    ARG_DURATION,
    ARG_FILLED,
    ARG_HALF_FILLED,
    ARG_EMPTY,
];

pub trait Cli
where
    Self: Sized,
{
    fn from_args() -> Option<Self>;
    fn print_usage();
}

impl Cli for MediaController {
    fn from_args() -> Option<Self> {
        let args = std::env::args().skip(1).collect::<Vec<_>>();
        let mut controller = Self::default();
        let mut action_i = 0;
        for arg in &args {
            if !arg.starts_with("--") {
                break;
            }
            action_i += 1;
            let (option, value) = arg.split_once('=')?;
            let option = &option[2..];
            match option {
                ARG_WIDTH | ARG_HEIGHT | ARG_BOTTOM => {
                    if let Ok(parsed) = value.parse::<u32>() {
                        match option {
                            ARG_WIDTH => controller.width = parsed,
                            ARG_HEIGHT => controller.height = parsed,
                            ARG_BOTTOM => controller.bottom = parsed,
                            _ => panic!(),
                        }
                        continue;
                    }
                }
                ARG_FONT_DESCRIPTION => {
                    controller.font_description = value.trim_matches('"').to_string();
                    continue;
                }
                ARG_COLOR => {
                    if let Some(parsed) = Color::from_hex(value.trim_matches('"')) {
                        controller.color = parsed;
                        continue;
                    }
                }
                ARG_DURATION => {
                    if let Ok(parsed) = value.parse::<f32>() {
                        controller.duration = parsed;
                        continue;
                    }
                }
                ARG_FILLED => {
                    if let Ok(parsed) = value.parse::<char>() {
                        controller.filled = parsed;
                        continue;
                    }
                }
                ARG_HALF_FILLED => {
                    if let Ok(parsed) = value.parse::<char>() {
                        controller.half_filled = parsed;
                        continue;
                    }
                }
                ARG_EMPTY => {
                    if let Ok(parsed) = value.parse::<char>() {
                        controller.empty = parsed;
                        continue;
                    }
                }
                _ => {}
            }
            return None;
        }
        let action_args = &args[action_i..];
        controller.action = match action_args.len() {
            2 => {
                if action_args[0] == "v" && action_args[1] == "mute" {
                    Some(Action::VolumeToggleMute)
                } else if action_args[0] == "m" && action_args[1] == "mute" {
                    Some(Action::MicrophoneToggleMute)
                } else {
                    None
                }
            }
            3 => {
                let parsed = action_args[2].parse::<u8>().ok()?;
                match (action_args[0].as_str(), action_args[1].as_str()) {
                    ("v", "up") => Some(Action::VolumeUp(parsed)),
                    ("v", "down") => Some(Action::VolumeDown(parsed)),
                    ("b", "up") => Some(Action::BrightnessUp(parsed)),
                    ("b", "down") => Some(Action::BrightnessDown(parsed)),
                    _ => None,
                }
            }
            _ => None,
        }?;
        Some(controller)
    }

    fn print_usage() {
        let default_controller = Self::default();
        let def_str = "Default: ";

        println!("{NAME} v{VERSION}");
        println!("{AUTHORS}");
        println!("\nUSAGE:");
        println!("{TAB}{NAME} [OPTIONS] v up|down|mute {{number}}");
        println!("{TAB}{NAME} [OPTIONS] m mute");
        println!("{TAB}{NAME} [OPTIONS] b up|down {{number}}");
        println!("\nOPTIONS:");
        println!("Format --{{option}}={{value}}");

        let biggest_arg_len = ARGS
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len();
        let pad = |s: &str| format!("{s:biggest_arg_len$}");

        println!(
            "{TAB}{}{TAB}Lifespan of the window in seconds. {def_str}{}",
            pad(ARG_DURATION),
            default_controller.duration
        );
        println!(
            "{TAB}{}{TAB}Width of the window in px. {def_str}{}",
            pad(ARG_WIDTH),
            default_controller.width
        );
        println!(
            "{TAB}{}{TAB}Height of the window in px. {def_str}{}",
            pad(ARG_HEIGHT),
            default_controller.height
        );
        println!(
            "{TAB}{}{TAB}Offset from the bottom of the screen in px. {def_str}{}",
            pad(ARG_BOTTOM),
            default_controller.bottom
        );
        println!(
            "{TAB}{}{TAB}Color of the window in hex (#RRGGBB or #RRGGBBAA). {def_str}\"{}\"",
            pad(ARG_COLOR),
            default_controller.color
        );
        println!(
            "{TAB}{}{TAB}Font used. {def_str}\"{}\"",
            pad(ARG_FONT_DESCRIPTION),
            default_controller.font_description
        );
        println!(
            "{TAB}{}{TAB}Filled character used in the progress bar. {def_str}\"{}\"",
            pad(ARG_FILLED),
            default_controller.filled
        );
        println!(
            "{TAB}{}{TAB}Half filled character used in the progress bar. {def_str}\"{}\"",
            pad(ARG_HALF_FILLED),
            default_controller.half_filled
        );
        println!(
            "{TAB}{}{TAB}Empty character used in the progress bar. {def_str}\"{}\"",
            pad(ARG_EMPTY),
            default_controller.empty
        );
        println!("\n");
    }
}
