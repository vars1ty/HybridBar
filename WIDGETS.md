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
Since `0.2.5`, unofficial Cava support has been added, although with limitations such as:

- There are no keys available for each Cava widget, there's only one inside `hybrid` which is called `cava_sed`. If left empty, it'll be using this value: `s/;//g;s/0/▁/g;s/1/▂/g;s/2/▃/g;s/3/▄/g;s/4/▅/g;s/5/▆/g;s/6/▇/g;s/7/█/g;`.
- The implementation is **unsafe**, hence why it's in beta. Expect full-on crashes.
- You can't currently change how many bars you want for Cava in an official way, it may be added in the future though.

*If you compile from source and modify `cava.rs`, then you may modify the configuration.*

Here's an example of how you may setup Cava: `"right-cava_cava_rside": {}`
