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

All variables have to be a child-key of `variables`, otherwise Hybrid won't be able to find it.

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

## Limitations
There's only two limitations with variables:
1. You can only specify up to `64`, although I don't expect you to exceed this limit for one single Hybrid session.
2. Tooltip commands do not currently support variables.
