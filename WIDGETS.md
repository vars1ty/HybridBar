# Widgets
> **Note**
> If you add widgets that aren't attached to a `box` widget, you have to specify how the widget should be placed.
>
> For example: `left-label_UNIQUE_NAME`
>
> For child widgets inside of boxes, you can simply define the widget as `label_UNIQUE_NAME`.

Available widgets:

`label`:

Keys Supported:
- text: String
   - Constant Text string
- command: String
   - Command to be executed and appended to the back of the text string
- update_rate: u64
   - How often the command should be called if `listen` isn't enabled
- tooltip: String
   - Constant Tooltip string
- tooltip_command: String
   - Command to be executed and appended to the back of the tooltip string
- listen: bool
   - Should Hybrid listen to the commands output, rather than calling it once and moving on?
- update_anim: String
   - **Dynamic Labels only**: What animation should be played when the content has been updated?
   - Values: `none`, `crossfade` (Default), `slide_left` and `slide_right`
- anim_speed: u32
  - **Dynamic Labels only**: How fast (in milliseconds) should the update animation last? Default is `250`.
***
`button`:

Keys Supported:
- text: String
   - Constant Text string
- command: String
   - Command to be executed when pressing on the button
- tooltip: String
   - Constant Tooltip string
- tooltip_command: String
   - Command to be executed and appended to the back of the tooltip string
***
`spacing`:

Keys Supported:
- spacing_start: i32
   - Start spacing
- spacing_end: i32
   - End spacing
***
`box`:

Keys Supported:
- width: i32
   - Box custom requested width
- widgets: JSON
   - Embedded child widgets
***
`cava`:

Keys Supported:
- Shared: `hybrid` -> `cava_update_rate`: u64
   - How often the new Cava output should be displayed, in milliseconds
- Shared: `hybrid` -> `cava_sed`: String
   - SED to be used for translating the raw stdout into custom content
- Shared: `hybrid` -> `cava_bars`: i32
   - How many internal Cava bars that should be outputted
- Shared: `hybrid` -> `cava_framerate`: i32
   - Internal Cava framerate
***
`tray`:

> **Warning**: Experimental Widget, expect issues.

Keys Supported:
- None
***
To actually use a widget, here's an example:

```json
"left-label_UNIQUE_NAME": {
    "text": "whomai stdout ",
    "command": "whoami",
    "?": "The update-rate below is set to 1 second in milliseconds.",
    "update_rate": 1000
}
```

Every widget **has** to contain an underscore (`_`) after the type, then you add the unique name for that widget.

If you don't specify a name for the widget after the underscore, the bar will crash and say that discarded names aren't supported.

**NOTE**: Widgets with the same name (regardless of type) aren't officially supported and may suffer from weird behavior.

The `text` and `command` nested JSON keys are simply described as:
- text: Raw Label Text
- command: Optional bash command

**All** keys are optional, if you skip `text` for example, it'll be using an empty value.

No, the unique name isn't actually displayed anywhere, it's just to be able to differ each component from another.

## Cava
Here's an example of how you may setup Cava: `"right-cava_0": {}`.

You may also change how all Cava widgets are displayed on the bar through these keys in `hybrid`:
- `cava_sed`: String - The sed for Cava. If left empty, `s/;//g;s/0/▁/g;s/1/▂/g;s/2/▃/g;s/3/▄/g;s/4/▅/g;s/5/▆/g;s/6/▇/g;s/7/█/g;` will be used;
- `cava_framerate`: u32 (min 60, max 360) - How fast Cava should check for audio levels and output it to `stdout` for Hybrid to then sync to the viewport;
- `cava_bars`: u32 (min 2, max 16) - How many bars that should be rendered for each Cava widget;
- `cava_update_rate`: u64 (min 1, default 1) - How often (in milliseconds) Hybrid should check for back-end Cava updates, parse and then display it.

### Performance
Because the implementation isn't perfect and relies on listening to a raw Cava stdout, the performance may fluctuate.

On mid/high-end systems this should really not even be noticeable, going from `~0.0%` CPU-Usage without Cava, to `~0.4%` with Cava.

If you don't want the very small performance impact, simply don't use Cava. Or if your bar is already active and you want to disable Cava; `killall -I cava -9` - This will kill Cava and disable its functionality from Hybrid until you restart the bar.

### What if Cava crashes unexpectedly?
If Cava crashes (or closes) unexpectedly then Hybrid will effectively cut off the module entirely and all of its update-loops, allowing the session to keep on running.
