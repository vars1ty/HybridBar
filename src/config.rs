use crate::{constants::*, environment, math, structures::ConfigData};
use heapless::Vec;
use json::JsonValue;
use lxinfo::info::{self, SystemInfo};
use std::{fs, sync::RwLock};

lazy_static! {
    /// Cached config.
    pub static ref CONFIG: RwLock<JsonValue> = RwLock::new(JsonValue::Null);
    /// Cached system information.
    pub static ref SYSINFO: Option<SystemInfo> = info::get_system_information();
}

/// Gets the root home path to Hybrid.
pub fn get_path() -> String {
    let username = if let Some(info) = info::get_system_information() {
        info.username
    } else {
        // lxinfo isn't available, fallback to execute.
        execute!("whoami")
    };

    format!("/home/{}/.config/HybridBar/", username)
}

/// Returns the set update-rate.
pub fn get_update_rate() -> u64 {
    let mut update_rate = 100;
    if let Some(c_update_rate) = conf!(HYBRID_ROOT_JSON, "update_rate", false, false).number {
        update_rate = math::clamp_i32(c_update_rate, 5, 10_000)
    }

    update_rate
        .try_into()
        .expect("[ERROR] Cannot convert update_rate into u64!")
}

/// Caches the config so we don't have to re-parse it every time.
/// Works as a fix for issue #13
pub fn cache() {
    *CONFIG.write().unwrap() = read_config_raw();
}

/// Parses and returns the config.
fn read_config_raw() -> JsonValue {
    let mut conf_path = get_path();
    conf_path.push_str(&environment::try_get_var("HYBRID_CONFIG", DEFAULT_CONFIG));
    json::parse(
        &fs::read_to_string(&conf_path)
            .unwrap_or_else(|_| panic!("[ERROR] Failed reading config file from '{conf_path}'!")),
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
fn get_custom_variables() -> Vec<(String, String), 64> {
    let cfg = &CONFIG.read().unwrap()[HYBRID_V_ROOT_JSON];
    // 0.3.0: Only allow for 64 variables.
    let mut vector: Vec<(String, String), 64> = Vec::new();
    for entry in cfg.entries() {
        vector
            .push((entry.0.to_owned(), entry.1.to_string()))
            .expect("[ERROR] You cannot have more than `64` variables!");
    }

    vector
}

/// Replaces any variable-matching patterns in the `String` with the variables value.
fn with_variables(input: String) -> String {
    let mut input = input;
    for variable in get_custom_variables() {
        // Only replace if `result` actually contains the defined variable.
        if input.contains(&variable.0) {
            input = input.replace(&variable.0, &variable.1);
        }
    }

    input
}
