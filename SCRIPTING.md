# Scripting
> **Warning**: Scripting is experimental and locked behind the `experimental_rune_vm` feature.

## Functions
### Core
- `execute(&str) [String]` -> Executes the specified shell-command and returns it.
- `log(&str) [()]` -> Prints the specified message to stdout, assuming `HYBRID_LOG` is set to `1`.
- `is_feature_active(&str) [bool]` -> Checks if the specified feature is active, then returns `true`/`false`.
- `use_aliases(&str) [String]` -> Checks for aliases in the given content, then replaces it with their real values.

### Called Functions
These functions are called automatically by Hybrid internally if found.

- `main() [()]` -> Main function, called once on Hybrid startup.
- `tick() [()]` -> Update loop, called every 250ms unless overridden.
- `get_update_rate() [u64]` -> Changes the update-rate for `tick` by returning the desired rate, 250 is the default.

### Builder
`Builder::add_label(name [&str], content [&str], alignment [&str]) [()]` -> Adds a new label widget. Note that the alignment has to be lowercase and be one out of:
- left
- centered
- right

- `Builder::set_label_text(name [&str], content [&str]) [()]` -> Changes the text content of a label.
- `Builder::set_label_visible(name [&str], visible [bool]) -> [()]` -> Changes the labels visibility status.

## Example
```rust
pub fn main() {
    log("Hello!");
}

pub fn tick() {
    let date = execute("date");
    log(`Ticking! Date is: ${date}`);
}

pub fn get_update_rate() {
    5000 // 5 seconds
}
```
