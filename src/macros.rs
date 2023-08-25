#[macro_export]
/// Logs a [HYBRID] [DEBUG] formatted message to stdout.
macro_rules! log {
    ($msg:expr) => {
        if $crate::utils::environment::try_get_var("HYBRID_LOG", "0") == "1" {
            println!("[LOG]: {}", $msg)
        }
    };
}
#[macro_export]
/// Executes a bash command and outputs it to `result`.
macro_rules! execute {
    ($cmd:expr) => {{
        let mut result = unsafe {
            String::from_utf8_unchecked(
                std::process::Command::new($crate::constants::PROC_TARGET)
                    .args(["-c", $cmd])
                    .output()
                    .unwrap()
                    .stdout,
            )
        };

        // Remove the last character as its a new line.
        result.pop();

        result
    }};
}
