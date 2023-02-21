/// Tries to get the value from a specific environment variable.
pub fn try_get_var(variable: &str, fallback_value: &str) -> String {
    std::env::var(variable).unwrap_or_else(|_| fallback_value.to_owned())
}
