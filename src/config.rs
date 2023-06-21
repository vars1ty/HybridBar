use crate::{constants::*, utils::environment};
use json::JsonValue;
use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Config {
    /// Hybrid Config Directory path.
    path: &'static str,

    /// Raw config data.
    config_data: JsonValue,

    /// Update Frequency for dynamic widgets.
    update_rate: u64,
}

impl Config {
    pub fn init() -> Self {
        // Get the directory path.
        let path = format!(
            "{}/.config/HybridBar/",
            std::env::var("HOME").unwrap_or_else(|_| format!("/home/{}", execute!("whoami")))
        );

        // Read the config.
        let config_path = format!(
            "{path}{}",
            environment::try_get_var("HYBRID_CONFIG", DEFAULT_CONFIG)
        );
        let config_data = json::parse(
            &fs::read_to_string(config_path)
                .unwrap_or_else(|_| include_str!("../examples/config.json").to_owned()),
        )
        .expect(ERR_PARSE_CONFIG);

        // Get update-rate.
        let update_rate = config_data[HYBRID_ROOT_JSON]["update_rate"]
            .as_u64()
            .unwrap_or_else(|| 100)
            .clamp(5, 10_000);

        Self {
            path: Box::leak(Box::new(path)),
            config_data,
            update_rate: update_rate.try_into().expect(ERR_PARSE_UPDATE_RATE),
        }
    }

    /// Gets the root home path to Hybrid.
    pub fn get_path(&self) -> &str {
        self.path
    }

    /// Returns the set update-rate.
    pub fn get_update_rate(&self) -> u64 {
        self.update_rate
    }

    /// Returns the config.
    pub fn read_config_raw(&self) -> &JsonValue {
        &self.config_data
    }

    /// Gets all the custom variables.
    pub fn get_custom_variables(&self) -> HashMap<String, String> {
        // TODO(varsity): Cache the variables so there's no need to create a new HashMap<> and
        // iterate over all the entries every time.

        let cfg = &self.config_data[HYBRID_V_ROOT_JSON];
        let mut map: HashMap<String, String> = HashMap::new();
        for entry in cfg.entries() {
            map.insert(entry.0.to_owned(), entry.1.to_string());
        }

        map
    }

    /// Replaces any variable-matching patterns in the `String` with the variables value.
    pub fn with_variables(
        &self,
        input: String,
        custom_variables: &HashMap<String, String>,
    ) -> String {
        let mut input = input;
        for variable in custom_variables {
            // Only replace if `result` actually contains the defined variable.
            if input.contains(variable.0) {
                input = input.replace(variable.0, variable.1);
            }
        }

        input
    }
}
