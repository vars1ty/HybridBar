# Hybrid Bar
A status bar focused on wlroots Wayland compositors

## Notice
> **Warning**:
> Hybrid is basically feature-complete for me.
>
> Which in turn means that commits might not happen as often, and new features might not get added by myself unless I need them.
>
> Although if you know Rust and you want something implemented, feel free to fork and work on said thing, then submit a PR.

## Preview
With blur through Hyprland.
![image](https://user-images.githubusercontent.com/54314240/197680577-2bc0cff4-2438-4c8d-8428-11499d0519c6.png)

The bottom bar is also made with Hybrid.

## What does it support?
It supports:
- Straight-forward configuration;
- Labels with commands (+ listen support);
- Spacings (a.k.a. Separators if styled);
- Boxes with child widgets;
- Custom update-rate for dynamic labels;
- Cava;
- Buttons with commands;
- Tooltips for buttons and labels;
- Markup for buttons and static labels;
- Top / Bottom positioning;
- Transparency (+ blur if your compositor allows for blurring surface layers);
- System Tray via `stray`;
- Compatible with different monitors;

## I have no config
If the AUR version for whatever reason didn't give you the example one, copy the example from `examples/config.json` into `~/.config/HybridBar/`.

## Does it only work on wlroots Compositors?
Nope, it's been tested on KDE as well and it worked just fine. It just won't work on GNOME as it hasn't implemented the `wlr-layer-shell` protocol.

# Config Layout
I'm assuming you are familiar with JSON. If you aren't, well too bad.

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
`HYBRID_LOG` = `0` OR `1` : Logs Hybrid output to stdout.

`HYBRID_CONFIG` = `name.json` : Locates the config inside the HybridBar config path, then uses it for the rest of the bars session.

## Background Updates
Hybrid automatically performs background updates for dynamic labels/tooltips and Cava widgets.

The rate for updating labels is read from the labels `update_rate` key (u64).

It's worth noting that low update-rates may lead to performance decreases, the value specified is in **milliseconds**.

**NOTE**: Cava widgets do __not__ support a custom `update_rate` to be specified, this is to ensure better precision for the animations.

# Build dependencies
1. rust
2. gtk-layer-shell
3. gtk3
4. a compositor that supports layer-shells
   - This excludes GNOME. KDE, Hyprland and Sway have been confirmed working.

## Arch Linux
Git Version: `yay/paru -S hybrid-bar-git`

**NOTE**: Git may include changes that are experimental. It's heavily advised that you use the package listed below if you care about stability.

Latest Binary: `yay/paru -S hybrid-bar`

## Building
1. `git clone https://github.com/vars1ty/HybridBar`
2. `cd HybridBar`
3. `cargo build --release`
4. `cd target/release`
5. Done, the executable is called `hybrid-bar`.

**TIP**: `chmod +x hybrid-bar` - So you can run the executable directly.
