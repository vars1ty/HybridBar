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
