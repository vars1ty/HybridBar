# Experimental Features
> **Warning**: Experimental features are under development. They may break, change or be removed entirely.

1. Widgets inside boxes
   - Introduced in version `0.4.3`
   - New key inside the `box` widget: `widgets`
   - Allows for adding child widgets to boxes, which allows for better styling through CSS.
```json
{
   "left-box_my_box": {
      "widgets": {
         "label_experimental": {
            "text": "Experimental"
         }
      }
   }
}
```
2. System Tray
   - Introduced in version `0.4.4`, marked as experimental in version `0.4.6`
   - New widget: `tray` - using the `stray` crate
   - Not likely to become available as a stable feature anytime soon, because `stray` is overall hacky and a mess.
```json
{
   "left-tray_tray": {}
}
```
