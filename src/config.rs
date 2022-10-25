use crate::environment;
use json::JsonValue;
use std::fs;

/// Gets the root home path to Hybrid.
pub fn get_path() -> String {
    format!(
        "/home/{}/.config/HybridBar/",
        execute!(&String::from("whoami"))
    )
}

/// Parses the config and returns it.
pub fn read_config() -> JsonValue {
    let mut conf_path = get_path();
    conf_path.push_str(&environment::try_get_var("HYBRID_CONFIG", "config.json"));
    json::parse(
        &fs::read_to_string(&conf_path)
            .unwrap_or_else(|_| panic!("[ERROR] Failed reading config file from '{conf_path}'!\n")),
    )
    .unwrap_or_else(|_| panic!("[ERROR] Failed parsing config from '{conf_path}'!\n"))
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
                    .unwrap_or_else(|| panic!("[ERROR] Failed parsing {root}:{key} as i32!\n")),
            );
        }

        (cfg[key].to_string(), 0)
    } else {
        (String::default(), 0)
    }
}

/// Gets all the custom variables.
pub fn get_custom_variables() -> Vec<(String, String)> {
    let cfg = &read_config()["variables"];
    let mut vector: Vec<(String, String)> = Vec::new();
    for entry in cfg.entries() {
        vector.push((entry.0.to_string(), entry.1.to_string()))
    }

    vector
}

/// Replaces any variable-matching patterns in the `String` with the variables value.
pub fn with_variables(input: String) -> String {
    let mut result = input;
    for variable in get_custom_variables() {
        result = result.replace(&variable.0, &variable.1);
    }

    result
}
