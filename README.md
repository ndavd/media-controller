# Media Controller

Handles the display of volume/brightness control.

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
