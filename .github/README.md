# Media Controller

<div>
  <img alt="Crates.io Version" src="https://img.shields.io/crates/v/media-controller?style=flat-square">
</div>
<br/>

`media-controller` provides a GTK always-on-top window with transparency support
that displays the current volume/brightness right after changing it accordingly.

It makes use of UNIX sockets so that if another instance is created while the
first one is running, it doesn't create another window and simply updates the
content of the existing one providing a smooth experience.

![Demo](https://raw.githubusercontent.com/ndavd/media-controller/main/.github/demo.gif)

[Options used in the demo:
`--color=#000000aa --font-description="BigBlueTerm437 Nerd Font Mono"`]

In order to build the binary or the library you must pick a Cargo feature:

- `regular`: For X11 and other systems, uses GTK3
- `wayland`: For Wayland systems, uses GTK4 and GTK4 Layer Shell (make sure to
  have [`gtk-layer-shell`](https://github.com/wmww/gtk-layer-shell) installed)

```
media-controller v0.3.1
Nuno David <email@ndavd.com>

USAGE:
    media-controller [OPTIONS] v up|down|mute {number}
    media-controller [OPTIONS] m mute
    media-controller [OPTIONS] b up|down {number}

OPTIONS:
Format --{option}={value}
    duration            Lifespan of the window in seconds. Default: 2
    width               Width of the window in px. Default: 300
    height              Height of the window in px. Default: 20
    bottom              Offset from the bottom of the screen in px. Default: 100
    color               Color of the window in hex (#RRGGBB or #RRGGBBAA). Default: "#000000FF"
    font-description    Font used. Default: "Monospace 13"
    filled              Filled character used in the progress bar. Default: "█"
    half-filled         Half filled character used in the progress bar. Default: "▌"
    empty               Empty character used in the progress bar. Default: " "
```

In order to make it work in your specific system, simply create a new cargo
project and add the library with the respective feature enabled:

```
cargo add media-controller --features wayland
```

Then all it takes is implementing some functions. A concrete example for a Linux
system that uses `wpctl` and `brightnessctl` can be found at
[src/main.rs](https://github.com/ndavd/media-controller/blob/main/src/main.rs).

It is particularly useful to map `media-controller` to your media keys. E.g.
using `sxhkd`:

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
