use std::env;

/// Tries to get the value from a specific environment variable.
pub fn try_get_var(variable: &str, fallback_value: &str) -> String {
    let this = env::var(variable);
    match this {
        Ok(t) => t,
        Err(_) => {
            println!("[HYBRID] [WARN] Unassigned environment variable '{variable}', using fallback '{fallback_value}'.");
            String::from(fallback_value)
        }
    }
}
