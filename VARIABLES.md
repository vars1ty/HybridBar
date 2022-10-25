# Variables
This is only supported on Hybrid version 0.2.9 and higher.
***
## How do I make variables?
You can make variables by specifying them in a similar format as `hybrid`, here's an example:

```json
"variables": {
    "cool_message": "a very cool message indeed"
}
```

The `variables` naming for the master key is required, and no I'm not going to call it "main" key in this example.

## But.. How do I use my epic variable?
To actually use your variable, you can do something like this:

```json
"left-label_mvp": {
    "text": "cool_message"
}
```

Then the text will automatically be replaced with the value from `cool_message`. Variables are supported on these keys:
- `text`
- `command`
- `tooltip`
