# Hybrid Bar
A Wayland status bar made in Rust
## What does it support?
It supports:
- Basic labels;
- Labels with output from a bash command;
- Transparency

A.K.A It's a simple status bar I put together with gtk-layer-shell and GTK 3 because I couldn't be bothered with more weird, half-broken bars.
## Does it support buttons?
No, if you want buttons support then you either wait for me to add it (a.k.a when I need it), or you can submit a PR and add it yourself.

**Make sure all PRs follow the current code style**.
## I have no config
You have to make one yourself.

```sh
mkdir -p ~/.config/HybridBar
cd ~/.config/HybridBar
touch config.json
```

Now edit the config in your favorite text-editor (better be NeoVim) and begin configuring it.
# Config Layout
I'm assuming you are familiar with JSON, if you aren't, well too bad.
## Base
Before you can use the bar, you have to adjust the color and alpha.

Here's an example:

```json
{
    "background": {
        "r": 10,
        "g": 10,
        "b": 10,
        "a": 0.5
    }
}
```
## Components
Available "components":
- label: Left-aligned label
- centered-label: Centered label
- right-label: Right-aligned label

To actually use a component, here's an example:

```json
"label_UNIQUE_NAME": {
        "text": "whomai stdout ",
        "command": "whoami"
    }
```

Every component **has** to end with `_`, then you add the unique name for that component.

The `text` and `command` nested JSON keys are simply described as:
- text: Raw Label Text
- command: Optional bash command

Before you ask; yes, they are both required but you don't have to specify a value to them.

No, the unique name isn't actually displayed anywhere, it's just to be able to differ each component from another.
## Example Config
Made for [Hyprland](https://github.com/hyprwm/Hyprland).

```json
{
    "background": {
        "r": 10,
        "g": 10,
        "b": 10,
        "a": 0.5
    },
    "label_username": {
        "text": "user: ",
        "command": "whoami"
    },
    "label_current_workspace": {
        "text": " | workspace: ",
        "command": "hyprctl monitors -j | jq -r \".[].activeWorkspace.id\""
    },
    "label_max_workspaces": {
        "text": "/10",
        "command": ""
    },
    "centered-label_active_window": {
        "text": "",
        "command": "hyprctl activewindow -j | jq -r \".title\""
    },
    "right-label_volume": {
        "text": " | vol: ",
        "command": "echo $(pactl get-sink-volume @DEFAULT_SINK@ | rg -o '[0-9]{1,3}%' | head -n 1 | cut -d '%' -f 1)%"
    },
    "right-label_time": {
        "text": " | time: ",
        "command": "date +%H:%M" 
    }
}
```
# Building
Make sure you have Rust installed before.
***
1. `git clone https://github.com/dev11n/HybridBar`
2. `cd HybridBar`
3. `cargo build --release`
4. `cd target/release`
5. Run the `hybrid_bar` executable.
