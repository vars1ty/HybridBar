use crate::{constants::*, structures::ConfigData, utils::environment};
use json::JsonValue;
use std::{
    collections::HashMap,
    fs,
    sync::{RwLock, RwLockReadGuard},
};

lazy_static! {
    /// Cached config.
    pub static ref CONFIG: RwLock<JsonValue> = RwLock::new(read_config_raw());
}

/// Gets the root home path to Hybrid.
pub fn get_path() -> String {
    format!(
        "{}/.config/HybridBar/",
        std::env::var("HOME").unwrap_or_else(|_| execute!("whoami"))
    )
}

/// Returns the set update-rate.
pub fn get_update_rate() -> u64 {
    let update_rate = conf!(HYBRID_ROOT_JSON, "update_rate", false, false)
        .number
        .unwrap_or_else(|| 100)
        .clamp(5, 10_000);

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
    .unwrap_or_else(|error| panic!("[ERROR] Error parsing config: {error}"))
}

/// Tries to fetch a value from the config. Supported types are `String` and `i32`.
pub fn try_get(root: &str, key: &str, is_string: bool, with_custom_variables: bool) -> ConfigData {
    let cfg = &get_config()[root];
    if cfg.has_key(key) {
        let grabbed_value = &cfg[key];

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
            ConfigData::new(
                Some(with_variables(
                    grabbed_value.to_string(),
                    &get_custom_variables(),
                )),
                None,
            )
        } else {
            ConfigData::new(Some(grabbed_value.to_string()), None)
        }
    } else {
        // The key wasn't found, so just return None on all values.
        ConfigData::default()
    }
}

/// Returns the entire config.
pub fn get_config<'a>() -> RwLockReadGuard<'a, JsonValue> {
    CONFIG.read().expect(ERR_ACCESS_CONFIG)
}

/// Gets all the custom variables.
pub fn get_custom_variables() -> HashMap<String, String> {
    let cfg = &get_config()[HYBRID_V_ROOT_JSON];
    let mut map: HashMap<String, String> = HashMap::new();
    for entry in cfg.entries() {
        map.insert(entry.0.to_owned(), entry.1.to_string());
    }

    map
}

/// Replaces any variable-matching patterns in the `String` with the variables value.
pub fn with_variables(input: String, custom_variables: &HashMap<String, String>) -> String {
    let mut input = input;
    for variable in custom_variables {
        // Only replace if `result` actually contains the defined variable.
        if input.contains(variable.0) {
            input = input.replace(variable.0, variable.1);
        }
    }

    input
}
