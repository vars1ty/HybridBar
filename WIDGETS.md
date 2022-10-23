# Widgets
> **Warning**:
> Widgets with a command set are updated every few milliseconds from the value at `hybrid:update_rate`.
>
> An unspecified value (or one below `5`) will default to `5`, meaning it calls bash-commands every 5 milliseconds.
>
> This can be performance intensive, so it's recommended that you set the update-rate to something like `100`.
***
**NOTE**: In order to use a widget, you'll have to specify how it should be aligned. For example: `left-button`, `centered-button` and `right-button`.

Available widgets:

`label`:

Keys Supported:
- text: String
- command: String
- tooltip: String
***
`button`:

Keys Supported:
- text: String
- command: String
- tooltip: String
***
`spacing`:

Keys Supported:
- spacing_start: i32
- spacing_end: i32
***
`box`:

Keys Supported:
- width: i32
***
`cava` - **BETA**:

Keys Supported:
- Shared: `hybrid` -> `cava_sed`: String
- Shared: `hybrid` -> `cava_bars`: i32
- Shared: `hybrid` -> `cava_framerate`: i32
***
To actually use a widget, here's an example:

```json
"left-label_UNIQUE_NAME": {
        "text": "whomai stdout ",
        "command": "whoami"
    }
```

Every widget **has** to contain an underscore (`_`) after the type, then you add the unique name for that widget.

The `text` and `command` nested JSON keys are simply described as:
- text: Raw Label Text
- command: Optional bash command

**All** keys are optional, if you skip `text` for example, it'll be using an empty value.

No, the unique name isn't actually displayed anywhere, it's just to be able to differ each component from another.
## Cava
Since `0.2.5`, unofficial Cava support has been added.

Here's an example of how you may setup Cava: `"right-cava_cava_rside": {}`.

You may also change how all Cava widgets are displayed on the bar through these keys in `hybrid`:
- `cava_sed`: String - The sed for Cava. If left empty, `s/;//g;s/0/▁/g;s/1/▂/g;s/2/▃/g;s/3/▄/g;s/4/▅/g;s/5/▆/g;s/6/▇/g;s/7/█/g;` is being used;
- `cava_framerate`: i32 (min 60, max 200) - How fast Cava should check for audio levels and output it to `stdout` for Hybrid to then sync to the viewport;
- `cava_bars`: i32 (min 2, max 16) - How many bars that should be rendered for each Cava widget
### Performance
Because the implementation isn't perfect and uses what I'd like to call for "hacks" to work, the performance may fluctuate.

On mid/high-end systems this should really not even be noticeable, going from `~0.0%` CPU-Usage without Cava, to `~0.4%` with Cava.

If you don't want the very small performance impact, simply don't use Cava. Or if your bar is already active and you want to disable Cava; `killall -I cava -9` - This will kill Cava and disable its functionality from Hybrid until you restart the bar.

#### What if Cava crashes unexpectedly? Do I still lose performance?
No, if Cava crashes unexpectedly then Hybrid will effectively cut off the module entirely and all of its update-loops, making the performance 1:1 to what it would be if you weren't using Cava.
