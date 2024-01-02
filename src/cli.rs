use crate::{Action, Color, MediaController};

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const TAB: &str = "    ";

pub trait PrintUsage {
    fn print();
}

impl PrintUsage for MediaController {
    fn print() {
        let default_controller = Self::default();
        let def_str = "Default: ";
        println!("{NAME} v{VERSION}");
        println!("{AUTHORS}");
        println!("\nUSAGE:");
        println!("{TAB}{NAME} [OPTIONS] v up|down|mute {{number}}");
        println!("{TAB}{NAME} [OPTIONS] b up|down {{number}}");
        println!("\nOPTIONS:");
        println!("Format --{{option}}={{value}}");
        println!(
            "{TAB}duration        {TAB}Lifespan of the window in seconds. {def_str}{}",
            default_controller.duration
        );
        println!(
            "{TAB}width           {TAB}Width of the window in px. {def_str}{}",
            default_controller.width
        );
        println!(
            "{TAB}height          {TAB}Height of the window in px. {def_str}{}",
            default_controller.height
        );
        println!(
            "{TAB}bottom          {TAB}Offset from the bottom of the screen in px. {def_str}{}",
            default_controller.bottom
        );
        println!("{TAB}color           {TAB}Color of the window in hex (#RRGGBB or #RRGGBBAA). {def_str}\"{}\"", default_controller.color);
        println!(
            "{TAB}font-description{TAB}Font used. {def_str}\"{}\"",
            default_controller.font_description
        );
        println!(
            "{TAB}filled          {TAB}Filled character used in the window content. {def_str}\"{}\"",
            default_controller.filled
        );
        println!(
            "{TAB}half-filled     {TAB}Half filled character used in the window content. {def_str}\"{}\"",
            default_controller.half_filled
        );
        println!(
            "{TAB}empty           {TAB}Empty character used in the window content. {def_str}\"{}\"",
            default_controller.empty
        );
        println!("\n");
    }
}

pub trait FromArgs
where
    Self: Sized,
{
    fn from_args() -> Option<Self>;
}

impl FromArgs for MediaController {
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
            match &option[2..] {
                "width" | "height" | "bottom" => {
                    if let Ok(parsed) = value.parse::<u32>() {
                        match option {
                            "width" => controller.width = parsed,
                            "height" => controller.height = parsed,
                            "bottom" => controller.bottom = parsed,
                            _ => panic!(),
                        }
                        continue;
                    }
                }
                "font-description" => {
                    controller.font_description = value.trim_matches('"').to_string();
                    continue;
                }
                "color" => {
                    if let Some(parsed) = Color::from_hex(value.trim_matches('"')) {
                        controller.color = parsed;
                        continue;
                    }
                }
                "duration" => {
                    if let Ok(parsed) = value.parse::<f32>() {
                        controller.duration = parsed;
                        continue;
                    }
                }
                "filled" => {
                    if let Ok(parsed) = value.parse::<char>() {
                        controller.filled = parsed;
                        continue;
                    }
                }
                "half-filled" => {
                    if let Ok(parsed) = value.parse::<char>() {
                        controller.half_filled = parsed;
                        continue;
                    }
                }
                "empty" => {
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
}
