# FAQ
- Can I hot-reload my changes?
  - No, there's no built-in support for this.

- Are PRs accepted?
  - Yeah, as long as they follow the existing code structure and don't introduce breaking changes without a really good reason as to why it's needed.

- Can I create custom widget types?
  - You can create custom-ish types by utilizing scripting.

- How do I move widgets up and down?
  - You can't move them up/down through the actual config, although you can somewhat do it via CSS.

- Can I split the bar into sections, like with Waybar and/or eww?
  - **Available since 0.3.4**
  - Yes you can specify where the bar should be located at.
  - Use `hybrid` -> `expand_left` / `expand_right` (`bool`) and mess around with them until you find what you like.
  - Default values are `true` for both; a.k.a expanded across your monitor.
  - > **Note**: Aligning your bar to one side only may result in weird bugs, such as it not allocating a region of free space below it.

- Does this only act as a status bar?
  - It's intended purpose is to be a status bar, but you can turn it into a smaller application shortcut launcher through scripts and buttons.

- Hybrid crashed for me
  - Send in a bug report with the logs and your configuration (JSON + CSS).

- Why is markup only supported on static labels?
  - Because if you for say, focus a window with some special characters and it's being drawn in markup, it'll freak out. I might be able to fix it later on, but for now it's not happening.

- I want to add widgets at runtime
  - **Available since 0.5.0**
  - Read `SCRIPTING.md`.

- I'm tired of repeating long commands in the JSON, can variables be added?
  - **Available since 0.2.9**
  - Read `VARIABLES.md`.

- Does Hybrid work outside Sway and Hyprland?
  - Yes, it's been proven to work on KDE and Sway.

- Does Hybrid work on X11?
  - Which part of **Wayland status bar** did you not understand?

- The config is too hard for me to understand
  - Then you either lack a functional brain, or you are the definition of lazy.

- Can I make the bar not take up my entire top/bottom part of the screen?
  - To an extent yeah, read `Can I split the bar into sections, like with Waybar and/or eww?`.

- Is there support for distros other than Arch?
  - Hybrid should work on every distro, only difference being that I won't officially ship it to anything but the AUR Repo and on GitHub Releases.
  - If you however decide to package it and want to ship it yourself, go ahead and submit an issue which holds the URL to the package, if it's git or not, etc.

- I want to move Hybrid to a different monitor
  - **Available since 0.3.2**
  - You can specify your monitor via `hybrid` -> `monitor` (`i32`)

- Can I change the namespace for blurring Hybrid Bar on Hyprland?
  - **Available since 0.3.9**
  - Yes you can thanks to this PR: https://github.com/vars1ty/HybridBar/pull/27, option: `hybrid` -> `namespace` (`String`)

- How do I enable optional features/experiments?
  - **Available since 0.4.9**
  - Create a key named `features` in `hybrid`, then specify what features you want. For example:
  - `"features": ["tray_experimental", "hyprland]`

- Why not GTK4?
  - Because it removed support for transparent windows (unless you make everything transparent).
  - Plus I don't see any benefit of using GTK4 over GTK3, when it doesn't have anything that's really needed.
