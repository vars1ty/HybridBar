use std::env;
use std::fmt::*;

/// Logs a message to the terminal if the environment variable `HYBRID_LOG` is set to `1`.
pub fn debug_log<T>(msg: T)
where
    T: Display + Clone + Debug,
{
    if env!("HYBRID_LOG") == "1" {
        println!("[DEBUG] {}", msg);
    }
}
