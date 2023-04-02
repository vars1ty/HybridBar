# Scripting
> **Warning**: Scripting is experimental and locked behind the `experimental_rune_vm` feature.

## Format
`function_name(parameters) [Return Type]`

## Functions
### Core
- `Hybrid::execute(&str) [String]` -> Executes the specified shell-command and returns it.
- `Hybrid::log(&str) [()]` -> Prints the specified message to stdout, assuming `HYBRID_LOG` is set to `1`.
- `Hybrid::is_feature_active(&str) [bool]` -> Checks if the specified feature is active, then returns `true`/`false`.
- `Hybrid::use_aliases(&str) [String]` -> Checks for aliases in the given content, then replaces it with their real values.

### Feature-dependent
> **Warning**: These functions require that the associated feature is enabled, otherwise it won't work.

- `Hyprland::get_current_workspace() [i32]` -> Returns the currently active workspace.
- `Hyprland::get_current_window() [String]` -> Returns the focused window.

### Called Functions
These functions are called automatically by Hybrid internally if found.

- `main() [()]` -> Main function, called once on Hybrid startup.
- `tick() [()]` -> Called every 250 milliseconds*.
  - * Unless overridden by the `UPDATE_RATE` constant.

### Builder
- `Builder::add_label(name [&str], content [&str], alignment [&str]) [()]` -> Adds a new label widget.
- `Builder::add_button(name [&str], content [&str], alignment [&str]) [()]` -> Adds a new button widget.
- `Builder::set_label_text(name [&str], content [&str]) [()]` -> Changes the text content of a label.
- `Builder::set_button_text(name [&str], content [&str]) [()]` -> Changes the text content of a button.
- `Builder::set_button_command(name [&str], shell_command [&str]) [()]` -> Changes the shell-command to be executed upon pressing the button.
- `Builder::set_tooltip(name [&str], content [&str], markup [bool]) [()]` -> Changes the tooltip content of a widget.
- `Builder::set_visible(name [&str], visible [bool]) -> [()]` -> Changes the labels visibility status.
- `Builder::is_visible(name [&str]) -> [bool]` -> Checks whether or not the specified widget is visible.

## Example
```rust
pub const UPDATE_RATE = 5000; // 5 seconds, default is 250 milliseconds.

pub fn main() {
    Hybrid::log("Hello, script loaded!");
}

pub fn tick() {
    let date = Hybrid::execute("date");
    Hybrid::log(`Ticking! Date is: ${date}`);
}
```

## Constants
> **Warning**: Modifying these constants may impact performance and overall stability, you have been warned!

- `UPDATE_RATE [u64]` -> The update-rate (in milliseconds) for how often Hybrid should call the `tick` function.

# Warning
The `alignment` key is case-sensitive can only be 1 out of 3 values:
- left
- centered
- right
