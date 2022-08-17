use crate::proc;
use json::JsonValue;
use std::fs;

/// Parses the config and returns it.
pub fn read_config() -> JsonValue {
    let path = format!(
        "/home/{}/.config/HybridBar/config.json",
        proc::execute(String::from("whoami"))
    );
    let b_path = &path;
    json::parse(
        &fs::read_to_string(b_path)
            .expect(format!("[ERROR] Failed reading config file from \"{b_path}\"!\n").as_str()),
    )
    .expect("[ERROR] Failed parsing config!\n")
}
