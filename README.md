# Media Controller

Handles the display of volume/brightness control.

```
media-controller v0.1.0
Nuno David <email@ndavd.com>

USAGE:
    media-controller [OPTIONS] v up|down|mute {number}
    media-controller [OPTIONS] b up|down {number}

OPTIONS:
Format --{option}={value}
    duration            Lifespan of the window in seconds. Default: 2
    width               Width of the window in px. Default: 300
    height              Height of the window in px. Default: 20
    bottom              Offset from the bottom of the screen in px. Default: 100
    color               Color of the window in hex (#RRGGBB or #RRGGBBAA). Default: "#000000FF"
    font-description    Font used. Default: "Monospace 13"
```

In order to make it work in your system simply modify it to your liking, all it
takes to integrate are some agnostic function implementations. Your `main.rs`
should look something like this:

```rust
fn toggle_mute() {
    todo!();
}
fn get_mute() -> bool {
    todo!();
}
fn get_volume() -> u8 {
    todo!();
}
fn get_brightness() -> u8 {
    todo!();
}
fn inc_volume(value: i8) {
    todo!();
}
fn inc_brightness(value: i8) {
    todo!();
}
fn main() {
    controller::MediaControllerApp {
        toggle_mute,
        get_mute,
        get_volume,
        get_brightness,
        inc_volume,
        inc_brightness,
    }
    .run();
}
```

Here's a basic example of how to use it with `sxhkd`:

```
# Volume Control
XF86AudioLowerVolume
  media-controller v down 5
XF86AudioRaiseVolume
  media-controller v up 5
XF86AudioMute
  media-controller v mute

# Brightness Control
XF86MonBrightnessDown
  media-controller b down 5
XF86MonBrightnessUp
  media-controller b up 5
```

Installing:
```
cargo install --path .
```

Uninstalling:
```
cargo uninstall media-controller
```
