use std::env;

/// Tries to get the value from a specific environment variable.
pub fn try_get_var(variable: &str, fallback_value: &str) -> String {
    env::var(variable).unwrap_or(String::from(fallback_value))
}
