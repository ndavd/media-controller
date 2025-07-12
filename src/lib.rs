mod cli;

#[cfg(feature = "regular")]
mod window;
#[cfg(feature = "wayland")]
mod wl_window;

use cli::{Cli, NAME};
use fs2::FileExt;
use std::io::{Read, Write};

#[cfg(all(feature = "regular", feature = "wayland"))]
compile_error!("Features \"regular\" and \"wayland\" cannot be enabled at the same time");

#[derive(Debug, Default, Clone, Copy)]
pub enum Action {
    #[default]
    VolumeToggleMute,
    MicrophoneToggleMute,
    VolumeUp(u8),
    VolumeDown(u8),
    BrightnessUp(u8),
    BrightnessDown(u8),
}
impl Action {
    fn is_volume_kind(&self) -> bool {
        match self {
            Self::MicrophoneToggleMute => true,
            Self::VolumeToggleMute => true,
            Self::VolumeUp(_) => true,
            Self::VolumeDown(_) => true,
            Self::BrightnessUp(_) => false,
            Self::BrightnessDown(_) => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl std::default::Default for Color {
    fn default() -> Self {
        let f = 0.0;
        Self::new(f, f, f, 1.0)
    }
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = (self.r * 255.0).round() as u8;
        let g = (self.g * 255.0).round() as u8;
        let b = (self.b * 255.0).round() as u8;
        let a = (self.a * 255.0).round() as u8;
        write!(f, "#{r:02X}{g:02X}{b:02X}{a:02X}")
    }
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn from_hex(hex_str: &str) -> Option<Self> {
        match hex_str.len() {
            7 => {}
            9 => {}
            _ => return None,
        };
        let mut chars = hex_str.chars();
        if chars.next().unwrap() != '#' {
            return None;
        }
        let chars_vec = chars.collect::<Vec<_>>();
        let mut chunks = chars_vec.chunks(2).map(String::from_iter);
        let parse_chunk = |chunk: Option<String>| -> Option<f32> {
            if let Some(chunk) = chunk {
                let integer_representation = u8::from_str_radix(&chunk, 16).ok()?;
                return Some(integer_representation as f32 / 255.0);
            }
            Some(1.0)
        };
        let r = parse_chunk(chunks.next().clone())?;
        let g = parse_chunk(chunks.next().clone())?;
        let b = parse_chunk(chunks.next().clone())?;
        let a = parse_chunk(chunks.next().clone())?;
        Some(Self { r, g, b, a })
    }
}

#[derive(Debug, Clone)]
pub struct MediaController {
    pub action: Action,
    pub color: Color,
    pub font_description: String,
    pub width: u32,
    pub height: u32,
    pub bottom: u32,
    pub duration: f32,
    pub filled: char,
    pub half_filled: char,
    pub empty: char,
}
impl std::default::Default for MediaController {
    fn default() -> Self {
        Self {
            action: Action::default(),
            color: Color::default(),
            font_description: "Monospace 13".to_string(),
            width: 300,
            height: 20,
            bottom: 100,
            duration: 2.0,
            filled: '█',
            half_filled: '▌',
            empty: ' ',
        }
    }
}

pub struct MediaControllerApp {
    /// Should return whether the volume is muted.
    pub get_volume_mute: fn() -> bool,

    /// Should return whether the microphone is muted.
    pub get_microphone_mute: fn() -> bool,

    /// Should return the volume (0-100).
    pub get_volume: fn() -> u8,
    /// Should return the brightness (0-100).
    pub get_brightness: fn() -> u8,

    /// Should increment the volume. To decrement use a negative value.
    pub inc_volume: fn(i8),
    /// Should increment the brightness. To decrement use a negative value.
    pub inc_brightness: fn(i8),

    /// Should toggle volume mute.
    pub toggle_volume_mute: fn(),

    /// Should toggle microphone mute.
    pub toggle_microphone_mute: fn(),

    /// Pass `Some` to use custom options.
    /// Pass `None` to manage them through command line arguments.
    pub custom_controller: Option<MediaController>,
}
impl MediaControllerApp {
    pub fn run(&self) {
        let controller = match &self.custom_controller {
            Some(controller) => controller.clone(),
            None => match MediaController::from_args() {
                Some(controller) => controller,
                None => {
                    MediaController::print_usage();
                    return;
                }
            },
        };

        match controller.action {
            Action::VolumeUp(v) => (self.inc_volume)(v as i8),
            Action::VolumeDown(v) => (self.inc_volume)(-(v as i8)),
            Action::VolumeToggleMute => (self.toggle_volume_mute)(),
            Action::MicrophoneToggleMute => (self.toggle_microphone_mute)(),
            Action::BrightnessUp(v) => (self.inc_brightness)(v as i8),
            Action::BrightnessDown(v) => (self.inc_brightness)(-(v as i8)),
        };

        let label_text = self.label(
            controller.action,
            controller.filled,
            controller.half_filled,
            controller.empty,
        );
        println!("{label_text}");

        let lock_p = format!("/tmp/{NAME}.lock");
        let socket_p = format!("/tmp/{NAME}.sock");

        let lock = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(lock_p)
            .unwrap();

        if lock.try_lock_exclusive().is_err() {
            println!("Another instance is already running. Updating existing window...");
            std::os::unix::net::UnixStream::connect(socket_p)
                .unwrap()
                .write_all(label_text.as_bytes())
                .unwrap();
            return;
        }

        let shared = std::sync::Arc::new(std::sync::Mutex::new(label_text.clone()));

        let kill_countdown = std::sync::Arc::new(std::sync::Mutex::new(1));

        let shared_2 = shared.clone();
        let kill_countdown_2 = kill_countdown.clone();
        std::thread::spawn(move || {
            let _ = std::fs::remove_file(&socket_p);
            let listener = std::os::unix::net::UnixListener::bind(socket_p).unwrap();
            for mut stream in listener.incoming().flatten() {
                let mut b = [0; 1024];
                let data_size = stream.read(&mut b).unwrap();
                let data = std::str::from_utf8(&b[..data_size]).unwrap();
                println!("Received from another instance: {data}");
                let mut label = shared_2.lock().unwrap();
                let mut kill_countdown = kill_countdown_2.lock().unwrap();
                *kill_countdown = if *kill_countdown >= 2 {
                    2
                } else {
                    *kill_countdown + 1
                };
                *label = data.to_string();
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                drop(stream);
            }
        });
        std::thread::spawn(move || {
            while *kill_countdown.lock().unwrap() != 0 {
                std::thread::sleep(std::time::Duration::from_secs_f32(controller.duration));
                *kill_countdown.lock().unwrap() -= 1;
            }
            println!("Closing...");
            std::process::exit(0);
        });

        #[cfg(feature = "regular")]
        window::spawn_window(controller.clone(), shared);

        #[cfg(feature = "wayland")]
        wl_window::spawn_wl_window(controller.clone(), shared);
    }
    pub fn label(&self, action: Action, full: char, half_full: char, empty: char) -> String {
        if matches!(action, Action::MicrophoneToggleMute) {
            if (self.get_microphone_mute)() {
                return "MIC OFF".to_string();
            } else {
                return "MIC ON".to_string();
            }
        }
        let is_volume = action.is_volume_kind();
        if !is_volume {
            let brightness = (self.get_brightness)();
            return format!(
                "BRT: {}",
                Self::_progress(brightness, full, half_full, empty)
            );
        }
        if (self.get_volume_mute)() {
            return "MUTED".to_string();
        }
        let volume = (self.get_volume)();
        format!("VOL: {}", Self::_progress(volume, full, half_full, empty))
    }
    fn _progress(percentage: u8, full: char, half_full: char, empty: char) -> String {
        assert!(percentage <= 100);
        let progress = percentage as f32 / 10.0;
        let filled_count = progress as usize;
        let middle_count = (percentage != 100) as usize;
        let empty_count = 10_usize.saturating_sub(progress as usize).saturating_sub(1);
        let progress_str = std::iter::repeat_n(full, filled_count)
            .chain(std::iter::repeat_n(
                if progress.ceil() - progress >= 0.5 {
                    half_full
                } else {
                    empty
                },
                middle_count,
            ))
            .chain(std::iter::repeat_n(empty, empty_count))
            .collect::<String>();
        format!("{progress_str}{percentage:>4}%")
    }
}
