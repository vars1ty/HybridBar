# FAQ
- Can I hot-reload my changes?
  - No, there's no built-in support for this. What you can do though is make a button on your bar and set the command as: `killall -I hybrid-bar -9 && hybrid-bar` - Which kills the bar and starts it again, acting as hot-reload.

- Are PRs accepted?
  - Yeah, as long as they follow the existing code structure and don't introduce breaking changes without a really good reason as to why it's needed.

- Can I create custom widget types?
  - No you can't create full-on custom types.

- How do I move widgets up and down?
  - You can't move them up/down through the actual config, although you can somewhat do it via CSS.

- Can I split the bar into sections, like with Waybar and/or eww?
  - **Available since 0.3.4**
  - Yes you can specify where the bar should be located at.
  - Use `hybrid` -> `expand_left` / `expand_right` (`bool`) and mess around with them until you find what you like.
  - Default values are `true` for both; a.k.a expanded across your monitor.
  - **NOTE**: Aligning your bar to one side only may result in weird bugs, such as it not allocating a region of free space below it.

- Does this only act as a status bar?
  - It's intended purpose is to be a status bar, but you can turn it into a smaller application shortcut launcher through scripts and buttons.

- Hybrid crashed for me
  - Send in a bug report with the logs and your configuration (JSON + CSS).

- Why is markup only supported on static labels?
  - Because if you for say, focus a window with some special characters and it's being drawn in markup, it'll freak out. I might be able to fix it later on, but for now it's not happening.

- I want to add widgets at runtime
  - Not possible for now, what you can do instead as a workaround is to read the `hot-reload` part and implement that when you add new widgets through some script or whatever.

- I'm tired of repeating long commands in the JSON, can variables be added?
  - **Available since 0.2.9**
  - Read `VARIABLES.md`.

- Does Hybrid work outside Sway and Hyprland?
  - Yes, it's been proven to work on KDE and will most definitely work on GNOME.

- Does Hybrid work on X11?
  - Which part of **Wayland status bar** did you not understand?

- The config is too hard for me to understand
  - Then you either lack a functional brain, or you are the definition of lazy.

- Can I make the bar not take up my entire top/bottom part of the screen?
  - Yes, read `Can I split the bar into sections, like with Waybar and/or eww?`.

- Is there support for distros other than Arch?
  - Hybrid should work on every distro, only difference being that I won't officially ship it to anything but the AUR Repo and on GitHub Releases.
  - If you however decide to package it and want to ship it yourself, go ahead and submit an issue which holds the URL to the package, if it's git or not, etc.

- I can't focus my command field(s)!
  - **Available since 0.3.2**
  - Set `hybrid` -> `allow_keyboard` (`bool`) to `true`, restart Hybrid and it should work

- I want to move Hybrid to a different monitor
  - **Available since 0.3.2**
  - You can specify your monitor via `hybrid` -> `monitor` (`i32`)

- Can I change the namespace for blurring Hybrid Bar on Hyprland?
  - **Available since 0.3.9**
  - Yes you can thanks to this PR: https://github.com/vars1ty/HybridBar/pull/27, option: `hybrid` -> `namespace` (`String`)
