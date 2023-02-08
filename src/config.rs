use crate::{constants::*, environment, math, structures::ConfigData};
use json::JsonValue;
use std::{collections::HashMap, fs, sync::RwLock};

lazy_static! {
    /// Cached config.
    pub static ref CONFIG: RwLock<JsonValue> = RwLock::new(read_config_raw());
}

/// Gets the root home path to Hybrid.
pub fn get_path() -> String {
    format!(
        "{}/.config/HybridBar/",
        std::env::var("HOME").unwrap_or(execute!("whoami"))
    )
}

/// Returns the set update-rate.
pub fn get_update_rate() -> u64 {
    let update_rate = math::clamp_i32(
        conf!(HYBRID_ROOT_JSON, "update_rate", false, false)
            .number
            .unwrap_or(100),
        5,
        10_000,
    );

    update_rate.try_into().expect(ERR_PARSE_UPDATE_RATE)
}

// Parses and returns the config.
fn read_config_raw() -> JsonValue {
    let mut conf_path = get_path();
    conf_path.push_str(&environment::try_get_var("HYBRID_CONFIG", DEFAULT_CONFIG));
    json::parse(
        &fs::read_to_string(&conf_path)
            // Don't panic if the file doesn't exist/couldn't be read. Instead use the example config.
            .unwrap_or_else(|_| include_str!("../examples/config.json").to_owned()),
    )
    .unwrap_or_else(|_| panic!("[ERROR] Failed parsing config from '{conf_path}'!"))
}

/// Tries to fetch a value from the config. Supported types are `String` and `i32`.
pub fn try_get(root: &str, key: &str, is_string: bool, with_custom_variables: bool) -> ConfigData {
    let config = &CONFIG.read().unwrap()[root];
    if config.has_key(key) {
        let grabbed_value = &config[key];

        // If the desired value isn't a string, try and get it as a 32-bit integer.
        if !is_string {
            return ConfigData::new(
                None,
                Some(
                    grabbed_value
                        .as_i32()
                        .unwrap_or_else(|| panic!("[ERROR] Failed parsing {root}:{key} as i32!")),
                ),
            );
        }

        // Convert it to a string-value.
        if with_custom_variables {
            ConfigData::new(Some(with_variables(grabbed_value.to_string())), None)
        } else {
            ConfigData::new(Some(grabbed_value.to_string()), None)
        }
    } else {
        // The key wasn't found, so just return None on all values.
        ConfigData::default()
    }
}

/// Gets all the custom variables.
pub fn get_custom_variables() -> HashMap<String, String> {
    let cfg = &CONFIG.read().unwrap()[HYBRID_V_ROOT_JSON];
    let mut map: HashMap<String, String> = HashMap::new();
    for entry in cfg.entries() {
        map.insert(entry.0.to_owned(), entry.1.to_string());
    }

    map
}

/// Replaces any variable-matching patterns in the `String` with the variables value.
pub fn with_variables(input: String) -> String {
    let mut input = input;
    for variable in get_custom_variables() {
        // Only replace if `result` actually contains the defined variable.
        if input.contains(&variable.0) {
            input = input.replace(&variable.0, &variable.1);
        }
    }

    input
}
