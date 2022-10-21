use crate::environment;
use std::fmt::*;

/// Logs a message to the terminal if the environment variable `HYBRID_LOG` is set to `1`.
/// This accepts any type, as long as it implements one of `Display`, `Clone` or `Debug`.
pub fn log(msg: impl Display + Clone + Debug) {
    if environment::try_get_var("HYBRID_LOG", "0") == "1" {
        println!("[HYBRID] [DEBUG] {msg}");
    }
}
