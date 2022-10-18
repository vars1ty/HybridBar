# Widgets
Available widgets:

Labels:
- left-label: Left-aligned label
- centered-label: Centered label
- right-label: Right-aligned label

Keys Supported:
- text: String
- command: String
***
Buttons:
- left-button: Left-aligned button
- centered-button: Centered button
- right-button: Right-aligned button

Keys Supported:
- text: String
- command: String
***
Spacing:
- left-spacing: Left-focused spacing
- centered-spacing: Centered-focused spacing
- right-spacing: Right-focused spacing

Keys Supported:
- spacing_start: i32
- spacing_end: i32
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
