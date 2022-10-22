# Hybrid Bar
A status bar focused on wlroots Wayland compositors

## Preview
With blur through Hyprland.
![image](https://user-images.githubusercontent.com/54314240/185250795-b5c1b948-ef69-4293-bd1b-4deedbbaa82d.png)

## What does it support?
It supports:
- Labels (static and inherit text from a command);
- Buttons with `on click` actions via a bash command;
- Spacings;
- Transparency (+ blur if your compositor supports it);
- Custom CSS;
- Custom update-frequency for dynamic labels (ones with a `command` set);
- Beta Cava implementation;
- Multi-Config;
- Top and Bottom positioning

In other words, it's a simple Wayland status bar that does one thing: Display the stuff __you__ put onto it. Nothing more, nothing less, no over-engineered dogshit.
## I have no config
If the AUR version for whatever reason didn't give you the example one, copy the example from `examples/config.json` into `~/.config/HybridBar/`.
# Config Layout
I'm assuming you are familiar with JSON. If you aren't, well too bad.
## Base
Before you can use the bar, you have to adjust the color and alpha.

RGB Colors are 0-255 as an integer, Alpha is 0.0-1.0 as a float.

Here's an example:

```json
{
    "hybrid": {
        "update_rate": 100,
        "r": 10,
        "g": 10,
        "b": 10,
        "a": 0.5
    }
}
```
## Video Tutorial
You can watch a video tutorial made by Foren [here](https://www.youtube.com/watch?v=5g7MX3jgv8A) - **Outdated but may help some**.
## CSS Support
Starting from `0.1.3`, CSS is now supported and you can make it auto-load on startup by making a `style.css` file next to your `config.json` at the same path.

If you want a sample CSS which has been used up until recently and has good defaults, check `examples/style.css`.
## Environment Variables
`HYBRID_LOG` = `0` OR `1` : Logs debug output to stdout.

`HYBRID_POS` = `TOP` OR `BOTTOM` : Tells the bar where to position itself, TOP or BOTTOM.

`HYBRID_CONFIG` = `name.json` : Locates the config inside the HybridBar config path, then uses it for the rest of the bars session.
## Background Updates
Hybrid performs background updates for labels with the key `command` set, and for updating Cava labels.

The rate for updating labels is read from `hybrid`, `update_rate` (i32).

It's worth noting that low update-rates may lead to performance decreases, the value specified is in **milliseconds**.

**NOTE**: Cava-widgets does __not__ respect the `update_rate` specified, this is to ensure better precision for the animations.
# Installation
Dependencies:

1. gtk-layer-shell
2. gtk3
3. bash
4. a brain

## Arch Linux
AUR: `paru -S hybrid-bar-git`

**NOTE**: This builds the bar, so you'll need Rust installed.
## Building
1. `git clone https://github.com/vars1ty/HybridBar`
2. `cd HybridBar`
3. `cargo build --release`
4. `cd target/release`
5. Done, the executable is called `hybrid-bar`.

Tip: `chmod +x hybrid-bar` - So you can run the executable directly.
# Roadmap
- ~~Make the code for widgets cleaner and more portable~~ - **Done**
- ~~Quit blocking the UI Thread when executing bash-commands and retrieving the output~~ - **Done**
- Port over to GTK4 - Not possible right now due to GTK being 0IQ and [screwing shit up](https://github.com/wmww/gtk-layer-shell/issues/37)
- Refactor parts of the code to make it more readable and easier to maintain - **In progress**
- Publish a non-git AUR package which uses the latest built binary
- Potentially more widgets - **In progress, feel free to suggest widgets**
