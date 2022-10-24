# Dual-Configuration
If you want to make for say, a basic static taskbar at the bottom whilst still preserving your top-positioned config and stylesheet, you are able to do just that.

## How?
First, make a secondary configuration-file that you want to use for your bottom-bar, name it whatever. If you want to use separate stylesheets, you may make a new one as well.

In order to tell Hybrid **what** stylesheet to use for *(x)* configuration, put the full name of the stylesheet inside `hybrid` -> `stylesheet`, for example: `"stylesheet": "bottom.css"`.

To actually launch Hybrid and pin it to the bottom with a separate configuration-file, follow these simple steps:
1. Open the config you want to use
2. At `hybrid`, set the `position` keys value to `Bottom`, non-case sensitive
3. Save the config and then run `HYBRID_CONFIG=(name_of_config.json) hybrid-bar` and done!
