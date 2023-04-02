# Hybrid Bar
A simple status bar focused on wlroots Wayland compositors

## Preview
With blur through Hyprland.
![image](https://user-images.githubusercontent.com/54314240/197680577-2bc0cff4-2438-4c8d-8428-11499d0519c6.png)

The bottom bar is also made with Hybrid.

## Why another status bar?
Because I never found any good other alternative, and I needed:
- Something simple to configure;
- Basic-ish features. Even if alternatives did have it, it often required more effort than what I'd consider necessary;
- A real project to make so I could learn more about Rust, hence the many refactors and frequent code-changes
  - Hybrid was also made open-source so that others can make use of it and not only be limited to the other alternatives

Hybrid implements everything I need in a status bar and a bit more.

If something's missing and I really need it, then I'll most likely add it.

## What does it support?
Hybrid supports a variety of features, most notably:
- Cava embedded right into your bar;
- Easy, documented configuration in form of JSON;
- Scripting through [Rune](https://rune-rs.github.io/) and easy-to-use methods, all documented with examples provided;
- Native GTK3 Transparency;
- Ability to opt-in for special [features](https://github.com/vars1ty/HybridBar/blob/main/FEATURES.md);
- Listening support for Label widgets;
- Animations for when the content of a Label widget updates;
- And much more!

## I have no config
If you installed outside the AUR, copy the example from `examples/config.json` into `~/.config/HybridBar/`.

## Does it only work on wlroots Compositors?
Nope, it's been tested on KDE as well and it worked just fine.

It just won't work on GNOME as it hasn't implemented the `wlr-layer-shell` protocol.

# Config Layout
The configuration is written in plain JSON, the layout is setup to be as easy to read and write as possible.

## Base Setup
Before you can use the bar, you have to adjust the color and alpha.

RGB Colors are 0-255 as a 32-bit integer, Alpha is 0.0-1.0 as a 32-bit float.

Here's an example:

```json
{
    "hybrid": {
        "namespace": "hybrid-bar",
        "r": 10,
        "g": 10,
        "b": 10,
        "a": 0.5
    }
}
```
## CSS Support
CSS is supported and you can make it auto-load on startup by making a `style.css` file next to your `config.json` on the same path.

If you want a sample CSS which has good defaults, check `examples/style.css`.

## Environment Variables
`HYBRID_LOG` = `1` : Logs Hybrid output to stdout.

`HYBRID_CONFIG` = `name.json` : Locates the config inside the HybridBar config path, then uses it for the rest of the bars session.

## Background Updates
Hybrid has background-loops running for **dynamic** content, aka:
- Widgets that have a property like `command` or `tooltip_command` set;
- Your script (if any), assuming it has the `tick` function present;
- The Cava widget, otherwise it can't fetch new Cava data and display it

# Build dependencies
1. rust
2. gtk-layer-shell
3. gtk3
4. A compositor that supports layer-shells
   - This excludes GNOME. KDE, Hyprland and Sway have been confirmed working.

## Installation
Arch Linux: `yay/paru -S hybrid-bar`

Other distros: `cargo install hybrid-bar`

## Building
1. `git clone https://github.com/vars1ty/HybridBar`
2. `cd HybridBar`
3. `cargo build --release`
4. `cd target/release`
5. Done, the executable is called `hybrid-bar`.
