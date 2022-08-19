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
            .expect(format!("[ERROR] Failed reading config file from '{b_path}'!\n").as_str()),
    )
    .expect("[ERROR] Failed parsing config!\n")
}

/// If the `key` exists inside `root`, the value of it is returned.
/// If not, an empty value is instead returned.
pub fn try_get_string(root: &str, key: &str) -> String {
    let cfg = &read_config()[root];
    if cfg.has_key(key) {
        cfg[key].to_string()
    } else {
        String::from("")
    }
}

/// If the `key` exists inside `root`, the value of it is returned.
/// If not, `0` is instead returned.
pub fn try_get_i32(root: &str, key: &str) -> i32 {
    let cfg = &read_config()[root];
    if cfg.has_key(key) {
        cfg[key]
            .as_i32()
            .expect("[ERROR] Failed returning value as i32!\n")
    } else {
        0
    }
}
