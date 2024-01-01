use crate::{
    cli::{FromArgs, PrintUsage, NAME},
    window::spawn_window,
};
use fs2::FileExt;
use std::io::{Read, Write};

#[derive(Debug, Default, Clone, Copy)]
pub enum Action {
    #[default]
    VolumeToggleMute,
    VolumeUp(u8),
    VolumeDown(u8),
    BrightnessUp(u8),
    BrightnessDown(u8),
}
impl Action {
    fn is_volume_kind(&self) -> bool {
        match self {
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
        let f = 0.05;
        Self::new(f, f, f, 0.8)
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
        let mut chunks = chars_vec.chunks(2).map(|c| String::from_iter(c));
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
    pub duration: u32,
}
impl std::default::Default for MediaController {
    fn default() -> Self {
        Self {
            action: Action::default(),
            color: Color::default(),
            font_description: "Monospace 15".to_string(),
            width: 400,
            height: 30,
            bottom: 100,
            duration: 2,
        }
    }
}

pub struct MediaControllerApp {
    pub get_mute: fn() -> bool,
    pub get_volume: fn() -> u8,
    pub get_brightness: fn() -> u8,

    pub inc_volume: fn(i8),
    pub inc_brightness: fn(i8),

    pub toggle_mute: fn(),
}

impl MediaControllerApp {
    pub fn run(&self) {
        let controller = match MediaController::from_args() {
            Some(controller) => controller,
            None => {
                MediaController::print();
                return;
            }
        };

        match controller.action {
            Action::VolumeUp(v) => (self.inc_volume)(v as i8),
            Action::VolumeDown(v) => (self.inc_volume)(-(v as i8)),
            Action::VolumeToggleMute => (self.toggle_mute)(),
            Action::BrightnessUp(v) => (self.inc_brightness)(v as i8),
            Action::BrightnessDown(v) => (self.inc_brightness)(-(v as i8)),
        };

        let label = self.label(controller.action);

        let lock_p = format!("/tmp/{}.lock", NAME);
        let socket_p = format!("/tmp/{}.sock", NAME);

        let lock = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(lock_p)
            .unwrap();

        if lock.try_lock_exclusive().is_err() {
            println!("Another instance is already running. Updating existing window...");
            std::os::unix::net::UnixStream::connect(socket_p)
                .unwrap()
                .write_all(label.as_bytes())
                .unwrap();
            return;
        }

        let shared = std::sync::Arc::new(std::sync::Mutex::new(label.clone()));

        let shared_2 = shared.clone();
        std::thread::spawn(move || {
            let _ = std::fs::remove_file(&socket_p);
            let listener = std::os::unix::net::UnixListener::bind(socket_p).unwrap();
            for stream in listener.incoming() {
                if let Ok(mut stream) = stream {
                    let mut b = [0; 1024];
                    let data_size = stream.read(&mut b).unwrap();
                    let data = std::str::from_utf8(&b[..data_size]).unwrap();
                    println!("Received from another instance: {}", data);
                    let mut label = shared_2.lock().unwrap();
                    *label = data.to_string();
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                    drop(stream);
                }
            }
        });

        spawn_window(controller.clone(), shared);
    }
    pub fn label(&self, action: Action) -> String {
        let is_volume = action.is_volume_kind();
        if !is_volume {
            let brightness = (self.get_brightness)();
            return format!("BRT: {}", Self::_progress(brightness));
        }
        if (self.get_mute)() {
            return "MUTED".to_string();
        }
        let volume = (self.get_volume)();
        return format!("VOL: {}", Self::_progress(volume));
    }
    fn _progress(percentage: u8) -> String {
        assert!(percentage <= 100);
        let progress = (percentage / 10) as usize;
        let progress_str = std::iter::repeat('█')
            .take(progress)
            .chain(std::iter::repeat(' ').take(10 - progress))
            .collect::<String>();
        format!("{progress_str} {percentage}%")
    }
}
