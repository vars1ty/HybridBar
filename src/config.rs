use crate::{environment, proc};
use json::JsonValue;
use std::fs;

/// Gets the root home path to Hybrid.
pub fn get_path() -> String {
    format!(
        "/home/{}/.config/HybridBar/",
        proc::execute(&String::from("whoami"))
    )
}

/// Gets the root home path to Hybrid.
pub fn get_path_blocking() -> String {
    format!(
        "/home/{}/.config/HybridBar/",
        proc::execute(&String::from("whoami"))
    )
}

/// Parses the config and returns it.
pub fn read_config() -> JsonValue {
    let mut conf_path = get_path();
    conf_path.push_str(&environment::try_get_var("HYBRID_CONFIG", "config.json"));
    json::parse(
        &fs::read_to_string(&conf_path)
            .expect(format!("[ERROR] Failed reading config file from '{conf_path}'!\n").as_str()),
    )
    .expect(format!("[ERROR] Failed parsing config from '{conf_path}'!\n").as_str())
}

/// If the `key` exists inside `root`, the value of it is returned.
/// If not, a default value is instead returned.
pub fn try_get(root: &str, key: &str, string_value: bool) -> (String, i32) {
    let cfg = &read_config()[root];
    if cfg.has_key(key) {
        if !string_value {
            return (
                String::default(),
                cfg[key]
                    .as_i32()
                    .expect(format!("[ERROR] Failed parsing {root}:{key} as i32!\n").as_str()),
            );
        }

        (cfg[key].to_string(), 0)
    } else {
        (String::default(), 0)
    }
}
