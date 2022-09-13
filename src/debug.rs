use std::fmt::*;

use crate::environment;

/// Logs a message to the terminal if the environment variable `HYBRID_LOG` is set to `1`.
pub fn debug_log<T>(msg: T)
where
    T: Display + Clone + Debug,
{
    if environment::try_get_var("HYBRID_LOG") == "1" {
        println!("[DEBUG] {msg}");
    }
}
