# Hybrid Bar
A status bar focused on wlroots Wayland compositors

## Preview
With blur through Hyprland.
![image](https://user-images.githubusercontent.com/54314240/197680577-2bc0cff4-2438-4c8d-8428-11499d0519c6.png)

The bottom bar is also made with Hybrid.

## What does it support?
It supports:
- Easy Configuration;
  - Your grandma could probably get it setup too
- Split Configuration;
  - Setup dedicated configurations/stylesheets for each individual Hybrid session. Read more at `DUAL-CONFIG.md`!
- Labels;
  - `static`, `dynamic` and `dynamic listen`
     - Static being without a command set, cheapest one available and doesn't push redraws on its own
     - Dynamic firing specified bash-command every few milliseconds (from `hybrid` -> `update_rate`), gets the output and syncs it with a redraw
     - Dynamic Listen listens to a commands output (for example, `ping`), then syncs it at the same interval as Dynamic.
       - **WARNING**: Dynamic Listen if fed a slow command, may cause low performance
       - Cava Widgets use this very specific type, but with minor changes to make it update faster
- Spacings;
  - Which may also act as "Separators" if you customize them
- Boxes;
  - Draw a horizontal box and style it, or just treat it as an invisible cousin of Spacings (which is basically the default behavior)
- Custom update-rate for dynamic labels;
  - On lower-end computers this may save some processing power
- Cava embedded on your bar;
  - Yes, that Cava from unixporn
  - You may also customize the back-end framerate, bar-count and sed to be used
  - As an added bonus: If Cava unexpectedly closes/crashes, the back-end loops for updating it will be cancelled, leading to a happier CPU
  - **NOTE**: You may only have `8` Cava widgets active per Hybrid session
- Buttons;
  - Before you ask: yes, they can execute bash-commands
- Tooltips;
  - Supported for Buttons and Labels
  - No, they aren't shy and won't hide behind your windows
  - And they also support commands attached to them through `tooltip_command`, starting from version `0.3.6` and higher!
  - **NOTE**: `tooltip_command` uses a fixed update-rate of `1s`, this value cannot be changed by the user.
- Command Fields;
  - Write in your favorite command, hit `ENTER` and it'll spawn for you
  - **NOTE**: You have to enable `hybrid` -> `allow_keyboard` (`bool`) to focus command fields
- Markup;
  - Supported for Buttons and the `text` property on Labels
- Not only attached to the top;
  - You can choose between 2 places for where Hybrid should be placed; Top or Bottom
- Sizing;
  - Customize how the bar should expand, read more in `FAQ` -> `Can I split the bar into sections, like with Waybar and/or eww?`
- Transparency;
  - Plus Blur if your compositor allows for blurring layer-shells
- Efficient;
  - If there isn't a need for redrawing the bar; it just won't happen, simple as that
- Does what it's supposed to;
  - While also being easy to use, beginner-friendly and straight-forward.
- Always updated;
  - You can check the commits to see proof of this
  - If there hasn't been any commits for over 2 weeks, then I have either died from caffeine or I ran out of ideas
- Compatible with different monitors;
  - Specify the monitor to be used for each Hybrid config, read more in `FAQ`

## I have no config
If the AUR version for whatever reason didn't give you the example one, copy the example from `examples/config.json` into `~/.config/HybridBar/`.
## Does it only work on wlroots Compositors?
Nope, it's been tested on KDE as well and it worked just fine. GNOME should be the same story.
# Config Layout
I'm assuming you are familiar with JSON. If you aren't, well too bad.
## Base
Before you can use the bar, you have to adjust the color and alpha.

RGB Colors are 0-255 as a 32-bit integer, Alpha is 0.0-1.0 as a 32-bit float.

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
## CSS Support
Starting from `0.1.3`, CSS is now supported and you can make it auto-load on startup by making a `style.css` file next to your `config.json` at the same path.

If you want a sample CSS which has good defaults, check `examples/style.css`.
## Environment Variables
`HYBRID_LOG` = `0` OR `1` : Logs debug output to stdout.

`HYBRID_CONFIG` = `name.json` : Locates the config inside the HybridBar config path, then uses it for the rest of the bars session.
## Background Updates
Hybrid performs background updates for labels with the key `command` set, and for updating Cava labels.

The rate for updating labels is read from `hybrid`, `update_rate` (i32).

It's worth noting that low update-rates may lead to performance decreases, the value specified is in **milliseconds**.

**NOTE**: Cava-widgets does __not__ respect the `update_rate` specified, this is to ensure better precision for the animations.
# Installation
Dependencies:

1. rust
2. gtk-layer-shell
3. gtk3
4. bash
5. a brain

## Arch Linux
Git Version: `paru -S hybrid-bar-git`

**NOTE**: Git may include changes that are experimental. It's heavily advised that you use the package listed below if you care about stability.

Latest Binary: `paru -S hybrid-bar`
## Building
1. `git clone https://github.com/vars1ty/HybridBar`
2. `cd HybridBar`
3. `cargo build --release`
4. `cd target/release`
5. Done, the executable is called `hybrid-bar`.

**TIP**: `chmod +x hybrid-bar` - So you can run the executable directly.
# Roadmap
- ~~Make the code for widgets cleaner and more portable~~ - **Done**
- ~~Quit blocking the UI Thread when executing bash-commands and retrieving the output~~ - **Done**
- Port over to GTK4 - Not possible right now due to GTK being 0IQ and [screwing shit up](https://github.com/wmww/gtk-layer-shell/issues/37)
- ~~Publish a non-git AUR package which uses the latest built binary~~ - **Done**
- Potentially more widgets - **In progress, feel free to suggest widgets**
- System Tray - **Considering**
- Placing widgets into user-defined `box` widgets, allowing for further customization - **Considering**
