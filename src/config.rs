use crate::{environment, math, structures::ConfigData};
use heapless::Vec;
use json::JsonValue;
use std::{fs, sync::RwLock};

lazy_static! {
    /// Caches the config.
    // TODO: Switch to Mutex sometime in the future.
    pub static ref CONFIG: RwLock<JsonValue> = RwLock::new(JsonValue::Null);
}

/// Gets the root home path to Hybrid.
pub fn get_path() -> String {
    format!(
        "/home/{}/.config/HybridBar/",
        execute!(&heapless::String::<6>::from("whoami"))
    )
}

/// Returns the set update-rate.
pub fn get_update_rate() -> u64 {
    let mut update_rate = 100;
    if let Some(c_update_rate) = try_get("hybrid", "update_rate", false, false).number {
        update_rate = math::clamp_i32(c_update_rate, 5, 10_000)
    }

    update_rate
        .try_into()
        .expect("[ERROR] Cannot convert update_rate into u64!\n")
}

/// Caches the config so we don't have to re-parse it every time.
/// Works as a fix for issue #13
pub fn cache() {
    *CONFIG.write().unwrap() = read_config_raw();
}

/// Parses and returns the config.
fn read_config_raw() -> JsonValue {
    let mut conf_path = get_path();
    conf_path.push_str(&environment::try_get_var("HYBRID_CONFIG", "config.json"));
    json::parse(
        &fs::read_to_string(&conf_path)
            .unwrap_or_else(|_| panic!("[ERROR] Failed reading config file from '{conf_path}'!\n")),
    )
    .unwrap_or_else(|_| panic!("[ERROR] Failed parsing config from '{conf_path}'!\n"))
}

/// Tries to fetch a value from the config. Supported types are `String` and `i32`.
pub fn try_get(root: &str, key: &str, is_string: bool, with_custom_variables: bool) -> ConfigData {
    let config = &CONFIG.read().unwrap()[root];
    if config.has_key(key) {
        let grabbed_value = &config[key];

        // If the desired value isn't a string, try and get it as a 32-bit integer.
        if !is_string {
            return ConfigData {
                string: None,
                number: Some(
                    grabbed_value
                        .as_i32()
                        .unwrap_or_else(|| panic!("[ERROR] Failed parsing {root}:{key} as i32!\n")),
                ),
            };
        }

        // Convert it to a string-value.
        if with_custom_variables {
            ConfigData {
                string: Some(with_variables(grabbed_value.to_string())),
                number: None,
            }
        } else {
            ConfigData {
                string: Some(grabbed_value.to_string()),
                number: None,
            }
        }
    } else {
        // The key wasn't found, so just return None.
        ConfigData::default()
    }
}

/// Gets all the custom variables.
fn get_custom_variables() -> Vec<(String, String), 64> {
    let cfg = &CONFIG.read().unwrap()["variables"];
    // 0.3.0: Only allow for 64 variables.
    let mut vector: Vec<(String, String), 64> = Vec::new();
    for entry in cfg.entries() {
        vector
            .push((entry.0.to_string(), entry.1.to_string()))
            .expect("[ERROR] You cannot have more than `64` variables!\n");
    }

    vector
}

/// Replaces any variable-matching patterns in the `String` with the variables value.
fn with_variables(input: String) -> String {
    let mut result = input;
    for variable in get_custom_variables() {
        // Only replace if `result` actually contains the defined variable.
        if result.contains(&variable.0) {
            result = result.replace(&variable.0, &variable.1);
        }
    }

    result
}
