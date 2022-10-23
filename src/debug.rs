#[macro_export]
/// Logs a [HYBRID] [DEBUG] formatted message to stdout.
macro_rules! log {
    ($msg:expr) => {
        if crate::environment::try_get_var("HYBRID_LOG", "0") == "1" {
            println!("[HYBRID] [DEBUG] {}", $msg)
        }
    };
}
