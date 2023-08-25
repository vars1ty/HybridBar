# Features
In 0.4.9 and higher, you may enable additional features in Hybrid Bar, including experimental ones, if any are available.

The non-experimental features listed are usually disabled for a variety of reasons:
- Compatibility concerns
- It may start background tasks which the user might not want
- The feature is slow and is generally not recommended using

## Standard Features

1. System Information via lxinfo (`systemd`)
   - Allows for calling aliases such as `%username%` which utilize libc calls over commands, which are typically faster
   - This is disabled by default because it relies on files such as `/etc/os-release` to be present on your system.
2. Hyprland Support (`Hyprland`)
   - Adds the `%hl_workspace%` and `%hl_window%` aliases.
   - Workspace gets the current workspace id, whereas Window gets the focused window title.
