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

    /// Custom user-defined variables.
    variables: HashMap<String, String>,

    /// Explicitly enabled/disabled features.
    features: HashMap<String, bool>,
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
            .unwrap_or(100)
            .clamp(5, 10_000);

        let variables = Self::get_custom_variables_raw(&config_data);
        let features = Self::get_features_raw(&config_data);

        Self {
            path: Box::leak(Box::new(path)),
            config_data,
            update_rate,
            variables,
            features,
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

    /// Returns the config as a `JsonValue`.
    pub fn read_config_raw(&self) -> &JsonValue {
        &self.config_data
    }

    /// Gets all the user-defined variables without using the cache.
    fn get_custom_variables_raw(config_data: &JsonValue) -> HashMap<String, String> {
        let cfg = &config_data[HYBRID_V_ROOT_JSON];
        let mut map = HashMap::new();
        for entry in cfg.entries() {
            map.insert(entry.0.to_owned(), entry.1.to_string());
        }

        map
    }

    /// Gets the features without using the cache.
    fn get_features_raw(config_data: &JsonValue) -> HashMap<String, bool> {
        let cfg = &config_data[HYBRID_ROOT_JSON][HYBRID_F_ROOT_JSON];
        let mut map = HashMap::new();
        for entry in cfg.members() {
            map.insert(entry.to_string(), true);
        }

        map
    }

    /// Gets all the user-defined variables.
    pub fn get_custom_variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    /// Checks if the given feature is active or not.
    pub fn is_feature_active(&self, feature: &str) -> bool {
        *self.features.get(feature).unwrap_or(&false)
    }

    /// Replaces any variable-matching patterns in the `String` with the variables value.
    pub fn with_variables(&self, input: String) -> String {
        let mut input = input;
        let custom_variables = self.get_custom_variables();
        for variable in custom_variables {
            input = input.replace(variable.0, variable.1);
        }

        input
    }
}
