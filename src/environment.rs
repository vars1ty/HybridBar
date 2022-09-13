use std::env;

/// Tries to get the value from a specific environment variable.
pub fn try_get_var(variable: &str) -> String {
    env::var(variable)
        .expect(format!("[ERROR] Failed finding the environment variable '{variable}'!").as_str())
}
